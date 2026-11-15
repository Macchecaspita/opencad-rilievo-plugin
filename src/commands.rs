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

/// Every hatch of the drawing, as its boundary loops (arcs as short chords).
/// A hatch between two rooms marks a solid wall.
pub fn hatches(host: &dyn HostApi) -> Vec<Vec<Vec<P2>>> {
    use codec::entities::BoundaryEdge;
    let mut out = Vec::new();
    for e in host.document().entities() {
        let EntityType::Hatch(h) = e else { continue };
        let mut loops = Vec::new();
        for path in &h.paths {
            let mut pts: Vec<P2> = Vec::new();
            for edge in &path.edges {
                match edge {
                    BoundaryEdge::Line(l) => {
                        pts.push([l.start.x, l.start.y]);
                        pts.push([l.end.x, l.end.y]);
                    }
                    BoundaryEdge::Polyline(pl) => pts.extend(pl.vertices.iter().map(|v| [v.x, v.y])),
                    BoundaryEdge::CircularArc(a) => {
                        let (mut s0, mut s1) = (a.start_angle, a.end_angle);
                        if !a.counter_clockwise {
                            std::mem::swap(&mut s0, &mut s1);
                        }
                        if s1 < s0 {
                            s1 += std::f64::consts::TAU;
                        }
                        let mut arc: Vec<P2> = (0..=8).map(|k| s0 + (s1 - s0) * k as f64 / 8.0).map(|t| [a.center.x + a.radius * t.cos(), a.center.y + a.radius * t.sin()]).collect();
                        if !a.counter_clockwise {
                            arc.reverse();
                        }
                        pts.extend(arc);
                    }
                    BoundaryEdge::Spline(sp) => pts.extend(sp.fit_points.iter().map(|v| [v.x, v.y])),
                    _ => {}
                }
            }
            pts.dedup_by(|a, b| solve::dist(*a, *b) < 1e-9);
            if pts.len() >= 3 {
                loops.push(pts);
            }
        }
        if !loops.is_empty() {
            out.push(loops);
        }
    }
    out
}

/// Reach within which two facing sides with a hatch between them make a
/// solid wall: 1,20 m in drawing units.
fn inner_reach(unit: usize) -> f64 {
    1.2 / survey::unit_factor(unit)
}

/// Weight of an outer-wall measure: the outline is surveyed exactly and the
/// thickness is given, so the room keeps it.
const OUTER_WEIGHT: f64 = 30.0;

/// Add the fixed outer outline named in `saved`, read live from the drawing.
/// When it is gone, the survey goes on without it and the message says so.
fn add_outer(host: &dyn HostApi, survey: Survey, saved: &mut Saved) -> (Survey, Option<String>) {
    if saved.outer.trim().is_empty() {
        return (survey, None);
    }
    match parse_handle(&saved.outer).and_then(|h| fetch_room(host, h)) {
        Some(o) => (survey.with_outer(o.pts), None),
        None => {
            saved.outer.clear();
            (
                survey,
                Some(tr!(
                    "il poligono esterno non è più nel disegno: sceglilo di nuovo",
                    "the outer outline is no longer in the drawing: choose it again",
                    "el polígono exterior ya no está en el dibujo: elíjalo de nuevo",
                    "le polygone extérieur n'est plus dans le dessin : choisissez-le de nouveau",
                    "dış poligon artık çizimde yok: yeniden seçin",
                    "o polígono exterior já não está no desenho: escolha-o de novo",
                    "obrysu zewnętrznego nie ma już na rysunku: wybierz go ponownie",
                )
                .to_string()),
            )
        }
    }
}

/// Weights for the measures: outer walls held firmly.
fn weights_of(measures: &[Measure]) -> Vec<f64> {
    measures.iter().map(|m| if m.kind == Kind::Outer { OUTER_WEIGHT } else { 1.0 }).collect()
}

/// Reach within which a room side counts as running along the outline: four
/// times a usual outer wall (30 cm), or 5 % of the outline's longest side.
fn outer_reach(survey: &Survey, unit: usize) -> f64 {
    let o = survey.outline();
    let longest = (0..o.len()).map(|i| solve::dist(o[i], o[(i + 1) % o.len()])).fold(0.0f64, f64::max);
    (4.0 * 0.30 / survey::unit_factor(unit)).max(longest * 0.05)
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
/// Rooms for which the fixed perimeter has already been asked.
static ASKED_OUTER: Mutex<std::collections::BTreeSet<String>> = Mutex::new(std::collections::BTreeSet::new());
/// The letter height last chosen in Corner letters, used by the next
/// calculations too.
static LETTER_H: Mutex<Option<f64>> = Mutex::new(None);

pub fn letter_height() -> Option<f64> {
    *LETTER_H.lock().unwrap_or_else(|e| e.into_inner())
}
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

pub fn remember(hs: &[Handle], s: Saved) {
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
    if crate::i18n::decimal_comma() { s.replace('.', ",") } else { s }
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

/// Text height and tolerance for a set of rooms. The height follows the
/// typical side, not the longest: a plan full of short sides (recesses,
/// jambs, pillars) gets small letters that fit between its corners, a plain
/// room the same as [`default_sizes`]. Kept between 1/150 and 1/30 of the
/// longest side.
pub fn sizes_for(rooms: &[Vec<P2>]) -> (f64, f64) {
    let mut sides: Vec<f64> = rooms
        .iter()
        .flat_map(|p| (0..p.len()).map(move |i| solve::dist(p[i], p[(i + 1) % p.len()])))
        .filter(|d| *d > 0.0)
        .collect();
    if sides.is_empty() {
        return (1.0, 0.01);
    }
    sides.sort_by(f64::total_cmp);
    let longest = sides[sides.len() - 1];
    let median = sides[sides.len() / 2];
    let h = (median * 0.25).clamp(longest / 150.0, longest / 30.0);
    (nice(h), nice(longest / 250.0))
}

/// Corner letters are never taller than this share of the shortest side of
/// the smallest room.
pub const LETTER_CAP: f64 = 0.15;

/// Largest corner-letter height for these rooms: 15 % of the shortest side
/// of the smallest room (by area). Sides shorter than 5 % of their room's
/// longest side are left out (a double click, a tiny jog in the outline):
/// they would make the letters too small to see. Never below 1/400 of the
/// longest side of all, so the letters stay visible. Infinite when there is
/// nothing to measure.
pub fn letter_cap(rooms: &[Vec<P2>]) -> f64 {
    let sides = |p: &Vec<P2>| (0..p.len()).map(|i| solve::dist(p[i], p[(i + 1) % p.len()])).collect::<Vec<f64>>();
    let overall = rooms.iter().flat_map(sides).fold(0.0f64, f64::max);
    rooms
        .iter()
        .filter(|p| p.len() >= 3)
        .map(|p| (solve::signed_area(p).abs(), p))
        .filter(|(a, _)| *a > 0.0)
        .min_by(|a, b| a.0.total_cmp(&b.0))
        .and_then(|(_, p)| {
            let s = sides(p);
            let longest = s.iter().copied().fold(0.0f64, f64::max);
            s.into_iter().filter(|d| *d >= longest * 0.05).min_by(f64::total_cmp)
        })
        .map_or(f64::INFINITY, |d| (d * LETTER_CAP).max(overall / 400.0))
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
            // The rooms were chosen with the CAD's selection: clear it.
            let _ = host.set_selection(&[]);
            if hs.is_empty() {
                return true;
            }
            // First time for these rooms: the fixed perimeter of the building,
            // surveyed with a total station, may be chosen now (optional).
            let asked = saved_for(&hs).is_some_and(|s| !s.outer.is_empty()) || !ASKED_OUTER.lock().unwrap_or_else(|e| e.into_inner()).insert(key(&hs));
            if asked {
                open_dialog(host, &hs, None);
            } else {
                host.start_interactive(Box::new(tools::PickOuterTool::first(key(&hs))));
            }
        }
        "RV_VANOFIRST" => {
            let hs = arg.as_deref().map(parse_handles).unwrap_or_default();
            if !hs.is_empty() {
                open_dialog(host, &hs, None);
            }
        }
        "RV_VANOREOPEN" => {
            let hs = arg.as_deref().map(parse_handles).unwrap_or_default();
            if !hs.is_empty() {
                open_dialog_at(host, &hs, None, tools::Start::Hub, false);
            }
        }
        "RV_PORTAPICK" => {
            let hs = arg.as_deref().map(parse_handles).unwrap_or_default();
            match fetch_rooms(host, &hs) {
                Some(rooms) if !rooms.is_empty() => {
                    let z = rooms[0].z;
                    let pts = rooms.into_iter().map(|r| r.pts).collect();
                    host.start_interactive(Box::new(tools::PickDoorTool::new(pts, z, key(&hs))));
                }
                _ => host.push_error(tr!("RV_VANO: un vano scelto non è più nel disegno.", "RV_VANO: a chosen room is no longer in the drawing.", "RV_VANO: una estancia elegida ya no está en el dibujo.", "RV_VANO : une pièce choisie n'est plus dans le dessin.", "RV_VANO: seçilen bir oda artık çizimde yok.", "RV_VANO: um compartimento escolhido já não está no desenho.", "RV_VANO: wybranego pomieszczenia nie ma już na rysunku.")),
            }
        }
        "RV_ESTERNOPICK" => {
            let hs = arg.as_deref().map(parse_handles).unwrap_or_default();
            if !hs.is_empty() {
                host.start_interactive(Box::new(tools::PickOuterTool::new(key(&hs))));
            }
        }
        "RV_ESTERNOSET" => {
            let a = arg.unwrap_or_default();
            let mut parts = a.splitn(2, '|');
            let hs = parse_handles(parts.next().unwrap_or(""));
            let rest2 = parts.next().unwrap_or("");
            let mut more = rest2.splitn(2, '|');
            let picked = more.next().and_then(parse_handle);
            // Chosen right after the rooms: the corner letters are still to write.
            let first = more.next() == Some("first");
            if hs.is_empty() {
                return true;
            }
            let msg = match picked {
                Some(h) if hs.contains(&h) => Some(tr!(
                    "quella polilinea è uno dei vani: scegli il poligono esterno, rilevato a parte",
                    "that polyline is one of the rooms: choose the outer outline, surveyed apart",
                    "esa polilínea es una de las estancias: elija el polígono exterior, levantado aparte",
                    "cette polyligne est une des pièces : choisissez le polygone extérieur, relevé à part",
                    "bu çoklu çizgi odalardan biri: ayrıca ölçülmüş dış poligonu seçin",
                    "essa polilinha é um dos compartimentos: escolha o polígono exterior, levantado à parte",
                    "ta polilinia to jedno z pomieszczeń: wybierz obrys zewnętrzny, zmierzony osobno",
                )),
                Some(h) if fetch_room(host, h).is_some() => {
                    if let Some(rooms) = fetch_rooms(host, &hs) {
                        let mut s = saved_or_new(host, &hs, &rooms, &Survey::new(rooms.iter().map(|r| r.pts.clone()).collect()));
                        s.outer = hex(h);
                        let survey = Survey::new(rooms.iter().map(|r| r.pts.clone()).collect()).with_outer(fetch_room(host, h).map(|o| o.pts).unwrap_or_default());
                        s.outer_walls = survey.suggest_outer_walls(outer_reach(&survey, s.unit));
                        remember(&hs, s);
                    }
                    None
                }
                _ => Some(tr!(
                    "lì non c'è una polilinea chiusa: il poligono esterno va disegnato come polilinea chiusa",
                    "no closed polyline there: the outer outline must be a closed polyline",
                    "ahí no hay ninguna polilínea cerrada: el polígono exterior debe ser una polilínea cerrada",
                    "pas de polyligne fermée à cet endroit : le polygone extérieur doit être une polyligne fermée",
                    "orada kapalı çoklu çizgi yok: dış poligon kapalı bir çoklu çizgi olmalı",
                    "não há aí nenhuma polilinha fechada: o polígono exterior tem de ser uma polilinha fechada",
                    "nie ma tu zamkniętej polilinii: obrys zewnętrzny musi być zamkniętą polilinią",
                )),
            };
            match msg {
                Some(m) => open_dialog_at(host, &hs, Some(m.to_string()), tools::Start::Hub, first),
                None => open_dialog_at(host, &hs, None, tools::Start::Outer, first),
            }
        }
        "RV_PORTAOPEN" => {
            let a = arg.unwrap_or_default();
            let mut parts = a.splitn(2, '|');
            let hs = parse_handles(parts.next().unwrap_or(""));
            let start = parts.next().and_then(tools::DoorSpot::decode).map_or(tools::Start::Hub, tools::Start::Door);
            if !hs.is_empty() {
                open_dialog_at(host, &hs, None, start, false);
            }
        }
        "RV_ETICHETTEAPPLY" => {
            let hs = arg.as_deref().map(parse_handles).unwrap_or_default();
            // The rooms were chosen with the CAD's selection: clear it.
            let _ = host.set_selection(&[]);
            if hs.is_empty() {
                return true;
            }
            // Second word: the letter height chosen in the small dialog.
            match rest.split_whitespace().nth(1).and_then(parse_num) {
                Some(h) if h > 0.0 => {
                    if let Some(rooms) = fetch_rooms(host, &hs) {
                        let cap = letter_cap(&rooms.iter().map(|r| r.pts.clone()).collect::<Vec<_>>());
                        if h > cap {
                            let c = fmt_num(cap, 3);
                            host.push_info(&tr!(
                                format!("RV_ETICHETTE: lettere ridotte a {c}, il 15% del lato più corto del vano più piccolo."),
                                format!("RV_ETICHETTE: letters reduced to {c}, 15% of the shortest side of the smallest room."),
                                format!("RV_ETICHETTE: letras reducidas a {c}, el 15 % del lado más corto de la estancia más pequeña."),
                                format!("RV_ETICHETTE : lettres réduites à {c}, 15 % du plus petit côté de la plus petite pièce."),
                                format!("RV_ETICHETTE: harfler {c} değerine küçültüldü, en küçük odanın en kısa kenarının %15'i."),
                                format!("RV_ETICHETTE: letras reduzidas a {c}, 15% do lado mais curto do compartimento mais pequeno."),
                                format!("RV_ETICHETTE: litery zmniejszone do {c}, 15% najkrótszego boku najmniejszego pomieszczenia."),
                            ));
                        }
                    }
                    *LETTER_H.lock().unwrap_or_else(|e| e.into_inner()) = Some(h);
                    if let Some(mut s) = saved_for(&hs) {
                        s.h = h;
                        remember(&hs, s);
                    }
                    labels_only(host, &hs, h);
                }
                _ => match fetch_rooms(host, &hs) {
                    Some(rooms) => {
                        let pts: Vec<Vec<P2>> = rooms.iter().map(|r| r.pts.clone()).collect();
                        let cap = letter_cap(&pts);
                        let auto = sizes_for(&pts).0.min(cap);
                        let h = saved_for(&hs).map(|s| s.h).or_else(letter_height).unwrap_or(auto).min(cap);
                        host.start_interactive(Box::new(tools::LetterSizeDialog::new(key(&hs), h, auto)));
                    }
                    None => host.push_error(tr!("RV_ETICHETTE: un vano scelto non è più nel disegno.", "RV_ETICHETTE: a chosen room is no longer in the drawing.", "RV_ETICHETTE: una estancia elegida ya no está en el dibujo.", "RV_ETICHETTE : une pièce choisie n'est plus dans le dessin.", "RV_ETICHETTE: seçilen bir oda artık çizimde yok.", "RV_ETICHETTE: um compartimento escolhido já não está no desenho.", "RV_ETICHETTE: wybranego pomieszczenia nie ma już na rysunku.")),
                },
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
                "RV_LIBRETTO: there is no calculation to save yet. Run Sides and diagonals (RV_VANO) first.",
                "RV_LIBRETTO: aún no hay ningún cálculo que guardar. Ejecute primero Lados y diagonales (RV_VANO).",
                "RV_LIBRETTO : il n'y a pas encore de calcul à enregistrer. Lancez d'abord Côtés et diagonales (RV_VANO).",
                "RV_LIBRETTO: henüz kaydedilecek bir hesap yok. Önce Kenarlar ve köşegenler (RV_VANO) komutunu çalıştırın.",
                "RV_LIBRETTO: ainda não há nenhum cálculo para guardar. Execute primeiro Lados e diagonais (RV_VANO).",
                "RV_LIBRETTO: nie ma jeszcze obliczenia do zapisania. Najpierw uruchom Boki i przekątne (RV_VANO).",
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
                format!("{verb}: unknown command. RV_HELP lists the commands."),
                format!("{verb}: comando desconocido. RV_HELP muestra la lista de comandos."),
                format!("{verb} : commande inconnue. RV_HELP affiche la liste des commandes."),
                format!("{verb}: bilinmeyen komut. RV_HELP komut listesini gösterir."),
                format!("{verb}: comando desconhecido. RV_HELP mostra a lista de comandos."),
                format!("{verb}: nieznane polecenie. RV_HELP pokazuje listę poleceń."),
            ));
        }
    }
    true
}

