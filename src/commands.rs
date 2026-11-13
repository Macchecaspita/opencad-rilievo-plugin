//! Command dispatch: the glue between the host and the survey maths.
//!
//! `RV_VANO` picks one or more rooms and opens the measures dialog; the
//! dialog hands back `RV_VANOAPPLY`, which runs here with full `HostApi`
//! access, solves and draws the result as one undo step. When the measures
//! are not enough or do not fit together, the dialog opens again with what
//! was typed and the reason on top. `RV_LIBRETTO` saves the last result as
//! a measures book (CSV).

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Mutex;

use ocs_plugin_api::host::codec;
use ocs_plugin_api::host::{CommandRequest, HostApi, HostSettingValue, LayerConfig, TableOperation, TableStyleKind};

use codec::entities::{AttachmentPoint, LwPolyline, LwVertex, MText};
use codec::{EntityType, Handle, Vector2, Vector3};

use crate::solve::{self, Meas, SolveError, Solution, P2};
use crate::survey::{self, Kind, Measure, OpKind, Opening, Saved, Survey};
use crate::{labels, tools};

/// Layer of the adjusted rooms.
pub const LAYER_ROOM: &str = "RILIEVO_CORRETTO";
/// Layer of letters, dimensions and tables of the adjusted rooms.
pub const LAYER_NOTES: &str = "RILIEVO_QUOTE";
/// Layer of the working letters written on the rooms drawn by eye.
pub const LAYER_LABELS: &str = "RILIEVO_ETICHETTE";
/// Layer of doors, windows and openings.
pub const LAYER_OPENINGS: &str = "RILIEVO_APERTURE";
/// Dimension style the plugin keeps for its dimensions.
pub const DIM_STYLE: &str = "RILIEVO";

// ── Rooms ───────────────────────────────────────────────────────────────────

/// A closed polyline drawn as a room.
#[derive(Clone, Debug)]
pub struct Room {
    pub handle: Handle,
    pub pts: Vec<P2>,
    pub z: f64,
    /// Whether some segment was an arc (taken as its chord).
    pub had_arcs: bool,
}

fn room_of(e: &EntityType) -> Option<Room> {
    let EntityType::LwPolyline(p) = e else { return None };
    if p.normal.z < 0.999_999 || p.vertices.len() < 3 {
        return None;
    }
    let mut pts: Vec<P2> = p.vertices.iter().map(|v| [v.location.x, v.location.y]).collect();
    let explicit = pts.len() > 3 && solve::dist(pts[0], pts[pts.len() - 1]) < 1e-9;
    if !(p.is_closed || explicit) {
        return None;
    }
    if explicit {
        pts.pop();
    }
    let mut clean: Vec<P2> = Vec::with_capacity(pts.len());
    for q in pts {
        if clean.last().map_or(true, |l| solve::dist(*l, q) > 1e-9) {
            clean.push(q);
        }
    }
    if clean.len() > 1 && solve::dist(clean[0], clean[clean.len() - 1]) <= 1e-9 {
        clean.pop();
    }
    if clean.len() < 3 {
        return None;
    }
    let had_arcs = p.vertices.iter().any(|v| v.bulge.abs() > 1e-9);
    Some(Room { handle: e.common().handle, pts: clean, z: p.elevation, had_arcs })
}

/// Every closed polyline in the drawing (XY plane), except the plugin's own
/// output.
pub fn rooms(host: &dyn HostApi) -> Vec<Room> {
    host.document()
        .entities()
        .filter(|e| {
            let l = &e.common().layer;
            !l.eq_ignore_ascii_case(LAYER_ROOM) && !l.eq_ignore_ascii_case(LAYER_NOTES)
        })
        .filter_map(room_of)
        .collect()
}

/// Live read of one room by handle.
fn fetch_room(host: &dyn HostApi, h: Handle) -> Option<Room> {
    let live = host.entities_batch(&[h]).ok().and_then(|v| v.into_iter().next().flatten());
    match live {
        Some(e) => room_of(&e),
        None => host.document().get_entity(h).and_then(room_of),
    }
}

fn fetch_rooms(host: &dyn HostApi, hs: &[Handle]) -> Option<Vec<Room>> {
    hs.iter().map(|&h| fetch_room(host, h)).collect()
}

pub fn point_in_polygon(poly: &[P2], p: P2) -> bool {
    let mut inside = false;
    let n = poly.len();
    let mut j = n.wrapping_sub(1);
    for i in 0..n {
        let (a, b) = (poly[i], poly[j]);
        if (a[1] > p[1]) != (b[1] > p[1]) && p[0] < (b[0] - a[0]) * (p[1] - a[1]) / (b[1] - a[1]) + a[0] {
            inside = !inside;
        }
        j = i;
    }
    inside
}

fn centroid(p: &[P2]) -> P2 {
    // Area centroid; falls back to the mean of the corners.
    let n = p.len();
    let a = solve::signed_area(p);
    if a.abs() < 1e-12 {
        let (x, y) = p.iter().fold((0.0, 0.0), |(x, y), q| (x + q[0], y + q[1]));
        return [x / n as f64, y / n as f64];
    }
    let (mut cx, mut cy) = (0.0, 0.0);
    for i in 0..n {
        let (q, r) = (p[i], p[(i + 1) % n]);
        let c = q[0] * r[1] - r[0] * q[1];
        cx += (q[0] + r[0]) * c;
        cy += (q[1] + r[1]) * c;
    }
    [cx / (6.0 * a), cy / (6.0 * a)]
}

// ── Session state ───────────────────────────────────────────────────────────

static SAVED: Mutex<Option<HashMap<String, Saved>>> = Mutex::new(None);
static PENDING: Mutex<Option<(Vec<Handle>, Saved)>> = Mutex::new(None);
/// The last measures book (CSV text) and its suggested file.
static LAST_BOOK: Mutex<Option<(String, String)>> = Mutex::new(None);
/// The last calculation, for "apply to the drawing": per room, the original
/// polyline, its adjusted corners and the adjusted copy drawn on
/// RILIEVO_CORRETTO.
static LAST_RESULT: Mutex<Option<Vec<(Handle, Vec<P2>, Option<Handle>)>>> = Mutex::new(None);

fn key(hs: &[Handle]) -> String {
    hs.iter().map(|h| hex(*h)).collect::<Vec<_>>().join(",")
}

pub fn saved_for(hs: &[Handle]) -> Option<Saved> {
    SAVED.lock().unwrap_or_else(|e| e.into_inner()).as_ref().and_then(|m| m.get(&key(hs)).cloned())
}

fn remember(hs: &[Handle], s: Saved) {
    SAVED.lock().unwrap_or_else(|e| e.into_inner()).get_or_insert_with(HashMap::new).insert(key(hs), s);
}

pub fn set_pending(hs: Vec<Handle>, s: Saved) {
    *PENDING.lock().unwrap_or_else(|e| e.into_inner()) = Some((hs, s));
}

fn take_pending() -> Option<(Vec<Handle>, Saved)> {
    PENDING.lock().unwrap_or_else(|e| e.into_inner()).take()
}

pub fn last_book() -> Option<(String, String)> {
    LAST_BOOK.lock().unwrap_or_else(|e| e.into_inner()).clone()
}

static PENDING_BOOK_PATH: Mutex<Option<String>> = Mutex::new(None);

pub fn set_pending_book_path(p: String) {
    *PENDING_BOOK_PATH.lock().unwrap_or_else(|e| e.into_inner()) = Some(p);
}

// ── Numbers ─────────────────────────────────────────────────────────────────

/// A typed number: comma or point, spaces ignored. Empty → `None`.
pub fn parse_num(s: &str) -> Option<f64> {
    let t: String = s.chars().filter(|c| !c.is_whitespace()).collect();
    if t.is_empty() {
        return None;
    }
    t.replace(',', ".").parse::<f64>().ok().filter(|v| v.is_finite())
}

/// `v` with `dec` decimals, decimal comma in Italian.
pub fn fmt_num(v: f64, dec: usize) -> String {
    let mut s = format!("{v:.dec$}");
    // No "-0.000": a value that rounds to zero is just zero.
    if s.starts_with('-') && s[1..].chars().all(|c| c == '0' || c == '.') {
        s.remove(0);
    }
    if crate::i18n::italian() { s.replace('.', ",") } else { s }
}

/// Signed, for residuals.
fn fmt_signed(v: f64, dec: usize) -> String {
    let s = fmt_num(v.abs(), dec);
    let zero = s.chars().all(|c| c == '0' || c == ',' || c == '.');
    if zero { s } else if v < 0.0 { format!("−{s}") } else { format!("+{s}") }
}

/// 1, 2, 2.5 or 5 times a power of ten, nearest to `v` from above.
fn nice(v: f64) -> f64 {
    if !(v.is_finite() && v > 0.0) {
        return 1.0;
    }
    let p = 10f64.powf(v.log10().floor());
    for m in [1.0, 2.0, 2.5, 5.0, 10.0] {
        if v <= m * p * 1.0000001 {
            return m * p;
        }
    }
    10.0 * p
}

