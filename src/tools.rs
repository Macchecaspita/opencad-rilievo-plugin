//! Interactive steps: picking rooms, the measures dialog, the measures-book
//! dialog and the guide. Interactive callbacks get no host access, so each
//! step ends by handing the host a one-word apply command that comes back
//! to [`crate::commands`] with full `HostApi` access.
//!
//! Every dialog carries its own short help, and a fold-out "How to measure"
//! section, so the user never has to leave it to know what to type.

use std::collections::HashMap;
use std::sync::Mutex;

use ocs_plugin_api::dialog::{DialogControl, DialogDef, DialogResponse, DialogValue};
use ocs_plugin_api::host::codec::Handle;
use ocs_plugin_api::host::{CommandStep, InteractiveCommand, PreviewWire};

use crate::commands::{self, Room};
use crate::solve::{self, P2};
use crate::survey::{Saved, Survey};

const GREEN: [f32; 4] = [0.45, 0.85, 0.45, 1.0];

/// Arguments of a command handed to the host as a bare verb (see
/// [`deferred`]).
static DEFERRED: Mutex<Option<(String, String)>> = Mutex::new(None);

/// Hand `verb args` to the host as the bare `verb`, keeping `args` here.
///
/// The host runs a multi-word line by dispatching it and then feeding the
/// words after the verb — and a final Enter — to whatever command is active,
/// which would be the dialog the verb opens; that Enter would close it at
/// once. A single word is dispatched with nothing fed after it.
pub fn deferred(verb: &str, args: String) -> CommandStep {
    *DEFERRED.lock().unwrap_or_else(|e| e.into_inner()) = Some((verb.to_string(), args));
    CommandStep::Dispatch(verb.to_string())
}

/// The arguments [`deferred`] kept for `verb`, once.
pub fn take_deferred(verb: &str) -> Option<String> {
    let mut slot = DEFERRED.lock().unwrap_or_else(|e| e.into_inner());
    if slot.as_ref().is_some_and(|(v, _)| v.eq_ignore_ascii_case(verb)) {
        return slot.take().map(|(_, a)| a);
    }
    None
}

fn with_msg(msg: &Option<String>, prompt: String) -> String {
    match msg {
        Some(m) => format!("{m}  |  {prompt}"),
        None => prompt,
    }
}

fn ring3(pts: &[P2], z: f64) -> Vec<[f64; 3]> {
    let mut v: Vec<[f64; 3]> = pts.iter().map(|p| [p[0], p[1], z]).collect();
    if let Some(&f) = v.first() {
        v.push(f);
    }
    v
}

// ── Pick rooms (closed polylines) ───────────────────────────────────────────

/// Choose the rooms with the CAD's own selection: single picks, or a window
/// / crossing box dragged over many rooms at once. The CAD highlights what
/// is selected, and the chosen rooms are also outlined in green, so they
/// stand out from the ones not chosen yet. Enter confirms.
pub struct PickRoomTool {
    rooms: Vec<Room>,
    /// Chosen rooms, in the order that gives their numbers.
    chosen: Vec<usize>,
    /// Command dispatched (deferred) with the rooms' handles.
    verb: &'static str,
    title: &'static str,
    msg: Option<String>,
}

impl PickRoomTool {
    pub fn new(rooms: Vec<Room>, verb: &'static str, title: &'static str) -> Self {
        PickRoomTool { rooms, chosen: Vec::new(), verb, title, msg: None }
    }

    /// The smallest room containing a point.
    fn room_at(&self, pt: [f64; 3]) -> Option<usize> {
        self.rooms
            .iter()
            .enumerate()
            .filter(|(_, r)| commands::point_in_polygon(&r.pts, [pt[0], pt[1]]))
            .min_by(|a, b| solve::signed_area(&a.1.pts).abs().total_cmp(&solve::signed_area(&b.1.pts).abs()))
            .map(|(i, _)| i)
    }

    fn toggle(&mut self, idx: Option<usize>) -> CommandStep {
        match idx {
            Some(i) => {
                if let Some(k) = self.chosen.iter().position(|&c| c == i) {
                    self.chosen.remove(k);
                } else {
                    self.chosen.push(i);
                }
                self.msg = None;
            }
            None => self.msg = Some(tr!("lì non c'è una polilinea chiusa", "no closed polyline there").into()),
        }
        CommandStep::NeedPoint
    }

    /// Centre of a room's bounding box.
    fn centre(&self, i: usize) -> P2 {
        let p = &self.rooms[i].pts;
        let (x0, x1) = p.iter().fold((f64::MAX, f64::MIN), |a, q| (a.0.min(q[0]), a.1.max(q[0])));
        let (y0, y1) = p.iter().fold((f64::MAX, f64::MIN), |a, q| (a.0.min(q[1]), a.1.max(q[1])));
        [(x0 + x1) / 2.0, (y0 + y1) / 2.0]
    }

    /// Bring the choice in step with the CAD's selection: rooms no longer
    /// selected are dropped, the others keep their place, and the new ones
    /// follow — those taken together by a box from the top down, left to
    /// right, like reading a page.
    fn sync(&mut self, handles: &[Handle]) {
        let selected: Vec<usize> = (0..self.rooms.len()).filter(|&i| handles.contains(&self.rooms[i].handle)).collect();
        self.chosen.retain(|i| selected.contains(i));
        let mut new: Vec<usize> = selected.into_iter().filter(|i| !self.chosen.contains(i)).collect();
        let band = new
            .iter()
            .map(|&i| {
                let p = &self.rooms[i].pts;
                let (y0, y1) = p.iter().fold((f64::MAX, f64::MIN), |a, q| (a.0.min(q[1]), a.1.max(q[1])));
                y1 - y0
            })
            .fold(f64::MAX, f64::min)
            * 0.5;
        new.sort_by(|&a, &b| {
            let (ca, cb) = (self.centre(a), self.centre(b));
            if (ca[1] - cb[1]).abs() > band {
                cb[1].total_cmp(&ca[1])
            } else {
                ca[0].total_cmp(&cb[0])
            }
        });
        self.chosen.extend(new);
        let others = handles.len().saturating_sub(self.chosen.len());
        self.msg = (others > 0 && self.chosen.is_empty())
            .then(|| tr!("nessuna polilinea chiusa tra gli oggetti selezionati", "no closed polyline among the selected objects").into());
    }
}

impl InteractiveCommand for PickRoomTool {
    fn prompt(&self) -> String {
        let p = if self.chosen.is_empty() {
            tr!(
                format!(
                    "{}: seleziona i vani (polilinee chiuse): un clic sulla linea, oppure una finestra trascinando il mouse \
                     per prenderne tanti insieme. Invio per continuare, Esc per annullare",
                    self.title
                ),
                format!(
                    "{}: select the rooms (closed polylines): click a line, or drag a window to take many at once. \
                     Enter to go on, Esc to cancel",
                    self.title
                )
            )
        } else {
            tr!(
                format!(
                    "{}: {} vani selezionati (evidenziati, bordo verde). Aggiungine altri con clic o finestra, Maiusc+clic \
                     per toglierne uno, Invio per continuare",
                    self.title,
                    self.chosen.len()
                ),
                format!(
                    "{}: {} rooms selected (highlighted, green outline). Add more by click or window, Shift+click to drop \
                     one, Enter to go on",
                    self.title,
                    self.chosen.len()
                )
            )
        };
        with_msg(&self.msg, p)
    }
    fn is_selection_gathering(&self) -> bool {
        true
    }
    fn accepts_drag_selection(&self) -> bool {
        true
    }
    fn on_selection_complete(&mut self, handles: Vec<Handle>) -> CommandStep {
        self.sync(&handles);
        CommandStep::NeedPoint
    }
    // Hosts that send plain clicks instead of selections: a click inside a
    // room, or on its polyline, toggles it.
    fn on_point(&mut self, pt: [f64; 3]) -> CommandStep {
        let i = self.room_at(pt);
        self.toggle(i)
    }
    fn on_object_pick(&mut self, handle: Handle, pt: [f64; 3]) -> CommandStep {
        let i = self.rooms.iter().position(|r| r.handle == handle).or_else(|| self.room_at(pt));
        self.toggle(i)
    }
    fn on_cursor_move(&mut self, _pt: [f64; 3]) -> Vec<PreviewWire> {
        self.chosen.iter().map(|&i| PreviewWire::with_color(ring3(&self.rooms[i].pts, self.rooms[i].z), GREEN)).collect()
    }
    fn on_enter(&mut self) -> CommandStep {
        if self.chosen.is_empty() {
            return CommandStep::Cancel;
        }
        let hs: Vec<String> = self.chosen.iter().map(|&i| format!("{:X}", self.rooms[i].handle.value())).collect();
        deferred(self.verb, hs.join(","))
    }
}

// ── Dialog helpers ──────────────────────────────────────────────────────────

fn info(text: String) -> DialogControl {
    DialogControl::InfoText { text, align_h: None }
}

fn number(id: &str, label: &str, default: f64) -> DialogControl {
    DialogControl::Number { id: id.into(), label: label.into(), default, min: Some(0.0), max: None, step: None }
}

fn checkbox(id: &str, label: &str, default: bool) -> DialogControl {
    DialogControl::Checkbox { id: id.into(), label: label.into(), default }
}

/// A fold-out section of help texts (closed at first).
fn help_section(id: &str, title: &str, paragraphs: Vec<String>) -> DialogControl {
    DialogControl::Fields { id: id.into(), title: Some(title.into()), controls: paragraphs.into_iter().map(info).collect(), collapsed: true }
}

fn get_num(m: &HashMap<String, DialogValue>, id: &str) -> Option<f64> {
    match m.get(id)? {
        DialogValue::Number(v) => Some(*v),
        DialogValue::Text(s) => commands::parse_num(s),
        _ => None,
    }
}

fn get_bool(m: &HashMap<String, DialogValue>, id: &str, default: bool) -> bool {
    match m.get(id) {
        Some(DialogValue::Bool(b)) => *b,
        _ => default,
    }
}

fn get_path(m: &HashMap<String, DialogValue>, id: &str) -> Option<String> {
    match m.get(id)? {
        DialogValue::FilePath(s) | DialogValue::Text(s) => {
            let s = s.trim().trim_matches('"').to_string();
            (!s.is_empty()).then_some(s)
        }
        _ => None,
    }
}

