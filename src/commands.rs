//! Command dispatch: the glue between the host and the survey maths.
//!
//! `RV_VANO` picks a room and opens the measures dialog; the dialog hands
//! back `RV_VANOAPPLY`, which runs here with full `HostApi` access, solves
//! and draws the result as one undo step. When the measures are not enough
//! or do not fit together, the dialog opens again with what was typed and
//! the reason on top.

use std::collections::HashMap;
use std::sync::Mutex;

use ocs_plugin_api::host::codec;
use ocs_plugin_api::host::{CommandRequest, HostApi, HostSettingValue, LayerConfig, TableOperation, TableStyleKind};

use codec::entities::{AttachmentPoint, LwPolyline, LwVertex, MText};
use codec::{EntityType, Handle, Vector2, Vector3};

use crate::solve::{self, label, parse_label, Obs, SolveError, P2};
use crate::tools;

/// Layer of the adjusted rooms.
pub const LAYER_ROOM: &str = "RILIEVO_CORRETTO";
/// Layer of letters, dimensions and tables of the adjusted rooms.
pub const LAYER_NOTES: &str = "RILIEVO_QUOTE";
/// Layer of the working letters written on the rooms drawn by eye.
pub const LAYER_LABELS: &str = "RILIEVO_ETICHETTE";
/// Dimension style the plugin keeps for its dimensions.
pub const DIM_STYLE: &str = "RILIEVO";

// ── Room ────────────────────────────────────────────────────────────────────

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
    // Drop repeated vertices.
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

/// Every closed polyline in the drawing (model space, XY plane).
pub fn rooms(host: &dyn HostApi) -> Vec<Room> {
    host.document().entities().filter_map(room_of).collect()
}