/// Text height and residual tolerance suited to the rooms' size: a 5 m
/// room drawn in metres gets 0.2 and 0.02; the same in millimetres gets 200
/// and 20.
pub fn default_sizes(pts: &[P2]) -> (f64, f64) {
    let n = pts.len();
    let longest = (0..n).map(|i| solve::dist(pts[i], pts[(i + 1) % n])).fold(0.0f64, f64::max);
    (nice(longest / 30.0), nice(longest / 250.0))
}

/// Decimals to show for measures checked to `tol`: one more than the
/// tolerance needs (0.02 → 3, 20 → 0).
pub fn decimals(tol: f64) -> usize {
    if !(tol.is_finite() && tol > 0.0) {
        return 3;
    }
    ((-tol.log10()).ceil() + 1.0).clamp(0.0, 4.0) as usize
}

fn hex(h: Handle) -> String {
    format!("{:X}", h.value())
}

fn parse_handle(tok: &str) -> Option<Handle> {
    let t = tok.trim().trim_start_matches("0x").trim_start_matches("0X");
    u64::from_str_radix(t, 16).ok().filter(|v| *v != 0).map(Handle::new)
}

fn parse_handles(tok: &str) -> Vec<Handle> {
    tok.split(',').filter_map(parse_handle).collect()
}

// ── Dispatch ────────────────────────────────────────────────────────────────

pub fn handle(host: &mut dyn HostApi, cmd: &str) -> bool {
    let trimmed = cmd.trim();
    let verb = trimmed.split_whitespace().next().unwrap_or("").to_ascii_uppercase();
    if !verb.starts_with("RV_") {
        return false;
    }
    crate::i18n::refresh();
    let mut rest = trimmed[verb.len()..].trim().to_string();
    if rest.is_empty() {
        if let Some(k) = tools::take_deferred(&verb) {
            rest = k;
        }
    }
    let arg = rest.split_whitespace().next().map(str::to_string);

    match verb.as_str() {
        "RV_" | "RV_HELP" => help(host),
        "RV_LANG" => lang(host, &rest),
        "RV_GUIDE" => host.start_interactive(Box::new(tools::GuideDialog::new())),
        "RV_MANUALE" => open_manual(host),
        "RV_VANO" => start_pick(host, "RV_VANOOPEN", "RV_VANO"),
        "RV_ETICHETTE" => start_pick(host, "RV_ETICHETTEAPPLY", "RV_ETICHETTE"),
        "RV_VANOOPEN" => {
            let hs = arg.as_deref().map(parse_handles).unwrap_or_default();
            if !hs.is_empty() {
                open_dialog(host, &hs, None);
            }
        }
        "RV_ETICHETTEAPPLY" => {
            let hs = arg.as_deref().map(parse_handles).unwrap_or_default();
            if !hs.is_empty() {
                labels_only(host, &hs);
            }
        }
        "RV_VANOAPPLY" => {
            if let Some((hs, saved)) = take_pending() {
                apply(host, &hs, saved);
            }
        }
        "RV_APPLICA" => apply_to_drawing(host, true),
        "RV_LIBRETTO" => match last_book() {
            Some((_, default)) => host.start_interactive(Box::new(tools::BookDialog::new(default))),
            None => host.push_error(tr!(
                "RV_LIBRETTO: non c'è ancora un calcolo da salvare. Prima esegui Misure e diagonali (RV_VANO).",
                "RV_LIBRETTO: there is no calculation to save yet. Run Sides and diagonals (RV_VANO) first."
            )),
        },
        "RV_LIBRETTOAPPLY" => {
            let path = PENDING_BOOK_PATH.lock().unwrap_or_else(|e| e.into_inner()).take();
            if let (Some(p), Some((text, _))) = (path, last_book()) {
                write_book(host, &p, &text);
            }
        }
        _ => {
            host.push_error(&tr!(
                format!("{verb}: comando sconosciuto. RV_HELP elenca i comandi."),
                format!("{verb}: unknown command. RV_HELP lists the commands.")
            ));
        }
    }
    true
}

/// The surveying manual (PDF), built into the plugin.
const MANUAL: &[u8] = include_bytes!("../docs/Manuale_rilievo.pdf");

/// Write the manual next to the temporary files and open it with the
/// system's PDF viewer.
fn open_manual(host: &mut dyn HostApi) {
    let path = std::env::temp_dir().join(format!("RilievoVani_Manuale_{}.pdf", env!("CARGO_PKG_VERSION")));
    if let Err(e) = std::fs::write(&path, MANUAL) {
        host.push_error(&tr!(
            format!("RV_MANUALE: non riesco a scrivere il manuale in {}: {e}", path.display()),
            format!("RV_MANUALE: cannot write the manual to {}: {e}", path.display())
        ));
        return;
    }
    #[cfg(target_os = "windows")]
    let r = std::process::Command::new("cmd").args(["/C", "start", ""]).arg(&path).spawn();
    #[cfg(target_os = "macos")]
    let r = std::process::Command::new("open").arg(&path).spawn();
    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    let r = std::process::Command::new("xdg-open").arg(&path).spawn();
    match r {
        Ok(_) => host.push_info(&tr!(
            format!("RV_MANUALE: manuale di rilievo aperto ({}).", path.display()),
            format!("RV_MANUALE: surveying manual opened ({}).", path.display())
        )),
        Err(e) => host.push_error(&tr!(
            format!("RV_MANUALE: il manuale è in {} ma non riesco ad aprirlo ({e}): aprilo a mano.", path.display()),
            format!("RV_MANUALE: the manual is at {} but cannot be opened ({e}): open it by hand.", path.display())
        )),
    }
}

fn help(host: &mut dyn HostApi) {
    host.push_output(&tr!(
        format!(
            "Rilievo Vani {}: RV_VANO (misure di lati, diagonali e collegamenti di uno o più vani), RV_ETICHETTE \
             (lettere sui vertici), RV_APPLICA (porta i vani originali nella forma calcolata), RV_LIBRETTO (salva il libretto delle misure), RV_GUIDE (guida), RV_MANUALE (manuale di rilievo in PDF), RV_LANG IT|EN|AUTO.",
            env!("CARGO_PKG_VERSION")
        ),
        format!(
            "Rilievo Vani {}: RV_VANO (sides, diagonals and links of one or more rooms), RV_ETICHETTE (corner letters), \
             RV_APPLICA (move the original rooms to the computed shape), RV_LIBRETTO (save the measures book), RV_GUIDE (guide), RV_MANUALE (surveying manual, PDF), RV_LANG IT|EN|AUTO.",
            env!("CARGO_PKG_VERSION")
        )
    ));
}

fn lang(host: &mut dyn HostApi, rest: &str) {
    use crate::i18n::{parse_choice, set_choice, status, Choice, Lang};
    if !rest.trim().is_empty() {
        match parse_choice(rest) {
            Some(c) => {
                if let Err(e) = set_choice(c) {
                    host.push_error(&format!("RV_LANG: {e}"));
                    return;
                }
            }
            None => {
                host.push_error("RV_LANG: IT | EN | AUTO");
                return;
            }
        }
    }
    let (c, l) = status();
    let c = match c {
        Choice::Auto => "AUTO",
        Choice::It => "IT",
        Choice::En => "EN",
    };
    let l = if l == Lang::It { "italiano" } else { "English" };
    host.push_info(&format!("RV_LANG: {c} → {l}"));
}

/// Pick the rooms, or take the closed polylines already selected.
fn start_pick(host: &mut dyn HostApi, verb: &'static str, title: &'static str) {
    let all = rooms(host);
    if all.is_empty() {
        host.push_error(&tr!(
            format!("{title}: nel disegno non ci sono polilinee chiuse. Disegna ogni vano con PLINEA … Chiudi, poi riprova."),
            format!("{title}: the drawing has no closed polylines. Draw each room with PLINE … Close, then try again.")
        ));
        return;
    }
    let sel = host.selection();
    let pre: Vec<Handle> = sel.iter().copied().filter(|h| all.iter().any(|r| r.handle == *h)).collect();
    if !pre.is_empty() {
        let _ = host.set_selection(&[]);
        handle(host, &format!("{verb} {}", key(&pre)));
        return;
    }
    host.start_interactive(Box::new(tools::PickRoomTool::new(all, verb, title)));
}

/// Default path of the measures book: next to the drawing.
fn default_book_path(host: &dyn HostApi) -> String {
    let doc = host.document_path(host.tab_id());
    let (dir, stem) = match &doc {
        Some(p) => (
            p.parent().map(PathBuf::from),
            p.file_stem().map(|s| s.to_string_lossy().to_string()).unwrap_or_else(|| "rilievo".into()),
        ),
        None => (None, "rilievo".to_string()),
    };
    let dir = dir.or_else(|| std::env::var_os("USERPROFILE").or_else(|| std::env::var_os("HOME")).map(|h| PathBuf::from(h).join("Documents")));
    let name = format!("{stem}_{}.csv", tr!("libretto_misure", "measures_book"));
    match dir {
        Some(d) => d.join(name).to_string_lossy().to_string(),
        None => name,
    }
}