fn csv_picker(default: &str) -> DialogControl {
    DialogControl::FilePicker {
        id: "csv_path".into(),
        label: tr!("File del libretto", "Book file").into(),
        default_path: default.into(),
        filter_name: tr!("Libretto misure (CSV)", "Measures book (CSV)").into(),
        extensions: vec!["csv".into()],
        save: true,
    }
}

/// The paragraphs of the fold-out help in the measures dialog.
fn how_to_measure(multi: bool) -> Vec<String> {
    let mut v = vec![
        tr!(
            "COME SI USA — scegli una voce dall'elenco e premi Continua. Apri un vano alla volta, scrivi le sue \
             misure e premi OK: torni qui e l'elenco passa da solo al vano successivo. Quando tutto è a posto \
             l'elenco propone Calcola. Quello che scrivi resta memorizzato anche se chiudi la finestra.",
            "HOW IT WORKS — pick an entry from the list and press Continue. Open one room at a time, type its \
             measures and press OK: you come back here and the list moves on to the next room. When all is set the \
             list proposes Compute. What you type is kept even if you close the window."
        )
        .to_string(),
        tr!(
            "LATI E DIAGONALI — misura ogni parete da spigolo a spigolo, sempre alla stessa altezza. Un vano di n \
             spigoli vuole tutti i lati e almeno n−3 diagonali (1 per un rettangolo, 3 per una L): quelle proposte \
             passano dentro il vano. Una diagonale in più fa da controllo: è l'unico modo per accorgersi di una \
             cifra letta male.",
            "SIDES AND DIAGONALS — measure each wall from corner to corner, always at the same height. A room of n \
             corners needs all sides and at least n−3 diagonals (1 for a rectangle, 3 for an L): the proposed ones run \
             inside the room. One more diagonal is the check: the only way to notice a misread digit."
        )
        .to_string(),
        tr!(
            "VINCOLI — al posto di una diagonale: uno spigolo e il suo angolo (A, 90) oppure un lato orizzontale o \
             verticale (A-B, orizzontale). Ognuno vale una misura e riceve il suo scarto.",
            "CONSTRAINTS — instead of a diagonal: a corner and its angle (A, 90) or a horizontal or vertical side \
             (A-B, horizontal). Each counts as a measure and gets its residual."
        )
        .to_string(),
        tr!(
            "PORTE E FINESTRE — scegli \"Aggiungi porta o finestra\" e clicca sul disegno, sul muro, nel punto dove \
             si trova. Poi scrivi la distanza dallo spigolo allo stipite, la larghezza e lo spessore del muro \
             misurato nella porta.",
            "DOORS AND WINDOWS — choose \"Add a door or window\" and click on the drawing, on the wall, where it is. \
             Then type the distance from the corner to the jamb, the width and the wall thickness measured in the door."
        )
        .to_string(),
    ];
    if multi {
        v.push(
            tr!(
                "NOMI DEGLI SPIGOLI — sul disegno ogni spigolo ha solo la sua lettera e il vano il suo numero: lo spigolo \
                 B del vano 3 si scrive 3B (nella scheda del vano 3 basta B).",
                "CORNER NAMES — on the drawing each corner has its letter only and the room its number: corner B of room 3 \
                 is written 3B (in room 3's page B is enough)."
            )
            .to_string(),
        );
        v.push(
            tr!(
                "COLLEGARE I VANI — una porta tra due vani li collega da sola: cliccando sul muro tra i due, il plugin \
                 riconosce i due lati; servono lo spessore del muro e la distanza della porta dallo spigolo misurata \
                 in tutti e due i vani. Senza porta, usa \"Altre misure tra vani\": due spigoli (1B, 2D) per una \
                 distanza, oppure un lato e uno spigolo (1B-1C, 2A) per lo spessore di un muro.",
                "LINKING ROOMS — a door between two rooms links them by itself: click the wall between them and the \
                 plugin finds both sides; it needs the wall thickness and the door's distance from the corner measured \
                 in both rooms. With no door, use \"Other measures between rooms\": two corners (1B, 2D) for a distance, \
                 or a side and a corner (1B-1C, 2A) for a wall thickness."
            )
            .to_string(),
        );
    }
    v.push(
        tr!(
            "MISURE MANCANTI E UNITÀ — una casella vuota non viene usata; se mancano misure (mobili, ostacoli) il \
             plugin completa la forma con gli angoli del disegno e lo segnala in rosso (si può spegnere nelle \
             Opzioni). Le misure vanno nelle unità del disegno; la virgola decimale va bene.",
            "MISSING MEASURES AND UNITS — an empty box is not used; if measures are missing (furniture, obstacles) the \
             plugin finishes the shape with the drawing's angles and says so in red (can be turned off in Options). \
             Type the measures in drawing units; a decimal comma is fine."
        )
        .to_string(),
    );
    v
}

// ── Doors by a click on the drawing ─────────────────────────────────────────

/// The wall a door or window was clicked on: a side of a room, from the
/// corner nearer the click, and — when another room lies just behind it —
/// the facing side of that room, from the corner facing the first one.
#[derive(Clone, Debug, PartialEq)]
pub struct DoorSpot {
    pub room: usize,
    pub from: usize,
    pub to: usize,
    pub other: Option<(usize, usize, usize)>,
}

impl DoorSpot {
    pub fn encode(&self) -> String {
        let o = match self.other {
            Some((q, k, l)) => format!("{q},{k},{l}"),
            None => "-".into(),
        };
        format!("{},{},{};{o}", self.room, self.from, self.to)
    }

    pub fn decode(s: &str) -> Option<Self> {
        let mut parts = s.split(';');
        let three = |t: &str| -> Option<(usize, usize, usize)> {
            let v: Vec<usize> = t.split(',').filter_map(|x| x.trim().parse().ok()).collect();
            (v.len() == 3).then(|| (v[0], v[1], v[2]))
        };
        let (room, from, to) = three(parts.next()?)?;
        let other = parts.next().and_then(three);
        Some(DoorSpot { room, from, to, other })
    }

    /// Whether the indices exist in `survey` and name real sides.
    fn fits(&self, survey: &Survey) -> bool {
        let side = |r: usize, i: usize, j: usize| {
            survey.rooms.get(r).is_some_and(|room| {
                let n = room.len();
                i < n && j < n && (j == (i + 1) % n || i == (j + 1) % n)
            })
        };
        side(self.room, self.from, self.to) && self.other.is_none_or(|(q, k, l)| q != self.room && side(q, k, l))
    }
}

/// The side nearest to `pt` and, if any, the facing side of the room behind
/// that wall (roughly parallel, opposite the click, on the outside).
pub fn find_spot(rooms: &[Vec<P2>], pt: P2) -> Option<DoorSpot> {
    let longest = rooms
        .iter()
        .flat_map(|r| (0..r.len()).map(move |i| solve::dist(r[i], r[(i + 1) % r.len()])))
        .fold(0.0f64, f64::max);
    let mut best: Option<(f64, usize, usize)> = None;
    for (r, room) in rooms.iter().enumerate() {
        for i in 0..room.len() {
            let d = crate::labels::seg_dist(pt, room[i], room[(i + 1) % room.len()]);
            if best.is_none_or(|b| d < b.0) {
                best = Some((d, r, i));
            }
        }
    }
    let (d, r, i) = best?;
    if d > longest * 0.3 {
        return None;
    }
    let j = (i + 1) % rooms[r].len();
    let (a, b) = (rooms[r][i], rooms[r][j]);
    let len = solve::dist(a, b);
    if len <= 0.0 {
        return None;
    }
    let (from, to) = if solve::dist(pt, a) <= solve::dist(pt, b) { (i, j) } else { (j, i) };
    let u = [(b[0] - a[0]) / len, (b[1] - a[1]) / len];
    let on_first = solve::foot(pt, a, b);

    let mut other: Option<(f64, usize, usize, usize)> = None;
    for (q, room) in rooms.iter().enumerate() {
        if q == r {
            continue;
        }
        for k in 0..room.len() {
            let l = (k + 1) % room.len();
            let (c, e) = (room[k], room[l]);
            let le = solve::dist(c, e);
            if le <= 0.0 {
                continue;
            }
            let v = [(e[0] - c[0]) / le, (e[1] - c[1]) / le];
            if (u[0] * v[1] - u[1] * v[0]).abs() > 0.5 {
                continue; // not parallel enough
            }
            let t = ((pt[0] - c[0]) * v[0] + (pt[1] - c[1]) * v[1]) / le;
            if !(0.0..=1.0).contains(&t) {
                continue; // the click is not in front of this side
            }
            let on_second = solve::foot(pt, c, e);
            let gap = solve::dist(on_first, on_second);
            if gap > longest * 0.2 {
                continue;
            }
            // The wall lies outside both rooms.
            if gap > longest * 1e-6 {
                let mid = [(on_first[0] + on_second[0]) / 2.0, (on_first[1] + on_second[1]) / 2.0];
                if commands::point_in_polygon(&rooms[r], mid) || commands::point_in_polygon(room, mid) {
                    continue;
                }
            }
            if other.is_none_or(|o| gap < o.0) {
                let near = rooms[r][from];
                let (f2, t2) = if solve::dist(near, c) <= solve::dist(near, e) { (k, l) } else { (l, k) };
                other = Some((gap, q, f2, t2));
            }
        }
    }
    Some(DoorSpot { room: r, from, to, other: other.map(|o| (o.1, o.2, o.3)) })
}

/// Click on a wall to place a door or window; then the measures dialog
/// opens again on the door's page.
pub struct PickDoorTool {
    rooms: Vec<Vec<P2>>,
    z: f64,
    /// The rooms' handles, as the dialog commands take them.
    key: String,
    msg: Option<String>,
}

impl PickDoorTool {
    pub fn new(rooms: Vec<Vec<P2>>, z: f64, key: String) -> Self {
        PickDoorTool { rooms, z, key, msg: None }
    }

    fn pick(&mut self, pt: [f64; 3]) -> CommandStep {
        match find_spot(&self.rooms, [pt[0], pt[1]]) {
            Some(s) => deferred("RV_PORTAOPEN", format!("{}|{}", self.key, s.encode())),
            None => {
                self.msg = Some(tr!("lì non c'è il muro di un vano scelto", "no wall of a chosen room there").into());
                CommandStep::NeedPoint
            }
        }
    }
}

