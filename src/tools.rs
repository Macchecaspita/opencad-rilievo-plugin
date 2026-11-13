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

const CYAN: [f32; 4] = [0.43, 0.72, 0.93, 1.0];
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

pub struct PickRoomTool {
    rooms: Vec<Room>,
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
                    self.msg = Some(tr!("vano tolto dalla scelta", "room removed from the choice").into());
                } else {
                    self.chosen.push(i);
                    self.msg = Some(tr!(format!("vano {} scelto", self.chosen.len()), format!("room {} chosen", self.chosen.len())));
                }
            }
            None => self.msg = Some(tr!("lì non c'è una polilinea chiusa", "no closed polyline there").into()),
        }
        CommandStep::NeedPoint
    }
}

impl InteractiveCommand for PickRoomTool {
    fn prompt(&self) -> String {
        let p = if self.chosen.is_empty() {
            tr!(
                format!(
                    "{}: clicca dentro il vano (o sulla sua polilinea chiusa). Per collegare più vani cliccali uno dopo l'altro: \
                     l'ordine dà il numero del vano. Invio per continuare, Esc per annullare",
                    self.title
                ),
                format!(
                    "{}: click inside the room (or on its closed polyline). To link several rooms click them one after the \
                     other: the order gives the room number. Enter to go on, Esc to cancel",
                    self.title
                )
            )
        } else {
            tr!(
                format!(
                    "{}: {} vani scelti (in verde). Clicca altri vani da collegare, di nuovo su un vano per toglierlo, Invio per continuare",
                    self.title,
                    self.chosen.len()
                ),
                format!(
                    "{}: {} rooms chosen (green). Click more rooms to link, a chosen room again to drop it, Enter to go on",
                    self.title,
                    self.chosen.len()
                )
            )
        };
        with_msg(&self.msg, p)
    }
    fn on_point(&mut self, pt: [f64; 3]) -> CommandStep {
        let i = self.room_at(pt);
        self.toggle(i)
    }
    fn needs_object_pick(&self) -> bool {
        true
    }
    fn on_object_pick(&mut self, handle: Handle, pt: [f64; 3]) -> CommandStep {
        let i = self.rooms.iter().position(|r| r.handle == handle).or_else(|| self.room_at(pt));
        self.toggle(i)
    }
    fn on_cursor_move(&mut self, pt: [f64; 3]) -> Vec<PreviewWire> {
        let mut w: Vec<PreviewWire> =
            self.chosen.iter().map(|&i| PreviewWire::with_color(ring3(&self.rooms[i].pts, self.rooms[i].z), GREEN)).collect();
        if let Some(i) = self.room_at(pt).filter(|i| !self.chosen.contains(i)) {
            w.push(PreviewWire::with_color(ring3(&self.rooms[i].pts, self.rooms[i].z), CYAN));
        }
        w
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
            "LATI — misura ogni parete da spigolo a spigolo, all'altezza più comoda ma sempre la stessa. \
             Una misura lasciata vuota non viene usata.",
            "SIDES — measure each wall from corner to corner, at a comfortable height but always the same one. \
             An empty measure is not used."
        )
        .to_string(),
        tr!(
            "DIAGONALI — tra due spigoli non vicini dello stesso vano. Un vano di n spigoli ne vuole almeno n−3 \
             (1 per un rettangolo, 3 per una L): quelle proposte passano tutte dentro il vano. Per aggiungerne una, \
             aggiungi una riga e scrivi le due lettere nelle colonne Da e A.",
            "DIAGONALS — between two corners of the same room that are not next to each other. A room of n corners \
             needs at least n−3 (1 for a rectangle, 3 for an L): the proposed ones all run inside the room. To add one, \
             add a row and type the two letters in the From and To columns."
        )
        .to_string(),
        tr!(
            "CONTROLLO — misura sempre almeno una diagonale in più del minimo: è l'unico modo per accorgersi di una \
             cifra letta male. Il plugin calcola allora gli scarti (calcolata − misurata) e colora in rosso quelli \
             oltre la tolleranza.",
            "CHECK — always measure at least one diagonal more than the minimum: it is the only way to notice a \
             misread digit. The plugin then computes the residuals (computed − measured) and shows in red those over \
             the tolerance."
        )
        .to_string(),
    ];
    if multi {
        v.push(
            tr!(
                "VANI COLLEGATI — i vertici si scrivono con il numero del vano: 1A, 2C … Ogni vano oltre al primo ha \
                 bisogno di almeno 3 misure di collegamento, prese verso vertici diversi: per esempio lo spessore del \
                 muro e due misure attraverso la porta (dallo spigolo 1B allo spigolo 2D, da 1C a 2A). Con 4 o più \
                 misure il collegamento è anche controllato.",
                "LINKED ROOMS — corners are written with the room number: 1A, 2C … Each room after the first needs at \
                 least 3 linking measures, to different corners: for example the wall thickness and two measures through \
                 the door (from corner 1B to corner 2D, from 1C to 2A). With 4 or more the link is also checked."
            )
            .to_string(),
        );
        v.push(
            tr!(
                "SPESSORE DEL MURO — nella colonna Da scrivi il lato di un vano (per esempio 1B-1C), nella colonna A il \
                 vertice dell'altro vano dietro quel muro (per esempio 2A); in Misurata lo spessore, preso in squadra \
                 nel vano della porta.",
                "WALL THICKNESS — in From write the side of one room (for example 1B-1C), in To the corner of the other \
                 room behind that wall (for example 2A); in Measured the thickness, taken square in the door opening."
            )
            .to_string(),
        );
    } else {
        v.push(
            tr!(
                "PIÙ VANI — per collegare due o più vani (spessori dei muri, misure attraverso le porte) avvia di nuovo \
                 Misure e diagonali e clicca tutti i vani uno dopo l'altro prima di premere Invio.",
                "SEVERAL ROOMS — to link two or more rooms (wall thicknesses, measures through doors) start Sides and \
                 diagonals again and click all the rooms one after the other before pressing Enter."
            )
            .to_string(),
        );
    }
    v.push(
        tr!(
            "VINCOLI — al posto di una diagonale puoi dire che uno spigolo è retto (C, 90) o ha un altro angolo \
             (C, 135; per uno spigolo rientrante va bene sia 270 sia 90), oppure che un lato è orizzontale o \
             verticale (A-B, orizzontale). Ogni vincolo vale una misura e riceve il suo scarto: se un angolo \
             \"retto\" in realtà non lo è, lo scarto lo mostra. Un lato orizzontale o verticale fissa anche la \
             rotazione del vano sul foglio.",
            "CONSTRAINTS — instead of a diagonal you can say a corner is square (C, 90) or has another angle \
             (C, 135; for an inner corner 270 or 90 both work), or that a side is horizontal or vertical (A-B, \
             horizontal). Each constraint counts as a measure and gets its residual: if a \"square\" corner is \
             not, the residual shows it. A horizontal or vertical side also fixes the room's rotation on the sheet."
        )
        .to_string(),
    );
    v.push(
        tr!(
            "PORTE E FINESTRE — scrivi il lato (per esempio B-C), il tipo, la distanza dal primo spigolo scritto \
             (B) fino allo stipite e la larghezza del vano porta. Scrivendo C-B la distanza si misura da C e la \
             porta si apre dall'altro lato. La porta è disegnata sul layer RILIEVO_APERTURE, incernierata allo \
             stipite più vicino al primo spigolo, con l'anta verso l'interno.",
            "DOORS AND WINDOWS — write the side (for example B-C), the kind, the distance from the first corner \
             written (B) to the jamb and the opening width. Writing C-B measures from C and hinges the door on the \
             other side. Doors are drawn on layer RILIEVO_APERTURE, hinged at the jamb nearer the first corner, \
             opening inwards."
        )
        .to_string(),
    );
    if multi {
        v.push(
            tr!(
                "PORTA COMUNE — la stessa porta vista dai due vani: scrivila una volta per ciascun vano con lo \
                 stesso nome (P1) e, in una delle due righe, lo spessore del muro. Basta questo a collegare i due \
                 vani: non servono altre misure di collegamento (aggiungerne una fa da controllo).",
                "SHARED DOOR — the same door seen from both rooms: write it once for each room with the same name \
                 (P1) and, on one of the two rows, the wall thickness. That alone links the two rooms: no other \
                 linking measures are needed (adding one is a check)."
            )
            .to_string(),
        );
    }
    v.push(
        tr!(
            "MISURE MANCANTI — se un mobile impedisce una diagonale, lasciala vuota: con l'opzione \"completa la \
             forma\" (pagina 2, attiva di solito) il plugin usa per quella parte gli angoli del disegno e lo segnala in \
             rosso. Il risultato è buono quanto il disegno in quel punto: meglio un vincolo (angolo retto) o una \
             diagonale tra altri spigoli, se si può.",
            "MISSING MEASURES — if furniture blocks a diagonal, leave it empty: with \"finish the shape\" (page 2, \
             usually on) the plugin uses the drawing's angles for that part and says so in red. The result is as good \
             as the drawing there: a constraint (square corner) or a diagonal between other corners is better, if \
             possible."
        )
        .to_string(),
    );
    v.push(
        tr!(
            "UNITÀ — le misure vanno nelle unità del disegno (metri, centimetri o millimetri, come hai disegnato); la \
             virgola decimale va bene. Il vertice A (o 1A) resta dov'è e il lato A–B mantiene la direzione disegnata.",
            "UNITS — type the measures in drawing units (metres, centimetres or millimetres, as you drew); a decimal \
             comma is fine. Corner A (or 1A) stays where it is and side A–B keeps its drawn direction."
        )
        .to_string(),
    );
    v
}