fn open_dialog(host: &mut dyn HostApi, hs: &[Handle], msg: Option<String>) {
    let Some(rooms) = fetch_rooms(host, hs) else {
        host.push_error(tr!("RV_VANO: un vano scelto non è più nel disegno.", "RV_VANO: a chosen room is no longer in the drawing."));
        return;
    };
    let survey = Survey::new(rooms.iter().map(|r| r.pts.clone()).collect());
    let n_sides: usize = rooms.iter().map(|r| r.pts.len()).sum();
    let saved = match saved_for(hs) {
        Some(s) if s.sides.len() == n_sides => s,
        _ => {
            let all: Vec<P2> = rooms.iter().flat_map(|r| r.pts.iter().copied()).collect();
            let longest = rooms.iter().map(|r| default_sizes(&r.pts)).fold((0.0f64, 0.0f64), |a, b| (a.0.max(b.0), a.1.max(b.1)));
            let (h, tol) = if all.is_empty() { (1.0, 0.01) } else { longest };
            let (sides, others) = survey.initial(decimals(tol), &fmt_num);
            let longest = rooms.iter().flat_map(|r| (0..r.pts.len()).map(move |i| solve::dist(r.pts[i], r.pts[(i + 1) % r.pts.len()]))).fold(0.0f64, f64::max);
            Saved {
                sides,
                others,
                h,
                tol,
                dims: true,
                table: true,
                csv: false,
                csv_path: default_book_path(host),
                constraints: Vec::new(),
                openings: Vec::new(),
                unit: survey::guess_unit(longest),
                fill: true,
                replace: false,
            }
        }
    };
    if msg.is_none() {
        if rooms.iter().any(|r| r.had_arcs) {
            host.push_info(tr!(
                "RV_VANO: i tratti ad arco delle polilinee sono presi come corde (lati diritti).",
                "RV_VANO: the polylines' arc segments are taken as chords (straight sides)."
            ));
        }
        host.begin_undo_group("RV_ETICHETTE");
        write_labels(host, &rooms, &survey, saved.h);
        host.end_undo_group();
        host.set_dirty();
    }
    host.start_interactive(Box::new(tools::MeasureDialog::new(hs.to_vec(), &survey, &saved, msg)));
}

fn labels_only(host: &mut dyn HostApi, hs: &[Handle]) {
    let Some(rooms) = fetch_rooms(host, hs) else {
        host.push_error(tr!("RV_ETICHETTE: un vano scelto non è più nel disegno.", "RV_ETICHETTE: a chosen room is no longer in the drawing."));
        return;
    };
    let survey = Survey::new(rooms.iter().map(|r| r.pts.clone()).collect());
    let h = saved_for(hs).map(|s| s.h).unwrap_or_else(|| rooms.iter().map(|r| default_sizes(&r.pts).0).fold(0.0, f64::max));
    host.begin_undo_group("RV_ETICHETTE");
    write_labels(host, &rooms, &survey, h);
    host.end_undo_group();
    host.set_dirty();
    let n: usize = rooms.iter().map(|r| r.pts.len()).sum();
    host.push_info(&tr!(
        format!(
            "RV_ETICHETTE: {n} lettere scritte sul layer {LAYER_LABELS}. Stampa la pianta e usala in cantiere per segnare \
             lati, diagonali e collegamenti misurati; poi inserisci le misure con Misure e diagonali."
        ),
        format!(
            "RV_ETICHETTE: {n} letters written on layer {LAYER_LABELS}. Print the plan and use it on site to note the \
             measured sides, diagonals and links; then type them with Sides and diagonals."
        )
    ));
}

// ── Layers, text ────────────────────────────────────────────────────────────

fn ensure_layer(host: &mut dyn HostApi, name: &str, aci: i16) {
    if host.document().layers.get(name).is_some() {
        return;
    }
    let config = LayerConfig { name: name.to_string(), color: Some(codec::types::Color::from_index(aci)), ..Default::default() };
    if host.add_layer(config).is_none() {
        let _ = host.table_operation(TableOperation::LayerCreate {
            config: LayerConfig { name: name.to_string(), color: Some(codec::types::Color::from_index(aci)), ..Default::default() },
        });
    }
}

fn mtext(value: &str, at: P2, z: f64, h: f64, att: AttachmentPoint, layer: &str) -> EntityType {
    let mut t = MText::with_value(value, Vector3::new(at[0], at[1], z)).with_height(h);
    t.attachment_point = att;
    t.common.layer = layer.to_string();
    EntityType::MText(Box::new(t))
}

fn pline(pts: &[P2], z: f64, closed: bool, layer: &str) -> EntityType {
    let mut p = LwPolyline::new();
    p.vertices = pts.iter().map(|q| LwVertex::new(Vector2::new(q[0], q[1]))).collect();
    p.is_closed = closed;
    p.elevation = z;
    p.common.layer = layer.to_string();
    EntityType::LwPolyline(p)
}

fn room_title(r: usize) -> String {
    tr!(format!("Vano {}", r + 1), format!("Room {}", r + 1))
}

/// MText on the labels layer: (handle, value, position).
fn existing_letters(host: &dyn HostApi) -> Vec<(Handle, String, P2)> {
    host.document()
        .entities()
        .filter_map(|e| match e {
            EntityType::MText(t) if t.common.layer.eq_ignore_ascii_case(LAYER_LABELS) => {
                Some((t.common.handle, t.value.trim().to_string(), [t.insertion_point.x, t.insertion_point.y]))
            }
            _ => None,
        })
        .collect()
}

/// Leader lines on the labels layer: (handle, start point).
fn existing_leaders(host: &dyn HostApi) -> Vec<(Handle, P2)> {
    host.document()
        .entities()
        .filter_map(|e| match e {
            EntityType::LwPolyline(p) if p.common.layer.eq_ignore_ascii_case(LAYER_LABELS) && p.vertices.len() == 2 && !p.is_closed => {
                Some((p.common.handle, [p.vertices[0].location.x, p.vertices[0].location.y]))
            }
            _ => None,
        })
        .collect()
}

/// Letters (with leaders where moved) and, with several rooms, room titles,
/// placed so that none covers another.
fn label_entities(rooms: &[Vec<P2>], survey: &Survey, h: f64, z: f64, layer: &str, letter_h: f64, area_unit: Option<usize>) -> Vec<EntityType> {
    let mut titles: Vec<(String, P2, f64)> = Vec::new();
    for (r, pts) in rooms.iter().enumerate() {
        let c = centroid(pts);
        let has_title = survey.multi();
        if has_title {
            titles.push((room_title(r), c, h * 1.4));
        }
        if let Some(u) = area_unit {
            let f = survey::unit_factor(u);
            let area = solve::signed_area(pts).abs() * f * f;
            let at = if has_title { [c[0], c[1] - h * 1.9] } else { c };
            titles.push((format!("{} m²", fmt_num(area, 2)), at, h * 1.1));
        }
    }
    let names: Vec<Vec<String>> = rooms.iter().enumerate().map(|(r, pts)| (0..pts.len()).map(|i| survey.corner(r, i)).collect()).collect();
    let mut out: Vec<EntityType> =
        titles.iter().map(|(t, at, hh)| mtext(t, *at, z, *hh, AttachmentPoint::MiddleCenter, layer)).collect();
    for p in labels::place(rooms, &names, letter_h, &titles) {
        out.push(mtext(&p.text, p.at, z, p.height, AttachmentPoint::MiddleCenter, layer));
        if let Some((a, b)) = p.leader {
            out.push(pline(&[a, b], z, false, layer));
        }
    }
    out
}

/// Write the working letters on the rooms drawn by eye, replacing those
/// already there.
fn write_labels(host: &mut dyn HostApi, rooms: &[Room], survey: &Survey, h: f64) {
    ensure_layer(host, LAYER_LABELS, 6);
    remove_labels(host, rooms, survey, h);
    let z = rooms.first().map_or(0.0, |r| r.z);
    let pts: Vec<Vec<P2>> = rooms.iter().map(|r| r.pts.clone()).collect();
    let new = label_entities(&pts, survey, h, z, LAYER_LABELS, h, None);
    if !new.is_empty() {
        host.add_entities(new);
    }
}