impl InteractiveCommand for PickDoorTool {
    fn prompt(&self) -> String {
        with_msg(
            &self.msg,
            tr!(
                "Porta o finestra: clicca sul muro nel punto dove si trova (tra due vani il plugin riconosce i due lati, \
                 in verde). Invio per tornare alla finestra senza aggiungerla"
                    .to_string(),
                "Door or window: click the wall where it is (between two rooms the plugin finds both sides, in green). \
                 Enter to go back to the window without adding it"
                    .to_string()
            ),
        )
    }
    fn on_point(&mut self, pt: [f64; 3]) -> CommandStep {
        self.pick(pt)
    }
    fn needs_object_pick(&self) -> bool {
        true
    }
    fn on_object_pick(&mut self, _handle: Handle, pt: [f64; 3]) -> CommandStep {
        self.pick(pt)
    }
    fn on_cursor_move(&mut self, pt: [f64; 3]) -> Vec<PreviewWire> {
        let Some(s) = find_spot(&self.rooms, [pt[0], pt[1]]) else { return Vec::new() };
        let seg = |r: usize, i: usize, j: usize| {
            let (a, b) = (self.rooms[r][i], self.rooms[r][j]);
            PreviewWire::with_color(vec![[a[0], a[1], self.z], [b[0], b[1], self.z]], GREEN)
        };
        let mut w = vec![seg(s.room, s.from, s.to)];
        if let Some((q, k, l)) = s.other {
            w.push(seg(q, k, l));
        }
        w
    }
    fn on_enter(&mut self) -> CommandStep {
        deferred("RV_VANOREOPEN", self.key.clone())
    }
}

// ── Measures dialog ─────────────────────────────────────────────────────────

fn tf(id: String, label: String, default: String) -> DialogControl {
    DialogControl::Text { id, label, default }
}

/// Lay controls out `per_row` to a row; the last row is padded with empty
/// texts so every box keeps the same width.
fn grid(controls: Vec<DialogControl>, per_row: usize) -> Vec<DialogControl> {
    let mut out = Vec::new();
    let mut it = controls.into_iter().peekable();
    while it.peek().is_some() {
        let mut row: Vec<DialogControl> = it.by_ref().take(per_row).collect();
        while row.len() < per_row {
            row.push(info(String::new()));
        }
        out.push(DialogControl::row(row));
    }
    out
}

fn get_text(m: &HashMap<String, DialogValue>, id: &str) -> String {
    match m.get(id) {
        Some(DialogValue::Text(s)) => s.trim().to_string(),
        Some(DialogValue::Number(v)) => v.to_string(),
        _ => String::new(),
    }
}

fn get_choice(m: &HashMap<String, DialogValue>, id: &str, default: usize) -> usize {
    match m.get(id) {
        Some(DialogValue::Choice { index, .. }) => *index,
        _ => default,
    }
}

fn filled(row: &[String], cols: &[usize]) -> bool {
    cols.iter().any(|&c| row.get(c).is_some_and(|s| !s.trim().is_empty()))
}

fn cell(row: &[String], c: usize) -> String {
    row.get(c).cloned().unwrap_or_default()
}

/// Where the measures dialog opens.
#[derive(Clone, Debug, PartialEq)]
pub enum Start {
    Hub,
    Door(DoorSpot),
}

#[derive(Clone, Debug, PartialEq)]
enum Page {
    Hub,
    Room(usize),
    Links,
    Openings,
    Options,
    Door(DoorSpot),
    Done,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Action {
    Room(usize),
    AddDoor,
    Links,
    Openings,
    Options,
    Compute,
}

/// The measures dialog: a small hub listing the rooms with their state,
/// from which one room at a time is opened (its sides, diagonals and
/// constraints), doors and windows are added by a click on the drawing,
/// and the calculation is started. Each page stays small whatever the
/// number of rooms.
///
/// Plain text boxes rather than table controls: the host sizes its dialog
/// window to the content and measures a table's rows at their minimum
/// height, so a table opens squashed.
pub struct MeasureDialog {
    handles: Vec<Handle>,
    survey: Survey,
    msg: Option<String>,
    saved: Saved,
    page: Page,
    /// What was typed on the door page, kept when it has to be shown again.
    draft: HashMap<String, DialogValue>,
}

impl MeasureDialog {
    /// `saved`: what the user typed last time for these rooms.
    pub fn new(handles: Vec<Handle>, survey: &Survey, saved: &Saved, msg: Option<String>) -> Self {
        MeasureDialog { handles, survey: survey.clone(), msg, saved: saved.clone(), page: Page::Hub, draft: HashMap::new() }
    }

    /// Open on the door page for a wall clicked on the drawing.
    pub fn starting(mut self, start: Start) -> Self {
        if let Start::Door(spot) = start {
            if spot.fits(&self.survey) {
                self.page = Page::Door(spot);
            }
        }
        self
    }

    fn key(&self) -> String {
        self.handles.iter().map(|h| format!("{:X}", h.value())).collect::<Vec<_>>().join(",")
    }

    fn keep(&self) {
        commands::remember(&self.handles, self.saved.clone());
    }

    fn multi(&self) -> bool {
        self.survey.multi()
    }

    fn room_name(&self, r: usize) -> String {
        if self.multi() {
            tr!(format!("vano {}", r + 1), format!("room {}", r + 1))
        } else {
            tr!("vano".to_string(), "room".to_string())
        }
    }

    /// First row of room `r` in the sides table.
    fn offset(&self, r: usize) -> usize {
        self.survey.rooms[..r].iter().map(|x| x.len()).sum()
    }

    /// The room a diagonal row belongs to (both corners in it); with one
    /// room, every row.
    fn other_room(&self, row: &[String]) -> Option<usize> {
        if !self.multi() {
            return Some(0);
        }
        let a = self.survey.parse_corner(&cell(row, 0))?;
        let b = self.survey.parse_corner(&cell(row, 1))?;
        (a.0 == b.0).then_some(a.0)
    }

    /// The room a constraint row belongs to.
    fn con_room(&self, row: &[String]) -> usize {
        let at = cell(row, 0);
        self.survey
            .parse_corner(&at)
            .map(|c| c.0)
            .or_else(|| self.survey.parse_room_side(&at).map(|s| s.0))
            .or_else(|| self.survey.parse_side(&at).and_then(|(a, _)| self.survey.rooms_of(a).first().copied()))
            .unwrap_or(0)
    }

    /// Diagonal rows room by room, and the rows between rooms.
    fn split_others(&self) -> (Vec<Vec<Vec<String>>>, Vec<Vec<String>>) {
        let mut per = vec![Vec::new(); self.survey.rooms.len()];
        let mut links = Vec::new();
        for row in &self.saved.others {
            match self.other_room(row) {
                Some(r) => per[r].push(row.clone()),
                None => links.push(row.clone()),
            }
        }
        (per, links)
    }

    fn split_cons(&self) -> Vec<Vec<Vec<String>>> {
        let mut per = vec![Vec::new(); self.survey.rooms.len()];
        for row in &self.saved.constraints {
            per[self.con_room(row)].push(row.clone());
        }
        per
    }

    /// In room `r`'s page a bare letter means that room's corner: A → 3A.
    fn norm(&self, r: usize, s: &str) -> String {
        if !self.multi() {
            return s.trim().to_string();
        }
        s.split(['-', '–', '—', '/'])
            .map(str::trim)
            .filter(|p| !p.is_empty())
            .map(|p| {
                if p.chars().all(|c| c.is_ascii_alphabetic()) {
                    format!("{}{}", r + 1, p.to_ascii_uppercase())
                } else {
                    p.to_ascii_uppercase()
                }
            })
            .collect::<Vec<_>>()
            .join("-")
    }

    /// Whether room `r` has what it needs, and its state in words.
    fn room_status(&self, r: usize) -> (bool, String) {
        let n = self.survey.rooms[r].len();
        let off = self.offset(r);
        let sides = (0..n).filter(|i| self.saved.sides.get(off + i).is_some_and(|row| !cell(row, 2).is_empty())).count();
        let (per, _) = self.split_others();
        let diags = per[r].iter().filter(|row| !cell(row, 3).is_empty()).count();
        let cons = self.split_cons()[r].iter().filter(|row| !cell(row, 0).is_empty() && !cell(row, 1).is_empty()).count();
        let need = n.saturating_sub(3);
        let have = diags + cons;
        if sides == n && have >= need {
            let extra = have - need;
            let s = if extra > 0 {
                tr!(format!("✔ completo, {extra} di controllo"), format!("✔ complete, {extra} as a check"))
            } else {
                tr!("✔ completo (senza misura di controllo)".to_string(), "✔ complete (no check measure)".to_string())
            };
            (true, s)
        } else {
            let mut parts = Vec::new();
            if sides < n {
                parts.push(tr!(format!("{} lati", n - sides), format!("{} sides", n - sides)));
            }
            if have < need {
                parts.push(tr!(format!("{} diagonali o vincoli", need - have), format!("{} diagonals or constraints", need - have)));
            }
            (false, tr!(format!("✘ mancano {}", parts.join(" e ")), format!("✘ missing {}", parts.join(" and "))))
        }
    }

    /// Rooms (1-based) the measures do not tie to room 1 yet.
    fn unlinked(&self) -> Result<Vec<usize>, String> {
        let p = self.survey.parse(&self.saved, &commands::parse_num)?;
        let meas: Vec<solve::Meas> = p.measures.iter().map(|m| m.meas).collect();
        let groups = solve::groups(self.survey.points.len(), &meas);
        let group_of = |r: usize| groups.iter().position(|g| g.contains(&self.survey.point_of[r][0]));
        let main = group_of(0);
        Ok((1..self.survey.rooms.len()).filter(|&r| group_of(r) != main).map(|r| r + 1).collect())
    }

    /// One line per door or window: "P1 porta: 1B-1C ↔ 2D-2A · 0,90 · muro 0,30".
    fn opening_lines(&self) -> Vec<String> {
        let mut out: Vec<(String, String)> = Vec::new();
        for row in self.saved.openings.iter().filter(|r| filled(r, &[0, 1, 2, 3, 4, 5])) {
            let name = cell(row, 0);
            if let Some(line) = out.iter_mut().find(|(n, _)| n.eq_ignore_ascii_case(&name)) {
                line.1 = line.1.replacen(" · ", &format!(" ↔ {} · ", cell(row, 1)), 1);
                if !cell(row, 5).is_empty() && !line.1.contains(tr!("muro", "wall")) {
                    line.1 += &format!(" · {} {}", tr!("muro", "wall"), cell(row, 5));
                }
                continue;
            }
            let mut s = format!("{name} {}: {} · {}", cell(row, 2), cell(row, 1), cell(row, 4));
            if !cell(row, 5).is_empty() {
                s += &format!(" · {} {}", tr!("muro", "wall"), cell(row, 5));
            }
            out.push((name, s));
        }
        out.into_iter().map(|x| x.1).collect()
    }