/// The surveying manual (PDF) in each language, built into the plugin.
fn manual_pdf() -> &'static [u8] {
    tr!(
        include_bytes!("../docs/Manuale_rilievo.pdf").as_slice(),
        include_bytes!("../docs/manual_en.pdf").as_slice(),
        include_bytes!("../docs/manual_es.pdf").as_slice(),
        include_bytes!("../docs/manual_fr.pdf").as_slice(),
        include_bytes!("../docs/manual_tr.pdf").as_slice(),
        include_bytes!("../docs/manual_pt.pdf").as_slice(),
        include_bytes!("../docs/manual_pl.pdf").as_slice(),
    )
}

/// Write the manual next to the temporary files and open it with the
/// system's PDF viewer.
fn open_manual(host: &mut dyn HostApi) {
    let lang = crate::i18n::current().code();
    let path = std::env::temp_dir().join(format!("RilievoVani_manual_{lang}_{}.pdf", env!("CARGO_PKG_VERSION")));
    if let Err(e) = std::fs::write(&path, manual_pdf()) {
        host.push_error(&tr!(
            format!("RV_MANUALE: non riesco a scrivere il manuale in {}: {e}", path.display()),
            format!("RV_MANUALE: cannot write the manual to {}: {e}", path.display()),
            format!("RV_MANUALE: no se puede escribir el manual en {}: {e}", path.display()),
            format!("RV_MANUALE : impossible d'écrire le manuel dans {} : {e}", path.display()),
            format!("RV_MANUALE: kılavuz {} konumuna yazılamıyor: {e}", path.display()),
            format!("RV_MANUALE: não é possível escrever o manual em {}: {e}", path.display()),
            format!("RV_MANUALE: nie można zapisać podręcznika w {}: {e}", path.display()),
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
            format!("RV_MANUALE: surveying manual opened ({}).", path.display()),
            format!("RV_MANUALE: manual de levantamiento abierto ({}).", path.display()),
            format!("RV_MANUALE : manuel de relevé ouvert ({}).", path.display()),
            format!("RV_MANUALE: ölçüm kılavuzu açıldı ({}).", path.display()),
            format!("RV_MANUALE: manual de levantamento aberto ({}).", path.display()),
            format!("RV_MANUALE: otwarto podręcznik pomiaru ({}).", path.display()),
        )),
        Err(e) => host.push_error(&tr!(
            format!("RV_MANUALE: il manuale è in {} ma non riesco ad aprirlo ({e}): aprilo a mano.", path.display()),
            format!("RV_MANUALE: the manual is at {} but cannot be opened ({e}): open it by hand.", path.display()),
            format!("RV_MANUALE: el manual está en {} pero no se puede abrir ({e}): ábralo manualmente.", path.display()),
            format!("RV_MANUALE : le manuel se trouve dans {} mais ne peut pas être ouvert ({e}) : ouvrez-le à la main.", path.display()),
            format!("RV_MANUALE: kılavuz {} konumunda ama açılamıyor ({e}): elle açın.", path.display()),
            format!("RV_MANUALE: o manual está em {} mas não pode ser aberto ({e}): abra-o manualmente.", path.display()),
            format!("RV_MANUALE: podręcznik jest w {}, ale nie można go otworzyć ({e}): otwórz go ręcznie.", path.display()),
        )),
    }
}

