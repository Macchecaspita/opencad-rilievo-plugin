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

fn get_table(m: &HashMap<String, DialogValue>, id: &str) -> Option<Vec<Vec<String>>> {
    match m.get(id)? {
        DialogValue::Table(rows) => Some(rows.clone()),
        _ => None,
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

pub struct MeasureDialog {
    handles: Vec<Handle>,
    n_sides: usize,
    def: DialogDef,
    done: bool,
}

impl MeasureDialog {
    /// `saved`: what the user typed last time for these rooms (kept when the
    /// calculation failed, so nothing has to be typed again).
    pub fn new(handles: Vec<Handle>, survey: &Survey, saved: &Saved, msg: Option<String>) -> Self {
        let multi = survey.multi();
        let need: usize = survey.rooms.iter().map(|r| r.len().saturating_sub(3)).sum();
        let links = if multi { 3 * (survey.rooms.len() - 1) } else { 0 };
        let title = if multi {
            tr!(format!("Rilievo di {} vani collegati", survey.rooms.len()), format!("Survey of {} linked rooms", survey.rooms.len()))
        } else {
            tr!("Rilievo vano: lati e diagonali".to_string(), "Room survey: sides and diagonals".to_string())
        };
        let mut def = DialogDef::new(title);
        if let Some(m) = msg {
            def = def.with_control(info(tr!(
                format!("⚠ Da correggere: {m}. Le misure che avevi scritto sono rimaste: correggi e premi di nuovo Calcola."),
                format!("⚠ To fix: {m}. The measures you typed are still here: fix them and press Compute again.")
            )));
        }
        let summary = if multi {
            tr!(
                format!(
                    "Vertici: {} (lettere scritte sul disegno). Servono tutti i lati, almeno {need} diagonali e almeno {links} \
                     misure di collegamento tra i vani. Ogni misura in più fa da controllo.",
                    survey.ranges()
                ),
                format!(
                    "Corners: {} (letters written on the drawing). All sides are needed, at least {need} diagonals and at \
                     least {links} measures linking the rooms. Every extra measure is a check.",
                    survey.ranges()
                )
            )
        } else {
            tr!(
                format!(
                    "Vertici: {} (lettere scritte sul disegno). Servono tutti i lati e almeno {need} diagonali; ogni misura \
                     in più fa da controllo e viene compensata ai minimi quadrati.",
                    survey.ranges()
                ),
                format!(
                    "Corners: {} (letters written on the drawing). All sides are needed plus at least {need} diagonals; every \
                     extra measure is a check, adjusted by least squares.",
                    survey.ranges()
                )
            )
        };
        def = def
            .with_control(info(summary))
            .with_control(help_section("help", tr!("Come misurare (aiuto) ▸", "How to measure (help) ▸"), how_to_measure(multi)))
            .with_control(DialogControl::Table {
                id: "sides".into(),
                label: Some(
                    tr!(
                        "Lati — scrivi la misura nella colonna Misurato (la colonna Disegnato è solo un riferimento)",
                        "Sides — type the measure in the Measured column (Drawn is only for reference)"
                    )
                    .into(),
                ),
                headers: vec![tr!("Lato", "Side").into(), tr!("Disegnato", "Drawn").into(), tr!("Misurato", "Measured").into()],
                rows: saved.sides.clone(),
                allow_add: false,
                allow_delete: false,
            })
            .with_control(DialogControl::Table {
                id: "others".into(),
                label: Some(if multi {
                    tr!(
                        "Diagonali, collegamenti e spessori — due vertici (1A, 2C) per una distanza; un lato (1B-1C) e un vertice (2A) per lo spessore di un muro",
                        "Diagonals, links and thicknesses — two corners (1A, 2C) for a distance; a side (1B-1C) and a corner (2A) for a wall thickness"
                    )
                    .into()
                } else {
                    tr!(
                        "Diagonali — quelle proposte corrono dentro il vano; aggiungi una riga per misurarne altre (due lettere in Da e A)",
                        "Diagonals — the proposed ones run inside the room; add a row to measure more (two letters in From and To)"
                    )
                    .into()
                }),
                headers: vec![
                    tr!("Da", "From").into(),
                    tr!("A", "To").into(),
                    tr!("Disegnata", "Drawn").into(),
                    tr!("Misurata", "Measured").into(),
                ],
                rows: saved.others.clone(),
                allow_add: true,
                allow_delete: true,
            })
            .with_control(DialogControl::row(vec![
                number("h", tr!("Altezza testi e quote", "Text and dimension height"), saved.h),
                number("tol", tr!("Tolleranza scarti", "Residual tolerance"), saved.tol),
            ]))
            .with_control(info(
                tr!(
                    "Altezza testi: nelle unità del disegno (0,20 = 20 cm se disegni in metri). Tolleranza: lo scarto oltre il \
                     quale una misura viene segnalata in rosso (di solito 1–2 cm).",
                    "Text height: in drawing units (0.20 = 20 cm when drawing in metres). Tolerance: the residual beyond which \
                     a measure is flagged in red (usually 1–2 cm)."
                )
                .to_string(),
            ))
            .with_control(DialogControl::row(vec![
                checkbox("dims", tr!("Quota lati, diagonali e spessori", "Dimension sides, diagonals and thicknesses"), saved.dims),
                checkbox("table", tr!("Tabella misure e scarti", "Table of measures and residuals"), saved.table),
            ]))
            .with_control(checkbox(
                "csv",
                tr!("Salva anche il libretto delle misure (CSV per Excel / Calc)", "Also save the measures book (CSV for Excel / Calc)"),
                saved.csv,
            ))
            .with_control(csv_picker(&saved.csv_path))
            .with_control(info(
                tr!(
                    "Il risultato va sul layer RILIEVO_CORRETTO; il disegno originale resta per confronto. Annulla (Ctrl+Z) toglie tutto in un passo.",
                    "The result goes on layer RILIEVO_CORRETTO; the original drawing stays for comparison. Undo (Ctrl+Z) removes it all in one step."
                )
                .to_string(),
            ))
            .submit_label(tr!("Calcola", "Compute"))
            .cancel_label(tr!("Annulla", "Cancel"))
            .width(680.0);
        let n_sides = survey.rooms.iter().map(|r| r.len()).sum();
        MeasureDialog { handles, n_sides, def, done: false }
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
        (!self.done).then(|| self.def.clone())
    }
    fn on_dialog_response(&mut self, response: DialogResponse) -> CommandStep {
        self.done = true;
        let DialogResponse::Submitted(m) = response else {
            return CommandStep::Cancel;
        };
        let base = commands::saved_for(&self.handles);
        let mut sides = get_table(&m, "sides").unwrap_or_else(|| base.as_ref().map(|s| s.sides.clone()).unwrap_or_default());
        sides.truncate(self.n_sides);
        let saved = Saved {
            sides,
            others: get_table(&m, "others").unwrap_or_default(),
            h: get_num(&m, "h").filter(|v| *v > 0.0).unwrap_or_else(|| base.as_ref().map_or(1.0, |s| s.h)),
            tol: get_num(&m, "tol").filter(|v| *v >= 0.0).unwrap_or_else(|| base.as_ref().map_or(0.0, |s| s.tol)),
            dims: get_bool(&m, "dims", true),
            table: get_bool(&m, "table", true),
            csv: get_bool(&m, "csv", false),
            csv_path: get_path(&m, "csv_path").or_else(|| base.as_ref().map(|s| s.csv_path.clone())).unwrap_or_default(),
        };
        commands::set_pending(self.handles.clone(), saved);
        CommandStep::Dispatch("RV_VANOAPPLY".into())
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
            tr!("4. Risultato", "4. Result"),
            tr!(
                "I vani veri vanno sul layer RILIEVO_CORRETTO; il disegno originale resta. Su RILIEVO_QUOTE: lettere, quote \
                 dei lati (stile RILIEVO), diagonali, collegamenti, spessori e la tabella misurata / calcolata / scarto, \
                 con in rosso gli scarti oltre la tolleranza. Se le misure non bastano o non tornano, la finestra si \
                 riapre con il motivo.",
                "The true rooms go on layer RILIEVO_CORRETTO; the original drawing stays. On RILIEVO_QUOTE: letters, side \
                 dimensions (style RILIEVO), diagonals, links, thicknesses and the measured / computed / residual table, \
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
            tr!("Comandi", "Commands"),
            tr!(
                "RV_VANO, RV_ETICHETTE, RV_LIBRETTO, RV_GUIDE; RV_LANG IT|EN|AUTO cambia la lingua.",
                "RV_VANO, RV_ETICHETTE, RV_LIBRETTO, RV_GUIDE; RV_LANG IT|EN|AUTO changes the language."
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