    /// The hub's list: what each entry does, its label, and the one
    /// proposed (the first thing still to do).
    fn actions(&self) -> (Vec<Action>, Vec<String>, usize) {
        let mut acts = Vec::new();
        let mut labels = Vec::new();
        let mut first_open = None;
        for r in 0..self.survey.rooms.len() {
            let (ok, status) = self.room_status(r);
            if !ok && first_open.is_none() {
                first_open = Some(acts.len());
            }
            acts.push(Action::Room(r));
            labels.push(tr!(
                format!("Misure del {} — {status}", self.room_name(r)),
                format!("Measures of {} — {status}", self.room_name(r))
            ));
        }
        let door = acts.len();
        acts.push(Action::AddDoor);
        labels.push(tr!("Aggiungi porta o finestra (clic sul disegno)", "Add a door or window (click on the drawing)").to_string());
        if self.saved.openings.iter().any(|r| filled(r, &[0, 1, 2, 3, 4, 5])) {
            acts.push(Action::Openings);
            labels.push(tr!("Modifica o elimina porte e finestre", "Edit or delete doors and windows").to_string());
        }
        if self.multi() {
            acts.push(Action::Links);
            labels.push(tr!("Altre misure tra vani (senza porta)", "Other measures between rooms (no door)").to_string());
        }
        acts.push(Action::Options);
        labels.push(tr!("Opzioni (testi, tolleranza, quote, libretto…)", "Options (texts, tolerance, dimensions, book…)").to_string());
        let compute = acts.len();
        acts.push(Action::Compute);
        labels.push(tr!("▶ Calcola", "▶ Compute").to_string());
        let default = first_open.unwrap_or_else(|| {
            if self.multi() && self.unlinked().is_ok_and(|u| !u.is_empty()) {
                door
            } else {
                compute
            }
        });
        (acts, labels, default)
    }

    fn hub(&self) -> DialogDef {
        let n_rooms = self.survey.rooms.len();
        let mut def = DialogDef::new(if self.multi() {
            tr!(format!("Rilievo di {n_rooms} vani"), format!("Survey of {n_rooms} rooms"))
        } else {
            tr!("Rilievo del vano".to_string(), "Room survey".to_string())
        });
        if let Some(m) = &self.msg {
            def = def.with_control(info(tr!(format!("⚠ Da correggere: {m}."), format!("⚠ To fix: {m}."))));
        }
        let lines: Vec<DialogControl> = (0..n_rooms)
            .map(|r| {
                let n = self.survey.rooms[r].len();
                let name = self.room_name(r);
                let name = name[..1].to_uppercase() + &name[1..];
                info(tr!(
                    format!("{name} ({n} spigoli): {}", self.room_status(r).1),
                    format!("{name} ({n} corners): {}", self.room_status(r).1)
                ))
            })
            .collect();
        for row in grid(lines, if n_rooms > 6 { 2 } else { 1 }) {
            def = def.with_control(row);
        }
        if self.multi() {
            def = def.with_control(info(match self.unlinked() {
                Ok(u) if u.is_empty() => tr!("Collegamenti: ✔ tutti i vani sono collegati tra loro.".to_string(), "Links: ✔ all rooms are linked.".to_string()),
                Ok(u) => {
                    let list = u.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", ");
                    tr!(
                        format!("Collegamenti: ✘ vani non ancora collegati al vano 1: {list}. Aggiungi una porta sul muro che li separa."),
                        format!("Links: ✘ rooms not yet linked to room 1: {list}. Add a door on the wall between them.")
                    )
                }
                Err(e) => format!("⚠ {e}"),
            }));
        }
        let doors = self.opening_lines();
        if !doors.is_empty() {
            let shown: Vec<String> = doors.iter().take(8).cloned().collect();
            let more = doors.len() - shown.len();
            let mut text = format!("{}: {}", tr!("Porte e finestre", "Doors and windows"), shown.join("   |   "));
            if more > 0 {
                text += &tr!(format!("   … e altre {more}"), format!("   … and {more} more"));
            }
            def = def.with_control(info(text));
        }
        let (_, labels, default) = self.actions();
        def.with_control(DialogControl::Choice {
            id: "do".into(),
            label: tr!("Cosa vuoi fare?", "What next?").into(),
            options: labels,
            default_index: default,
        })
        .with_control(help_section("help", tr!("Come si usa (aiuto) ▸", "How it works (help) ▸"), how_to_measure(self.multi())))
        .submit_label(tr!("Continua ▸", "Continue ▸"))
        .cancel_label(tr!("Chiudi", "Close"))
        .width(660.0)
    }

    /// The diagonal and constraint rows shown on room `r`'s page: the ones
    /// kept, then empty spare rows.
    fn room_view(&self, r: usize) -> (Vec<Vec<String>>, Vec<Vec<String>>) {
        let (per, _) = self.split_others();
        let mut others: Vec<Vec<String>> = per[r].iter().filter(|x| filled(x, &[0, 1, 3])).cloned().collect();
        others.extend(std::iter::repeat_n(vec![String::new(); 4], 2));
        let mut cons: Vec<Vec<String>> = self.split_cons()[r].iter().filter(|x| filled(x, &[0, 1])).cloned().collect();
        let total = (cons.len() + 1).max(3).div_ceil(3) * 3;
        cons.resize(total, vec![String::new(); 2]);
        (others, cons)
    }

    fn links_view(&self) -> Vec<Vec<String>> {
        let (_, links) = self.split_others();
        let mut v: Vec<Vec<String>> = links.into_iter().filter(|x| filled(x, &[0, 1, 3])).collect();
        v.extend(std::iter::repeat_n(vec![String::new(); 4], 4));
        v
    }

    fn openings_view(&self) -> Vec<Vec<String>> {
        let mut v: Vec<Vec<String>> = self.saved.openings.iter().filter(|x| filled(x, &[0, 1, 2, 3, 4, 5])).cloned().collect();
        v.extend(std::iter::repeat_n(vec![String::new(); 6], 2));
        v
    }

    fn distance_rows(rows: &[Vec<String>]) -> Vec<DialogControl> {
        let mut cells = Vec::new();
        for (k, r) in rows.iter().enumerate() {
            let drawn = r.get(2).filter(|d| !d.is_empty()).map(|d| format!(" ({d})")).unwrap_or_default();
            cells.push(tf(format!("o{k}a"), tr!("Da", "From").into(), cell(r, 0)));
            cells.push(tf(format!("o{k}b"), tr!("A", "To").into(), cell(r, 1)));
            cells.push(tf(format!("o{k}m"), format!("{}{drawn}", tr!("Misurata", "Measured")), cell(r, 3)));
        }
        grid(cells, 6)
    }

    fn room_page(&self, r: usize) -> DialogDef {
        let n = self.survey.rooms[r].len();
        let off = self.offset(r);
        let need = n.saturating_sub(3);
        let name = self.room_name(r);
        let name = name[..1].to_uppercase() + &name[1..];
        let (first, last) = (self.survey.corner(r, 0), self.survey.corner(r, n - 1));
        let mut def = DialogDef::new(tr!(format!("{name} — lati, diagonali, vincoli"), format!("{name} — sides, diagonals, constraints")))
            .with_control(info(tr!(
                format!("LATI — spigoli {first} … {last}. Scrivi la misura sotto ogni lato (tra parentesi la lunghezza disegnata)."),
                format!("SIDES — corners {first} … {last}. Type the measure under each side (the drawn length in brackets).")
            )));
        let sides: Vec<DialogControl> = (0..n)
            .map(|i| {
                let row = self.saved.sides.get(off + i).cloned().unwrap_or_default();
                tf(format!("s{i}"), format!("{}  ({})", cell(&row, 0), cell(&row, 1)), cell(&row, 2))
            })
            .collect();
        for row in grid(sides, 4) {
            def = def.with_control(row);
        }
        let (others, cons) = self.room_view(r);
        def = def.with_control(info(if need == 0 {
            tr!(
                "DIAGONALI — un triangolo non ne ha bisogno.".to_string(),
                "DIAGONALS — a triangle needs none.".to_string()
            )
        } else {
            tr!(
                format!(
                    "DIAGONALI — ne servono almeno {need} (contando anche i vincoli); una in più fa da controllo. Quelle \
                     proposte passano dentro il vano; nelle righe vuote puoi scriverne altre{}.",
                    if self.multi() { " (basta la lettera: A, C)" } else { "" }
                ),
                format!(
                    "DIAGONALS — at least {need} are needed (constraints count too); one more is a check. The proposed \
                     ones run inside the room; use the empty rows for more{}.",
                    if self.multi() { " (the letter is enough: A, C)" } else { "" }
                )
            )
        }));
        for row in Self::distance_rows(&others) {
            def = def.with_control(row);
        }
        def = def.with_control(info(
            tr!(
                "VINCOLI (facoltativi) — uno spigolo e il suo angolo (A, 90), oppure un lato e orizzontale / verticale (A-B, orizzontale).",
                "CONSTRAINTS (optional) — a corner and its angle (A, 90), or a side and horizontal / vertical (A-B, horizontal)."
            )
            .to_string(),
        ));
        let mut cells = Vec::new();
        for (k, c) in cons.iter().enumerate() {
            cells.push(tf(format!("c{k}a"), tr!("Spigolo o lato", "Corner or side").into(), cell(c, 0)));
            cells.push(tf(format!("c{k}b"), tr!("Vincolo", "Constraint").into(), cell(c, 1)));
        }
        for row in grid(cells, 6) {
            def = def.with_control(row);
        }
        def.submit_label("OK ▸").cancel_label(tr!("◂ Indietro", "◂ Back")).width(700.0)
    }

    fn links_page(&self) -> DialogDef {
        let mut def = DialogDef::new(tr!("Altre misure tra vani", "Other measures between rooms")).with_control(info(
            tr!(
                "Servono solo dove non c'è una porta, o come controllo. Da e A: due spigoli di vani diversi (1B, 2D) per \
                 una distanza presa attraverso un'apertura, oppure un lato e uno spigolo (1B-1C, 2A) per lo spessore di un muro.",
                "Needed only where there is no door, or as a check. From and To: two corners of different rooms (1B, 2D) \
                 for a distance taken through an opening, or a side and a corner (1B-1C, 2A) for a wall thickness."
            )
            .to_string(),
        ));
        for row in Self::distance_rows(&self.links_view()) {
            def = def.with_control(row);
        }
        def.submit_label("OK ▸").cancel_label(tr!("◂ Indietro", "◂ Back")).width(700.0)
    }