fn help(host: &mut dyn HostApi) {
    host.push_output(&tr!(
        format!(
            "Room Survey {}: RV_VANO (misure di lati, diagonali e collegamenti di uno o più vani), RV_ETICHETTE \
             (lettere sui vertici), RV_APPLICA (porta i vani originali nella forma calcolata), RV_LIBRETTO (salva il libretto delle misure), RV_GUIDE (guida), RV_MANUALE (manuale di rilievo in PDF), RV_LANG IT|EN|ES|FR|TR|PT|PL|AUTO.",
            env!("CARGO_PKG_VERSION")
        ),
        format!(
            "Room Survey {}: RV_VANO (sides, diagonals and links of one or more rooms), RV_ETICHETTE (corner letters), \
             RV_APPLICA (move the original rooms to the computed shape), RV_LIBRETTO (save the measures book), RV_GUIDE (guide), RV_MANUALE (surveying manual, PDF), RV_LANG IT|EN|ES|FR|TR|PT|PL|AUTO.",
            env!("CARGO_PKG_VERSION")
        ),
        format!(
            "Room Survey {}: RV_VANO (lados, diagonales y enlaces de una o varias estancias), RV_ETICHETTE (letras de las esquinas), RV_APPLICA (lleva las estancias originales a la forma calculada), RV_LIBRETTO (guarda el libro de medidas), RV_GUIDE (guía), RV_MANUALE (manual de levantamiento, PDF), RV_LANG IT|EN|ES|FR|TR|PT|PL|AUTO.",
            env!("CARGO_PKG_VERSION")
        ),
        format!(
            "Room Survey {} : RV_VANO (côtés, diagonales et liaisons d'une ou plusieurs pièces), RV_ETICHETTE (lettres des angles), RV_APPLICA (amène les pièces d'origine à la forme calculée), RV_LIBRETTO (enregistre le carnet de mesures), RV_GUIDE (guide), RV_MANUALE (manuel de relevé, PDF), RV_LANG IT|EN|ES|FR|TR|PT|PL|AUTO.",
            env!("CARGO_PKG_VERSION")
        ),
        format!(
            "Room Survey {}: RV_VANO (bir veya birden çok odanın kenarları, köşegenleri ve bağlantıları), RV_ETICHETTE (köşe harfleri), RV_APPLICA (özgün odaları hesaplanan şekle getirir), RV_LIBRETTO (ölçü defterini kaydeder), RV_GUIDE (rehber), RV_MANUALE (ölçüm kılavuzu, PDF), RV_LANG IT|EN|ES|FR|TR|PT|PL|AUTO.",
            env!("CARGO_PKG_VERSION")
        ),
        format!(
            "Room Survey {}: RV_VANO (lados, diagonais e ligações de um ou mais compartimentos), RV_ETICHETTE (letras dos cantos), RV_APPLICA (leva os compartimentos originais à forma calculada), RV_LIBRETTO (guarda o caderno de medidas), RV_GUIDE (guia), RV_MANUALE (manual de levantamento, PDF), RV_LANG IT|EN|ES|FR|TR|PT|PL|AUTO.",
            env!("CARGO_PKG_VERSION")
        ),
        format!(
            "Room Survey {}: RV_VANO (boki, przekątne i powiązania jednego lub wielu pomieszczeń), RV_ETICHETTE (litery narożników), RV_APPLICA (doprowadza oryginalne pomieszczenia do obliczonego kształtu), RV_LIBRETTO (zapisuje dziennik pomiarów), RV_GUIDE (przewodnik), RV_MANUALE (podręcznik pomiaru, PDF), RV_LANG IT|EN|ES|FR|TR|PT|PL|AUTO.",
            env!("CARGO_PKG_VERSION")
        ),
    ));
}

fn lang(host: &mut dyn HostApi, rest: &str) {
    use crate::i18n::{parse_choice, set_choice, status, Choice};
    if !rest.trim().is_empty() {
        match parse_choice(rest) {
            Some(c) => {
                if let Err(e) = set_choice(c) {
                    host.push_error(&format!("RV_LANG: {e}"));
                    return;
                }
            }
            None => {
                host.push_error("RV_LANG: IT | EN | ES | FR | TR | PT | PL | AUTO");
                return;
            }
        }
    }
    let (c, l) = status();
    let c = match c {
        Choice::Auto => "AUTO".to_string(),
        Choice::Fixed(l) => l.code().to_ascii_uppercase(),
    };
    host.push_info(&format!("RV_LANG: {c} → {}", l.name()));
}