/// Remove the working letters (and their leaders) of the rooms.
fn remove_labels(host: &mut dyn HostApi, rooms: &[Room], survey: &Survey, h: f64) {
    let have = existing_letters(host);
    let leaders = existing_leaders(host);
    let mut gone: Vec<Handle> = Vec::new();
    for (r, room) in rooms.iter().enumerate() {
        let hh = h.max(default_sizes(&room.pts).0);
        let mut targets: Vec<(String, P2)> = (0..room.pts.len()).map(|i| (survey.corner(r, i), room.pts[i])).collect();
        if survey.multi() {
            targets.push((room_title(r), centroid(&room.pts)));
        }
        for (name, near) in targets {
            let best = have
                .iter()
                .filter(|(hd, v, _)| *v == name && !gone.contains(hd))
                .map(|(hd, _, p)| (*hd, solve::dist(*p, near)))
                .filter(|(_, d)| *d < hh * 8.5)
                .min_by(|a, b| a.1.total_cmp(&b.1));
            if let Some((hd, _)) = best {
                gone.push(hd);
            }
        }
        for (hd, start) in &leaders {
            if !gone.contains(hd) && room.pts.iter().any(|v| solve::dist(*v, *start) < hh * 0.3) {
                gone.push(*hd);
            }
        }
    }
    for hd in gone {
        host.remove_entity(hd);
    }
}

// ── Apply ───────────────────────────────────────────────────────────────────

fn explain(e: &SolveError, survey: &Survey, measures: &[Measure]) -> String {
    let names = |g: &[usize]| g.iter().map(|&p| survey.names[p].clone()).collect::<Vec<_>>().join(", ");
    match e {
        SolveError::TooFewPoints => tr!("un vano ha meno di 3 vertici".to_string(), "a room has fewer than 3 corners".to_string()),
        SolveError::BadObs(k) => {
            let name = measures.get(*k).map(|m| m.name.clone()).unwrap_or_default();
            tr!(format!("la misura {name} non è valida (unisce un vertice a sé stesso?)"), format!("measure {name} is not valid (does it join a corner to itself?)"))
        }
        SolveError::NotEnough { needed, have, group } => {
            if group.len() < 3 {
                return tr!(
                    format!("i vertici {} non hanno misure: inserisci i lati che li toccano", names(group)),
                    format!("corners {} have no measures: type the sides that touch them", names(group))
                );
            }
            let rooms = survey.room_numbers(group);
            let missing = needed - have;
            let sides_missing: Vec<String> = rooms
                .iter()
                .flat_map(|&r1| {
                    let r = r1 - 1;
                    let n = survey.rooms[r].len();
                    (0..n)
                        .filter(move |&i| {
                            let (a, b) = (survey.point_of[r][i], survey.point_of[r][(i + 1) % n]);
                            !measures.iter().any(|m| matches!(m.meas, Meas::Dist { i, j, .. } if (i, j) == (a, b) || (i, j) == (b, a)))
                        })
                        .map(move |i| format!("{}–{}", survey.corner(r, i), survey.corner(r, (i + 1) % n)))
                })
                .collect();
            let mut s = if rooms.len() > 1 {
                tr!(
                    format!(
                        "per i vani {} collegati tra loro servono almeno {needed} misure, ne hai inserite {have}: ne mancano {missing}. \
                         Ogni vano ha bisogno dei suoi lati e diagonali, più almeno 3 misure di collegamento con l'altro vano",
                        join_numbers(&rooms)
                    ),
                    format!(
                        "rooms {} are linked and need at least {needed} measures, {have} typed: {missing} missing. \
                         Each room needs its sides and diagonals, plus at least 3 measures linking it to the other room",
                        join_numbers(&rooms)
                    )
                )
            } else {
                tr!(
                    format!("servono almeno {needed} misure, ne hai inserite {have}: ne mancano {missing}"),
                    format!("at least {needed} measures are needed, {have} typed: {missing} missing")
                )
            };
            if !sides_missing.is_empty() {
                s += &tr!(format!(" (lati senza misura: {})", sides_missing.join(", ")), format!(" (sides with no measure: {})", sides_missing.join(", ")));
            }
            s
        }
        SolveError::NotRigid { group } => {
            let rooms = survey.room_numbers(group);
            let mut proposed = Vec::new();
            for &r1 in &rooms {
                let r = r1 - 1;
                for (i, j) in solve::suggested_diagonals(&survey.rooms[r]) {
                    let (a, b) = (survey.point_of[r][i], survey.point_of[r][j]);
                    let have = measures.iter().any(|m| matches!(m.meas, Meas::Dist { i, j, .. } if (i, j) == (a, b) || (i, j) == (b, a)));
                    if !have {
                        proposed.push(format!("{}–{}", survey.corner(r, i), survey.corner(r, j)));
                    }
                }
            }
            let mut s = if rooms.len() > 1 && proposed.is_empty() {
                tr!(
                    format!(
                        "le misure di collegamento tra i vani {} non bastano a fissarne la posizione: prendile verso vertici diversi \
                         (per esempio lo spessore del muro più due misure attraverso la porta, da vertici diversi)",
                        join_numbers(&rooms)
                    ),
                    format!(
                        "the measures linking rooms {} do not fix their position: take them to different corners \
                         (for example the wall thickness plus two measures through the door, from different corners)",
                        join_numbers(&rooms)
                    )
                )
            } else {
                tr!(
                    "le diagonali non bloccano tutta la forma: una parte del vano può ancora deformarsi".to_string(),
                    "the diagonals do not brace the whole shape: part of the room can still move".to_string()
                )
            };
            if !proposed.is_empty() {
                s += &tr!(format!(". Misura anche: {}", proposed.join(", ")), format!(". Also measure: {}", proposed.join(", ")));
            }
            s
        }
        SolveError::NoConvergence => tr!(
            "le misure non stanno insieme (per esempio una diagonale più lunga di due lati sommati, due vertici scambiati \
             o un collegamento scritto al contrario): ricontrollale"
                .to_string(),
            "the measures do not fit together (for example a diagonal longer than two sides together, two corners swapped \
             or a link written the wrong way round): check them"
                .to_string()
        ),
    }
}

fn join_numbers(v: &[usize]) -> String {
    let s: Vec<String> = v.iter().map(|n| n.to_string()).collect();
    match s.len() {
        0 => String::new(),
        1 => s[0].clone(),
        _ => format!("{} {} {}", s[..s.len() - 1].join(", "), tr!("e", "and"), s[s.len() - 1]),
    }
}

fn kind_name(k: Kind) -> &'static str {
    match k {
        Kind::Side => tr!("lato", "side"),
        Kind::Diagonal => tr!("diagonale", "diagonal"),
        Kind::Link => tr!("collegamento", "link"),
        Kind::Wall => tr!("spessore muro", "wall thickness"),
        Kind::Angle => tr!("angolo", "angle"),
        Kind::Level => tr!("orizzontale / verticale", "horizontal / vertical"),
        Kind::Door => tr!("porta comune", "shared door"),
    }
}

/// A measure's value as shown: angles in degrees.
fn show(m: &Measure, v: f64, dec: usize) -> String {
    match m.kind {
        Kind::Angle => format!("{}°", fmt_num(v.to_degrees(), 2)),
        _ => fmt_num(v, dec),
    }
}

/// A residual as shown: angles in degrees, signed.
fn show_res(m: &Measure, v: f64, dec: usize) -> String {
    match m.kind {
        Kind::Angle => format!("{}°", fmt_signed(v.to_degrees(), 2)),
        _ => fmt_signed(v, dec),
    }
}