// ── Measures dialog ─────────────────────────────────────────────────────────

/// The measures dialog, in two pages: 1. sides and diagonals (the
/// measures), 2. constraints, doors and windows, options. "Back" on page 2
/// returns to page 1 with everything typed kept.
///
/// Plain text boxes rather than table controls: the host sizes its dialog
/// window to the content, and a table's rows are measured at their minimum
/// height, so a table opened squashed and had to be stretched by hand.
/// Boxes are measured at their real height, so the whole page shows at once.
/// Spare empty rows take extra measures; filled ones survive a reopening,
/// with new spare rows after them.
pub struct MeasureDialog {
    handles: Vec<Handle>,
    survey: Survey,
    msg: Option<String>,
    saved: Saved,
    page: u8,
    /// Rows shown on the current page, by kind (for reading the answer).
    shown_others: usize,
    shown_constraints: usize,
    shown_openings: usize,
}

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

fn filled(row: &[String], cols: &[usize]) -> bool {
    cols.iter().any(|&c| row.get(c).is_some_and(|s| !s.trim().is_empty()))
}

impl MeasureDialog {
    /// `saved`: what the user typed last time for these rooms (kept when the
    /// calculation failed, so nothing has to be typed again).
    pub fn new(handles: Vec<Handle>, survey: &Survey, saved: &Saved, msg: Option<String>) -> Self {
        MeasureDialog {
            handles,
            survey: survey.clone(),
            msg,
            saved: saved.clone(),
            page: 1,
            shown_others: 0,
            shown_constraints: 0,
            shown_openings: 0,
        }
    }