    fn openings_page(&self) -> DialogDef {
        let mut def = DialogDef::new(tr!("Porte e finestre inserite", "Doors and windows")).with_control(info(
            tr!(
                "Correggi i valori, oppure svuota una riga per eliminare la porta o la finestra. Una porta tra due vani \
                 occupa due righe con lo stesso nome, una per vano. La distanza si misura dal primo spigolo scritto nel lato.",
                "Fix the values, or empty a row to delete the door or window. A door between two rooms takes two rows \
                 with the same name, one per room. The distance is taken from the first corner written in the side."
            )
            .to_string(),
        ));
        let labels = [
            tr!("Nome", "Name"),
            tr!("Lato", "Side"),
            tr!("Tipo", "Kind"),
            tr!("Distanza", "Distance"),
            tr!("Larghezza", "Width"),
            tr!("Spessore muro", "Wall"),
        ];
        let mut cells = Vec::new();
        for (k, r) in self.openings_view().iter().enumerate() {
            for (c, l) in labels.iter().enumerate() {
                cells.push(tf(format!("p{k}_{c}"), l.to_string(), cell(r, c)));
            }
        }
        for row in grid(cells, 6) {
            def = def.with_control(row);
        }
        def.submit_label("OK ▸").cancel_label(tr!("◂ Indietro", "◂ Back")).width(700.0)
    }

    fn options_page(&self) -> DialogDef {
        let saved = &self.saved;
        DialogDef::new(tr!("Opzioni del rilievo", "Survey options"))
            .with_control(DialogControl::row(vec![
                number("h", tr!("Altezza testi", "Text height"), saved.h),
                number("tol", tr!("Tolleranza scarti", "Residual tolerance"), saved.tol),
                DialogControl::Choice {
                    id: "unit".into(),
                    label: tr!("Unità del disegno", "Drawing unit").into(),
                    options: vec![tr!("metri", "metres").into(), tr!("centimetri", "centimetres").into(), tr!("millimetri", "millimetres").into()],
                    default_index: saved.unit.min(2),
                },
            ]))
            .with_control(checkbox(
                "fill",
                tr!(
                    "Se mancano misure (mobili, ostacoli), completa la forma con gli angoli del disegno",
                    "If measures are missing (furniture, obstacles), finish the shape with the drawing's angles"
                ),
                saved.fill,
            ))
            .with_control(checkbox(
                "replace",
                tr!(
                    "Modifica subito le polilinee originali (senza copia su RILIEVO_CORRETTO); altrimenti usa poi Applica al disegno",
                    "Change the original polylines straight away (no copy on RILIEVO_CORRETTO); otherwise use Apply to drawing later"
                ),
                saved.replace,
            ))
            .with_control(DialogControl::row(vec![
                checkbox("dims", tr!("Quote", "Dimensions"), saved.dims),
                checkbox("table", tr!("Tabella scarti", "Residuals table"), saved.table),
                checkbox("csv", tr!("Salva libretto (CSV)", "Save book (CSV)"), saved.csv),
            ]))
            .with_control(csv_picker(&saved.csv_path))
            .submit_label("OK ▸")
            .cancel_label(tr!("◂ Indietro", "◂ Back"))
            .width(660.0)
    }

    /// A name not used yet: P1, P2 … (F1 … for windows).
    fn free_name(&self, prefix: &str) -> String {
        (1..)
            .map(|k| format!("{prefix}{k}"))
            .find(|n| !self.saved.openings.iter().any(|r| cell(r, 0).eq_ignore_ascii_case(n)))
            .unwrap_or_default()
    }

    fn door_page(&self, spot: &DoorSpot) -> DialogDef {
        let s = &self.survey;
        let d = |id: &str| get_text(&self.draft, id);
        let (c1, c2) = (s.corner(spot.room, spot.from), s.corner(spot.room, spot.to));
        let mut def = DialogDef::new(match spot.other {
            Some((q, _, _)) if self.multi() => tr!(
                format!("Porta tra il vano {} e il vano {}", spot.room + 1, q + 1),
                format!("Door between room {} and room {}", spot.room + 1, q + 1)
            ),
            _ => tr!(format!("Porta o finestra sul lato {c1}–{c2}"), format!("Door or window on side {c1}–{c2}")),
        });
        if let Some(m) = &self.msg {
            def = def.with_control(info(format!("⚠ {m}.")));
        }
        def = def.with_control(info(match spot.other {
            Some((q, k, l)) => tr!(
                format!(
                    "Muro tra il lato {c1}–{c2} (vano {}) e il lato {}–{} (vano {}). Con lo spessore del muro e le due distanze \
                     dagli spigoli, questa porta collega i due vani.",
                    spot.room + 1,
                    s.corner(q, k),
                    s.corner(q, l),
                    q + 1
                ),
                format!(
                    "Wall between side {c1}–{c2} (room {}) and side {}–{} (room {}). With the wall thickness and the two \
                     distances from the corners, this door links the two rooms.",
                    spot.room + 1,
                    s.corner(q, k),
                    s.corner(q, l),
                    q + 1
                )
            ),
            None => tr!(
                format!("Lato {c1}–{c2}: dietro questo muro non c'è un altro vano scelto (finestra o porta verso l'esterno)."),
                format!("Side {c1}–{c2}: no other chosen room behind this wall (a window or an outside door).")
            ),
        }));
        let kind_default = if spot.other.is_some() { 0 } else { 1 };
        let name_default = if d("name").is_empty() { self.free_name(if spot.other.is_some() { "P" } else { "F" }) } else { d("name") };
        def = def
            .with_control(DialogControl::row(vec![
                tf("name".into(), tr!("Nome", "Name").into(), name_default),
                DialogControl::Choice {
                    id: "kind".into(),
                    label: tr!("Tipo", "Kind").into(),
                    options: vec![tr!("porta", "door").into(), tr!("finestra", "window").into(), tr!("apertura senza serramento", "plain opening").into()],
                    default_index: get_choice(&self.draft, "kind", kind_default),
                },
            ]))
            .with_control(DialogControl::row(vec![
                DialogControl::Choice {
                    id: "from1".into(),
                    label: tr!("Distanza misurata dallo spigolo", "Distance measured from corner").into(),
                    options: vec![c1.clone(), c2.clone()],
                    default_index: get_choice(&self.draft, "from1", 0),
                },
                tf("d1".into(), tr!("Distanza dallo spigolo allo stipite", "Distance from corner to jamb").into(), d("d1")),
                tf("w".into(), tr!("Larghezza", "Width").into(), d("w")),
            ]))
            .with_control(tf(
                "t".into(),
                tr!("Spessore del muro, misurato nella porta", "Wall thickness, measured in the door").into(),
                d("t"),
            ));
        if let Some((q, k, l)) = spot.other {
            def = def
                .with_control(DialogControl::row(vec![
                    DialogControl::Choice {
                        id: "from2".into(),
                        label: tr!(format!("Nel vano {}: distanza dallo spigolo", q + 1), format!("In room {}: distance from corner", q + 1)),
                        options: vec![s.corner(q, k), s.corner(q, l)],
                        default_index: get_choice(&self.draft, "from2", 0),
                    },
                    tf(
                        "d2".into(),
                        tr!(format!("Distanza allo stipite, misurata nel vano {}", q + 1), format!("Distance to the jamb, measured in room {}", q + 1)),
                        d("d2"),
                    ),
                ]))
                .with_control(info(
                    tr!(
                        "Se lasci vuota la seconda distanza, i vani restano legati dal solo spessore del muro e la loro posizione \
                         lungo il muro viene presa dal disegno.",
                        "If the second distance is left empty, the rooms are tied by the wall thickness only and their position \
                         along the wall is taken from the drawing."
                    )
                    .to_string(),
                ));
        }
        def.submit_label(tr!("Aggiungi ▸", "Add ▸")).cancel_label(tr!("◂ Indietro", "◂ Back")).width(700.0)
    }

    fn read_room(&mut self, r: usize, m: &HashMap<String, DialogValue>) {
        let off = self.offset(r);
        for i in 0..self.survey.rooms[r].len() {
            let id = format!("s{i}");
            if let (true, Some(row)) = (m.contains_key(&id), self.saved.sides.get_mut(off + i)) {
                row.resize(3, String::new());
                row[2] = get_text(m, &id);
            }
        }
        let (others, cons) = self.room_view(r);
        let rows = self.read_distances(Some(r), &others, m);
        let (mut per, links) = self.split_others();
        per[r] = rows;
        self.saved.others = per.into_iter().flatten().chain(links).collect();

        let mut mine = Vec::new();
        for (k, old) in cons.iter().enumerate() {
            let (a, b) = if m.contains_key(&format!("c{k}a")) {
                (self.norm(r, &get_text(m, &format!("c{k}a"))), get_text(m, &format!("c{k}b")))
            } else {
                (cell(old, 0), cell(old, 1))
            };
            if !(a.is_empty() && b.is_empty()) {
                mine.push(vec![a, b]);
            }
        }
        let mut per = self.split_cons();
        per[r] = mine;
        self.saved.constraints = per.into_iter().flatten().collect();
    }

    /// The Da / A / Misurata rows of a page, with the drawn value refreshed.
    fn read_distances(&self, room: Option<usize>, shown: &[Vec<String>], m: &HashMap<String, DialogValue>) -> Vec<Vec<String>> {
        let dec = commands::decimals(self.saved.tol);
        let mut rows = Vec::new();
        for (k, old) in shown.iter().enumerate() {
            let (a, b, v) = if m.contains_key(&format!("o{k}a")) {
                let (a, b) = (get_text(m, &format!("o{k}a")), get_text(m, &format!("o{k}b")));
                match room {
                    Some(r) => (self.norm(r, &a), self.norm(r, &b), get_text(m, &format!("o{k}m"))),
                    None => (a.to_ascii_uppercase(), b.to_ascii_uppercase(), get_text(m, &format!("o{k}m"))),
                }
            } else {
                (cell(old, 0), cell(old, 1), cell(old, 3))
            };
            if a.is_empty() && b.is_empty() && v.is_empty() {
                continue;
            }
            let drawn = self.survey.drawn_other(&a, &b).map(|d| commands::fmt_num(d, dec)).unwrap_or_default();
            rows.push(vec![a, b, drawn, v]);
        }
        rows
    }