fn apply(host: &mut dyn HostApi, hs: &[Handle], saved: Saved) {
    remember(hs, saved.clone());
    let Some(rooms) = fetch_rooms(host, hs) else {
        host.push_error(tr!("RV_VANO: un vano scelto non è più nel disegno.", "RV_VANO: a chosen room is no longer in the drawing."));
        return;
    };
    let survey = Survey::new(rooms.iter().map(|r| r.pts.clone()).collect());
    let (measures, openings) = match survey.parse(&saved, &parse_num) {
        Ok(p) => (p.measures, p.openings),
        Err(msg) => {
            host.push_error(&format!("RV_VANO: {msg}."));
            open_dialog(host, hs, Some(msg));
            return;
        }
    };
    let meas: Vec<Meas> = measures.iter().map(|m| m.meas).collect();
    // When the measures leave part of the shape free (furniture in the way of
    // a diagonal…), finish it from the drawing if the user allows it.
    let mut filled: Option<String> = None;
    let sol = match solve::solve_network(&survey.points, &meas, &survey.datum()) {
        Ok(s) => s,
        Err(e @ (SolveError::NotEnough { .. } | SolveError::NotRigid { .. }))
            if saved.fill && solve_filled(&survey, &meas).is_some() =>
        {
            filled = Some(explain(&e, &survey, &measures));
            solve_filled(&survey, &meas).unwrap()
        }
        Err(e) => {
            let msg = explain(&e, &survey, &measures);
            host.push_error(&format!("RV_VANO: {msg}."));
            open_dialog(host, hs, Some(msg));
            return;
        }
    };

    let th = saved.h.max(1e-9);
    let dec = decimals(saved.tol);
    let z = rooms[0].z;
    let bad: Vec<usize> = (0..measures.len()).filter(|&k| saved.tol > 0.0 && sol.scaled[k].abs() > saved.tol).collect();

    host.begin_undo_group("RV_VANO");
    ensure_layer(host, LAYER_ROOM, 3);
    ensure_layer(host, LAYER_NOTES, 2);
    remove_labels(host, &rooms, &survey, th);

    let adjusted: Vec<Vec<P2>> = (0..rooms.len()).map(|r| survey.room_points(r, &sol.pts)).collect();
    let mut ents: Vec<EntityType> = adjusted.iter().map(|pts| pline(pts, z, true, LAYER_ROOM)).collect();
    ents.extend(label_entities(&adjusted, &survey, th, z, LAYER_NOTES, th * 1.2, Some(saved.unit)));
    ensure_layer(host, LAYER_OPENINGS, 4);
    let (op_ents, op_warn) = openings_entities(&survey, &openings, &sol.pts, th, dec, z);
    ents.extend(op_ents);
    if saved.table {
        ents.extend(table(&sol, &measures, &bad, th, dec, z, filled.is_some()));
    }
    let added = host.add_entities(ents);
    *LAST_RESULT.lock().unwrap_or_else(|e| e.into_inner()) = Some(
        rooms
            .iter()
            .zip(&adjusted)
            .enumerate()
            .map(|(r, (room, pts))| (room.handle, pts.clone(), added.get(r).copied()))
            .collect(),
    );

    let mut dims_failed = 0;
    if saved.dims {
        dims_failed = dimension(host, &survey, &sol.pts, &measures, th, dec, z);
    }
    host.end_undo_group();
    host.set_dirty();

    // Measures book: kept for RV_LIBRETTO, written now when asked.
    let doc = host.document_path(host.tab_id()).map(|p| p.to_string_lossy().to_string());
    let book = book_csv(&survey, &sol, &measures, &openings, &bad, &saved, dec, doc.as_deref(), filled.as_deref());
    let path = if saved.csv_path.trim().is_empty() { default_book_path(host) } else { saved.csv_path.trim().to_string() };
    *LAST_BOOK.lock().unwrap_or_else(|e| e.into_inner()) = Some((book.clone(), path.clone()));

    // Report on the command line.
    let nr = rooms.len();
    let mut lines = vec![if nr == 1 {
        tr!(
            format!("RV_VANO: vano di {} vertici ricalcolato con {} misure → layer {LAYER_ROOM}.", rooms[0].pts.len(), measures.len()),
            format!("RV_VANO: room of {} corners recomputed from {} measures → layer {LAYER_ROOM}.", rooms[0].pts.len(), measures.len())
        )
    } else {
        tr!(
            format!("RV_VANO: {nr} vani ricalcolati con {} misure ({} gruppi indipendenti) → layer {LAYER_ROOM}.", measures.len(), sol.groups),
            format!("RV_VANO: {nr} rooms recomputed from {} measures ({} independent groups) → layer {LAYER_ROOM}.", measures.len(), sol.groups)
        )
    }];
    if nr > 1 && sol.groups > 1 {
        lines.push(tr!(
            "  Alcuni vani non sono collegati tra loro: ciascuno resta al suo posto. Per collegarli aggiungi spessori dei muri e misure attraverso le porte.".to_string(),
            "  Some rooms are not linked: each stays where it is. To link them add wall thicknesses and measures through the doors.".to_string()
        ));
    }
    match sol.sigma0 {
        Some(s0) => lines.push(tr!(
            format!("  Misure in più (controllo): {} · scarto quadratico medio σ₀ = {}", sol.redundancy, fmt_num(s0, dec)),
            format!("  Extra (check) measures: {} · standard deviation σ₀ = {}", sol.redundancy, fmt_num(s0, dec))
        )),
        None => lines.push(tr!(
            "  Nessuna misura di controllo: la forma è esatta ma un errore di misura non si vede. Misura una diagonale in più.".to_string(),
            "  No check measure: the shape is exact but a wrong measure cannot show. Measure one more diagonal.".to_string()
        )),
    }
    if sol.redundancy > 0 {
        for (k, m) in measures.iter().enumerate() {
            let flag = if bad.contains(&k) { "  !" } else { "" };
            let v = sol.residuals[k];
            lines.push(format!("  {:<18} {} → {}  ({}){flag}", m.name, show(m, m.meas.value(), dec), show(m, m.meas.value() + v, dec), show_res(m, v, dec)));
        }
    }
    for l in &lines {
        host.push_output(l);
    }
    if !bad.is_empty() {
        let names: Vec<&str> = bad.iter().map(|&k| measures[k].name.as_str()).collect();
        host.push_error(&tr!(
            format!(
                "RV_VANO: scarti oltre la tolleranza ({}) su {}: ricontrolla queste misure (cifra letta male, vertice sbagliato?). \
                 Con una sola misura in più l'errore si spalma su più misure: misurarne un'altra aiuta a trovarlo.",
                fmt_num(saved.tol, dec),
                names.join(", ")
            ),
            format!(
                "RV_VANO: residuals over the tolerance ({}) on {}: check these measures (misread digit, wrong corner?). \
                 With a single extra measure the error spreads over several measures: one more measure helps find it.",
                fmt_num(saved.tol, dec),
                names.join(", ")
            )
        ));
    }
    if let Some(why) = &filled {
        host.push_error(&tr!(
            format!(
                "RV_VANO: misure non sufficienti ({why}). Il calcolo è stato completato con gli angoli e le posizioni del \
                 disegno: le parti non misurate sono solo indicative. Se puoi, aggiungi le misure mancanti o dei vincoli."
            ),
            format!(
                "RV_VANO: not enough measures ({why}). The calculation was finished with the drawing's angles and \
                 positions: the parts not measured are only indicative. If you can, add the missing measures or constraints."
            )
        ));
    }
    for w in &op_warn {
        host.push_error(&format!("RV_VANO: {w}."));
    }
    if dims_failed > 0 {
        host.push_error(&tr!(
            format!("RV_VANO: {dims_failed} quote non create dal CAD; le misure sono scritte come testo."),
            format!("RV_VANO: the CAD did not create {dims_failed} dimensions; the measures are written as text.")
        ));
    }
    if saved.replace {
        apply_to_drawing(host, false);
    } else {
        host.push_info(tr!(
            "RV_VANO: controllato il risultato, Applica al disegno (RV_APPLICA) porta le polilinee originali nella forma calcolata.",
            "RV_VANO: once the result is checked, Apply to drawing (RV_APPLICA) moves the original polylines to the computed shape."
        ));
    }
    if saved.csv {
        write_book(host, &path, &book);
    } else {
        host.push_info(tr!(
            "RV_VANO: per salvare il libretto delle misure (CSV) usa Libretto misure (RV_LIBRETTO).",
            "RV_VANO: to save the measures book (CSV) use Measures book (RV_LIBRETTO)."
        ));
    }
}

/// Move the corners of the original polylines to the last calculation's
/// result, keeping each polyline (handle, layer, properties) and removing
/// the adjusted copy on RILIEVO_CORRETTO, which would now lie on top of it.
/// One undo step.
fn apply_to_drawing(host: &mut dyn HostApi, own_undo: bool) {
    let Some(last) = LAST_RESULT.lock().unwrap_or_else(|e| e.into_inner()).take() else {
        host.push_error(tr!(
            "RV_APPLICA: non c'è un calcolo da applicare. Prima esegui Misure e diagonali (RV_VANO) e controlla il risultato.",
            "RV_APPLICA: there is no calculation to apply. Run Sides and diagonals (RV_VANO) first and check the result."
        ));
        return;
    };
    if own_undo {
        host.begin_undo_group("RV_APPLICA");
    }
    let mut done = 0;
    let mut missing = 0;
    for (h, pts, copy) in &last {
        let live = host.entities_batch(&[*h]).ok().and_then(|v| v.into_iter().next().flatten()).or_else(|| host.document().get_entity(*h).cloned());
        match live {
            Some(EntityType::LwPolyline(mut p)) => {
                p.vertices = pts.iter().map(|q| LwVertex::new(Vector2::new(q[0], q[1]))).collect();
                p.is_closed = true;
                if host.update_entity(EntityType::LwPolyline(p)) {
                    done += 1;
                    if let Some(c) = copy {
                        host.remove_entity(*c);
                    }
                } else {
                    missing += 1;
                }
            }
            _ => missing += 1,
        }
    }
    if own_undo {
        host.end_undo_group();
    }
    host.set_dirty();
    if done > 0 {
        host.push_info(&tr!(
            format!("RV_APPLICA: {done} polilinee portate nella forma calcolata (Ctrl+Z per annullare)."),
            format!("RV_APPLICA: {done} polylines moved to the computed shape (Ctrl+Z to undo).")
        ));
    }
    if missing > 0 {
        host.push_error(&tr!(
            format!("RV_APPLICA: {missing} polilinee non più nel disegno o non modificabili: restano le copie su {LAYER_ROOM}."),
            format!("RV_APPLICA: {missing} polylines no longer in the drawing or not editable: the copies on {LAYER_ROOM} stay.")
        ));
    }
}