/// Pick the rooms, or take the closed polylines already selected.
fn start_pick(host: &mut dyn HostApi, verb: &'static str, title: &'static str) {
    let all = rooms(host);
    if all.is_empty() {
        host.push_error(&tr!(
            format!("{title}: nel disegno non ci sono polilinee chiuse. Disegna ogni vano con PLINEA … Chiudi, poi riprova."),
            format!("{title}: the drawing has no closed polylines. Draw each room with PLINE … Close, then try again."),
            format!("{title}: el dibujo no tiene polilíneas cerradas. Dibuje cada estancia con PLINE … Cerrar e inténtelo de nuevo."),
            format!("{title} : le dessin ne contient aucune polyligne fermée. Dessinez chaque pièce avec PLINE … Clore, puis réessayez."),
            format!("{title}: çizimde kapalı çoklu çizgi yok. Her odayı PLINE … Kapat ile çizip yeniden deneyin."),
            format!("{title}: o desenho não tem polilinhas fechadas. Desenhe cada compartimento com PLINE … Fechar e tente de novo."),
            format!("{title}: rysunek nie zawiera zamkniętych polilinii. Narysuj każde pomieszczenie poleceniem PLINE … Zamknij i spróbuj ponownie."),
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
    let name = format!("{stem}_{}.csv", tr!("libretto_misure", "measures_book", "libro_medidas", "carnet_mesures", "olcu_defteri", "caderno_medidas", "dziennik_pomiarow"));
    match dir {
        Some(d) => d.join(name).to_string_lossy().to_string(),
        None => name,
    }
}

fn open_dialog(host: &mut dyn HostApi, hs: &[Handle], msg: Option<String>) {
    let labels = msg.is_none();
    open_dialog_at(host, hs, msg, tools::Start::Hub, labels);
}

/// What the user typed for these rooms, or a fresh start: every side, the
/// diagonals each room needs, sizes from the drawing.
fn saved_or_new(host: &dyn HostApi, hs: &[Handle], rooms: &[Room], survey: &Survey) -> Saved {
    let n_sides: usize = rooms.iter().map(|r| r.pts.len()).sum();
    match saved_for(hs) {
        Some(s) if s.sides.len() == n_sides => s,
        _ => {
            let all: Vec<P2> = rooms.iter().flat_map(|r| r.pts.iter().copied()).collect();
            let longest = sizes_for(&rooms.iter().map(|r| r.pts.clone()).collect::<Vec<_>>());
            let (h, tol) = if all.is_empty() { (1.0, 0.01) } else { longest };
            let h = letter_height().unwrap_or(h);
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
                outer: String::new(),
                outer_t: String::new(),
                outer_walls: vec![],
                // Solid walls between rooms, from the hatches drawn in them.
                inner_walls: survey.clone().with_hatches(hatches(host)).suggest_inner_walls(inner_reach(survey::guess_unit(longest))),
            }
        }
    }
}

/// Open the measures dialog on its hub or on a door's page; `labels`
/// (re)writes the corner letters on the rooms drawn by eye.
fn open_dialog_at(host: &mut dyn HostApi, hs: &[Handle], msg: Option<String>, start: tools::Start, labels: bool) {
    let Some(rooms) = fetch_rooms(host, hs) else {
        host.push_error(tr!("RV_VANO: un vano scelto non è più nel disegno.", "RV_VANO: a chosen room is no longer in the drawing.", "RV_VANO: una estancia elegida ya no está en el dibujo.", "RV_VANO : une pièce choisie n'est plus dans le dessin.", "RV_VANO: seçilen bir oda artık çizimde yok.", "RV_VANO: um compartimento escolhido já não está no desenho.", "RV_VANO: wybranego pomieszczenia nie ma już na rysunku."));
        return;
    };
    let survey = Survey::new(rooms.iter().map(|r| r.pts.clone()).collect());
    let saved = saved_or_new(host, hs, &rooms, &survey);
    if labels {
        if rooms.iter().any(|r| r.had_arcs) {
            host.push_info(tr!(
                "RV_VANO: i tratti ad arco delle polilinee sono presi come corde (lati diritti).",
                "RV_VANO: the polylines' arc segments are taken as chords (straight sides).",
                "RV_VANO: los tramos de arco de las polilíneas se toman como cuerdas (lados rectos).",
                "RV_VANO : les segments en arc des polylignes sont pris comme cordes (côtés droits).",
                "RV_VANO: çoklu çizgilerin yay parçaları kiriş (düz kenar) olarak alındı.",
                "RV_VANO: os troços em arco das polilinhas são tomados como cordas (lados retos).",
                "RV_VANO: łukowe odcinki polilinii przyjęto jako cięciwy (proste boki).",
            ));
        }
        host.begin_undo_group("RV_ETICHETTE");
        write_labels(host, &rooms, &survey, saved.h);
        host.end_undo_group();
        host.set_dirty();
    }
    let mut saved = saved;
    let (survey, lost) = add_outer(host, survey, &mut saved);
    let survey = survey.with_hatches(hatches(host));
    let msg = msg.or(lost);
    remember(hs, saved.clone());
    host.start_interactive(Box::new(tools::MeasureDialog::new(hs.to_vec(), &survey, &saved, msg).starting(start)));
}

fn labels_only(host: &mut dyn HostApi, hs: &[Handle], h: f64) {
    let Some(rooms) = fetch_rooms(host, hs) else {
        host.push_error(tr!("RV_ETICHETTE: un vano scelto non è più nel disegno.", "RV_ETICHETTE: a chosen room is no longer in the drawing.", "RV_ETICHETTE: una estancia elegida ya no está en el dibujo.", "RV_ETICHETTE : une pièce choisie n'est plus dans le dessin.", "RV_ETICHETTE: seçilen bir oda artık çizimde yok.", "RV_ETICHETTE: um compartimento escolhido já não está no desenho.", "RV_ETICHETTE: wybranego pomieszczenia nie ma już na rysunku."));
        return;
    };
    let survey = Survey::new(rooms.iter().map(|r| r.pts.clone()).collect());
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
        ),
        format!(
            "RV_ETICHETTE: {n} letras escritas en la capa {LAYER_LABELS}. Imprima el plano y úselo en obra para anotar los lados, diagonales y enlaces medidos; después introdúzcalos con Lados y diagonales."
        ),
        format!(
            "RV_ETICHETTE : {n} lettres écrites sur le calque {LAYER_LABELS}. Imprimez le plan et servez-vous-en sur le chantier pour noter les côtés, diagonales et liaisons mesurés ; saisissez-les ensuite avec Côtés et diagonales."
        ),
        format!(
            "RV_ETICHETTE: {LAYER_LABELS} katmanına {n} harf yazıldı. Planı yazdırın ve sahada ölçülen kenarları, köşegenleri ve bağlantıları not etmek için kullanın; sonra bunları Kenarlar ve köşegenler ile girin."
        ),
        format!(
            "RV_ETICHETTE: {n} letras escritas na camada {LAYER_LABELS}. Imprima a planta e use-a na obra para anotar os lados, diagonais e ligações medidos; depois introduza-os com Lados e diagonais."
        ),
        format!(
            "RV_ETICHETTE: zapisano {n} liter na warstwie {LAYER_LABELS}. Wydrukuj rzut i użyj go na budowie do zapisania zmierzonych boków, przekątnych i powiązań; potem wprowadź je w Boki i przekątne."
        ),
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
    // Width 0: one line, no wrapping (the default box is 10 units wide and
    // breaks a sentence into one word per line).
    let mut t = MText::with_value(value, Vector3::new(at[0], at[1], z)).with_height(h).with_width(0.0);
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
    tr!(format!("Vano {}", r + 1), format!("Room {}", r + 1), format!("Estancia {}", r + 1), format!("Pièce {}", r + 1), format!("Oda {}", r + 1), format!("Compartimento {}", r + 1), format!("Pomieszczenie {}", r + 1))
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
    // Never taller than 15 % of the shortest side of the smallest room.
    let letter_h = letter_h.min(letter_cap(rooms));
    // Room name and area, fitted inside the room with some room to spare.
    let mut titles: Vec<(String, P2, f64)> = Vec::new();
    let mut title_ents: Vec<EntityType> = Vec::new();
    for (r, pts) in rooms.iter().enumerate() {
        let mut lines: Vec<(String, f64)> = Vec::new();
        if survey.multi() {
            lines.push((room_title(r), 1.4));
        }
        if let Some(u) = area_unit {
            let f = survey::unit_factor(u);
            let area = solve::signed_area(pts).abs() * f * f;
            lines.push((format!("{} m²", fmt_num(area, 2)), 1.1));
        }
        if lines.is_empty() {
            continue;
        }
        let spec: Vec<(usize, f64)> = lines.iter().map(|(t, k)| (t.chars().count(), *k)).collect();
        let fit = labels::fit_block(pts, &spec, h, centroid(pts));
        for ((t, _), (at, hh)) in lines.iter().zip(fit.at.iter().zip(&fit.heights)) {
            let mut e = mtext(t, *at, z, *hh, AttachmentPoint::MiddleCenter, layer);
            if fit.vertical {
                if let EntityType::MText(m) = &mut e {
                    m.rotation = std::f64::consts::FRAC_PI_2;
                }
            }
            title_ents.push(e);
            // Letters keep clear of the title (as a circle around each line).
            titles.push((t.clone(), *at, *hh));
        }
    }
    // On the drawing the corners carry the letter only: the room's number is
    // in its title, and "1B" would double the text to fit between corners.
    let names: Vec<Vec<String>> = rooms.iter().map(|pts| (0..pts.len()).map(solve::label).collect()).collect();
    let mut out: Vec<EntityType> = title_ents;
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
        // Letters as written now (A) and by earlier versions (1A).
        let mut targets: Vec<(String, P2)> = (0..room.pts.len()).map(|i| (solve::label(i), room.pts[i])).collect();
        if survey.multi() {
            targets.extend((0..room.pts.len()).map(|i| (survey.corner(r, i), room.pts[i])));
        }
        if survey.multi() {
            targets.push((room_title(r), centroid(&room.pts)));
        }
        let title = room_title(r);
        for (name, near) in targets {
            // A room's name may sit anywhere inside it.
            if name == title {
                if let Some((hd, _, _)) = have.iter().find(|(hd, v, p)| *v == title && !gone.contains(hd) && point_in_polygon(&room.pts, *p)) {
                    gone.push(*hd);
                    continue;
                }
            }
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
        SolveError::TooFewPoints => tr!("un vano ha meno di 3 vertici".to_string(), "a room has fewer than 3 corners".to_string(), "una estancia tiene menos de 3 esquinas".to_string(), "une pièce a moins de 3 angles".to_string(), "bir odanın 3'ten az köşesi var".to_string(), "um compartimento tem menos de 3 cantos".to_string(), "pomieszczenie ma mniej niż 3 narożniki".to_string()),
        SolveError::BadObs(k) => {
            let name = measures.get(*k).map(|m| m.name.clone()).unwrap_or_default();
            tr!(format!("la misura {name} non è valida (unisce un vertice a sé stesso?)"), format!("measure {name} is not valid (does it join a corner to itself?)"), format!("la medida {name} no es válida (¿une una esquina consigo misma?)"), format!("la mesure {name} n'est pas valable (relie-t-elle un angle à lui-même ?)"), format!("{name} ölçüsü geçerli değil (bir köşeyi kendisine mi bağlıyor?)"), format!("a medida {name} não é válida (liga um canto a si próprio?)"), format!("pomiar {name} jest nieprawidłowy (łączy narożnik z nim samym?)"))
        }
        SolveError::NotEnough { needed, have, group } => {
            if group.len() < 3 {
                return tr!(
                    format!("i vertici {} non hanno misure: inserisci i lati che li toccano", names(group)),
                    format!("corners {} have no measures: type the sides that touch them", names(group)),
                    format!("las esquinas {} no tienen medidas: introduzca los lados que las tocan", names(group)),
                    format!("les angles {} n'ont aucune mesure : saisissez les côtés qui les touchent", names(group)),
                    format!("{} köşelerinin hiç ölçüsü yok: onlara değen kenarları girin", names(group)),
                    format!("os cantos {} não têm medidas: introduza os lados que lhes tocam", names(group)),
                    format!("narożniki {} nie mają pomiarów: wprowadź boki, które ich dotykają", names(group)),
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
                    ),
                    format!(
                        "las estancias {} están enlazadas y necesitan al menos {needed} medidas, introducidas {have}: faltan {missing}. Cada estancia necesita sus lados y diagonales, más al menos 3 medidas que la enlacen con la otra estancia",
                        join_numbers(&rooms)
                    ),
                    format!(
                        "les pièces {} sont liées et demandent au moins {needed} mesures, {have} saisies : il en manque {missing}. Chaque pièce demande ses côtés et ses diagonales, plus au moins 3 mesures qui la lient à l'autre pièce",
                        join_numbers(&rooms)
                    ),
                    format!(
                        "{} odaları bağlı ve en az {needed} ölçü gerektiriyor, girilen {have}: {missing} eksik. Her oda kendi kenarlarını ve köşegenlerini, ayrıca onu diğer odaya bağlayan en az 3 ölçüyü gerektirir",
                        join_numbers(&rooms)
                    ),
                    format!(
                        "os compartimentos {} estão ligados e precisam de pelo menos {needed} medidas, introduzidas {have}: faltam {missing}. Cada compartimento precisa dos seus lados e diagonais, mais pelo menos 3 medidas que o liguem ao outro compartimento",
                        join_numbers(&rooms)
                    ),
                    format!(
                        "pomieszczenia {} są powiązane i wymagają co najmniej {needed} pomiarów, wprowadzono {have}: brakuje {missing}. Każde pomieszczenie wymaga swoich boków i przekątnych oraz co najmniej 3 pomiarów wiążących je z drugim pomieszczeniem",
                        join_numbers(&rooms)
                    ),
                )
            } else {
                tr!(
                    format!("servono almeno {needed} misure, ne hai inserite {have}: ne mancano {missing}"),
                    format!("at least {needed} measures are needed, {have} typed: {missing} missing"),
                    format!("se necesitan al menos {needed} medidas, introducidas {have}: faltan {missing}"),
                    format!("il faut au moins {needed} mesures, {have} saisies : il en manque {missing}"),
                    format!("en az {needed} ölçü gerekli, girilen {have}: {missing} eksik"),
                    format!("são precisas pelo menos {needed} medidas, introduzidas {have}: faltam {missing}"),
                    format!("potrzeba co najmniej {needed} pomiarów, wprowadzono {have}: brakuje {missing}"),
                )
            };
            if !sides_missing.is_empty() {
                s += &tr!(format!(" (lati senza misura: {})", sides_missing.join(", ")), format!(" (sides with no measure: {})", sides_missing.join(", ")), format!(" (lados sin medida: {})", sides_missing.join(", ")), format!(" (côtés sans mesure : {})", sides_missing.join(", ")), format!(" (ölçüsüz kenarlar: {})", sides_missing.join(", ")), format!(" (lados sem medida: {})", sides_missing.join(", ")), format!(" (boki bez pomiaru: {})", sides_missing.join(", ")));
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
                    ),
                    format!(
                        "las medidas que enlazan las estancias {} no fijan su posición: tómelas hacia esquinas distintas (por ejemplo el espesor del muro más dos medidas a través de la puerta, desde esquinas distintas)",
                        join_numbers(&rooms)
                    ),
                    format!(
                        "les mesures qui lient les pièces {} ne fixent pas leur position : prenez-les vers des angles différents (par exemple l'épaisseur du mur plus deux mesures à travers la porte, depuis des angles différents)",
                        join_numbers(&rooms)
                    ),
                    format!(
                        "{} odalarını bağlayan ölçüler konumlarını sabitlemiyor: onları farklı köşelere doğru alın (örneğin duvar kalınlığı artı kapıdan geçen iki ölçü, farklı köşelerden)",
                        join_numbers(&rooms)
                    ),
                    format!(
                        "as medidas que ligam os compartimentos {} não fixam a sua posição: tome-as para cantos diferentes (por exemplo a espessura da parede mais duas medidas através da porta, a partir de cantos diferentes)",
                        join_numbers(&rooms)
                    ),
                    format!(
                        "pomiary wiążące pomieszczenia {} nie ustalają ich położenia: weź je do różnych narożników (np. grubość ściany plus dwa pomiary przez drzwi, z różnych narożników)",
                        join_numbers(&rooms)
                    ),
                )
            } else {
                tr!(
                    "le diagonali non bloccano tutta la forma: una parte del vano può ancora deformarsi".to_string(),
                    "the diagonals do not brace the whole shape: part of the room can still move".to_string(),
                    "las diagonales no arriostran toda la forma: una parte de la estancia aún puede deformarse".to_string(),
                    "les diagonales ne bloquent pas toute la forme : une partie de la pièce peut encore se déformer".to_string(),
                    "köşegenler tüm şekli sabitlemiyor: odanın bir kısmı hâlâ şekil değiştirebilir".to_string(),
                    "as diagonais não travam toda a forma: uma parte do compartimento ainda se pode deformar".to_string(),
                    "przekątne nie usztywniają całego kształtu: część pomieszczenia nadal może się odkształcać".to_string(),
                )
            };
            if !proposed.is_empty() {
                s += &tr!(format!(". Misura anche: {}", proposed.join(", ")), format!(". Also measure: {}", proposed.join(", ")), format!(". Mida también: {}", proposed.join(", ")), format!(". Mesurez aussi : {}", proposed.join(", ")), format!(". Şunları da ölçün: {}", proposed.join(", ")), format!(". Meça também: {}", proposed.join(", ")), format!(". Zmierz także: {}", proposed.join(", ")));
            }
            s
        }
        SolveError::NoConvergence => tr!(
            "le misure non stanno insieme (per esempio una diagonale più lunga di due lati sommati, due vertici scambiati \
             o un collegamento scritto al contrario): ricontrollale"
                .to_string(),
            "the measures do not fit together (for example a diagonal longer than two sides together, two corners swapped \
             or a link written the wrong way round): check them"
                .to_string(),
            "las medidas no encajan (por ejemplo una diagonal más larga que dos lados juntos, dos esquinas intercambiadas o un enlace escrito al revés): revíselas"
                .to_string(),
            "les mesures ne concordent pas (par exemple une diagonale plus longue que deux côtés réunis, deux angles intervertis ou une liaison écrite à l'envers) : vérifiez-les"
                .to_string(),
            "ölçüler uyuşmuyor (örneğin iki kenarın toplamından uzun bir köşegen, yer değiştirmiş iki köşe veya ters yazılmış bir bağlantı): kontrol edin"
                .to_string(),
            "as medidas não batem certo (por exemplo uma diagonal mais longa do que dois lados juntos, dois cantos trocados ou uma ligação escrita ao contrário): verifique-as"
                .to_string(),
            "pomiary się nie zgadzają (np. przekątna dłuższa niż dwa boki razem, zamienione dwa narożniki lub powiązanie wpisane odwrotnie): sprawdź je"
                .to_string(),
        ),
    }
}