    fn page1(&mut self) -> DialogDef {
        let multi = self.survey.multi();
        let n_rooms = self.survey.rooms.len();
        let need: usize = self.survey.rooms.iter().map(|r| r.len().saturating_sub(3)).sum();
        let ranges = self.survey.ranges();
        let links = if multi { 3 * (n_rooms - 1) } else { 0 };
        let title = if multi {
            tr!(format!("Rilievo di {n_rooms} vani collegati — 1/2: misure"), format!("Survey of {n_rooms} linked rooms — 1/2: measures"))
        } else {
            tr!("Rilievo vano — 1/2: lati e diagonali".to_string(), "Room survey — 1/2: sides and diagonals".to_string())
        };
        let mut def = DialogDef::new(title);
        if let Some(m) = &self.msg {
            def = def.with_control(info(tr!(
                format!("⚠ Da correggere: {m}. Le misure scritte sono rimaste: correggi e premi di nuovo Calcola."),
                format!("⚠ To fix: {m}. The measures typed are still here: fix them and press Compute again.")
            )));
        }
        let summary = if multi {
            tr!(
                format!(
                    "Vertici: {ranges}. Servono tutti i lati, almeno {need} diagonali (o vincoli, a pagina 2) e almeno \
                     {links} misure di collegamento, oppure una porta comune per ogni vano collegato."
                ),
                format!(
                    "Corners: {ranges}. All sides are needed, at least {need} diagonals (or constraints, on page 2) and at \
                     least {links} linking measures, or one shared door per linked room."
                )
            )
        } else {
            tr!(
                format!("Vertici: {ranges}. Servono tutti i lati e almeno {need} diagonali (o vincoli, a pagina 2); ogni misura in più fa da controllo."),
                format!("Corners: {ranges}. All sides are needed plus at least {need} diagonals (or constraints, on page 2); every extra measure is a check.")
            )
        };
        def = def
            .with_control(info(summary))
            .with_control(help_section("help", tr!("Come misurare (aiuto) ▸", "How to measure (help) ▸"), how_to_measure(multi)))
            .with_control(info(
                tr!(
                    "LATI — scrivi la misura sotto ogni lato (tra parentesi la lunghezza disegnata).",
                    "SIDES — type the measure under each side (the drawn length in brackets)."
                )
                .to_string(),
            ));
        let sides: Vec<DialogControl> = self
            .saved
            .sides
            .iter()
            .enumerate()
            .map(|(i, r)| {
                let name = r.first().cloned().unwrap_or_default();
                let drawn = r.get(1).cloned().unwrap_or_default();
                tf(format!("s{i}"), format!("{name}  ({drawn})"), r.get(2).cloned().unwrap_or_default())
            })
            .collect();
        for row in grid(sides, 4) {
            def = def.with_control(row);
        }

        def = def.with_control(info(if multi {
            tr!(
                "DIAGONALI E ALTRE MISURE — Da e A: due vertici (1A, 2C) per una distanza, oppure un lato (1B-1C) e un vertice (2A) per lo spessore di un muro. Le righe vuote servono per misure in più.",
                "DIAGONALS AND OTHER MEASURES — From and To: two corners (1A, 2C) for a distance, or a side (1B-1C) and a corner (2A) for a wall thickness. Empty rows take extra measures."
            )
            .to_string()
        } else {
            tr!(
                "DIAGONALI — quelle proposte passano dentro il vano; nelle righe vuote puoi scriverne altre (Da, A, Misurata).",
                "DIAGONALS — the proposed ones run inside the room; use the empty rows for more (From, To, Measured)."
            )
            .to_string()
        }));
        let kept: Vec<Vec<String>> = self.saved.others.iter().filter(|r| filled(r, &[0, 1, 3])).cloned().collect();
        let spare = if multi { 4 } else { 2 };
        let total = kept.len() + spare;
        self.shown_others = total;
        let mut cells = Vec::new();
        for k in 0..total {
            let r = kept.get(k).cloned().unwrap_or_else(|| vec![String::new(); 4]);
            let drawn = r.get(2).filter(|d| !d.is_empty()).map(|d| format!(" ({d})")).unwrap_or_default();
            cells.push(tf(format!("o{k}a"), tr!("Da", "From").into(), r[0].clone()));
            cells.push(tf(format!("o{k}b"), tr!("A", "To").into(), r[1].clone()));
            cells.push(tf(format!("o{k}m"), format!("{}{drawn}", tr!("Misurata", "Measured")), r.get(3).cloned().unwrap_or_default()));
        }
        for row in grid(cells, 6) {
            def = def.with_control(row);
        }
        def.submit_label(tr!("Avanti ▸", "Next ▸")).cancel_label(tr!("Annulla", "Cancel")).width(700.0)
    }