fn write_book(host: &mut dyn HostApi, path: &str, text: &str) {
    let mut p = PathBuf::from(path.trim().trim_matches('"'));
    if p.extension().is_none() {
        p.set_extension("csv");
    }
    if let Some(d) = p.parent() {
        if !d.as_os_str().is_empty() {
            let _ = std::fs::create_dir_all(d);
        }
    }
    match std::fs::write(&p, text.as_bytes()) {
        Ok(()) => host.push_info(&tr!(
            format!("Libretto delle misure salvato in {}. Si apre con Excel o LibreOffice Calc.", p.display()),
            format!("Measures book saved to {}. It opens in Excel or LibreOffice Calc.", p.display())
        )),
        Err(e) => host.push_error(&tr!(
            format!("Non riesco a salvare il libretto in {}: {e}. Scegli un'altra cartella con Libretto misure (RV_LIBRETTO).", p.display()),
            format!("Cannot save the book to {}: {e}. Choose another folder with Measures book (RV_LIBRETTO).", p.display())
        )),
    }
}

/// The measures book as CSV: semicolons and decimal commas in Italian (as
/// Excel and Calc expect in Italy), commas and points in English; UTF-8
/// with BOM so Excel reads the accents.
#[allow(clippy::too_many_arguments)]
pub fn book_csv(
    survey: &Survey,
    sol: &Solution,
    measures: &[Measure],
    openings: &[Opening],
    bad: &[usize],
    saved: &Saved,
    dec: usize,
    doc: Option<&str>,
    filled: Option<&str>,
) -> String {
    let tol = saved.tol;
    let sep = if crate::i18n::italian() { ";" } else { "," };
    let cell = |s: &str| {
        if s.contains(sep) || s.contains('"') {
            format!("\"{}\"", s.replace('"', "\"\""))
        } else {
            s.to_string()
        }
    };
    let row = |v: Vec<String>| v.iter().map(|s| cell(s)).collect::<Vec<_>>().join(sep);
    let mut out = String::from("\u{feff}");
    let mut push = |v: Vec<String>| {
        out.push_str(&row(v));
        out.push_str("\r\n");
    };
    push(vec![tr!("RILIEVO VANI — LIBRETTO DELLE MISURE", "RILIEVO VANI — MEASURES BOOK").into()]);
    push(vec![
        tr!("Progetto hobbistico, non adatto a uso professionale: controllare sempre i risultati.", "Hobby project, not for professional use: always check the results.").into(),
    ]);
    if let Some(d) = doc {
        push(vec![tr!("Disegno", "Drawing").into(), d.to_string()]);
    }
    push(vec![tr!("Vani", "Rooms").into(), survey.rooms.len().to_string()]);
    push(vec![tr!("Misure", "Measures").into(), measures.len().to_string()]);
    push(vec![tr!("Misure in più (ridondanza)", "Extra measures (redundancy)").into(), sol.redundancy.to_string()]);
    push(vec![
        tr!("Scarto quadratico medio σ₀", "Standard deviation σ₀").into(),
        sol.sigma0.map(|s| fmt_num(s, dec + 1)).unwrap_or_else(|| "—".into()),
    ]);
    push(vec![tr!("Tolleranza", "Tolerance").into(), fmt_num(tol, dec)]);
    if let Some(why) = filled {
        push(vec![
            tr!("ATTENZIONE", "WARNING").into(),
            tr!(
                format!("misure non sufficienti ({why}): forma completata con gli angoli del disegno, parti non misurate indicative"),
                format!("not enough measures ({why}): shape finished with the drawing's angles, unmeasured parts indicative")
            ),
        ]);
    }
    push(vec![]);
    push(vec![tr!("MISURE", "MEASURES").into()]);
    push(vec![
        tr!("Misura", "Measure").into(),
        tr!("Tipo", "Kind").into(),
        tr!("Misurata", "Measured").into(),
        tr!("Calcolata", "Computed").into(),
        tr!("Scarto", "Residual").into(),
        tr!("Oltre tolleranza", "Over tolerance").into(),
    ]);
    for (k, m) in measures.iter().enumerate() {
        let v = sol.residuals[k];
        push(vec![
            m.name.clone(),
            kind_name(m.kind).into(),
            show(m, m.meas.value(), dec + 1),
            show(m, m.meas.value() + v, dec + 1),
            show_res(m, v, dec + 1),
            if bad.contains(&k) { tr!("SÌ", "YES").into() } else { String::new() },
        ]);
    }
    push(vec![]);
    push(vec![tr!("COORDINATE DEI VERTICI (sistema del disegno)", "CORNER COORDINATES (drawing system)").into()]);
    push(vec![tr!("Vertice", "Corner").into(), "X".into(), "Y".into()]);
    for (p, name) in survey.names.iter().enumerate() {
        push(vec![name.clone(), fmt_num(sol.pts[p][0], dec + 1), fmt_num(sol.pts[p][1], dec + 1)]);
    }
    push(vec![]);
    push(vec![tr!("SUPERFICI E PERIMETRI", "AREAS AND PERIMETERS").into()]);
    push(vec![tr!("Vano", "Room").into(), tr!("Superficie (m²)", "Area (m²)").into(), tr!("Perimetro (unità del disegno)", "Perimeter (drawing units)").into()]);
    let f = survey::unit_factor(saved.unit);
    for r in 0..survey.rooms.len() {
        let pts = survey.room_points(r, &sol.pts);
        let per: f64 = (0..pts.len()).map(|i| solve::dist(pts[i], pts[(i + 1) % pts.len()])).sum();
        push(vec![(r + 1).to_string(), fmt_num(solve::signed_area(&pts).abs() * f * f, 2), fmt_num(per, dec)]);
    }
    if !openings.is_empty() {
        push(vec![]);
        push(vec![tr!("PORTE E FINESTRE", "DOORS AND WINDOWS").into()]);
        push(vec![
            tr!("Nome", "Name").into(),
            tr!("Tipo", "Kind").into(),
            tr!("Lato (dallo spigolo)", "Side (from corner)").into(),
            tr!("Distanza", "Distance").into(),
            tr!("Larghezza", "Width").into(),
            tr!("Spessore muro", "Wall thickness").into(),
            tr!("Comune con", "Shared with").into(),
        ]);
        for o in openings {
            push(vec![
                o.name.clone(),
                op_kind_name(o.kind).into(),
                format!("{}–{}", survey.names[o.a], survey.names[o.b]),
                fmt_num(o.s, dec),
                fmt_num(o.w, dec),
                o.thick.map(|t| fmt_num(t, dec)).unwrap_or_default(),
                o.twin.map(|t| room_title(openings[t].room)).unwrap_or_default(),
            ]);
        }
    }
    out
}

/// The table of measures, right of the rooms: measure, measured, computed
/// and residual; residuals over the tolerance in red.
/// Solve with stand-ins from the drawing added (low weight), returning only
/// the real measures' residuals.
fn solve_filled(survey: &Survey, meas: &[Meas]) -> Option<Solution> {
    let extra = survey.stand_ins(meas);
    let mut all = meas.to_vec();
    all.extend(extra.iter().copied());
    let mut w = vec![1.0; meas.len()];
    w.extend(std::iter::repeat_n(0.05, extra.len()));
    let mut s = solve::solve_network_weighted(&survey.points, &all, &w, meas.len(), &survey.datum()).ok()?;
    s.residuals.truncate(meas.len());
    s.scaled.truncate(meas.len());
    Some(s)
}