fn join_numbers(v: &[usize]) -> String {
    let s: Vec<String> = v.iter().map(|n| n.to_string()).collect();
    match s.len() {
        0 => String::new(),
        1 => s[0].clone(),
        _ => format!("{} {} {}", s[..s.len() - 1].join(", "), tr!("e", "and", "y", "et", "ve", "e", "i"), s[s.len() - 1]),
    }
}

fn kind_name(k: Kind) -> &'static str {
    match k {
        Kind::Side => tr!("lato", "side", "lado", "côté", "kenar", "lado", "bok"),
        Kind::Diagonal => tr!("diagonale", "diagonal", "diagonal", "diagonale", "köşegen", "diagonal", "przekątna"),
        Kind::Link => tr!("collegamento", "link", "enlace", "liaison", "bağlantı", "ligação", "powiązanie"),
        Kind::Wall => tr!("spessore muro", "wall thickness", "espesor del muro", "épaisseur du mur", "duvar kalınlığı", "espessura da parede", "grubość ściany"),
        Kind::Angle => tr!("angolo", "angle", "ángulo", "angle", "açı", "ângulo", "kąt"),
        Kind::Level => tr!("orizzontale / verticale", "horizontal / vertical", "horizontal / vertical", "horizontal / vertical", "yatay / dikey", "horizontal / vertical", "poziomy / pionowy"),
        Kind::Door => tr!("porta comune", "shared door", "puerta común", "porte commune", "ortak kapı", "porta comum", "wspólne drzwi"),
        Kind::Outer => tr!("muro esterno", "outer wall", "muro exterior", "mur extérieur", "dış duvar", "parede exterior", "ściana zewnętrzna"),
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
        host.push_error(tr!("RV_VANO: un vano scelto non è più nel disegno.", "RV_VANO: a chosen room is no longer in the drawing.", "RV_VANO: una estancia elegida ya no está en el dibujo.", "RV_VANO : une pièce choisie n'est plus dans le dessin.", "RV_VANO: seçilen bir oda artık çizimde yok.", "RV_VANO: um compartimento escolhido já não está no desenho.", "RV_VANO: wybranego pomieszczenia nie ma już na rysunku."));
        return;
    };
    let mut saved = saved;
    let (survey, lost) = add_outer(host, Survey::new(rooms.iter().map(|r| r.pts.clone()).collect()), &mut saved);
    if let Some(msg) = lost {
        remember(hs, saved.clone());
        host.push_error(&format!("RV_VANO: {msg}."));
        open_dialog(host, hs, Some(msg));
        return;
    }
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
    let weights = weights_of(&measures);
    let sol = match solve::solve_network_fixed(&survey.points, &meas, &weights, meas.len(), &survey.datum(), &survey.fixed()) {
        Ok(s) => s,
        Err(e @ (SolveError::NotEnough { .. } | SolveError::NotRigid { .. }))
            if saved.fill && solve_filled(&survey, &meas, &weights).is_some() =>
        {
            filled = Some(explain(&e, &survey, &measures));
            solve_filled(&survey, &meas, &weights).unwrap()
        }
        Err(e) => {
            let msg = explain(&e, &survey, &measures);
            host.push_error(&format!("RV_VANO: {msg}."));
            open_dialog(host, hs, Some(msg));
            return;
        }
    };

    // Text height and tolerance suited to the computed rooms: the dialog
    // proposed them from the drawing by eye, which may be at another scale.
    let adjusted_pts: Vec<Vec<P2>> = (0..rooms.len()).map(|r| survey.room_points(r, &sol.pts)).collect();
    let (h_auto, tol_auto) = sizes_for(&adjusted_pts);
    let mut saved = saved;
    let mut resized = false;
    if h_auto > 0.0 && !(saved.h >= h_auto / 3.0 && saved.h <= h_auto * 3.0) {
        saved.h = h_auto;
        resized = true;
    }
    if tol_auto > 0.0 && !(saved.tol >= tol_auto / 5.0 && saved.tol <= tol_auto * 5.0) {
        saved.tol = tol_auto;
        resized = true;
    }
    if resized {
        remember(hs, saved.clone());
        host.push_info(&tr!(
            format!(
                "RV_VANO: il disegno a occhio era a una scala diversa dalle misure: altezza testi {} e tolleranza {} adattate ai vani calcolati.",
                fmt_num(saved.h, 3),
                fmt_num(saved.tol, 3)
            ),
            format!(
                "RV_VANO: the drawing by eye was at another scale than the measures: text height {} and tolerance {} fitted to the computed rooms.",
                fmt_num(saved.h, 3),
                fmt_num(saved.tol, 3)
            ),
            format!(
                "RV_VANO: el dibujo a ojo estaba a otra escala que las medidas: altura de texto {} y tolerancia {} adaptadas a las estancias calculadas.",
                fmt_num(saved.h, 3),
                fmt_num(saved.tol, 3)
            ),
            format!(
                "RV_VANO : le dessin à main levée était à une autre échelle que les mesures : hauteur de texte {} et tolérance {} adaptées aux pièces calculées.",
                fmt_num(saved.h, 3),
                fmt_num(saved.tol, 3)
            ),
            format!(
                "RV_VANO: göz kararı çizim ölçülerden farklı bir ölçekteydi: yazı yüksekliği {} ve tolerans {} hesaplanan odalara uyarlandı.",
                fmt_num(saved.h, 3),
                fmt_num(saved.tol, 3)
            ),
            format!(
                "RV_VANO: o desenho à mão estava numa escala diferente das medidas: altura do texto {} e tolerância {} adaptadas aos compartimentos calculados.",
                fmt_num(saved.h, 3),
                fmt_num(saved.tol, 3)
            ),
            format!(
                "RV_VANO: szkic był w innej skali niż pomiary: wysokość tekstu {} i tolerancję {} dopasowano do obliczonych pomieszczeń.",
                fmt_num(saved.h, 3),
                fmt_num(saved.tol, 3)
            ),
        ));
    }
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
            format!("RV_VANO: room of {} corners recomputed from {} measures → layer {LAYER_ROOM}.", rooms[0].pts.len(), measures.len()),
            format!("RV_VANO: estancia de {} esquinas recalculada con {} medidas → capa {LAYER_ROOM}.", rooms[0].pts.len(), measures.len()),
            format!("RV_VANO : pièce de {} angles recalculée à partir de {} mesures → calque {LAYER_ROOM}.", rooms[0].pts.len(), measures.len()),
            format!("RV_VANO: {} köşeli oda {} ölçüyle yeniden hesaplandı → {LAYER_ROOM} katmanı.", rooms[0].pts.len(), measures.len()),
            format!("RV_VANO: compartimento de {} cantos recalculado a partir de {} medidas → camada {LAYER_ROOM}.", rooms[0].pts.len(), measures.len()),
            format!("RV_VANO: pomieszczenie o {} narożnikach przeliczone z {} pomiarów → warstwa {LAYER_ROOM}.", rooms[0].pts.len(), measures.len()),
        )
    } else {
        tr!(
            format!("RV_VANO: {nr} vani ricalcolati con {} misure ({} gruppi indipendenti) → layer {LAYER_ROOM}.", measures.len(), sol.groups),
            format!("RV_VANO: {nr} rooms recomputed from {} measures ({} independent groups) → layer {LAYER_ROOM}.", measures.len(), sol.groups),
            format!("RV_VANO: {nr} estancias recalculadas con {} medidas ({} grupos independientes) → capa {LAYER_ROOM}.", measures.len(), sol.groups),
            format!("RV_VANO : {nr} pièces recalculées à partir de {} mesures ({} groupes indépendants) → calque {LAYER_ROOM}.", measures.len(), sol.groups),
            format!("RV_VANO: {nr} oda {} ölçüyle yeniden hesaplandı ({} bağımsız grup) → {LAYER_ROOM} katmanı.", measures.len(), sol.groups),
            format!("RV_VANO: {nr} compartimentos recalculados a partir de {} medidas ({} grupos independentes) → camada {LAYER_ROOM}.", measures.len(), sol.groups),
            format!("RV_VANO: {nr} pomieszczeń przeliczono z {} pomiarów ({} niezależnych grup) → warstwa {LAYER_ROOM}.", measures.len(), sol.groups),
        )
    }];
    if nr > 1 && sol.groups > 1 {
        lines.push(tr!(
            "  Alcuni vani non sono collegati tra loro: ciascuno resta al suo posto. Per collegarli aggiungi spessori dei muri e misure attraverso le porte.".to_string(),
            "  Some rooms are not linked: each stays where it is. To link them add wall thicknesses and measures through the doors.".to_string(),
            "  Algunas estancias no están enlazadas: cada una queda donde está. Para enlazarlas añada espesores de muro y medidas a través de las puertas.".to_string(),
            "  Certaines pièces ne sont pas liées : chacune reste à sa place. Pour les lier, ajoutez des épaisseurs de mur et des mesures à travers les portes.".to_string(),
            "  Bazı odalar bağlı değil: her biri yerinde kalır. Bağlamak için duvar kalınlıkları ve kapılardan geçen ölçüler ekleyin.".to_string(),
            "  Alguns compartimentos não estão ligados: cada um fica onde está. Para os ligar, acrescente espessuras de parede e medidas através das portas.".to_string(),
            "  Niektóre pomieszczenia nie są powiązane: każde zostaje na swoim miejscu. Aby je powiązać, dodaj grubości ścian i pomiary przez drzwi.".to_string(),
        ));
    }
    match sol.sigma0 {
        Some(s0) => lines.push(tr!(
            format!("  Misure in più (controllo): {} · scarto quadratico medio σ₀ = {}", sol.redundancy, fmt_num(s0, dec)),
            format!("  Extra (check) measures: {} · standard deviation σ₀ = {}", sol.redundancy, fmt_num(s0, dec)),
            format!("  Medidas de más (control): {} · desviación típica σ₀ = {}", sol.redundancy, fmt_num(s0, dec)),
            format!("  Mesures en plus (contrôle) : {} · écart type σ₀ = {}", sol.redundancy, fmt_num(s0, dec)),
            format!("  Fazla ölçüler (kontrol): {} · standart sapma σ₀ = {}", sol.redundancy, fmt_num(s0, dec)),
            format!("  Medidas a mais (controlo): {} · desvio-padrão σ₀ = {}", sol.redundancy, fmt_num(s0, dec)),
            format!("  Pomiary nadliczbowe (kontrola): {} · odchylenie standardowe σ₀ = {}", sol.redundancy, fmt_num(s0, dec)),
        )),
        None if filled.is_some() => {}
        None => lines.push(tr!(
            "  Nessuna misura di controllo: la forma è esatta ma un errore di misura non si vede. Misura una diagonale in più.".to_string(),
            "  No check measure: the shape is exact but a wrong measure cannot show. Measure one more diagonal.".to_string(),
            "  Ninguna medida de control: la forma es exacta pero un error de medida no se nota. Mida una diagonal más.".to_string(),
            "  Aucune mesure de contrôle : la forme est exacte mais une mesure erronée ne se voit pas. Mesurez une diagonale de plus.".to_string(),
            "  Kontrol ölçüsü yok: şekil kesin ama bir ölçü hatası fark edilmez. Bir köşegen daha ölçün.".to_string(),
            "  Nenhuma medida de controlo: a forma é exata mas um erro de medida não se nota. Meça mais uma diagonal.".to_string(),
            "  Brak pomiaru kontrolnego: kształt jest dokładny, ale błąd pomiaru nie zostanie zauważony. Zmierz jeszcze jedną przekątną.".to_string(),
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
            ),
            format!(
                "RV_VANO: residuos por encima de la tolerancia ({}) en {}: revise estas medidas (¿cifra mal leída, esquina equivocada?). Con una sola medida de más el error se reparte entre varias medidas: una medida más ayuda a encontrarlo.",
                fmt_num(saved.tol, dec),
                names.join(", ")
            ),
            format!(
                "RV_VANO : résidus au-delà de la tolérance ({}) sur {} : vérifiez ces mesures (chiffre mal lu, mauvais angle ?). Avec une seule mesure en plus, l'erreur se répartit sur plusieurs mesures : une mesure de plus aide à la trouver.",
                fmt_num(saved.tol, dec),
                names.join(", ")
            ),
            format!(
                "RV_VANO: {} üzerinde toleransı ({}) aşan artıklar: bu ölçüleri kontrol edin (yanlış okunmuş rakam, yanlış köşe?). Yalnızca bir fazla ölçüyle hata birkaç ölçüye dağılır: bir ölçü daha onu bulmaya yardım eder.",
                fmt_num(saved.tol, dec),
                names.join(", ")
            ),
            format!(
                "RV_VANO: resíduos acima da tolerância ({}) em {}: verifique estas medidas (algarismo mal lido, canto errado?). Com uma só medida a mais o erro reparte-se por várias medidas: mais uma medida ajuda a encontrá-lo.",
                fmt_num(saved.tol, dec),
                names.join(", ")
            ),
            format!(
                "RV_VANO: odchyłki powyżej tolerancji ({}) w {}: sprawdź te pomiary (źle odczytana cyfra, zły narożnik?). Przy jednym pomiarze nadliczbowym błąd rozkłada się na kilka pomiarów: kolejny pomiar pomaga go znaleźć.",
                fmt_num(saved.tol, dec),
                names.join(", ")
            ),
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
            ),
            format!(
                "RV_VANO: medidas insuficientes ({why}). El cálculo se ha completado con los ángulos y posiciones del dibujo: las partes no medidas son solo indicativas. Si puede, añada las medidas o restricciones que faltan."
            ),
            format!(
                "RV_VANO : mesures insuffisantes ({why}). Le calcul a été complété avec les angles et positions du dessin : les parties non mesurées sont seulement indicatives. Si possible, ajoutez les mesures ou contraintes manquantes."
            ),
            format!(
                "RV_VANO: yetersiz ölçü ({why}). Hesap çizimdeki açı ve konumlarla tamamlandı: ölçülmeyen kısımlar yalnızca yaklaşıktır. Mümkünse eksik ölçüleri veya kısıtları ekleyin."
            ),
            format!(
                "RV_VANO: medidas insuficientes ({why}). O cálculo foi completado com os ângulos e posições do desenho: as partes não medidas são apenas indicativas. Se puder, acrescente as medidas ou restrições em falta."
            ),
            format!(
                "RV_VANO: za mało pomiarów ({why}). Obliczenie uzupełniono kątami i położeniami z rysunku: niezmierzone części są tylko orientacyjne. Jeśli możesz, dodaj brakujące pomiary lub warunki."
            ),
        ));
    }
    for w in &op_warn {
        host.push_error(&format!("RV_VANO: {w}."));
    }
    if dims_failed > 0 {
        host.push_error(&tr!(
            format!("RV_VANO: {dims_failed} quote non create dal CAD; le misure sono scritte come testo."),
            format!("RV_VANO: the CAD did not create {dims_failed} dimensions; the measures are written as text."),
            format!("RV_VANO: el CAD no ha creado {dims_failed} cotas; las medidas se han escrito como texto."),
            format!("RV_VANO : le CAO n'a pas créé {dims_failed} cotes ; les mesures sont écrites en texte."),
            format!("RV_VANO: CAD {dims_failed} ölçülendirmeyi oluşturmadı; ölçüler metin olarak yazıldı."),
            format!("RV_VANO: o CAD não criou {dims_failed} cotas; as medidas foram escritas como texto."),
            format!("RV_VANO: CAD nie utworzył {dims_failed} wymiarów; pomiary zapisano jako tekst."),
        ));
    }
    if saved.replace {
        apply_to_drawing(host, false);
    } else {
        host.push_info(tr!(
            "RV_VANO: controllato il risultato, Applica al disegno (RV_APPLICA) porta le polilinee originali nella forma calcolata.",
            "RV_VANO: once the result is checked, Apply to drawing (RV_APPLICA) moves the original polylines to the computed shape.",
            "RV_VANO: una vez comprobado el resultado, Aplicar al dibujo (RV_APPLICA) lleva las polilíneas originales a la forma calculada.",
            "RV_VANO : une fois le résultat vérifié, Appliquer au dessin (RV_APPLICA) amène les polylignes d'origine à la forme calculée.",
            "RV_VANO: sonucu kontrol ettikten sonra Çizime uygula (RV_APPLICA) özgün çoklu çizgileri hesaplanan şekle getirir.",
            "RV_VANO: depois de verificar o resultado, Aplicar ao desenho (RV_APPLICA) leva as polilinhas originais à forma calculada.",
            "RV_VANO: po sprawdzeniu wyniku Zastosuj na rysunku (RV_APPLICA) doprowadza oryginalne polilinie do obliczonego kształtu.",
        ));
    }
    if saved.csv {
        write_book(host, &path, &book);
    } else {
        host.push_info(tr!(
            "RV_VANO: per salvare il libretto delle misure (CSV) usa Libretto misure (RV_LIBRETTO).",
            "RV_VANO: to save the measures book (CSV) use Measures book (RV_LIBRETTO).",
            "RV_VANO: para guardar el libro de medidas (CSV) use Libro de medidas (RV_LIBRETTO).",
            "RV_VANO : pour enregistrer le carnet de mesures (CSV), utilisez Carnet de mesures (RV_LIBRETTO).",
            "RV_VANO: ölçü defterini (CSV) kaydetmek için Ölçü defteri (RV_LIBRETTO) kullanın.",
            "RV_VANO: para guardar o caderno de medidas (CSV) use Caderno de medidas (RV_LIBRETTO).",
            "RV_VANO: aby zapisać dziennik pomiarów (CSV), użyj Dziennik pomiarów (RV_LIBRETTO).",
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
            "RV_APPLICA: there is no calculation to apply. Run Sides and diagonals (RV_VANO) first and check the result.",
            "RV_APPLICA: no hay ningún cálculo que aplicar. Ejecute primero Lados y diagonales (RV_VANO) y compruebe el resultado.",
            "RV_APPLICA : il n'y a aucun calcul à appliquer. Lancez d'abord Côtés et diagonales (RV_VANO) et vérifiez le résultat.",
            "RV_APPLICA: uygulanacak hesap yok. Önce Kenarlar ve köşegenler (RV_VANO) çalıştırın ve sonucu kontrol edin.",
            "RV_APPLICA: não há nenhum cálculo para aplicar. Execute primeiro Lados e diagonais (RV_VANO) e verifique o resultado.",
            "RV_APPLICA: nie ma obliczenia do zastosowania. Najpierw uruchom Boki i przekątne (RV_VANO) i sprawdź wynik.",
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
            format!("RV_APPLICA: {done} polylines moved to the computed shape (Ctrl+Z to undo)."),
            format!("RV_APPLICA: {done} polilíneas llevadas a la forma calculada (Ctrl+Z para deshacer)."),
            format!("RV_APPLICA : {done} polylignes amenées à la forme calculée (Ctrl+Z pour annuler)."),
            format!("RV_APPLICA: {done} çoklu çizgi hesaplanan şekle getirildi (geri almak için Ctrl+Z)."),
            format!("RV_APPLICA: {done} polilinhas levadas à forma calculada (Ctrl+Z para anular)."),
            format!("RV_APPLICA: {done} polilinii doprowadzono do obliczonego kształtu (Ctrl+Z, aby cofnąć)."),
        ));
    }
    if missing > 0 {
        host.push_error(&tr!(
            format!("RV_APPLICA: {missing} polilinee non più nel disegno o non modificabili: restano le copie su {LAYER_ROOM}."),
            format!("RV_APPLICA: {missing} polylines no longer in the drawing or not editable: the copies on {LAYER_ROOM} stay."),
            format!("RV_APPLICA: {missing} polilíneas ya no están en el dibujo o no se pueden modificar: se quedan las copias en {LAYER_ROOM}."),
            format!("RV_APPLICA : {missing} polylignes ne sont plus dans le dessin ou ne sont pas modifiables : les copies sur {LAYER_ROOM} restent."),
            format!("RV_APPLICA: {missing} çoklu çizgi artık çizimde değil veya değiştirilemiyor: {LAYER_ROOM} üzerindeki kopyalar kalır."),
            format!("RV_APPLICA: {missing} polilinhas já não estão no desenho ou não podem ser alteradas: ficam as cópias em {LAYER_ROOM}."),
            format!("RV_APPLICA: {missing} polilinii nie ma już na rysunku lub nie można ich zmienić: kopie na {LAYER_ROOM} pozostają."),
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
            format!("Measures book saved to {}. It opens in Excel or LibreOffice Calc.", p.display()),
            format!("Libro de medidas guardado en {}. Se abre con Excel o LibreOffice Calc.", p.display()),
            format!("Carnet de mesures enregistré dans {}. Il s'ouvre avec Excel ou LibreOffice Calc.", p.display()),
            format!("Ölçü defteri {} konumuna kaydedildi. Excel veya LibreOffice Calc ile açılır.", p.display()),
            format!("Caderno de medidas guardado em {}. Abre-se com Excel ou LibreOffice Calc.", p.display()),
            format!("Dziennik pomiarów zapisano w {}. Otwiera się w Excelu lub LibreOffice Calc.", p.display()),
        )),
        Err(e) => host.push_error(&tr!(
            format!("Non riesco a salvare il libretto in {}: {e}. Scegli un'altra cartella con Libretto misure (RV_LIBRETTO).", p.display()),
            format!("Cannot save the book to {}: {e}. Choose another folder with Measures book (RV_LIBRETTO).", p.display()),
            format!("No se puede guardar el libro en {}: {e}. Elija otra carpeta con Libro de medidas (RV_LIBRETTO).", p.display()),
            format!("Impossible d'enregistrer le carnet dans {} : {e}. Choisissez un autre dossier avec Carnet de mesures (RV_LIBRETTO).", p.display()),
            format!("Defter {} konumuna kaydedilemiyor: {e}. Ölçü defteri (RV_LIBRETTO) ile başka bir klasör seçin.", p.display()),
            format!("Não é possível guardar o caderno em {}: {e}. Escolha outra pasta com Caderno de medidas (RV_LIBRETTO).", p.display()),
            format!("Nie można zapisać dziennika w {}: {e}. Wybierz inny folder w Dziennik pomiarów (RV_LIBRETTO).", p.display()),
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
    let sep = if crate::i18n::decimal_comma() { ";" } else { "," };
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
    push(vec![tr!("ROOM SURVEY — LIBRETTO DELLE MISURE", "ROOM SURVEY — MEASURES BOOK", "ROOM SURVEY — LIBRO DE MEDIDAS", "ROOM SURVEY — CARNET DE MESURES", "ROOM SURVEY — ÖLÇÜ DEFTERİ", "ROOM SURVEY — CADERNO DE MEDIDAS", "ROOM SURVEY — DZIENNIK POMIARÓW").into()]);
    push(vec![
        tr!("Progetto hobbistico, non adatto a uso professionale: controllare sempre i risultati.", "Hobby project, not for professional use: always check the results.", "Proyecto aficionado, no apto para uso profesional: compruebe siempre los resultados.", "Projet amateur, non destiné à un usage professionnel : vérifiez toujours les résultats.", "Hobi projesi, profesyonel kullanıma uygun değildir: sonuçları her zaman kontrol edin.", "Projeto amador, não adequado a uso profissional: verifique sempre os resultados.", "Projekt hobbystyczny, nieprzeznaczony do użytku zawodowego: zawsze sprawdzaj wyniki.").into(),
    ]);
    if let Some(d) = doc {
        push(vec![tr!("Disegno", "Drawing", "Dibujo", "Dessin", "Çizim", "Desenho", "Rysunek").into(), d.to_string()]);
    }
    push(vec![tr!("Vani", "Rooms", "Estancias", "Pièces", "Odalar", "Compartimentos", "Pomieszczenia").into(), survey.rooms.len().to_string()]);
    push(vec![tr!("Misure", "Measures", "Medidas", "Mesures", "Ölçüler", "Medidas", "Pomiary").into(), measures.len().to_string()]);
    push(vec![tr!("Misure in più (ridondanza)", "Extra measures (redundancy)", "Medidas de más (redundancia)", "Mesures en plus (redondance)", "Fazla ölçüler (artıklık)", "Medidas a mais (redundância)", "Pomiary nadliczbowe (nadmiarowość)").into(), sol.redundancy.to_string()]);
    push(vec![
        tr!("Scarto quadratico medio σ₀", "Standard deviation σ₀", "Desviación típica σ₀", "Écart type σ₀", "Standart sapma σ₀", "Desvio-padrão σ₀", "Odchylenie standardowe σ₀").into(),
        sol.sigma0.map(|s| fmt_num(s, dec + 1)).unwrap_or_else(|| "—".into()),
    ]);
    push(vec![tr!("Tolleranza", "Tolerance", "Tolerancia", "Tolérance", "Tolerans", "Tolerância", "Tolerancja").into(), fmt_num(tol, dec)]);
    if let Some(why) = filled {
        push(vec![
            tr!("ATTENZIONE", "WARNING", "ATENCIÓN", "ATTENTION", "DİKKAT", "ATENÇÃO", "UWAGA").into(),
            tr!(
                format!("misure non sufficienti ({why}): forma completata con gli angoli del disegno, parti non misurate indicative"),
                format!("not enough measures ({why}): shape finished with the drawing's angles, unmeasured parts indicative"),
                format!("medidas insuficientes ({why}): forma completada con los ángulos del dibujo, partes no medidas indicativas"),
                format!("mesures insuffisantes ({why}) : forme complétée avec les angles du dessin, parties non mesurées indicatives"),
                format!("yetersiz ölçü ({why}): şekil çizimin açılarıyla tamamlandı, ölçülmeyen kısımlar yaklaşık"),
                format!("medidas insuficientes ({why}): forma completada com os ângulos do desenho, partes não medidas indicativas"),
                format!("za mało pomiarów ({why}): kształt uzupełniony kątami z rysunku, niezmierzone części orientacyjne"),
            ),
        ]);
    }
    push(vec![]);
    push(vec![tr!("MISURE", "MEASURES", "MEDIDAS", "MESURES", "ÖLÇÜLER", "MEDIDAS", "POMIARY").into()]);
    push(vec![
        tr!("Misura", "Measure", "Medida", "Mesure", "Ölçü", "Medida", "Pomiar").into(),
        tr!("Tipo", "Kind", "Tipo", "Type", "Tür", "Tipo", "Typ").into(),
        tr!("Misurata", "Measured", "Medida", "Mesurée", "Ölçülen", "Medida", "Zmierzony").into(),
        tr!("Calcolata", "Computed", "Calculada", "Calculée", "Hesaplanan", "Calculada", "Obliczony").into(),
        tr!("Scarto", "Residual", "Residuo", "Résidu", "Artık", "Resíduo", "Odchyłka").into(),
        tr!("Oltre tolleranza", "Over tolerance", "Fuera de tolerancia", "Hors tolérance", "Tolerans dışı", "Fora da tolerância", "Poza tolerancją").into(),
    ]);
    for (k, m) in measures.iter().enumerate() {
        let v = sol.residuals[k];
        push(vec![
            m.name.clone(),
            kind_name(m.kind).into(),
            show(m, m.meas.value(), dec + 1),
            show(m, m.meas.value() + v, dec + 1),
            show_res(m, v, dec + 1),
            if bad.contains(&k) { tr!("SÌ", "YES", "SÍ", "OUI", "EVET", "SIM", "TAK").into() } else { String::new() },
        ]);
    }
    push(vec![]);
    push(vec![tr!("COORDINATE DEI VERTICI (sistema del disegno)", "CORNER COORDINATES (drawing system)", "COORDENADAS DE LAS ESQUINAS (sistema del dibujo)", "COORDONNÉES DES ANGLES (système du dessin)", "KÖŞE KOORDİNATLARI (çizim sistemi)", "COORDENADAS DOS CANTOS (sistema do desenho)", "WSPÓŁRZĘDNE NAROŻNIKÓW (układ rysunku)").into()]);
    push(vec![tr!("Vertice", "Corner", "Esquina", "Angle", "Köşe", "Canto", "Narożnik").into(), "X".into(), "Y".into()]);
    for (p, name) in survey.names.iter().enumerate() {
        push(vec![name.clone(), fmt_num(sol.pts[p][0], dec + 1), fmt_num(sol.pts[p][1], dec + 1)]);
    }
    push(vec![]);
    push(vec![tr!("SUPERFICI E PERIMETRI", "AREAS AND PERIMETERS", "SUPERFICIES Y PERÍMETROS", "SURFACES ET PÉRIMÈTRES", "ALANLAR VE ÇEVRELER", "ÁREAS E PERÍMETROS", "POWIERZCHNIE I OBWODY").into()]);
    push(vec![tr!("Vano", "Room", "Estancia", "Pièce", "Oda", "Compartimento", "Pomieszczenie").into(), tr!("Superficie (m²)", "Area (m²)", "Superficie (m²)", "Surface (m²)", "Alan (m²)", "Área (m²)", "Powierzchnia (m²)").into(), tr!("Perimetro (unità del disegno)", "Perimeter (drawing units)", "Perímetro (unidades del dibujo)", "Périmètre (unités du dessin)", "Çevre (çizim birimi)", "Perímetro (unidades do desenho)", "Obwód (jednostki rysunku)").into()]);
    let f = survey::unit_factor(saved.unit);
    for r in 0..survey.rooms.len() {
        let pts = survey.room_points(r, &sol.pts);
        let per: f64 = (0..pts.len()).map(|i| solve::dist(pts[i], pts[(i + 1) % pts.len()])).sum();
        push(vec![(r + 1).to_string(), fmt_num(solve::signed_area(&pts).abs() * f * f, 2), fmt_num(per, dec)]);
    }
    if !openings.is_empty() {
        push(vec![]);
        push(vec![tr!("PORTE E FINESTRE", "DOORS AND WINDOWS", "PUERTAS Y VENTANAS", "PORTES ET FENÊTRES", "KAPILAR VE PENCERELER", "PORTAS E JANELAS", "DRZWI I OKNA").into()]);
        push(vec![
            tr!("Nome", "Name", "Nombre", "Nom", "Ad", "Nome", "Nazwa").into(),
            tr!("Tipo", "Kind", "Tipo", "Type", "Tür", "Tipo", "Typ").into(),
            tr!("Lato (dallo spigolo)", "Side (from corner)", "Lado (desde la esquina)", "Côté (depuis l'angle)", "Kenar (köşeden)", "Lado (a partir do canto)", "Bok (od narożnika)").into(),
            tr!("Distanza", "Distance", "Distancia", "Distance", "Uzaklık", "Distância", "Odległość").into(),
            tr!("Larghezza", "Width", "Anchura", "Largeur", "Genişlik", "Largura", "Szerokość").into(),
            tr!("Spessore muro", "Wall thickness", "Espesor del muro", "Épaisseur du mur", "Duvar kalınlığı", "Espessura da parede", "Grubość ściany").into(),
            tr!("Comune con", "Shared with", "Común con", "Commune avec", "Ortak olduğu", "Comum com", "Wspólne z").into(),
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
/// The widest number in the table, in characters.
fn rows_width(sol: &Solution, measures: &[Measure], dec: usize) -> usize {
    measures
        .iter()
        .enumerate()
        .map(|(k, m)| {
            let v = sol.residuals.get(k).copied().unwrap_or(0.0);
            show(m, m.meas.value(), dec).chars().count().max(show(m, m.meas.value() + v, dec).chars().count()).max(show_res(m, v, dec).chars().count())
        })
        .max()
        .unwrap_or(6)
}

/// Solve with stand-ins from the drawing added (low weight), returning only
/// the real measures' residuals.
fn solve_filled(survey: &Survey, meas: &[Meas], weights: &[f64]) -> Option<Solution> {
    let extra = survey.stand_ins(meas);
    let mut all = meas.to_vec();
    all.extend(extra.iter().copied());
    let mut w = weights.to_vec();
    w.extend(std::iter::repeat_n(0.05, extra.len()));
    let mut s = solve::solve_network_fixed(&survey.points, &all, &w, meas.len(), &survey.datum(), &survey.fixed()).ok()?;
    s.residuals.truncate(meas.len());
    s.scaled.truncate(meas.len());
    Some(s)
}

fn table(sol: &Solution, measures: &[Measure], bad: &[usize], h: f64, dec: usize, z: f64, filled: bool) -> Vec<EntityType> {
    let maxx = sol.pts.iter().map(|p| p[0]).fold(f64::MIN, f64::max);
    let maxy = sol.pts.iter().map(|p| p[1]).fold(f64::MIN, f64::max);
    let x0 = maxx + h * 5.0;
    let row_h = h * 1.9;
    // CAD fonts run about one text height per character, spacing included.
    let cw = |n: usize| h * (n as f64 * 0.95 + 1.6);
    let longest = measures.iter().map(|m| m.name.chars().count()).max().unwrap_or(5).max(6);
    let num_w = rows_width(sol, measures, dec);
    let col_w = [cw(longest), cw(num_w.max(9)), cw(num_w.max(9)), cw(num_w.max(7))];
    let width: f64 = col_w.iter().sum();
    let mut out = Vec::new();
    let mut y = maxy;

    out.push(mtext(tr!("RILIEVO — misure", "SURVEY — measures", "LEVANTAMIENTO — medidas", "RELEVÉ — mesures", "ÖLÇÜM — ölçüler", "LEVANTAMENTO — medidas", "POMIAR — pomiary"), [x0, y + row_h * 0.6], z, h, AttachmentPoint::MiddleLeft, LAYER_NOTES));
    let headers = [tr!("Misura", "Measure", "Medida", "Mesure", "Ölçü", "Medida", "Pomiar"), tr!("Misurata", "Measured", "Medida", "Mesurée", "Ölçülen", "Medida", "Zmierzony"), tr!("Calcolata", "Computed", "Calculada", "Calculée", "Hesaplanan", "Calculada", "Obliczony"), tr!("Scarto", "Residual", "Residuo", "Résidu", "Artık", "Resíduo", "Odchyłka")];
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
            format!("Extra measures: {} · σ₀ = {}", sol.redundancy, fmt_num(s0, dec)),
            format!("Medidas de más: {} · σ₀ = {}", sol.redundancy, fmt_num(s0, dec)),
            format!("Mesures en plus : {} · σ₀ = {}", sol.redundancy, fmt_num(s0, dec)),
            format!("Fazla ölçüler: {} · σ₀ = {}", sol.redundancy, fmt_num(s0, dec)),
            format!("Medidas a mais: {} · σ₀ = {}", sol.redundancy, fmt_num(s0, dec)),
            format!("Pomiary nadliczbowe: {} · σ₀ = {}", sol.redundancy, fmt_num(s0, dec)),
        ),
        None if filled => String::new(),
        None => tr!("Nessuna misura di controllo".to_string(), "No check measure".to_string(), "Ninguna medida de control".to_string(), "Aucune mesure de contrôle".to_string(), "Kontrol ölçüsü yok".to_string(), "Nenhuma medida de controlo".to_string(), "Brak pomiaru kontrolnego".to_string()),
    };
    out.push(mtext(&foot, [x0, bottom - row_h * 0.6], z, h * 0.85, AttachmentPoint::MiddleLeft, LAYER_NOTES));
    if filled {
        let mut t = mtext(
            tr!(
                "Misure non sufficienti: forma completata dal disegno (parti non misurate indicative)",
                "Not enough measures: shape finished from the drawing (unmeasured parts indicative)",
                "Medidas insuficientes: forma completada desde el dibujo (partes no medidas indicativas)",
                "Mesures insuffisantes : forme complétée d'après le dessin (parties non mesurées indicatives)",
                "Yetersiz ölçü: şekil çizimden tamamlandı (ölçülmeyen kısımlar yaklaşık)",
                "Medidas insuficientes: forma completada a partir do desenho (partes não medidas indicativas)",
                "Za mało pomiarów: kształt uzupełniony z rysunku (niezmierzone części orientacyjne)",
            ),
            [x0, bottom - row_h * 0.6],
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
    let sep = if crate::i18n::decimal_comma() { 44 } else { 46 };
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
                // A side shorter than its own number would only make a knot
                // of arrows: its length is in the table.
                if solve::dist(a, b) < h * 4.0 {
                    continue;
                }
                let at = [mid[0] + out[0] * h * 2.5, mid[1] + out[1] * h * 2.5];
                if !dim_aligned(host, a, b, at) {
                    failed.push((a, b, at));
                }
            }
            (Kind::Angle | Kind::Level | Kind::Door | Kind::Outer, _) => {}
            (_, Meas::Dist { i, j, .. }) => {
                // Diagonals cross near the middle of a room, so the length
                // goes at 35 % or 65 % of the way, alternately.
                let (a, b) = (pts[i], pts[j]);
                let t = if k_line % 2 == 0 { 0.35 } else { 0.65 };
                k_line += 1;
                let at = [a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t];
                marks.push(pline(&[a, b], z, false, LAYER_NOTES));
                if solve::dist(a, b) >= h * 5.0 {
                    marks.push(along_text(&fmt_num(solve::dist(a, b), dec), a, b, at, h, z));
                }
            }
            (_, Meas::Perp { p, a, b, .. }) => {
                let f = solve::foot(pts[p], pts[a], pts[b]);
                let mid = [(f[0] + pts[p][0]) / 2.0, (f[1] + pts[p][1]) / 2.0];
                marks.push(pline(&[f, pts[p]], z, false, LAYER_NOTES));
                let txt = format!("{} {}", tr!("sp.", "th.", "esp.", "ép.", "kal.", "esp.", "gr."), fmt_num(solve::perp_dist(pts[p], pts[a], pts[b]), dec));
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
        OpKind::Door => tr!("porta", "door", "puerta", "porte", "kapı", "porta", "drzwi"),
        OpKind::Window => tr!("finestra", "window", "ventana", "fenêtre", "pencere", "janela", "okno"),
        OpKind::Opening => tr!("apertura", "opening", "abertura", "ouverture", "açıklık", "abertura", "otwór"),
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
                format!("{} {} runs past the end of side {}–{} ({} long)", op_kind_name(o.kind), o.name, survey.names[o.a], survey.names[o.b], fmt_num(l, dec)),
                format!("{} {} sobrepasa el final del lado {}–{} ({} de largo)", op_kind_name(o.kind), o.name, survey.names[o.a], survey.names[o.b], fmt_num(l, dec)),
                format!("{} {} dépasse la fin du côté {}–{} ({} de long)", op_kind_name(o.kind), o.name, survey.names[o.a], survey.names[o.b], fmt_num(l, dec)),
                format!("{} {} kenar {}–{} sonunu aşıyor ({} uzunluğunda)", op_kind_name(o.kind), o.name, survey.names[o.a], survey.names[o.b], fmt_num(l, dec)),
                format!("{} {} ultrapassa o fim do lado {}–{} ({} de comprimento)", op_kind_name(o.kind), o.name, survey.names[o.a], survey.names[o.b], fmt_num(l, dec)),
                format!("{} {} wychodzi poza koniec boku {}–{} (długość {})", op_kind_name(o.kind), o.name, survey.names[o.a], survey.names[o.b], fmt_num(l, dec)),
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
    fn letters_capped_by_smallest_room() {
        let big = vec![(0.0, 0.0), (10.0, 0.0), (10.0, 8.0), (0.0, 8.0)];
        let small = vec![(20.0, 0.0), (22.0, 0.0), (22.0, 1.2), (20.0, 1.2)];
        let rooms: Vec<Vec<P2>> = [big.clone(), small].iter().map(|r| r.iter().map(|&(x, y)| [x, y]).collect()).collect();
        assert!((letter_cap(&rooms) - 0.18).abs() < 1e-9);
        assert!((letter_cap(&rooms[..1]) - 1.2).abs() < 1e-9);
        assert!(letter_cap(&[]).is_infinite());
        // A double click (a side of a millimetre) does not shrink the letters.
        let clicked: Vec<P2> = vec![[0.0, 0.0], [4.0, 0.0], [4.0, 3.0], [4.0, 3.001], [0.0, 3.0]];
        assert!((letter_cap(&[clicked]) - 0.45).abs() < 1e-9);
    }

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
    fn letters_shrink_on_plans_full_of_short_sides() {
        let plain: Vec<P2> = vec![[0.0, 0.0], [5.0, 0.0], [5.0, 4.0], [0.0, 4.0]];
        assert_eq!(sizes_for(&[plain.clone()]).0, default_sizes(&plain).0);
        // A 10 m wall outline with many 15–30 cm steps.
        let mut busy: Vec<P2> = vec![[0.0, 0.0], [10.0, 0.0]];
        for k in 0..10 {
            let x = 10.0 - k as f64;
            busy.extend([[x, 0.3], [x - 0.15, 0.3], [x - 0.15, 0.45], [x - 0.85, 0.45], [x - 0.85, 0.3]]);
        }
        busy.push([0.0, 0.3]);
        let (h, _) = sizes_for(&[busy.clone()]);
        assert!(h <= 0.1 && h >= 10.0 / 150.0, "{h}");
        assert!(h < default_sizes(&busy).0);
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
        let saved = Saved { sides: vec![], others: vec![], h: 0.2, tol: 0.02, dims: true, table: true, csv: false, csv_path: String::new(), constraints: vec![], openings: vec![], unit: 0, fill: true, replace: false, outer: String::new(), outer_t: String::new(), outer_walls: vec![], inner_walls: vec![] };
        let csv = book_csv(&survey, &sol, &meas, &[], &[], &saved, 3, Some("C:\\x.dwg"), None);
        assert!(csv.starts_with('\u{feff}'));
        assert!(csv.contains("A–C"));
        assert!(csv.lines().any(|l| l.starts_with("1") && l.contains("12")), "area row: {csv}");
    }
}