    fn read_links(&mut self, m: &HashMap<String, DialogValue>) {
        let rows = self.read_distances(None, &self.links_view(), m);
        let (per, _) = self.split_others();
        self.saved.others = per.into_iter().flatten().chain(rows).collect();
    }

    fn read_openings(&mut self, m: &HashMap<String, DialogValue>) {
        let mut ops = Vec::new();
        for (k, old) in self.openings_view().iter().enumerate() {
            let row: Vec<String> =
                (0..6).map(|c| if m.contains_key(&format!("p{k}_{c}")) { get_text(m, &format!("p{k}_{c}")) } else { cell(old, c) }).collect();
            if row.iter().any(|x| !x.is_empty()) {
                ops.push(row);
            }
        }
        self.saved.openings = ops;
    }

    fn read_options(&mut self, m: &HashMap<String, DialogValue>) {
        let s = &mut self.saved;
        if let Some(v) = get_num(m, "h").filter(|v| *v > 0.0) {
            s.h = v;
        }
        if let Some(v) = get_num(m, "tol").filter(|v| *v >= 0.0) {
            s.tol = v;
        }
        s.unit = get_choice(m, "unit", s.unit);
        s.dims = get_bool(m, "dims", s.dims);
        s.table = get_bool(m, "table", s.table);
        s.csv = get_bool(m, "csv", s.csv);
        s.fill = get_bool(m, "fill", s.fill);
        s.replace = get_bool(m, "replace", s.replace);
        if let Some(p) = get_path(m, "csv_path") {
            s.csv_path = p;
        }
    }

    /// Add the door or window typed on the door page; an error says what
    /// is missing and nothing is added.
    fn read_door(&mut self, spot: &DoorSpot, m: &HashMap<String, DialogValue>) -> Result<(), String> {
        let s = &self.survey;
        let num = |id: &str| commands::parse_num(&get_text(m, id));
        let name = match get_text(m, "name") {
            n if n.is_empty() => self.free_name("P"),
            n => n,
        };
        if self.saved.openings.iter().any(|r| cell(r, 0).eq_ignore_ascii_case(&name)) {
            return Err(tr!(format!("il nome {name} è già usato: scegline un altro"), format!("the name {name} is already used: choose another")));
        }
        let (d1, w, t) = (get_text(m, "d1"), get_text(m, "w"), get_text(m, "t"));
        if !(num("d1").is_some_and(|v| v >= 0.0) && num("w").is_some_and(|v| v > 0.0)) {
            return Err(tr!(
                "scrivi la distanza dallo spigolo allo stipite e la larghezza".to_string(),
                "type the distance from the corner to the jamb and the width".to_string()
            ));
        }
        if !t.is_empty() && !num("t").is_some_and(|v| v > 0.0) {
            return Err(tr!(format!("lo spessore \"{t}\" non è una misura valida"), format!("the thickness \"{t}\" is not a valid measure")));
        }
        let kind = match get_choice(m, "kind", if spot.other.is_some() { 0 } else { 1 }) {
            0 => tr!("porta", "door"),
            1 => tr!("finestra", "window"),
            _ => tr!("apertura", "opening"),
        };
        let (i, j) = if get_choice(m, "from1", 0) == 0 { (spot.from, spot.to) } else { (spot.to, spot.from) };
        let side1 = format!("{}-{}", s.corner(spot.room, i), s.corner(spot.room, j));
        let mut second = None;
        let mut walls = Vec::new();
        if let Some((q, k, l)) = spot.other {
            let (k, l) = if get_choice(m, "from2", 0) == 0 { (k, l) } else { (l, k) };
            let d2 = get_text(m, "d2");
            if !d2.is_empty() {
                if !num("d2").is_some_and(|v| v >= 0.0) {
                    return Err(tr!(format!("la distanza \"{d2}\" non è una misura valida"), format!("the distance \"{d2}\" is not a valid measure")));
                }
                if t.is_empty() {
                    return Err(tr!(
                        "per collegare i due vani scrivi lo spessore del muro, misurato nella porta".to_string(),
                        "to link the two rooms type the wall thickness, measured in the door".to_string()
                    ));
                }
                second = Some(vec![name.clone(), format!("{}-{}", s.corner(q, k), s.corner(q, l)), kind.to_string(), d2, w.clone(), String::new()]);
            } else if !t.is_empty() {
                let dec = commands::decimals(self.saved.tol);
                for c in [k, l] {
                    let b = s.corner(q, c);
                    if !self.saved.others.iter().any(|r| cell(r, 0).eq_ignore_ascii_case(&side1) && cell(r, 1).eq_ignore_ascii_case(&b)) {
                        let drawn = s.drawn_other(&side1, &b).map(|d| commands::fmt_num(d, dec)).unwrap_or_default();
                        walls.push(vec![side1.clone(), b, drawn, t.clone()]);
                    }
                }
            }
        }
        self.saved.openings.push(vec![name, side1, kind.to_string(), d1, w, t]);
        self.saved.openings.extend(second);
        self.saved.others.extend(walls);
        Ok(())
    }
}

impl InteractiveCommand for MeasureDialog {
    fn prompt(&self) -> String {
        tr!(
            "RV_VANO: scegli cosa fare nella finestra (apri \"Come si usa\" per l'aiuto)",
            "RV_VANO: choose what to do in the dialog (open \"How it works\" for help)"
        )
        .into()
    }
    fn on_point(&mut self, _pt: [f64; 3]) -> CommandStep {
        CommandStep::NeedPoint
    }
    fn on_enter(&mut self) -> CommandStep {
        CommandStep::Cancel
    }
    fn modal_dialog(&self) -> Option<DialogDef> {
        match &self.page {
            Page::Hub => Some(self.hub()),
            Page::Room(r) => Some(self.room_page(*r)),
            Page::Links => Some(self.links_page()),
            Page::Openings => Some(self.openings_page()),
            Page::Options => Some(self.options_page()),
            Page::Door(spot) => Some(self.door_page(spot)),
            Page::Done => None,
        }
    }
    fn on_dialog_response(&mut self, response: DialogResponse) -> CommandStep {
        let page = self.page.clone();
        match (page, response) {
            (Page::Done, _) => CommandStep::Cancel,
            (Page::Hub, DialogResponse::Submitted(m)) => {
                let (acts, _, default) = self.actions();
                let act = acts.get(get_choice(&m, "do", default)).copied().unwrap_or(Action::Compute);
                self.msg = None;
                self.keep();
                match act {
                    Action::Room(r) => self.page = Page::Room(r),
                    Action::Links => self.page = Page::Links,
                    Action::Openings => self.page = Page::Openings,
                    Action::Options => self.page = Page::Options,
                    Action::AddDoor => {
                        self.page = Page::Done;
                        return deferred("RV_PORTAPICK", self.key());
                    }
                    Action::Compute => {
                        self.page = Page::Done;
                        commands::set_pending(self.handles.clone(), self.saved.clone());
                        return CommandStep::Dispatch("RV_VANOAPPLY".into());
                    }
                }
                CommandStep::NeedPoint
            }
            (Page::Hub, _) => {
                self.keep();
                self.page = Page::Done;
                CommandStep::Cancel
            }
            (Page::Door(spot), DialogResponse::Submitted(m)) => {
                match self.read_door(&spot, &m) {
                    Ok(()) => {
                        self.msg = None;
                        self.draft.clear();
                        self.keep();
                        self.page = Page::Hub;
                    }
                    Err(e) => {
                        self.msg = Some(e);
                        self.draft = m;
                    }
                }
                CommandStep::NeedPoint
            }
            (page, DialogResponse::Submitted(m)) => {
                match page {
                    Page::Room(r) => self.read_room(r, &m),
                    Page::Links => self.read_links(&m),
                    Page::Openings => self.read_openings(&m),
                    Page::Options => self.read_options(&m),
                    _ => {}
                }
                self.keep();
                self.page = Page::Hub;
                CommandStep::NeedPoint
            }
            (_, _) => {
                // "Back": the host sends no values with it; nothing changes.
                self.msg = None;
                self.draft.clear();
                self.page = Page::Hub;
                CommandStep::NeedPoint
            }
        }
    }
}

// ── Letter size ─────────────────────────────────────────────────────────────

/// Asks the height of the corner letters before writing them. Texts in a
/// drawing have a height in drawing units: they grow with the zoom like the
/// lines. A smaller height keeps them apart where corners crowd together.
pub struct LetterSizeDialog {
    key: String,
    def: DialogDef,
    done: bool,
}

impl LetterSizeDialog {
    pub fn new(key: String, h: f64, auto: f64) -> Self {
        let fmt = |v: f64| commands::fmt_num(v, 3);
        let def = DialogDef::new(tr!("Lettere vertici — dimensione", "Corner letters — size"))
            .with_control(info(tr!(
                format!(
                    "Altezza delle lettere, nelle unità del disegno. Proposta per questi vani: {}. Se le lettere si \
                     ammucchiano su riseghe e spigoli vicini, rimpiccioliscile (per esempio alla metà); se si leggono male \
                     stampate, ingrandiscile. Rieseguendo il comando le lettere vengono riscritte con la nuova altezza.",
                    fmt(auto)
                ),
                format!(
                    "Letter height, in drawing units. Proposed for these rooms: {}. If the letters crowd on recesses and \
                     close corners, make them smaller (for example half); if they read badly in print, make them bigger. \
                     Running the command again rewrites the letters at the new height.",
                    fmt(auto)
                )
            )))
            .with_control(DialogControl::Number {
                id: "h".into(),
                label: tr!("Altezza lettere", "Letter height").into(),
                default: h,
                min: Some(0.0),
                max: None,
                step: Some(auto / 4.0),
            })
            .submit_label(tr!("Scrivi le lettere", "Write the letters"))
            .cancel_label(tr!("Annulla", "Cancel"))
            .width(520.0);
        LetterSizeDialog { key, def, done: false }
    }
}