fn table(sol: &Solution, measures: &[Measure], bad: &[usize], h: f64, dec: usize, z: f64, filled: bool) -> Vec<EntityType> {
    let maxx = sol.pts.iter().map(|p| p[0]).fold(f64::MIN, f64::max);
    let maxy = sol.pts.iter().map(|p| p[1]).fold(f64::MIN, f64::max);
    let x0 = maxx + h * 5.0;
    let row_h = h * 1.9;
    let longest = measures.iter().map(|m| m.name.chars().count()).max().unwrap_or(5).max(6) as f64;
    let col_w = [h * (longest * 0.75 + 1.5), h * 6.5, h * 6.5, h * 6.0];
    let width: f64 = col_w.iter().sum();
    let mut out = Vec::new();
    let mut y = maxy;

    out.push(mtext(tr!("RILIEVO — misure", "SURVEY — measures"), [x0, y + row_h * 0.6], z, h, AttachmentPoint::MiddleLeft, LAYER_NOTES));
    let headers = [tr!("Misura", "Measure"), tr!("Misurata", "Measured"), tr!("Calcolata", "Computed"), tr!("Scarto", "Residual")];
    let mut rows: Vec<[String; 4]> = vec![headers.map(str::to_string)];
    for (k, m) in measures.iter().enumerate() {
        let v = sol.residuals[k];
        rows.push([
            m.name.clone(),
            show(m, m.meas.value(), dec),
            show(m, m.meas.value() + v, dec),
            if sol.redundancy > 0 { show_res(m, v, dec) } else { "—".into() },
        ]);
    }
    let top = y;
    for (r, row) in rows.iter().enumerate() {
        let cy = y - row_h / 2.0;
        let mut cx = x0;
        for (c, cell) in row.iter().enumerate() {
            let (at, att) = if c == 0 {
                ([cx + h * 0.5, cy], AttachmentPoint::MiddleLeft)
            } else {
                ([cx + col_w[c] - h * 0.5, cy], AttachmentPoint::MiddleRight)
            };
            let mut t = mtext(cell, at, z, h, att, LAYER_NOTES);
            if r > 0 && c == 3 && bad.contains(&(r - 1)) {
                t.common_mut().color = codec::types::Color::from_index(1);
            }
            out.push(t);
            cx += col_w[c];
        }
        y -= row_h;
    }
    let bottom = y;
    out.push(pline(&[[x0, top], [x0 + width, top], [x0 + width, bottom], [x0, bottom]], z, true, LAYER_NOTES));
    out.push(pline(&[[x0, top - row_h], [x0 + width, top - row_h]], z, false, LAYER_NOTES));
    let mut cx = x0;
    for w in &col_w[..3] {
        cx += w;
        out.push(pline(&[[cx, top], [cx, bottom]], z, false, LAYER_NOTES));
    }
    let foot = match sol.sigma0 {
        Some(s0) => tr!(
            format!("Misure in più: {} · σ₀ = {}", sol.redundancy, fmt_num(s0, dec)),
            format!("Extra measures: {} · σ₀ = {}", sol.redundancy, fmt_num(s0, dec))
        ),
        None => tr!("Nessuna misura di controllo".to_string(), "No check measure".to_string()),
    };
    out.push(mtext(&foot, [x0, bottom - row_h * 0.6], z, h * 0.85, AttachmentPoint::MiddleLeft, LAYER_NOTES));
    if filled {
        let mut t = mtext(
            tr!(
                "Misure non sufficienti: forma completata dal disegno (parti non misurate indicative)",
                "Not enough measures: shape finished from the drawing (unmeasured parts indicative)"
            ),
            [x0, bottom - row_h * 1.5],
            z,
            h * 0.85,
            AttachmentPoint::MiddleLeft,
            LAYER_NOTES,
        );
        t.common_mut().color = codec::types::Color::from_index(1);
        out.push(t);
    }
    out
}

fn sysvar_text(host: &dyn HostApi, name: &str) -> Option<String> {
    match host.system_variable(name) {
        Some(HostSettingValue::Text(s)) if !s.trim().is_empty() => Some(s),
        _ => None,
    }
}

/// The plugin's dimension style, sized to the text height.
fn ensure_dim_style(host: &mut dyn HostApi, h: f64, dec: usize) -> bool {
    let sep = if crate::i18n::italian() { 44 } else { 46 };
    let props = format!(
        "{{\"dimtxt\":{h},\"dimasz\":{a},\"dimexo\":{eo},\"dimexe\":{ee},\"dimgap\":{g},\"dimdec\":{d},\
         \"dimtad\":1,\"dimtih\":false,\"dimtoh\":false,\"dimscale\":1.0,\"dimlfac\":1.0,\"dimdsep\":{sep}}}",
        a = h * 0.8,
        eo = h * 0.4,
        ee = h * 0.6,
        g = h * 0.3,
        d = dec.saturating_sub(1),
    );
    let exists = host.document().dim_styles.get(DIM_STYLE).is_some();
    let op = if exists {
        TableOperation::DimStyleModify { name: DIM_STYLE.into(), properties: props }
    } else {
        TableOperation::DimStyleCreate { name: DIM_STYLE.into(), copy_from: None, properties: props }
    };
    host.table_operation(op).is_ok()
}

/// One aligned dimension through the host's own DIMALIGNED, so the CAD
/// builds it exactly as when drawn by hand. `false` when it was not made.
fn dim_aligned(host: &mut dyn HostApi, a: P2, b: P2, at: P2) -> bool {
    let line = format!("DIMALIGNED {:.6},{:.6} {:.6},{:.6} {:.6},{:.6}", a[0], a[1], b[0], b[1], at[0], at[1]);
    match host.run_command(CommandRequest::RunQuiet { line: line.clone() }) {
        Ok(o) => o.error.is_none() && o.added > 0,
        Err(_) => host.execute_command(&line),
    }
}

/// Dimension every measured side (outside its room); draw diagonals and
/// links as lines with their length, wall thicknesses as a short line from
/// the wall to the corner. Returns how many sides could not be made as
/// dimensions (written as text instead).
fn dimension(host: &mut dyn HostApi, survey: &Survey, pts: &[P2], measures: &[Measure], h: f64, dec: usize, z: f64) -> usize {
    let style_ok = ensure_dim_style(host, h, dec);
    let (old_style, old_layer) = (sysvar_text(host, "DIMSTYLE"), sysvar_text(host, "CLAYER"));
    if style_ok {
        let _ = host.table_operation(TableOperation::StyleSetCurrent { kind: TableStyleKind::Dim, name: DIM_STYLE.into() });
    }
    let _ = host.table_operation(TableOperation::LayerSetCurrent { name: LAYER_NOTES.into() });

    let mut failed = Vec::new();
    let mut marks = Vec::new();
    let mut k_line = 0usize;
    for m in measures {
        match (m.kind, m.meas) {
            (Kind::Side, Meas::Dist { i, j, .. }) => {
                let (a, b) = (pts[i], pts[j]);
                // Outward normal, in the turning sense of the side's room.
                let r = survey.rooms_of(i).into_iter().find(|r| survey.rooms_of(j).contains(r)).unwrap_or(0);
                let ccw = solve::signed_area(&survey.room_points(r, pts)) > 0.0;
                let ids = &survey.point_of[r];
                let n = ids.len();
                let forward = ids.iter().position(|&p| p == i).is_some_and(|x| ids[(x + 1) % n] == j);
                let (p, q) = if forward { (a, b) } else { (b, a) };
                let (dx, dy) = (q[0] - p[0], q[1] - p[1]);
                let l = dx.hypot(dy).max(1e-12);
                let out = if ccw { [dy / l, -dx / l] } else { [-dy / l, dx / l] };
                let mid = [(a[0] + b[0]) / 2.0, (a[1] + b[1]) / 2.0];
                let at = [mid[0] + out[0] * h * 2.5, mid[1] + out[1] * h * 2.5];
                if !dim_aligned(host, a, b, at) {
                    failed.push((a, b, at));
                }
            }
            (Kind::Angle | Kind::Level | Kind::Door, _) => {}
            (_, Meas::Dist { i, j, .. }) => {
                // Diagonals cross near the middle of a room, so the length
                // goes at 35 % or 65 % of the way, alternately.
                let (a, b) = (pts[i], pts[j]);
                let t = if k_line % 2 == 0 { 0.35 } else { 0.65 };
                k_line += 1;
                let at = [a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t];
                marks.push(pline(&[a, b], z, false, LAYER_NOTES));
                marks.push(along_text(&fmt_num(solve::dist(a, b), dec), a, b, at, h, z));
            }
            (_, Meas::Perp { p, a, b, .. }) => {
                let f = solve::foot(pts[p], pts[a], pts[b]);
                let mid = [(f[0] + pts[p][0]) / 2.0, (f[1] + pts[p][1]) / 2.0];
                marks.push(pline(&[f, pts[p]], z, false, LAYER_NOTES));
                let txt = format!("{} {}", tr!("sp.", "th."), fmt_num(solve::perp_dist(pts[p], pts[a], pts[b]), dec));
                marks.push(along_text(&txt, f, pts[p], mid, h * 0.8, z));
            }
            _ => {}
        }
    }
    if !marks.is_empty() {
        host.add_entities(marks);
    }

    if let Some(s) = old_style {
        let _ = host.table_operation(TableOperation::StyleSetCurrent { kind: TableStyleKind::Dim, name: s });
    }
    if let Some(l) = old_layer {
        let _ = host.table_operation(TableOperation::LayerSetCurrent { name: l });
    }

    let texts: Vec<EntityType> =
        failed.iter().map(|&(a, b, at)| along_text(&fmt_num(solve::dist(a, b), dec), a, b, at, h, z)).collect();
    let k = texts.len();
    if k > 0 {
        host.add_entities(texts);
    }
    k
}

fn op_kind_name(k: OpKind) -> &'static str {
    match k {
        OpKind::Door => tr!("porta", "door"),
        OpKind::Window => tr!("finestra", "window"),
        OpKind::Opening => tr!("apertura", "opening"),
    }
}