    fn page2(&mut self) -> DialogDef {
        let mut def = DialogDef::new(tr!("Rilievo — 2/2: vincoli, porte e finestre, opzioni", "Survey — 2/2: constraints, doors and windows, options"))
            .with_control(info(
                tr!(
                    "VINCOLI (facoltativi, ognuno vale una misura) — uno spigolo e il suo angolo (C, 90), oppure un lato e orizzontale / verticale (A-B, orizzontale).",
                    "CONSTRAINTS (optional, each counts as a measure) — a corner and its angle (C, 90), or a side and horizontal / vertical (A-B, horizontal)."
                )
                .to_string(),
            ));
        let kept: Vec<Vec<String>> = self.saved.constraints.iter().filter(|r| filled(r, &[0, 1])).cloned().collect();
        let total = (kept.len() + 2).max(3).div_ceil(3) * 3;
        self.shown_constraints = total;
        let mut cells = Vec::new();
        for k in 0..total {
            let r = kept.get(k).cloned().unwrap_or_else(|| vec![String::new(); 2]);
            cells.push(tf(format!("c{k}a"), tr!("Spigolo o lato", "Corner or side").into(), r[0].clone()));
            cells.push(tf(format!("c{k}b"), tr!("Vincolo", "Constraint").into(), r.get(1).cloned().unwrap_or_default()));
        }
        for row in grid(cells, 6) {
            def = def.with_control(row);
        }

        def = def.with_control(info(
            tr!(
                "PORTE E FINESTRE (facoltative) — lato (B-C), tipo (porta, finestra, apertura), distanza dal primo spigolo scritto, larghezza; stesso nome in due vani = porta comune (con lo spessore del muro).",
                "DOORS AND WINDOWS (optional) — side (B-C), kind (door, window, opening), distance from the first corner written, width; same name in two rooms = shared door (with the wall thickness)."
            )
            .to_string(),
        ));
        let kept: Vec<Vec<String>> = self.saved.openings.iter().filter(|r| filled(r, &[0, 1, 2, 3, 4, 5])).cloned().collect();
        let total = kept.len() + 2;
        self.shown_openings = total;
        let labels = [
            tr!("Nome", "Name"),
            tr!("Lato", "Side"),
            tr!("Tipo", "Kind"),
            tr!("Distanza", "Distance"),
            tr!("Larghezza", "Width"),
            tr!("Spessore muro", "Wall"),
        ];
        let mut cells = Vec::new();
        for k in 0..total {
            let r = kept.get(k).cloned().unwrap_or_else(|| vec![String::new(); 6]);
            for (c, l) in labels.iter().enumerate() {
                cells.push(tf(format!("p{k}_{c}"), l.to_string(), r.get(c).cloned().unwrap_or_default()));
            }
        }
        for row in grid(cells, 6) {
            def = def.with_control(row);
        }

        let saved = &self.saved;
        def.with_control(DialogControl::row(vec![
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
        .with_control(DialogControl::row(vec![
            checkbox("dims", tr!("Quote", "Dimensions"), saved.dims),
            checkbox("table", tr!("Tabella scarti", "Residuals table"), saved.table),
            checkbox("csv", tr!("Salva libretto (CSV)", "Save book (CSV)"), saved.csv),
        ]))
        .with_control(csv_picker(&saved.csv_path))
        .submit_label(tr!("Calcola", "Compute"))
        .cancel_label(tr!("◂ Indietro", "◂ Back"))
        .width(700.0)
    }

    fn read_page1(&mut self, m: &HashMap<String, DialogValue>) {
        for (i, row) in self.saved.sides.iter_mut().enumerate() {
            if m.contains_key(&format!("s{i}")) {
                while row.len() < 3 {
                    row.push(String::new());
                }
                row[2] = get_text(m, &format!("s{i}"));
            }
        }
        let dec = commands::decimals(self.saved.tol);
        let mut others = Vec::new();
        for k in 0..self.shown_others {
            let (a, b, v) = (get_text(m, &format!("o{k}a")), get_text(m, &format!("o{k}b")), get_text(m, &format!("o{k}m")));
            if a.is_empty() && b.is_empty() && v.is_empty() {
                continue;
            }
            let drawn = self.survey.drawn_other(&a, &b).map(|d| commands::fmt_num(d, dec)).unwrap_or_default();
            others.push(vec![a, b, drawn, v]);
        }
        self.saved.others = others;
    }

    fn read_page2(&mut self, m: &HashMap<String, DialogValue>) {
        let mut cons = Vec::new();
        for k in 0..self.shown_constraints {
            let (a, b) = (get_text(m, &format!("c{k}a")), get_text(m, &format!("c{k}b")));
            if !(a.is_empty() && b.is_empty()) {
                cons.push(vec![a, b]);
            }
        }
        self.saved.constraints = cons;
        let mut ops = Vec::new();
        for k in 0..self.shown_openings {
            let row: Vec<String> = (0..6).map(|c| get_text(m, &format!("p{k}_{c}"))).collect();
            if row.iter().any(|x| !x.is_empty()) {
                ops.push(row);
            }
        }
        self.saved.openings = ops;
        let s = &mut self.saved;
        if let Some(v) = get_num(m, "h").filter(|v| *v > 0.0) {
            s.h = v;
        }
        if let Some(v) = get_num(m, "tol").filter(|v| *v >= 0.0) {
            s.tol = v;
        }
        if let Some(DialogValue::Choice { index, .. }) = m.get("unit") {
            s.unit = *index;
        }
        s.dims = get_bool(m, "dims", s.dims);
        s.table = get_bool(m, "table", s.table);
        s.csv = get_bool(m, "csv", s.csv);
        s.fill = get_bool(m, "fill", s.fill);
        if let Some(p) = get_path(m, "csv_path") {
            s.csv_path = p;
        }
    }
}

impl InteractiveCommand for MeasureDialog {
    fn prompt(&self) -> String {
        tr!(
            "RV_VANO: inserisci le misure nella finestra (apri \"Come misurare\" per l'aiuto)",
            "RV_VANO: type the measures in the dialog (open \"How to measure\" for help)"
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
        // Building a page records how many rows it shows; work on a copy so
        // this stays `&self`, and keep the counts in step through `shown`.
        let mut me = MeasureDialog {
            handles: Vec::new(),
            survey: self.survey.clone(),
            msg: self.msg.clone(),
            saved: self.saved.clone(),
            page: self.page,
            shown_others: 0,
            shown_constraints: 0,
            shown_openings: 0,
        };
        match self.page {
            1 => Some(me.page1()),
            2 => Some(me.page2()),
            _ => None,
        }
    }
    fn on_dialog_response(&mut self, response: DialogResponse) -> CommandStep {
        // The row counts the page was built with (same input → same layout).
        let _ = match self.page {
            1 => Some(self.page1()),
            2 => Some(self.page2()),
            _ => None,
        };
        match (self.page, response) {
            (1, DialogResponse::Submitted(m)) => {
                self.read_page1(&m);
                self.msg = None;
                self.page = 2;
                CommandStep::NeedPoint
            }
            (1, _) => {
                self.page = 0;
                CommandStep::Cancel
            }
            (2, DialogResponse::Submitted(m)) => {
                self.read_page2(&m);
                self.page = 0;
                commands::set_pending(self.handles.clone(), self.saved.clone());
                CommandStep::Dispatch("RV_VANOAPPLY".into())
            }
            (2, _) => {
                // "Back": the host sends no values with it, so page 2 keeps
                // what it had when it was opened.
                self.page = 1;
                CommandStep::NeedPoint
            }
            _ => CommandStep::Cancel,
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
                "Scegli i vani (cliccali uno dopo l'altro, Invio): sugli angoli compaiono A, B, C … o, con più vani, 1A, \
                 1B … 2A … Stampa la pianta e usala in cantiere per segnare le misure.",
                "Choose the rooms (click them one after the other, Enter): A, B, C … or, with several rooms, 1A, 1B … 2A … \
                 appear at the corners. Print the plan and use it on site to note the measures."
            ),
        ),
        (
            tr!("3. Misure e diagonali", "3. Sides and diagonals"),
            tr!(
                "Scegli gli stessi vani e scrivi le misure. Un vano di n spigoli vuole tutti i lati e almeno n−3 diagonali; \
                 ogni vano collegato ne vuole almeno 3 di collegamento (spessore del muro, misure attraverso la porta). \
                 Nella finestra, \"Come misurare\" spiega ogni colonna.",
                "Choose the same rooms and type the measures. A room of n corners needs all sides and at least n−3 \
                 diagonals; each linked room needs at least 3 linking measures (wall thickness, measures through the \
                 door). In the dialog, \"How to measure\" explains every column."
            ),
        ),
        (
            tr!("Vincoli, porte e finestre", "Constraints, doors and windows"),
            tr!(
                "Un angolo noto (per esempio retto) o un lato orizzontale / verticale può prendere il posto di una \
                 diagonale. Porte e finestre si danno con lato, distanza dallo spigolo e larghezza; la stessa porta \
                 scritta nei due vani con lo stesso nome e lo spessore del muro collega i vani da sola.",
                "A known angle (for example square) or a horizontal / vertical side can take the place of a diagonal. \
                 Doors and windows are given by side, distance from the corner and width; the same door written in both \
                 rooms with the same name and the wall thickness links the rooms by itself."
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
                "RV_VANO, RV_ETICHETTE, RV_LIBRETTO, RV_GUIDE, RV_MANUALE; RV_LANG IT|EN|AUTO cambia la lingua.",
                "RV_VANO, RV_ETICHETTE, RV_LIBRETTO, RV_GUIDE, RV_MANUALE; RV_LANG IT|EN|AUTO changes the language."
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

    fn saved(n: usize) -> Saved {
        Saved {
            sides: (0..n).map(|i| vec![format!("{i}"), "1".into(), String::new()]).collect(),
            others: vec![],
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
        }
    }

    #[test]
    fn two_pages_forth_and_back() {
        let survey = Survey::new(vec![vec![[0.0, 0.0], [4.0, 0.0], [4.0, 3.0], [0.0, 3.0]]]);
        let mut d = MeasureDialog::new(vec![Handle::new(0x2A)], &survey, &saved(4), None);
        let p1 = d.modal_dialog().unwrap();
        assert!(p1.title.contains("1/2"));
        // Page 1 has only the two measure tables (and short texts).
        let mut m = HashMap::new();
        for i in 0..4 {
            m.insert(format!("s{i}"), DialogValue::Text("4,01".into()));
        }
        m.insert("o0a".to_string(), DialogValue::Text("A".into()));
        m.insert("o0b".to_string(), DialogValue::Text("C".into()));
        m.insert("o0m".to_string(), DialogValue::Text("5".into()));
        assert!(matches!(d.on_dialog_response(DialogResponse::Submitted(m)), CommandStep::NeedPoint));
        assert!(d.modal_dialog().unwrap().title.contains("2/2"));
        // Back keeps page 1's values.
        assert!(matches!(d.on_dialog_response(DialogResponse::Cancelled), CommandStep::NeedPoint));
        assert_eq!(d.saved.sides[0][2], "4,01");
        assert_eq!(d.saved.others, vec![vec!["A".to_string(), "C".into(), "5.000".into(), "5".into()]]);
        assert!(d.modal_dialog().unwrap().title.contains("1/2"));
        // Forth again, then compute.
        d.on_dialog_response(DialogResponse::Submitted(HashMap::new()));
        let mut m = HashMap::new();
        m.insert("c0a".to_string(), DialogValue::Text("A".into()));
        m.insert("c0b".to_string(), DialogValue::Text("90".into()));
        assert!(matches!(d.on_dialog_response(DialogResponse::Submitted(m)), CommandStep::Dispatch(ref c) if c == "RV_VANOAPPLY"));
        assert!(d.modal_dialog().is_none());
        assert_eq!(d.saved.constraints, vec![vec!["A".to_string(), "90".into()]]);
    }

    #[test]
    fn cancel_on_page_one_ends() {
        let survey = Survey::new(vec![vec![[0.0, 0.0], [4.0, 0.0], [4.0, 3.0]]]);
        let mut d = MeasureDialog::new(vec![Handle::new(0x2A)], &survey, &saved(3), None);
        assert!(matches!(d.on_dialog_response(DialogResponse::Cancelled), CommandStep::Cancel));
        assert!(d.modal_dialog().is_none());
    }
}
