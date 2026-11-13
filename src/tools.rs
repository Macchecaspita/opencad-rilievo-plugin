//! Interactive steps: picking a room outline, the measures dialog and the
//! guide. Interactive callbacks get no host access, so each step ends by
//! handing the host a one-word apply command that comes back to
//! [`crate::commands`] with full `HostApi` access.

use std::collections::HashMap;
use std::sync::Mutex;

use ocs_plugin_api::dialog::{DialogControl, DialogDef, DialogResponse, DialogValue};
use ocs_plugin_api::host::codec::Handle;
use ocs_plugin_api::host::{CommandStep, InteractiveCommand, PreviewWire};

use crate::commands::{self, fmt_num, Room, Saved};
use crate::solve::{self, label, P2};

const CYAN: [f32; 4] = [0.43, 0.72, 0.93, 1.0];

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

// ── Pick a room (closed polyline) ───────────────────────────────────────────

pub struct PickRoomTool {
    rooms: Vec<Room>,
    /// Command dispatched (deferred) with the room's handle.
    verb: &'static str,
    title: &'static str,
    msg: Option<String>,
}

impl PickRoomTool {
    pub fn new(rooms: Vec<Room>, verb: &'static str, title: &'static str) -> Self {
        PickRoomTool { rooms, verb, title, msg: None }
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

    fn take(&mut self, idx: Option<usize>) -> CommandStep {
        match idx {
            Some(i) => deferred(self.verb, format!("{:X}", self.rooms[i].handle.value())),
            None => {
                self.msg = Some(tr!("non è una polilinea chiusa", "not a closed polyline").into());
                CommandStep::NeedPoint
            }
        }
    }
}

impl InteractiveCommand for PickRoomTool {
    fn prompt(&self) -> String {
        with_msg(
            &self.msg,
            tr!(
                format!("{}: seleziona la polilinea chiusa del vano (o clicca al suo interno)", self.title),
                format!("{}: select the room's closed polyline (or click inside it)", self.title)
            ),
        )
    }
    fn on_point(&mut self, pt: [f64; 3]) -> CommandStep {
        let i = self.room_at(pt);
        self.take(i)
    }
    fn needs_object_pick(&self) -> bool {
        true
    }
    fn on_object_pick(&mut self, handle: Handle, pt: [f64; 3]) -> CommandStep {
        let i = self.rooms.iter().position(|r| r.handle == handle).or_else(|| self.room_at(pt));
        self.take(i)
    }
    fn on_cursor_move(&mut self, pt: [f64; 3]) -> Vec<PreviewWire> {
        self.room_at(pt)
            .map(|i| vec![PreviewWire::with_color(ring3(&self.rooms[i].pts, self.rooms[i].z), CYAN)])
            .unwrap_or_default()
    }
    fn on_enter(&mut self) -> CommandStep {
        CommandStep::Cancel
    }
}

// ── Measures dialog ─────────────────────────────────────────────────────────

fn info(text: String) -> DialogControl {
    DialogControl::InfoText { text, align_h: None }
}

fn number(id: &str, label: &str, default: f64) -> DialogControl {
    DialogControl::Number { id: id.into(), label: label.into(), default, min: Some(0.0), max: None, step: None }
}

fn checkbox(id: &str, label: &str, default: bool) -> DialogControl {
    DialogControl::Checkbox { id: id.into(), label: label.into(), default }
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

fn get_table(m: &HashMap<String, DialogValue>, id: &str) -> Option<Vec<Vec<String>>> {
    match m.get(id)? {
        DialogValue::Table(rows) => Some(rows.clone()),
        _ => None,
    }
}

pub struct MeasureDialog {
    handle: Handle,
    n: usize,
    def: DialogDef,
    done: bool,
}

impl MeasureDialog {
    /// `saved`: what the user typed last time for this room (kept when the
    /// calculation failed, so nothing has to be typed again).
    pub fn new(room: &Room, saved: &Saved, msg: Option<String>) -> Self {
        let n = room.pts.len();
        let need = n.saturating_sub(3);
        let last = label(n - 1);
        let mut def = DialogDef::new(tr!("Rilievo vano: misure di lati e diagonali", "Room survey: sides and diagonals"));
        if let Some(m) = msg {
            def = def.with_control(info(format!("⚠ {m}")));
        }
        def = def
            .with_control(info(tr!(
                format!(
                    "Vano di {n} vertici (A … {last}), lettere scritte sul disegno. Inserisci le misure nelle unità \
                     del disegno (la virgola va bene). Servono tutti i lati e almeno {need} diagonali; ogni misura \
                     in più serve da controllo e viene compensata ai minimi quadrati."
                ),
                format!(
                    "Room with {n} corners (A … {last}), letters written on the drawing. Type the measures in drawing \
                     units. All sides are needed plus at least {need} diagonals; every extra measure is a check and \
                     is adjusted by least squares."
                )
            )))
            .with_control(DialogControl::Table {
                id: "sides".into(),
                label: Some(tr!("Lati", "Sides").into()),
                headers: vec![
                    tr!("Lato", "Side").into(),
                    tr!("Disegnato", "Drawn").into(),
                    tr!("Misurato", "Measured").into(),
                ],
                rows: saved.sides.clone(),
                allow_add: false,
                allow_delete: false,
            })
            .with_control(DialogControl::Table {
                id: "diags".into(),
                label: Some(tr!(
                    "Diagonali (quelle proposte corrono dentro il vano; puoi aggiungerne altre, indicando i due vertici)",
                    "Diagonals (the proposed ones run inside the room; add more by naming their two corners)"
                )
                .into()),
                headers: vec![
                    tr!("Da", "From").into(),
                    tr!("A", "To").into(),
                    tr!("Disegnata", "Drawn").into(),
                    tr!("Misurata", "Measured").into(),
                ],
                rows: saved.diags.clone(),
                allow_add: true,
                allow_delete: true,
            })
            .with_control(DialogControl::row(vec![
                number("h", tr!("Altezza testi e quote", "Text and dimension height"), saved.h),
                number("tol", tr!("Tolleranza scarti", "Residual tolerance"), saved.tol),
            ]))
            .with_control(DialogControl::row(vec![
                checkbox("dims", tr!("Quota lati e diagonali", "Dimension sides and diagonals"), saved.dims),
                checkbox("table", tr!("Tabella misure e scarti", "Table of measures and residuals"), saved.table),
            ]))
            .submit_label(tr!("Calcola", "Compute"))
            .cancel_label(tr!("Annulla", "Cancel"))
            .width(620.0);
        MeasureDialog { handle: room.handle, n, def, done: false }
    }
}

impl InteractiveCommand for MeasureDialog {
    fn prompt(&self) -> String {
        tr!("RV_VANO: inserisci le misure nella finestra", "RV_VANO: type the measures in the dialog").into()
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
        let base = commands::saved_for(self.handle);
        let mut sides = get_table(&m, "sides").unwrap_or_else(|| base.as_ref().map(|s| s.sides.clone()).unwrap_or_default());
        sides.truncate(self.n);
        let saved = Saved {
            sides,
            diags: get_table(&m, "diags").unwrap_or_default(),
            h: get_num(&m, "h").filter(|v| *v > 0.0).unwrap_or_else(|| base.as_ref().map_or(1.0, |s| s.h)),
            tol: get_num(&m, "tol").filter(|v| *v >= 0.0).unwrap_or_else(|| base.as_ref().map_or(0.0, |s| s.tol)),
            dims: get_bool(&m, "dims", true),
            table: get_bool(&m, "table", true),
        };
        commands::set_pending(self.handle, saved);
        CommandStep::Dispatch("RV_VANOAPPLY".into())
    }
}

/// The rows a fresh dialog starts with.
pub fn initial_saved(room: &Room) -> Saved {
    let n = room.pts.len();
    let (h, tol) = commands::default_sizes(&room.pts);
    let dec = commands::decimals(tol);
    let sides = (0..n)
        .map(|i| {
            let j = (i + 1) % n;
            vec![
                format!("{}–{}", label(i), label(j)),
                fmt_num(solve::dist(room.pts[i], room.pts[j]), dec),
                String::new(),
            ]
        })
        .collect();
    let diags = solve::suggested_diagonals(&room.pts)
        .into_iter()
        .map(|(i, j)| vec![label(i), label(j), fmt_num(solve::dist(room.pts[i], room.pts[j]), dec), String::new()])
        .collect();
    Saved { sides, diags, h, tol, dims: true, table: true }
}

// ── Guide ───────────────────────────────────────────────────────────────────

pub fn guide_sections() -> Vec<(&'static str, &'static str)> {
    vec![
        (
            tr!("1. Disegna il vano", "1. Draw the room"),
            tr!(
                "Disegna ogni vano come polilinea chiusa, a occhio, con i soli comandi del CAD (PLINEA … Chiudi). \
                 Contano solo il numero e l'ordine dei vertici: forma e dimensioni le sistema il plugin.",
                "Draw each room as a closed polyline, by eye, with the CAD's own commands (PLINE … Close). \
                 Only the number and order of the corners matter: the plugin fixes shape and size."
            ),
        ),
        (
            tr!("2. Lettere vertici (facoltativo)", "2. Corner letters (optional)"),
            tr!(
                "Scrive A, B, C … sugli angoli del vano (layer RILIEVO_ETICHETTE): stampa la pianta e usala come \
                 eidotipo in cantiere, segnando lati e diagonali misurati.",
                "Writes A, B, C … at the room's corners (layer RILIEVO_ETICHETTE): print the plan and take it on \
                 site as a sketch, noting the measured sides and diagonals."
            ),
        ),
        (
            tr!("3. Misure e diagonali", "3. Sides and diagonals"),
            tr!(
                "Seleziona il vano e inserisci le misure. Un vano di n vertici chiede tutti i lati e almeno n−3 \
                 diagonali (1 per un rettangolo, 3 per una L): il plugin le propone in modo che corrano dentro il vano. \
                 Una diagonale in più è il controllo: senza, un errore di misura non si vede.",
                "Select the room and type the measures. A room with n corners needs all its sides and at least n−3 \
                 diagonals (1 for a rectangle, 3 for an L): the plugin proposes ones that run inside the room. \
                 One more diagonal is the check: without it a wrong measure cannot show."
            ),
        ),
        (
            tr!("4. Risultato", "4. Result"),
            tr!(
                "Il vano vero va sul layer RILIEVO_CORRETTO con il vertice A al suo posto e il lato A–B nella \
                 direzione disegnata; il disegno originale resta per confronto. Su RILIEVO_QUOTE vanno lettere, \
                 quote dei lati (stile di quota RILIEVO), diagonali con la loro misura e la tabella con misurate, calcolate e scarti; gli scarti oltre la \
                 tolleranza sono in rosso. Con misure in più compare anche lo scarto quadratico medio σ₀.",
                "The true room goes on layer RILIEVO_CORRETTO, with corner A in place and side A–B in the drawn \
                 direction; the original drawing stays for comparison. Layer RILIEVO_QUOTE gets the letters, the \
                 side dimensions (dimension style RILIEVO), the diagonals with their length and the table of measured, computed and residual values; \
                 residuals over the tolerance are red. With extra measures the standard deviation σ₀ is shown too."
            ),
        ),
        (
            tr!("Comandi", "Commands"),
            tr!(
                "RV_VANO, RV_ETICHETTE, RV_GUIDE; RV_LANG IT|EN|AUTO cambia la lingua.",
                "RV_VANO, RV_ETICHETTE, RV_GUIDE; RV_LANG IT|EN|AUTO changes the language."
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
        def = def.submit_label("OK").cancel_label(tr!("Chiudi", "Close")).width(560.0);
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