/// Doors, windows and openings on the adjusted rooms: jambs, the door leaf
/// with its swing, the window's double line, a label with name and width;
/// a door shared by two rooms gets the wall's jamb lines across. Also
/// returns warnings (an opening running past the end of its side).
fn openings_entities(survey: &Survey, openings: &[Opening], pts: &[P2], h: f64, dec: usize, z: f64) -> (Vec<EntityType>, Vec<String>) {
    let mut out = Vec::new();
    let mut warn = Vec::new();
    let jambs = |o: &Opening| {
        let (a, b) = (pts[o.a], pts[o.b]);
        let l = solve::dist(a, b).max(1e-12);
        let u = [(b[0] - a[0]) / l, (b[1] - a[1]) / l];
        let j = [a[0] + u[0] * o.s, a[1] + u[1] * o.s];
        let k = [a[0] + u[0] * (o.s + o.w), a[1] + u[1] * (o.s + o.w)];
        (j, k, u, l)
    };
    for (x, o) in openings.iter().enumerate() {
        let (j, k, u, l) = jambs(o);
        if o.s + o.w > l + 1e-9 {
            warn.push(tr!(
                format!("{} {} esce dal lato {}–{} (lungo {})", op_kind_name(o.kind), o.name, survey.names[o.a], survey.names[o.b], fmt_num(l, dec)),
                format!("{} {} runs past the end of side {}–{} ({} long)", op_kind_name(o.kind), o.name, survey.names[o.a], survey.names[o.b], fmt_num(l, dec))
            ));
        }
        // Inward normal: towards the inside of the opening's room.
        let room = survey.room_points(o.room, pts);
        let mid = [(j[0] + k[0]) / 2.0, (j[1] + k[1]) / 2.0];
        let mut n = [-u[1], u[0]];
        let probe = [mid[0] + n[0] * h * 0.05, mid[1] + n[1] * h * 0.05];
        if !point_in_polygon(&room, probe) {
            n = [-n[0], -n[1]];
        }
        let out_len = o.thick.unwrap_or(h);
        // Jambs: across the wall when the door is shared (drawn once),
        // otherwise a tick outwards.
        match o.twin {
            Some(t) if t > x => {
                let (j2, k2, _, _) = jambs(&openings[t]);
                let (pj, pk) = if solve::dist(j, k2) + solve::dist(k, j2) <= solve::dist(j, j2) + solve::dist(k, k2) { (k2, j2) } else { (j2, k2) };
                out.push(pline(&[j, pj], z, false, LAYER_OPENINGS));
                out.push(pline(&[k, pk], z, false, LAYER_OPENINGS));
            }
            Some(_) => {}
            None => {
                out.push(pline(&[j, [j[0] - n[0] * out_len, j[1] - n[1] * out_len]], z, false, LAYER_OPENINGS));
                out.push(pline(&[k, [k[0] - n[0] * out_len, k[1] - n[1] * out_len]], z, false, LAYER_OPENINGS));
            }
        }
        // A shared door is drawn (leaf, label) once, from its first room.
        if o.twin.is_some_and(|t| t < x) {
            continue;
        }
        match o.kind {
            OpKind::Door => {
                // Leaf hinged at the first jamb, open inwards, and its swing.
                let tip = [j[0] + n[0] * o.w, j[1] + n[1] * o.w];
                out.push(pline(&[j, tip], z, false, LAYER_OPENINGS));
                let ccw = n[0] * u[1] - n[1] * u[0] > 0.0;
                let mut arc = LwPolyline::new();
                let mut v0 = LwVertex::new(Vector2::new(tip[0], tip[1]));
                v0.bulge = if ccw { (std::f64::consts::PI / 8.0).tan() } else { -(std::f64::consts::PI / 8.0).tan() };
                arc.vertices = vec![v0, LwVertex::new(Vector2::new(k[0], k[1]))];
                arc.elevation = z;
                arc.common.layer = LAYER_OPENINGS.to_string();
                out.push(EntityType::LwPolyline(arc));
            }
            OpKind::Window => {
                let d = (o.thick.unwrap_or(h) * 0.35).min(h * 0.6);
                for sgn in [-1.0, 1.0] {
                    let off = [n[0] * d * sgn * 0.5 - n[0] * d * 0.5, n[1] * d * sgn * 0.5 - n[1] * d * 0.5];
                    out.push(pline(&[[j[0] + off[0], j[1] + off[1]], [k[0] + off[0], k[1] + off[1]]], z, false, LAYER_OPENINGS));
                }
            }
            OpKind::Opening => {}
        }
        // Name and width, inside the room along the side.
        let at = [mid[0] + n[0] * h * 0.6, mid[1] + n[1] * h * 0.6];
        let first = if o.kind == OpKind::Door { 1.0 } else { 0.0 };
        let at = [at[0] + n[0] * h * 0.6 * first, at[1] + n[1] * h * 0.6 * first];
        let mut t = along_text(&format!("{} {}", o.name, fmt_num(o.w, dec)), j, k, at, h * 0.75, z);
        t.common_mut().layer = LAYER_OPENINGS.to_string();
        out.push(t);
    }
    (out, warn)
}

/// A text sitting on the segment a–b at `at`, turned to read along it
/// (never upside down).
fn along_text(value: &str, a: P2, b: P2, at: P2, h: f64, z: f64) -> EntityType {
    let mut ang = (b[1] - a[1]).atan2(b[0] - a[0]);
    if ang > std::f64::consts::FRAC_PI_2 + 1e-9 {
        ang -= std::f64::consts::PI;
    } else if ang < -std::f64::consts::FRAC_PI_2 - 1e-9 {
        ang += std::f64::consts::PI;
    }
    let lift = [-ang.sin() * h * 0.3, ang.cos() * h * 0.3];
    let mut e = mtext(value, [at[0] + lift[0], at[1] + lift[1]], z, h, AttachmentPoint::BottomCenter, LAYER_NOTES);
    if let EntityType::MText(t) = &mut e {
        t.rotation = ang;
    }
    e
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numbers() {
        assert_eq!(parse_num(" 5,12 "), Some(5.12));
        assert_eq!(parse_num("4.03"), Some(4.03));
        assert_eq!(parse_num(""), None);
        assert_eq!(parse_num("abc"), None);
        assert_eq!(nice(0.17), 0.2);
        assert_eq!(nice(166.0), 200.0);
        assert_eq!(nice(0.021), 0.025);
        assert_eq!(decimals(0.02), 3);
        assert_eq!(decimals(20.0), 0);
        assert_eq!(decimals(0.005), 4);
    }

    #[test]
    fn sizes_follow_units() {
        let m: Vec<P2> = vec![[0.0, 0.0], [5.0, 0.0], [5.0, 4.0], [0.0, 4.0]];
        let (h, t) = default_sizes(&m);
        assert!((h - 0.2).abs() < 1e-12 && (t - 0.02).abs() < 1e-12, "{h} {t}");
        let mm: Vec<P2> = m.iter().map(|p| [p[0] * 1000.0, p[1] * 1000.0]).collect();
        let (h, t) = default_sizes(&mm);
        assert!((h - 200.0).abs() < 1e-9 && (t - 20.0).abs() < 1e-9, "{h} {t}");
    }

    #[test]
    fn centroid_of_rectangle() {
        let sq: Vec<P2> = vec![[0.0, 0.0], [4.0, 0.0], [4.0, 2.0], [0.0, 2.0]];
        let c = centroid(&sq);
        assert!((c[0] - 2.0).abs() < 1e-12 && (c[1] - 1.0).abs() < 1e-12);
    }

    #[test]
    fn book_lists_everything() {
        let survey = Survey::new(vec![vec![[0.0, 0.0], [4.0, 0.0], [4.0, 3.0], [0.0, 3.0]]]);
        let meas: Vec<Measure> = vec![
            Measure { meas: Meas::Dist { i: 0, j: 1, d: 4.0 }, name: "A–B".into(), kind: Kind::Side },
            Measure { meas: Meas::Dist { i: 1, j: 2, d: 3.0 }, name: "B–C".into(), kind: Kind::Side },
            Measure { meas: Meas::Dist { i: 2, j: 3, d: 4.0 }, name: "C–D".into(), kind: Kind::Side },
            Measure { meas: Meas::Dist { i: 3, j: 0, d: 3.0 }, name: "D–A".into(), kind: Kind::Side },
            Measure { meas: Meas::Dist { i: 0, j: 2, d: 5.0 }, name: "A–C".into(), kind: Kind::Diagonal },
        ];
        let m: Vec<Meas> = meas.iter().map(|x| x.meas).collect();
        let sol = solve::solve_network(&survey.points, &m, &survey.datum()).unwrap();
        let saved = Saved { sides: vec![], others: vec![], h: 0.2, tol: 0.02, dims: true, table: true, csv: false, csv_path: String::new(), constraints: vec![], openings: vec![], unit: 0, fill: true, replace: false };
        let csv = book_csv(&survey, &sol, &meas, &[], &[], &saved, 3, Some("C:\\x.dwg"), None);
        assert!(csv.starts_with('\u{feff}'));
        assert!(csv.contains("A–C"));
        assert!(csv.lines().any(|l| l.starts_with("1") && l.contains("12")), "area row: {csv}");
    }
}