impl InteractiveCommand for LetterSizeDialog {
    fn prompt(&self) -> String {
        tr!("RV_ETICHETTE: scegli l'altezza delle lettere", "RV_ETICHETTE: choose the letter height").into()
    }
    fn on_point(&mut self, _pt: [f64; 3]) -> CommandStep {
        CommandStep::NeedPoint
    }
    fn on_enter(&mut self) -> CommandStep {
        CommandStep::Cancel
    }
    fn modal_dialog(&self) -> Option<DialogDef> {
        (!self.done).then(|| self.def.clone())
    }
    fn on_dialog_response(&mut self, response: DialogResponse) -> CommandStep {
        self.done = true;
        let DialogResponse::Submitted(m) = response else {
            return CommandStep::Cancel;
        };
        match get_num(&m, "h").filter(|v| *v > 0.0) {
            Some(h) => deferred("RV_ETICHETTEAPPLY", format!("{} {h}", self.key)),
            None => CommandStep::Cancel,
        }
    }
}

// ── Measures book dialog ────────────────────────────────────────────────────

pub struct BookDialog {
    def: DialogDef,
    done: bool,
}

impl BookDialog {
    pub fn new(default_path: String) -> Self {
        let def = DialogDef::new(tr!("Libretto delle misure", "Measures book"))
            .with_control(info(
                tr!(
                    "Salva l'ultimo calcolo come file CSV: misure con calcolate e scarti, σ₀, coordinate dei vertici, superfici \
                     e perimetri dei vani. Si apre con Excel o LibreOffice Calc.",
                    "Saves the last calculation as a CSV file: measures with computed values and residuals, σ₀, corner \
                     coordinates, room areas and perimeters. It opens in Excel or LibreOffice Calc."
                )
                .to_string(),
            ))
            .with_control(csv_picker(&default_path))
            .with_control(info(
                tr!(
                    "Le coordinate sono nel sistema del disegno, con il vertice A (o 1A) dove l'avevi disegnato.",
                    "Coordinates are in the drawing system, with corner A (or 1A) where you drew it."
                )
                .to_string(),
            ))
            .submit_label(tr!("Salva", "Save"))
            .cancel_label(tr!("Annulla", "Cancel"))
            .width(560.0);
        BookDialog { def, done: false }
    }
}

impl InteractiveCommand for BookDialog {
    fn prompt(&self) -> String {
        tr!("RV_LIBRETTO: scegli dove salvare il libretto", "RV_LIBRETTO: choose where to save the book").into()
    }
    fn on_point(&mut self, _pt: [f64; 3]) -> CommandStep {
        CommandStep::NeedPoint
    }
    fn on_enter(&mut self) -> CommandStep {
        CommandStep::Cancel
    }
    fn modal_dialog(&self) -> Option<DialogDef> {
        (!self.done).then(|| self.def.clone())
    }
    fn on_dialog_response(&mut self, response: DialogResponse) -> CommandStep {
        self.done = true;
        let DialogResponse::Submitted(m) = response else {
            return CommandStep::Cancel;
        };
        match get_path(&m, "csv_path") {
            Some(p) => {
                commands::set_pending_book_path(p);
                CommandStep::Dispatch("RV_LIBRETTOAPPLY".into())
            }
            None => CommandStep::Cancel,
        }
    }
}

// ── Guide ───────────────────────────────────────────────────────────────────

pub fn guide_sections() -> Vec<(&'static str, &'static str)> {
    vec![
        (
            tr!("Avvertenza", "Notice"),
            tr!(
                "Progetto hobbistico, realizzato per divertimento e non adatto a un uso professionale: controlla sempre i \
                 risultati. L'autore non si assume responsabilità per il suo uso.",
                "Hobby project, made for fun and not suitable for professional use: always check the results. The author \
                 takes no responsibility for its use."
            ),
        ),
        (
            tr!("1. Disegna i vani", "1. Draw the rooms"),
            tr!(
                "Disegna ogni vano come polilinea chiusa, a occhio, con i comandi del CAD (PLINEA … Chiudi). Contano solo \
                 il numero e l'ordine dei vertici: forma e dimensioni le sistema il plugin.",
                "Draw each room as a closed polyline, by eye, with the CAD's commands (PLINE … Close). Only the number and \
                 order of the corners matter: the plugin fixes shape and size."
            ),
        ),
        (
            tr!("2. Lettere vertici", "2. Corner letters"),
            tr!(
                "Seleziona i vani (un clic sulla linea, o una finestra per prenderne tanti; Invio): sugli angoli compaiono A, B, C … o, con più vani, 1A, \
                 1B … 2A … Una piccola finestra chiede l'altezza delle lettere: rimpiccioliscile dove gli spigoli sono fitti. \
                 Stampa la pianta e usala in cantiere per segnare le misure.",
                "Select the rooms (click a line, or drag a window for many; Enter): A, B, C … or, with several rooms, 1A, 1B … 2A … \
                 appear at the corners. A small window asks the letter height: make it smaller where corners crowd together. \
                 Print the plan and use it on site to note the measures."
            ),
        ),
        (
            tr!("3. Misure e diagonali", "3. Sides and diagonals"),
            tr!(
                "Scegli gli stessi vani: si apre un elenco dei vani con il loro stato. Apri un vano alla volta e scrivi lati e \
                 diagonali (un vano di n spigoli vuole tutti i lati e almeno n−3 diagonali o vincoli); a ogni OK l'elenco \
                 passa al vano successivo e alla fine propone Calcola. Le misure restano memorizzate anche chiudendo la finestra.",
                "Choose the same rooms: a list of the rooms with their state opens. Open one room at a time and type sides and \
                 diagonals (a room of n corners needs all sides and at least n−3 diagonals or constraints); at every OK the \
                 list moves to the next room and at the end proposes Compute. The measures are kept even if you close the window."
            ),
        ),
        (
            tr!("Porte, finestre e muri tra i vani", "Doors, windows and walls between rooms"),
            tr!(
                "Dall'elenco scegli \"Aggiungi porta o finestra\" e clicca sul disegno, sul muro, dove si trova. Tra due vani il \
                 plugin riconosce i due lati: scrivi la distanza dallo spigolo allo stipite nei due vani, la larghezza e lo \
                 spessore del muro misurato nella porta. Basta questo a collegare i due vani. Un angolo noto o un lato \
                 orizzontale / verticale può prendere il posto di una diagonale.",
                "From the list choose \"Add a door or window\" and click on the drawing, on the wall, where it is. Between two \
                 rooms the plugin finds both sides: type the distance from the corner to the jamb in both rooms, the width \
                 and the wall thickness measured in the door. That alone links the two rooms. A known angle or a horizontal / \
                 vertical side can take the place of a diagonal."
            ),
        ),
        (
            tr!("4. Risultato", "4. Result"),
            tr!(
                "I vani veri vanno sul layer RILIEVO_CORRETTO con la loro superficie in m²; il disegno originale resta. Porte \
                 e finestre su RILIEVO_APERTURE. Su RILIEVO_QUOTE: lettere, quote dei lati (stile RILIEVO), diagonali, \
                 collegamenti, spessori e la tabella misurata / calcolata / scarto, \
                 con in rosso gli scarti oltre la tolleranza. Se le misure non bastano o non tornano, la finestra si \
                 riapre con il motivo.",
                "The true rooms go on layer RILIEVO_CORRETTO with their area in m²; the original drawing stays. Doors and \
                 windows on RILIEVO_APERTURE. On RILIEVO_QUOTE: letters, side dimensions (style RILIEVO), diagonals, links, \
                 thicknesses and the measured / computed / residual table, \
                 residuals over the tolerance in red. If the measures are not enough or do not fit, the dialog reopens \
                 with the reason."
            ),
        ),
        (
            tr!("Applica al disegno", "Apply to drawing"),
            tr!(
                "Controllato il risultato, Applica al disegno (RV_APPLICA) porta i vertici delle polilinee originali nella \
                 posizione calcolata: restano le stesse polilinee, sullo stesso layer, e la copia su RILIEVO_CORRETTO viene \
                 tolta. Ctrl+Z annulla. Nelle Opzioni si può scegliere di farlo subito a ogni calcolo.",
                "Once the result is checked, Apply to drawing (RV_APPLICA) moves the corners of the original polylines to \
                 the computed position: same polylines, same layer, and the copy on RILIEVO_CORRETTO is removed. Ctrl+Z \
                 undoes it. In Options you can choose to do it at every calculation."
            ),
        ),
        (
            tr!("5. Libretto delle misure", "5. Measures book"),
            tr!(
                "Libretto misure (RV_LIBRETTO) salva l'ultimo calcolo in un CSV per Excel o Calc: misure, scarti, σ₀, \
                 coordinate dei vertici, superfici e perimetri. Si può anche spuntare nella finestra delle misure.",
                "Measures book (RV_LIBRETTO) saves the last calculation as a CSV for Excel or Calc: measures, residuals, \
                 σ₀, corner coordinates, areas and perimeters. It can also be ticked in the measures dialog."
            ),
        ),
        (
            tr!("Manuale di rilievo (PDF)", "Surveying manual (PDF)"),
            tr!(
                "Il pulsante Manuale PDF apre una guida passo passo, con disegni, per rilevare un intero appartamento: \
                 eidotipo, misure dei vani, diagonali e controlli, mobili che ostacolano, porte comuni, finestre e muri \
                 esterni, lettura degli scarti, lista di controllo per il cantiere. (In italiano.)",
                "The PDF manual button opens a step-by-step guide, with drawings, to survey a whole flat: sketch, room \
                 measures, diagonals and checks, furniture in the way, shared doors, windows and outer walls, reading the \
                 residuals, a site checklist. (In Italian.)"
            ),
        ),
        (
            tr!("Comandi", "Commands"),
            tr!(
                "RV_VANO, RV_ETICHETTE, RV_APPLICA, RV_LIBRETTO, RV_GUIDE, RV_MANUALE; RV_LANG IT|EN|AUTO cambia la lingua.",
                "RV_VANO, RV_ETICHETTE, RV_APPLICA, RV_LIBRETTO, RV_GUIDE, RV_MANUALE; RV_LANG IT|EN|AUTO changes the language."
            ),
        ),
    ]
}

pub struct GuideDialog {
    def: DialogDef,
    done: bool,
}

impl GuideDialog {
    pub fn new() -> Self {
        let mut def = DialogDef::new(tr!("Rilievo Vani — guida", "Rilievo Vani — guide"));
        for (title, body) in guide_sections() {
            def = def.with_control(info(title.to_string())).with_control(info(body.to_string()));
        }
        def = def.submit_label("OK").cancel_label(tr!("Chiudi", "Close")).width(600.0);
        GuideDialog { def, done: false }
    }
}

impl Default for GuideDialog {
    fn default() -> Self {
        Self::new()
    }
}