/// Live read of one room by handle.
fn fetch_room(host: &dyn HostApi, h: Handle) -> Option<Room> {
    let live = host.entities_batch(&[h]).ok().and_then(|v| v.into_iter().next().flatten());
    match live {
        Some(e) => room_of(&e),
        None => host.document().get_entity(h).and_then(room_of),
    }
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

// ── What the user typed, per room ───────────────────────────────────────────

/// The dialog's content: kept per room for the session, so a failed
/// calculation (or a later re-run) opens the dialog with the same values.
#[derive(Clone, Debug)]
pub struct Saved {
    /// Rows `[side, drawn, measured]`, one per side in vertex order.
    pub sides: Vec<Vec<String>>,
    /// Rows `[from, to, drawn, measured]`.
    pub diags: Vec<Vec<String>>,
    pub h: f64,
    pub tol: f64,
    pub dims: bool,
    pub table: bool,
}

static SAVED: Mutex<Option<HashMap<u64, Saved>>> = Mutex::new(None);
static PENDING: Mutex<Option<(Handle, Saved)>> = Mutex::new(None);

pub fn saved_for(h: Handle) -> Option<Saved> {
    SAVED.lock().unwrap_or_else(|e| e.into_inner()).as_ref().and_then(|m| m.get(&h.value()).cloned())
}

fn remember(h: Handle, s: Saved) {
    SAVED.lock().unwrap_or_else(|e| e.into_inner()).get_or_insert_with(HashMap::new).insert(h.value(), s);
}

pub fn set_pending(h: Handle, s: Saved) {
    *PENDING.lock().unwrap_or_else(|e| e.into_inner()) = Some((h, s));
}

fn take_pending() -> Option<(Handle, Saved)> {
    PENDING.lock().unwrap_or_else(|e| e.into_inner()).take()
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
    let s = format!("{v:.dec$}");
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

/// Text height and residual tolerance suited to the room's size: a 5 m
/// room drawn in metres gets 0.2 and 0.02; the same room in millimetres
/// gets 200 and 20.
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
        "RV_VANO" => start_pick(host, "RV_VANOOPEN", "RV_VANO"),
        "RV_ETICHETTE" => start_pick(host, "RV_ETICHETTEAPPLY", "RV_ETICHETTE"),
        "RV_VANOOPEN" => {
            if let Some(h) = arg.as_deref().and_then(parse_handle) {
                open_dialog(host, h, None);
            }
        }
        "RV_ETICHETTEAPPLY" => {
            if let Some(h) = arg.as_deref().and_then(parse_handle) {
                match fetch_room(host, h) {
                    Some(room) => {
                        let (hh, _) = default_sizes(&room.pts);
                        let h = saved_for(room.handle).map_or(hh, |s| s.h);
                        host.begin_undo_group("RV_ETICHETTE");
                        let n = write_labels(host, &room, h);
                        host.end_undo_group();
                        host.set_dirty();
                        host.push_info(&tr!(
                            format!("RV_ETICHETTE: {n} lettere scritte sul layer {LAYER_LABELS}."),
                            format!("RV_ETICHETTE: {n} letters written on layer {LAYER_LABELS}.")
                        ));
                    }
                    None => host.push_error(tr!("RV_ETICHETTE: il vano non è più nel disegno.", "RV_ETICHETTE: the room is no longer in the drawing.")),
                }
            }
        }
        "RV_VANOAPPLY" => {
            if let Some((h, saved)) = take_pending() {
                apply(host, h, saved);
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

fn help(host: &mut dyn HostApi) {
    host.push_output(&tr!(
        format!(
            "Rilievo Vani {}: RV_VANO (misure di lati e diagonali di un vano), RV_ETICHETTE (lettere sui vertici), \
             RV_GUIDE (guida), RV_LANG IT|EN|AUTO (lingua).",
            env!("CARGO_PKG_VERSION")
        ),
        format!(
            "Rilievo Vani {}: RV_VANO (sides and diagonals of a room), RV_ETICHETTE (corner letters), \
             RV_GUIDE (guide), RV_LANG IT|EN|AUTO (language).",
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

/// Pick a room, or take the one selected.
fn start_pick(host: &mut dyn HostApi, verb: &'static str, title: &'static str) {
    let all = rooms(host);
    if all.is_empty() {
        host.push_error(&tr!(
            format!("{title}: nel disegno non ci sono polilinee chiuse. Disegna il vano con PLINEA … Chiudi."),
            format!("{title}: the drawing has no closed polylines. Draw the room with PLINE … Close.")
        ));
        return;
    }
    let sel = host.selection();
    if sel.len() == 1 {
        if let Some(r) = all.iter().find(|r| r.handle == sel[0]) {
            let h = r.handle;
            let _ = host.set_selection(&[]);
            if verb == "RV_VANOOPEN" {
                open_dialog(host, h, None);
            } else {
                handle(host, &format!("{verb} {}", hex(h)));
            }
            return;
        }
    }
    host.start_interactive(Box::new(tools::PickRoomTool::new(all, verb, title)));
}

fn open_dialog(host: &mut dyn HostApi, h: Handle, msg: Option<String>) {
    let Some(room) = fetch_room(host, h) else {
        host.push_error(tr!("RV_VANO: il vano non è più nel disegno.", "RV_VANO: the room is no longer in the drawing."));
        return;
    };
    let saved = match saved_for(h) {
        Some(s) if s.sides.len() == room.pts.len() => s,
        _ => tools::initial_saved(&room),
    };
    if msg.is_none() {
        if room.had_arcs {
            host.push_info(tr!(
                "RV_VANO: i tratti ad arco della polilinea sono presi come corde (lati diritti).",
                "RV_VANO: the polyline's arc segments are taken as chords (straight sides)."
            ));
        }
        host.begin_undo_group("RV_ETICHETTE");
        write_labels(host, &room, saved.h);
        host.end_undo_group();
        host.set_dirty();
    }
    host.start_interactive(Box::new(tools::MeasureDialog::new(&room, &saved, msg)));
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

/// Where the letter of each corner goes: inside the room, on the bisector.
fn label_positions(pts: &[P2], h: f64) -> Vec<P2> {
    let n = pts.len();
    let ccw = solve::signed_area(pts) > 0.0;
    (0..n)
        .map(|i| {
            let (a, b, c) = (pts[(i + n - 1) % n], pts[i], pts[(i + 1) % n]);
            // Inward normals of the two sides meeting at b.
            let inward = |p: P2, q: P2| {
                let (dx, dy) = (q[0] - p[0], q[1] - p[1]);
                let l = dx.hypot(dy).max(1e-12);
                if ccw { [-dy / l, dx / l] } else { [dy / l, -dx / l] }
            };
            let (n1, n2) = (inward(a, b), inward(b, c));
            let mut d = [n1[0] + n2[0], n1[1] + n2[1]];
            let l = d[0].hypot(d[1]);
            if l < 1e-9 {
                d = n1;
            } else {
                d = [d[0] / l, d[1] / l];
            }
            [b[0] + d[0] * h * 1.3, b[1] + d[1] * h * 1.3]
        })
        .collect()
}

/// MText letters already on the labels layer: (handle, value, position).
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

/// Write the working letters on the room drawn by eye (skipping those
/// already there). Returns how many corners are labelled.
fn write_labels(host: &mut dyn HostApi, room: &Room, h: f64) -> usize {
    ensure_layer(host, LAYER_LABELS, 6);
    let have = existing_letters(host);
    let mut new = Vec::new();
    for (i, at) in label_positions(&room.pts, h).into_iter().enumerate() {
        let name = label(i);
        if have.iter().any(|(_, v, p)| *v == name && solve::dist(*p, at) < h * 0.6) {
            continue;
        }
        new.push(mtext(&name, at, room.z, h, AttachmentPoint::MiddleCenter, LAYER_LABELS));
    }
    if !new.is_empty() {
        host.add_entities(new);
    }
    room.pts.len()
}

/// Remove the working letters of a room (they move to the adjusted room).
fn remove_labels(host: &mut dyn HostApi, room: &Room) {
    let have = existing_letters(host);
    let n = room.pts.len();
    let mut gone = Vec::new();
    for i in 0..n {
        let name = label(i);
        // The nearest letter with this name, close to its corner.
        let (h, _) = default_sizes(&room.pts);
        let best = have
            .iter()
            .filter(|(hd, v, _)| *v == name && !gone.contains(hd))
            .map(|(hd, _, p)| (*hd, solve::dist(*p, room.pts[i])))
            .filter(|(_, d)| *d < h * 4.0)
            .min_by(|a, b| a.1.total_cmp(&b.1));
        if let Some((hd, _)) = best {
            gone.push(hd);
        }
    }
    for hd in gone {
        host.remove_entity(hd);
    }
}

// ── Apply ───────────────────────────────────────────────────────────────────

/// One measure as typed, with its row name for messages.
struct Measure {
    obs: Obs,
    name: String,
}

fn parse_measures(saved: &Saved, n: usize) -> Result<Vec<Measure>, String> {
    let mut out = Vec::new();
    for (i, row) in saved.sides.iter().enumerate().take(n) {
        let j = (i + 1) % n;
        let name = format!("{}–{}", label(i), label(j));
        let cell = row.get(2).map(String::as_str).unwrap_or("");
        if cell.trim().is_empty() {
            continue;
        }
        match parse_num(cell) {
            Some(d) if d > 0.0 => out.push(Measure { obs: Obs { i, j, d }, name }),
            _ => return Err(tr!(format!("lato {name}: \"{cell}\" non è una misura valida"), format!("side {name}: \"{cell}\" is not a valid measure"))),
        }
    }
    for (k, row) in saved.diags.iter().enumerate() {
        let cell = |c: usize| row.get(c).map(|s| s.trim().to_string()).unwrap_or_default();
        let (a, b, m) = (cell(0), cell(1), cell(3));
        if m.is_empty() {
            continue; // a proposed diagonal not measured
        }
        let r = k + 1;
        let (Some(i), Some(j)) = (parse_label(&a), parse_label(&b)) else {
            return Err(tr!(
                format!("diagonale alla riga {r}: indica i due vertici con le loro lettere (A … {})", label(n - 1)),
                format!("diagonal on row {r}: name its two corners by letter (A … {})", label(n - 1))
            ));
        };
        if i >= n || j >= n || i == j {
            return Err(tr!(
                format!("diagonale alla riga {r}: {a}–{b} non unisce due vertici diversi del vano (A … {})", label(n - 1)),
                format!("diagonal on row {r}: {a}–{b} does not join two different corners (A … {})", label(n - 1))
            ));
        }
        let name = format!("{}–{}", label(i.min(j)), label(i.max(j)));
        match parse_num(&m) {
            Some(d) if d > 0.0 => out.push(Measure { obs: Obs { i, j, d }, name }),
            _ => return Err(tr!(format!("diagonale {name}: \"{m}\" non è una misura valida"), format!("diagonal {name}: \"{m}\" is not a valid measure"))),
        }
    }
    Ok(out)
}

fn explain(e: &SolveError, n: usize, measures: &[Measure], room: &Room) -> String {
    match e {
        SolveError::TooFewPoints => tr!("il vano ha meno di 3 vertici".to_string(), "the room has fewer than 3 corners".to_string()),
        SolveError::BadObs(k) => {
            let name = measures.get(*k).map(|m| m.name.clone()).unwrap_or_default();
            tr!(format!("misura {name} non valida"), format!("measure {name} is not valid"))
        }
        SolveError::NotEnough { needed, have } => {
            let missing = needed - have;
            let sides_missing = (0..n)
                .filter(|&i| !measures.iter().any(|m| (m.obs.i, m.obs.j) == (i, (i + 1) % n)))
                .map(|i| format!("{}–{}", label(i), label((i + 1) % n)))
                .collect::<Vec<_>>();
            let mut s = tr!(
                format!("servono almeno {needed} misure, ne hai inserite {have}: ne mancano {missing}"),
                format!("at least {needed} measures are needed, {have} typed: {missing} missing")
            );
            if !sides_missing.is_empty() {
                s += &tr!(format!(" (lati senza misura: {})", sides_missing.join(", ")), format!(" (sides with no measure: {})", sides_missing.join(", ")));
            }
            s
        }
        SolveError::NotRigid => {
            let proposed: Vec<String> = solve::suggested_diagonals(&room.pts)
                .into_iter()
                .filter(|&(i, j)| !measures.iter().any(|m| (m.obs.i.min(m.obs.j), m.obs.i.max(m.obs.j)) == (i, j)))
                .map(|(i, j)| format!("{}–{}", label(i), label(j)))
                .collect();
            let mut s = tr!(
                "le diagonali non bloccano tutta la forma: una parte del vano può ancora deformarsi".to_string(),
                "the diagonals do not brace the whole shape: part of the room can still move".to_string()
            );
            if !proposed.is_empty() {
                s += &tr!(format!(". Misura anche: {}", proposed.join(", ")), format!(". Also measure: {}", proposed.join(", ")));
            }
            s
        }
        SolveError::NoConvergence => tr!(
            "le misure non stanno insieme (es. una diagonale più lunga di due lati sommati, o due vertici scambiati): ricontrollale".to_string(),
            "the measures do not fit together (e.g. a diagonal longer than two sides together, or two corners swapped): check them".to_string()
        ),
    }
}

fn apply(host: &mut dyn HostApi, h: Handle, saved: Saved) {
    remember(h, saved.clone());
    let Some(room) = fetch_room(host, h) else {
        host.push_error(tr!("RV_VANO: il vano non è più nel disegno.", "RV_VANO: the room is no longer in the drawing."));
        return;
    };
    let n = room.pts.len();
    let measures = match parse_measures(&saved, n) {
        Ok(m) => m,
        Err(msg) => {
            host.push_error(&format!("RV_VANO: {msg}."));
            open_dialog(host, h, Some(msg));
            return;
        }
    };
    let obs: Vec<Obs> = measures.iter().map(|m| m.obs).collect();
    let sol = match solve::solve(&room.pts, &obs) {
        Ok(s) => s,
        Err(e) => {
            let msg = explain(&e, n, &measures, &room);
            host.push_error(&format!("RV_VANO: {msg}."));
            open_dialog(host, h, Some(msg));
            return;
        }
    };

    let th = saved.h.max(1e-9);
    let dec = decimals(saved.tol);
    let z = room.z;

    host.begin_undo_group("RV_VANO");
    ensure_layer(host, LAYER_ROOM, 3);
    ensure_layer(host, LAYER_NOTES, 2);
    remove_labels(host, &room);

    let mut ents = vec![pline(&sol.pts, z, true, LAYER_ROOM)];
    for (i, at) in label_positions(&sol.pts, th).into_iter().enumerate() {
        ents.push(mtext(&label(i), at, z, th * 1.2, AttachmentPoint::MiddleCenter, LAYER_NOTES));
    }
    let bad: Vec<usize> = (0..measures.len())
        .filter(|&k| saved.tol > 0.0 && sol.residuals[k].abs() > saved.tol)
        .collect();
    if saved.table {
        ents.extend(table(&sol, &measures, &bad, th, dec, z));
    }
    host.add_entities(ents);

    let mut dims_failed = 0;
    if saved.dims {
        dims_failed = dimension(host, &sol.pts, &measures, th, dec, z);
    }
    host.end_undo_group();
    host.set_dirty();

    // Report on the command line.
    let mut lines = vec![tr!(
        format!("RV_VANO: vano di {n} vertici ricalcolato con {} misure → layer {LAYER_ROOM}.", measures.len()),
        format!("RV_VANO: room of {n} corners recomputed from {} measures → layer {LAYER_ROOM}.", measures.len())
    )];
    match sol.sigma0 {
        Some(s0) => lines.push(tr!(
            format!("  Misure in più: {} · scarto quadratico medio σ₀ = {}", sol.redundancy, fmt_num(s0, dec)),
            format!("  Extra measures: {} · standard deviation σ₀ = {}", sol.redundancy, fmt_num(s0, dec))
        )),
        None => lines.push(tr!(
            "  Nessuna misura di controllo: la forma è esatta ma un errore di misura non si vede. Misura una diagonale in più.".to_string(),
            "  No check measure: the shape is exact but a wrong measure cannot show. Measure one more diagonal.".to_string()
        )),
    }
    if sol.redundancy > 0 {
        for (k, m) in measures.iter().enumerate() {
            let flag = if bad.contains(&k) { "  !" } else { "" };
            lines.push(format!("  {:<7} {} → {}  ({}){flag}", m.name, fmt_num(m.obs.d, dec), fmt_num(m.obs.d + sol.residuals[k], dec), fmt_signed(sol.residuals[k], dec)));
        }
    }
    for l in &lines {
        host.push_output(l);
    }
    if !bad.is_empty() {
        let names: Vec<&str> = bad.iter().map(|&k| measures[k].name.as_str()).collect();
        host.push_error(&tr!(
            format!("RV_VANO: scarti oltre la tolleranza ({}) su {}: ricontrolla queste misure.", fmt_num(saved.tol, dec), names.join(", ")),
            format!("RV_VANO: residuals over the tolerance ({}) on {}: check these measures.", fmt_num(saved.tol, dec), names.join(", "))
        ));
    }
    if dims_failed > 0 {
        host.push_error(&tr!(
            format!("RV_VANO: {dims_failed} quote non create dal CAD; le misure sono scritte come testo."),
            format!("RV_VANO: the CAD did not create {dims_failed} dimensions; the measures are written as text.")
        ));
    }
}

/// The table of measures, right of the room: rows of measure, measured,
/// computed and residual; residuals over the tolerance in red.
fn table(sol: &solve::Solution, measures: &[Measure], bad: &[usize], h: f64, dec: usize, z: f64) -> Vec<EntityType> {
    let maxx = sol.pts.iter().map(|p| p[0]).fold(f64::MIN, f64::max);
    let maxy = sol.pts.iter().map(|p| p[1]).fold(f64::MIN, f64::max);
    let x0 = maxx + h * 5.0;
    let row_h = h * 1.9;
    let col_w = [h * 5.0, h * 6.5, h * 6.5, h * 6.0];
    let width: f64 = col_w.iter().sum();
    let mut out = Vec::new();
    let mut y = maxy;

    out.push(mtext(
        tr!("RILIEVO VANO — misure", "ROOM SURVEY — measures"),
        [x0, y + row_h * 0.6],
        z,
        h,
        AttachmentPoint::MiddleLeft,
        LAYER_NOTES,
    ));
    let headers = [
        tr!("Misura", "Measure"),
        tr!("Misurata", "Measured"),
        tr!("Calcolata", "Computed"),
        tr!("Scarto", "Residual"),
    ];
    let mut rows: Vec<[String; 4]> = vec![headers.map(str::to_string)];
    for (k, m) in measures.iter().enumerate() {
        let v = sol.residuals[k];
        rows.push([
            m.name.clone(),
            fmt_num(m.obs.d, dec),
            fmt_num(m.obs.d + v, dec),
            if sol.redundancy > 0 { fmt_signed(v, dec) } else { "—".into() },
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
    // Frame: outline, header line, column lines.
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
    let line = format!(
        "DIMALIGNED {:.6},{:.6} {:.6},{:.6} {:.6},{:.6}",
        a[0], a[1], b[0], b[1], at[0], at[1]
    );
    match host.run_command(CommandRequest::RunQuiet { line: line.clone() }) {
        Ok(o) => o.error.is_none() && o.added > 0,
        Err(_) => host.execute_command(&line),
    }
}

/// Dimension every measured side (outside the room) and diagonal (along
/// it). Returns how many could not be made as dimensions (written as text).
fn dimension(host: &mut dyn HostApi, pts: &[P2], measures: &[Measure], h: f64, dec: usize, z: f64) -> usize {
    let n = pts.len();
    let ccw = solve::signed_area(pts) > 0.0;
    let style_ok = ensure_dim_style(host, h, dec);
    let (old_style, old_layer) = (sysvar_text(host, "DIMSTYLE"), sysvar_text(host, "CLAYER"));
    if style_ok {
        let _ = host.table_operation(TableOperation::StyleSetCurrent { kind: TableStyleKind::Dim, name: DIM_STYLE.into() });
    }
    let _ = host.table_operation(TableOperation::LayerSetCurrent { name: LAYER_NOTES.into() });

    let mut failed = Vec::new();
    let mut diag_marks = Vec::new();
    let mut k_diag = 0usize;
    for m in measures {
        let (i, j) = (m.obs.i, m.obs.j);
        let (a, b) = (pts[i], pts[j]);
        let mid = [(a[0] + b[0]) / 2.0, (a[1] + b[1]) / 2.0];
        let is_side = j == (i + 1) % n || i == (j + 1) % n;
        if !is_side {
            // Diagonals cross each other near the middle of the room, so a
            // dimension text at mid-length would sit on top of the next
            // one: draw the diagonal as a line with its length at 35 % or
            // 65 % of the way, alternately.
            let t = if k_diag % 2 == 0 { 0.35 } else { 0.65 };
            k_diag += 1;
            let at = [a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t];
            diag_marks.push(pline(&[a, b], z, false, LAYER_NOTES));
            diag_marks.push(along_text(&fmt_num(solve::dist(a, b), dec), a, b, at, h, z));
            continue;
        }
        // Outward normal of the side, in the room's turning sense.
        let (p, q) = if j == (i + 1) % n { (a, b) } else { (b, a) };
        let (dx, dy) = (q[0] - p[0], q[1] - p[1]);
        let l = dx.hypot(dy).max(1e-12);
        let out = if ccw { [dy / l, -dx / l] } else { [-dy / l, dx / l] };
        let at = [mid[0] + out[0] * h * 2.5, mid[1] + out[1] * h * 2.5];
        if !dim_aligned(host, a, b, at) {
            failed.push((a, b, at));
        }
    }
    if !diag_marks.is_empty() {
        host.add_entities(diag_marks);
    }

    if let Some(s) = old_style {
        let _ = host.table_operation(TableOperation::StyleSetCurrent { kind: TableStyleKind::Dim, name: s });
    }
    if let Some(l) = old_layer {
        let _ = host.table_operation(TableOperation::LayerSetCurrent { name: l });
    }

    // Fallback: the length as text along the side.
    let texts: Vec<EntityType> =
        failed.iter().map(|&(a, b, at)| along_text(&fmt_num(solve::dist(a, b), dec), a, b, at, h, z)).collect();
    let k = texts.len();
    if k > 0 {
        host.add_entities(texts);
    }
    k
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
    // Lift the text a little off the line, on its upper side.
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
    fn labels_sit_inside() {
        let l: Vec<P2> = vec![[0.0, 0.0], [6.0, 0.0], [6.0, 2.0], [3.0, 2.0], [3.0, 5.0], [0.0, 5.0]];
        for pts in [l.clone(), l.iter().rev().cloned().collect::<Vec<_>>()] {
            for p in label_positions(&pts, 0.2) {
                assert!(point_in_polygon(&pts, p), "{p:?}");
            }
        }
    }

    #[test]
    fn measures_are_read_from_the_tables() {
        let s = Saved {
            sides: vec![
                vec!["A–B".into(), "5".into(), "5,12".into()],
                vec!["B–C".into(), "4".into(), "4.03".into()],
                vec!["C–D".into(), "5".into(), "5,27".into()],
                vec!["D–A".into(), "4".into(), "3,98".into()],
            ],
            diags: vec![
                vec!["a".into(), "c".into(), "".into(), "6,5".into()],
                vec!["B".into(), "D".into(), "".into(), "".into()],
            ],
            h: 0.2,
            tol: 0.02,
            dims: true,
            table: true,
        };
        let m = parse_measures(&s, 4).unwrap();
        assert_eq!(m.len(), 5);
        assert_eq!(m[4].obs, Obs { i: 0, j: 2, d: 6.5 });
        assert_eq!(m[4].name, "A–C");
        let mut bad = s.clone();
        bad.diags[0][1] = "Q".into();
        assert!(parse_measures(&bad, 4).is_err());
        let mut bad = s;
        bad.sides[1][2] = "4,0x".into();
        assert!(parse_measures(&bad, 4).is_err());
    }
}