impl InteractiveCommand for GuideDialog {
    fn prompt(&self) -> String {
        tr!("Guida di Rilievo Vani", "Rilievo Vani guide").into()
    }
    fn on_point(&mut self, _pt: [f64; 3]) -> CommandStep {
        CommandStep::NeedPoint
    }
    fn on_enter(&mut self) -> CommandStep {
        CommandStep::Done
    }
    fn modal_dialog(&self) -> Option<DialogDef> {
        (!self.done).then(|| self.def.clone())
    }
    fn on_dialog_response(&mut self, _response: DialogResponse) -> CommandStep {
        self.done = true;
        CommandStep::Done
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn saved_for(survey: &Survey) -> Saved {
        let (sides, others) = survey.initial(3, &|v, d| format!("{v:.d$}"));
        Saved {
            sides,
            others,
            h: 0.2,
            tol: 0.02,
            dims: true,
            table: true,
            csv: false,
            csv_path: String::new(),
            constraints: vec![],
            openings: vec![],
            unit: 0,
            fill: true,
            replace: false,
        }
    }

    fn two() -> Survey {
        Survey::new(vec![
            vec![[0.0, 0.0], [4.0, 0.0], [4.0, 3.0], [0.0, 3.0]],
            vec![[4.3, 0.0], [7.0, 0.0], [7.0, 3.0], [4.3, 3.0]],
        ])
    }

    fn text(m: &mut HashMap<String, DialogValue>, id: &str, v: &str) {
        m.insert(id.to_string(), DialogValue::Text(v.to_string()));
    }

    fn pick(d: &MeasureDialog, want: Action) -> HashMap<String, DialogValue> {
        let (acts, labels, _) = d.actions();
        let index = acts.iter().position(|a| *a == want).unwrap();
        let mut m = HashMap::new();
        m.insert("do".to_string(), DialogValue::Choice { index, text: labels[index].clone() });
        m
    }

    #[test]
    fn rooms_chosen_by_window_selection() {
        let room = |h: u64, x: f64, y: f64| Room {
            handle: Handle::new(h),
            pts: vec![[x, y], [x + 3.0, y], [x + 3.0, y + 2.0], [x, y + 2.0]],
            z: 0.0,
            had_arcs: false,
        };
        // Two rows of rooms: 1, 2 on top; 3, 4 below.
        let rooms = vec![room(1, 4.0, 0.0), room(2, 0.0, 3.0), room(3, 0.0, 0.1), room(4, 4.0, 3.2)];
        let mut t = PickRoomTool::new(rooms, "RV_VANOOPEN", "RV_VANO");
        assert!(t.is_selection_gathering() && t.accepts_drag_selection());
        // One room clicked first, then a window over everything (plus a stray object).
        t.on_selection_complete(vec![Handle::new(3)]);
        t.on_selection_complete(vec![Handle::new(1), Handle::new(2), Handle::new(3), Handle::new(4), Handle::new(99)]);
        let order: Vec<u64> = t.chosen.iter().map(|&i| t.rooms[i].handle.value()).collect();
        assert_eq!(order, vec![3, 2, 4, 1], "first pick stays first, the rest in reading order");
        assert_eq!(t.on_cursor_move([0.0; 3]).len(), 4);
        // One deselected in the CAD.
        t.on_selection_complete(vec![Handle::new(2), Handle::new(3), Handle::new(1)]);
        assert_eq!(t.chosen.len(), 3);
        assert!(matches!(t.on_enter(), CommandStep::Dispatch(ref c) if c == "RV_VANOOPEN"));
        assert_eq!(take_deferred("RV_VANOOPEN").as_deref(), Some("3,2,1"));
    }

    #[test]
    fn spot_between_two_rooms() {
        let s = two();
        // In the wall between the rooms, near the bottom.
        let spot = find_spot(&s.rooms, [4.1, 0.8]).unwrap();
        assert_eq!(spot, DoorSpot { room: 0, from: 1, to: 2, other: Some((1, 0, 3)) });
        assert_eq!(DoorSpot::decode(&spot.encode()), Some(spot));
        // On an outer wall: no room behind.
        let spot = find_spot(&s.rooms, [2.0, 3.05]).unwrap();
        assert_eq!((spot.room, spot.other), (0, None));
        // On the far side of room 2: not matched with room 1 across it.
        let spot = find_spot(&s.rooms, [7.02, 1.0]).unwrap();
        assert_eq!((spot.room, spot.other), (1, None));
        assert!(find_spot(&s.rooms, [50.0, 50.0]).is_none());
    }

    #[test]
    fn hub_walks_through_the_rooms() {
        let s = two();
        let mut d = MeasureDialog::new(vec![Handle::new(0x71), Handle::new(0x72)], &s, &saved_for(&s), None);
        let (acts, _, default) = d.actions();
        assert_eq!(acts[default], Action::Room(0), "first incomplete room is proposed");
        assert!(d.modal_dialog().unwrap().title.contains('2'));
        // Open room 2 and fill it, with bare letters for an extra diagonal.
        let m = pick(&d, Action::Room(1));
        assert!(matches!(d.on_dialog_response(DialogResponse::Submitted(m)), CommandStep::NeedPoint));
        let mut m = HashMap::new();
        for i in 0..4 {
            text(&mut m, &format!("s{i}"), "3,00");
        }
        text(&mut m, "o0a", "2A");
        text(&mut m, "o0b", "2C");
        text(&mut m, "o0m", "4,03");
        text(&mut m, "o1a", "b");
        text(&mut m, "o1b", "d");
        text(&mut m, "o1m", "4,04");
        text(&mut m, "o2a", "");
        text(&mut m, "o2b", "");
        text(&mut m, "o2m", "");
        text(&mut m, "c0a", "a");
        text(&mut m, "c0b", "90");
        d.on_dialog_response(DialogResponse::Submitted(m));
        assert_eq!(d.page, Page::Hub);
        assert_eq!(d.saved.sides[4][2], "3,00");
        assert!(d.saved.others.iter().any(|r| r[0] == "2B" && r[1] == "2D" && r[3] == "4,04"), "{:?}", d.saved.others);
        assert_eq!(d.saved.constraints, vec![vec!["2A".to_string(), "90".into()]]);
        // Room 1's proposed diagonal is untouched and still first.
        assert!(d.saved.others[0][0].starts_with('1'));
        let (ok, status) = d.room_status(1);
        assert!(ok && status.contains('2'), "{status}");
        assert!(!d.room_status(0).0);
        // Back from a page changes nothing.
        let m = pick(&d, Action::Options);
        d.on_dialog_response(DialogResponse::Submitted(m));
        assert_eq!(d.page, Page::Options);
        d.on_dialog_response(DialogResponse::Cancelled);
        assert_eq!(d.page, Page::Hub);
        // Compute hands over to the command.
        let m = pick(&d, Action::Compute);
        assert!(matches!(d.on_dialog_response(DialogResponse::Submitted(m)), CommandStep::Dispatch(ref c) if c == "RV_VANOAPPLY"));
        assert!(d.modal_dialog().is_none());
    }

    #[test]
    fn door_by_click_links_the_rooms() {
        let s = two();
        let mut saved = saved_for(&s);
        for r in saved.sides.iter_mut() {
            r[2] = r[1].clone();
        }
        for r in saved.others.iter_mut() {
            r[3] = r[2].clone();
        }
        let spot = find_spot(&s.rooms, [4.1, 0.8]).unwrap();
        let mut d = MeasureDialog::new(vec![Handle::new(0x73), Handle::new(0x74)], &s, &saved, None).starting(Start::Door(spot.clone()));
        assert_eq!(d.unlinked().unwrap(), vec![2]);
        assert!(d.modal_dialog().unwrap().title.contains('1'));
        // Missing width: stays on the page, keeps what was typed.
        let mut m = HashMap::new();
        text(&mut m, "name", "P1");
        text(&mut m, "d1", "1,00");
        d.on_dialog_response(DialogResponse::Submitted(m.clone()));
        assert_eq!(d.page, Page::Door(spot));
        assert!(d.msg.is_some() && d.saved.openings.is_empty());
        text(&mut m, "w", "0,90");
        text(&mut m, "t", "0,30");
        text(&mut m, "d2", "1,00");
        d.on_dialog_response(DialogResponse::Submitted(m));
        assert_eq!(d.page, Page::Hub);
        assert_eq!(d.saved.openings.len(), 2);
        assert_eq!(d.saved.openings[0][1], "1B-1C");
        assert_eq!(d.saved.openings[1][1], "2A-2D");
        assert!(d.unlinked().unwrap().is_empty(), "the door links the rooms");
        let (acts, _, default) = d.actions();
        assert_eq!(acts[default], Action::Compute);
        assert!(d.opening_lines()[0].contains("↔"), "{:?}", d.opening_lines());
    }

    #[test]
    fn door_without_second_distance_gives_wall_thickness() {
        let s = two();
        let spot = find_spot(&s.rooms, [4.1, 0.8]).unwrap();
        let mut d = MeasureDialog::new(vec![Handle::new(0x75), Handle::new(0x76)], &s, &saved_for(&s), None).starting(Start::Door(spot));
        let mut m = HashMap::new();
        text(&mut m, "d1", "1");
        text(&mut m, "w", "0,9");
        text(&mut m, "t", "0,30");
        d.on_dialog_response(DialogResponse::Submitted(m));
        assert_eq!(d.saved.openings.len(), 1);
        let walls: Vec<_> = d.saved.others.iter().filter(|r| r[0] == "1B-1C").collect();
        assert_eq!(walls.len(), 2);
        assert_eq!(walls[0][3], "0,30");
    }

    #[test]
    fn single_room_uses_plain_letters() {
        let s = Survey::new(vec![vec![[0.0, 0.0], [4.0, 0.0], [4.0, 3.0], [0.0, 3.0]]]);
        let mut d = MeasureDialog::new(vec![Handle::new(0x77)], &s, &saved_for(&s), None);
        let m = pick(&d, Action::Room(0));
        d.on_dialog_response(DialogResponse::Submitted(m));
        let mut m = HashMap::new();
        text(&mut m, "o1a", "b");
        text(&mut m, "o1b", "d");
        text(&mut m, "o1m", "5");
        d.on_dialog_response(DialogResponse::Submitted(m));
        assert!(d.saved.others.iter().any(|r| r[0] == "b" && r[3] == "5"));
        // Closing the hub ends the command.
        assert!(matches!(d.on_dialog_response(DialogResponse::Cancelled), CommandStep::Cancel));
    }
}
