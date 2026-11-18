//! Doors and windows cut into the walls of the finished drawing (button
//! "Doors and windows", `RV_FORO`).
//!
//! A window asks door or window, width and height; then a corner of a
//! polyline is clicked (the outer perimeter or a room), then the side of
//! that corner the opening lies on, then the distance from the corner. The
//! wall is cut: the clicked line and the parallel line on the other face of
//! the wall are opened where the opening is, the jambs are drawn across,
//! the wall's hatch gets a hole, and the frame and a "width over height"
//! label go on the openings layer, outside the building.
//!
//! Drawing only: an opened room polyline no longer takes part in the
//! survey calculation.

use std::sync::Mutex;

use ocs_plugin_api::dialog::{DialogControl, DialogDef, DialogResponse};
use ocs_plugin_api::host::codec;
use ocs_plugin_api::host::codec::Handle;
use ocs_plugin_api::host::{CommandStep, HostApi, InteractiveCommand, PreviewWire};

use codec::entities::{AttachmentPoint, BoundaryEdge, BoundaryPath, LwVertex, PolylineEdge};
use codec::{EntityType, Vector2, Vector3};

use crate::commands::{self, LAYER_LABELS, LAYER_NOTES, LAYER_OPENINGS, LAYER_ROOM};
use crate::solve::{self, P2};
use crate::survey;
use crate::tools;

const GREEN: [f32; 4] = [0.45, 0.85, 0.45, 1.0];
const ORANGE: [f32; 4] = [0.95, 0.55, 0.2, 1.0];

/// Colours of what is drawn (AutoCAD colour index): frame cyan, bracket
/// grey, values magenta, the line between them red.
const ACI_FRAME: i16 = 4;
const ACI_BRACKET: i16 = 8;
const ACI_TEXT: i16 = 6;
const ACI_RULE: i16 = 1;

// ── Geometry (host-free) ────────────────────────────────────────────────────

fn sub(a: P2, b: P2) -> P2 {
    [a[0] - b[0], a[1] - b[1]]
}
fn add(a: P2, b: P2) -> P2 {
    [a[0] + b[0], a[1] + b[1]]
}
fn mul(a: P2, k: f64) -> P2 {
    [a[0] * k, a[1] * k]
}
fn dot(a: P2, b: P2) -> f64 {
    a[0] * b[0] + a[1] * b[1]
}
fn cross(a: P2, b: P2) -> f64 {
    a[0] * b[1] - a[1] * b[0]
}
fn unit(a: P2, b: P2) -> P2 {
    let l = solve::dist(a, b).max(1e-12);
    [(b[0] - a[0]) / l, (b[1] - a[1]) / l]
}
fn left(u: P2) -> P2 {
    [-u[1], u[0]]
}
fn mid(a: P2, b: P2) -> P2 {
    [(a[0] + b[0]) / 2.0, (a[1] + b[1]) / 2.0]
}

/// A polyline whose corners can be picked: a closed room or perimeter, or
/// one a hole has already opened (its corners still go round the wall).
#[derive(Clone, Debug)]
pub struct Ring {
    pub handle: Handle,
    pub pts: Vec<P2>,
    pub closed: bool,
    pub z: f64,
}

impl Ring {
    /// The sides touching corner `i`: the other corner of each.
    pub fn neighbours(&self, i: usize) -> Vec<usize> {
        let n = self.pts.len();
        let mut v = Vec::new();
        if i > 0 || self.closed {
            v.push((i + n - 1) % n);
        }
        if i + 1 < n || self.closed {
            v.push((i + 1) % n);
        }
        v.dedup();
        v
    }

    /// Whether `p` is inside, the ring taken as closed.
    pub fn contains(&self, p: P2) -> bool {
        commands::point_in_polygon(&self.pts, p)
    }

    pub fn area(&self) -> f64 {
        solve::signed_area(&self.pts).abs()
    }
}

/// The ring's own inside, next to side c–d: the unit normal pointing there.
pub fn inward(ring: &Ring, c: P2, d: P2) -> P2 {
    let u = unit(c, d);
    let l = solve::dist(c, d);
    let probe = add(mid(c, d), mul(left(u), (l * 1e-3).max(1e-6)));
    if ring.contains(probe) { left(u) } else { mul(left(u), -1.0) }
}

/// Whether `ring` is the building's outline: the fixed perimeter, or a ring
/// with another ring wholly inside it.
pub fn is_outer(ring: &Ring, rings: &[Ring], perimeter: Option<Handle>) -> bool {
    if perimeter == Some(ring.handle) {
        return true;
    }
    rings.iter().filter(|r| r.handle != ring.handle).any(|r| r.pts.iter().all(|p| ring.contains(*p)))
}

/// A straight piece of a wall face: segment `seg` of an entity.
#[derive(Clone, Debug)]
pub struct Face {
    pub handle: Handle,
    pub seg: usize,
    pub a: P2,
    pub b: P2,
}

/// The face parallel to a–b on the other side of the wall, `n` pointing
/// into the wall: its index and distance. `full`: the face must cover all
/// of a–b (for a hole); otherwise any overlap will do (to suggest the
/// thickness). With `want`, the face nearest that thickness within `tol`;
/// otherwise the nearest face.
pub fn find_opposite(faces: &[Face], a: P2, b: P2, n: P2, eps: f64, max: f64, full: bool, want: Option<(f64, f64)>) -> Option<(usize, f64)> {
    let u = unit(a, b);
    let mut best: Option<(usize, f64, f64)> = None;
    for (x, f) in faces.iter().enumerate() {
        let len = solve::dist(f.a, f.b);
        if len < 1e-9 {
            continue;
        }
        let uf = unit(f.a, f.b);
        if cross(u, uf).abs() > 0.02 {
            continue;
        }
        let dn = (dot(sub(f.a, a), n) + dot(sub(f.b, a), n)) / 2.0;
        if dn <= eps || dn > max {
            continue;
        }
        let ta = dot(sub(add(a, mul(n, dn)), f.a), uf);
        let tb = dot(sub(add(b, mul(n, dn)), f.a), uf);
        let (lo, hi) = (ta.min(tb), ta.max(tb));
        let tol = eps;
        let ok = if full { lo >= -tol && hi <= len + tol } else { hi > tol && lo < len - tol };
        if !ok {
            continue;
        }
        let score = match want {
            Some((t, wt)) => {
                if (dn - t).abs() > wt {
                    continue;
                }
                (dn - t).abs()
            }
            None => dn,
        };
        if best.as_ref().map_or(true, |&(_, _, s)| score < s) {
            best = Some((x, dn, score));
        }
    }
    best.map(|(x, dn, _)| (x, dn))
}

/// The faces that hold the whole of p–q (lines lying on top of each other:
/// a room and the perimeter's offset line, say): their handles, once each.
pub fn faces_holding(faces: &[Face], p: P2, q: P2, tol: f64) -> Vec<Handle> {
    let mut out: Vec<Handle> = Vec::new();
    for f in faces {
        let len = solve::dist(f.a, f.b);
        if len < 1e-9 {
            continue;
        }
        let u = unit(f.a, f.b);
        let off = |r: P2| cross(u, sub(r, f.a)).abs();
        let at = |r: P2| dot(sub(r, f.a), u);
        if off(p) <= tol && off(q) <= tol && at(p).min(at(q)) >= -tol && at(p).max(at(q)) <= len + tol && !out.contains(&f.handle) {
            out.push(f.handle);
        }
    }
    out
}

/// The straight segment of a polyline (vertices with bulges) that holds the
/// whole of p–q: its index.
pub fn segment_holding(v: &[(P2, f64)], closed: bool, p: P2, q: P2, tol: f64) -> Option<usize> {
    let n = v.len();
    let segs = if closed { n } else { n.saturating_sub(1) };
    (0..segs).find(|&s| {
        let (a, b) = (v[s].0, v[(s + 1) % n].0);
        let len = solve::dist(a, b);
        if v[s].1.abs() > 1e-9 || len < 1e-9 {
            return false;
        }
        let u = unit(a, b);
        let off = |r: P2| cross(u, sub(r, a)).abs();
        let at = |r: P2| dot(sub(r, a), u);
        off(p) <= tol && off(q) <= tol && at(p).min(at(q)) >= -tol && at(p).max(at(q)) <= len + tol
    })
}

/// A polyline opened between p and q on its segment `s`: the pieces left,
/// each an open polyline (one for a closed polyline, up to two for an open
/// one).
pub fn cut(v: &[(P2, f64)], closed: bool, s: usize, p: P2, q: P2) -> Vec<Vec<(P2, f64)>> {
    let n = v.len();
    let a = v[s].0;
    // p1 nearer the segment's start, p2 nearer its end.
    let (p1, p2) = if solve::dist(a, p) <= solve::dist(a, q) { (p, q) } else { (q, p) };
    let mut pieces = Vec::new();
    if closed {
        let mut piece = vec![(p2, 0.0)];
        for k in 1..=n {
            let i = (s + k) % n;
            let bulge = if i == s { 0.0 } else { v[i].1 };
            piece.push((v[i].0, bulge));
        }
        piece.push((p1, 0.0));
        pieces.push(piece);
    } else {
        let mut first: Vec<(P2, f64)> = v[..=s].to_vec();
        if let Some(l) = first.last_mut() {
            l.1 = 0.0;
        }
        first.push((p1, 0.0));
        let mut second = vec![(p2, 0.0)];
        second.extend_from_slice(&v[s + 1..]);
        pieces.push(first);
        pieces.push(second);
    }
    pieces
        .into_iter()
        .map(|mut pc| {
            pc.dedup_by(|b, a| solve::dist(a.0, b.0) < 1e-9);
            pc
        })
        .filter(|pc| pc.len() >= 2)
        .collect()
}

/// Window frame in the hole j–k (clicked face) / j2–k2 (other face, maybe
/// not parallel): a band against the inside face of the wall (the other
/// face when `inner_far`), posts at the jambs and, when wide enough, a
/// mullion with two sashes.
pub fn window_lines(j: P2, k: P2, j2: P2, k2: P2, inner_far: bool, m: f64) -> Vec<Vec<P2>> {
    let w = solve::dist(j, k).max(1e-12);
    let t = solve::dist(j, j2).min(solve::dist(k, k2));
    let depth = (0.25 * t).max(1e-6);
    let lerp = |a: P2, b: P2, f: f64| add(a, mul(sub(b, a), f));
    let at = |s: f64, x: f64| {
        let f = s / w;
        let (c, o) = (lerp(j, k, f), lerp(j2, k2, f));
        let (inner, other) = if inner_far { (o, c) } else { (c, o) };
        add(inner, mul(unit(inner, other), x))
    };
    let f = (0.06 / m).min(w / 8.0);
    let mut out = vec![vec![at(0.0, 0.0), at(w, 0.0), at(w, depth), at(0.0, depth), at(0.0, 0.0)]];
    let mut posts = vec![f, w - f];
    let sashes: Vec<(f64, f64)> = if w > 0.7 / m {
        posts.push(w / 2.0 - f / 2.0);
        posts.push(w / 2.0 + f / 2.0);
        vec![(f, w / 2.0 - f / 2.0), (w / 2.0 + f / 2.0, w - f)]
    } else {
        vec![(f, w - f)]
    };
    for s in posts {
        out.push(vec![at(s, 0.0), at(s, depth)]);
    }
    for (s0, s1) in sashes {
        out.push(vec![at(s0, depth / 2.0), at(s1, depth / 2.0)]);
    }
    out
}

/// Where the ray p + n·s meets the faces roughly across it (within about
/// 25° of square, so walls not parallel are found too): (face, s), nearest
/// first.
pub fn ray_hits(faces: &[Face], p: P2, n: P2, eps: f64, max: f64, tol: f64) -> Vec<(usize, f64)> {
    let mut out = Vec::new();
    for (x, f) in faces.iter().enumerate() {
        let d = sub(f.b, f.a);
        let len = solve::dist(f.a, f.b);
        if len < 1e-9 || dot(mul(d, 1.0 / len), n).abs() > 0.42 {
            continue;
        }
        let den = cross(n, d);
        if den.abs() < 1e-12 {
            continue;
        }
        let ap = sub(f.a, p);
        let s = cross(ap, d) / den;
        let r = cross(ap, n) / den;
        if s > eps && s <= max && r >= -tol / len && r <= 1.0 + tol / len {
            out.push((x, s));
        }
    }
    out.sort_by(|a, b| a.1.total_cmp(&b.1));
    out
}

/// The other face of the wall behind the hole j–k: the distance across at
/// each jamb and the face hit, preferring the thickness `want`.
pub fn far_face(faces: &[Face], j: P2, k: P2, n: P2, want: f64, eps: f64, max: f64, tol: f64) -> Option<(f64, usize, f64, usize)> {
    let pick = |hits: Vec<(usize, f64)>, same: Option<usize>| -> Option<(usize, f64)> {
        if let Some(sx) = same {
            if let Some(h) = hits.iter().find(|h| h.0 == sx) {
                return Some(*h);
            }
        }
        let wt = (0.35 * want).max(eps * 3.0);
        hits.iter().filter(|h| (h.1 - want).abs() <= wt).min_by(|a, b| (a.1 - want).abs().total_cmp(&(b.1 - want).abs())).copied().or_else(|| hits.first().copied())
    };
    let (fj, dj) = pick(ray_hits(faces, j, n, eps, max, tol), None)?;
    let (fk, dk) = pick(ray_hits(faces, k, n, eps, max, tol), Some(fj))?;
    Some((dj, fj, dk, fk))
}

/// Where the "width over height" label goes: the base side of the opening
/// (on the outer face), the direction out of the building, and whether a
/// building outline was found on that side.
pub fn label_side(j: P2, k: P2, j2: P2, k2: P2, n: P2, outline: Option<&Ring>, m: f64) -> (P2, P2, P2) {
    let near_out = add(mid(j, k), mul(n, -0.05 / m));
    let far_out = add(mid(j2, k2), mul(n, 0.05 / m));
    match outline {
        Some(o) if !o.contains(near_out) => (j, k, mul(n, -1.0)),
        Some(o) if !o.contains(far_out) => (j2, k2, n),
        _ => (j, k, mul(n, -1.0)),
    }
}

/// A value in centimetres, as written in the label: whole when it is.
pub fn cm_text(v: f64, m: f64) -> String {
    let cm = v * m * 100.0;
    if (cm - cm.round()).abs() < 0.05 {
        format!("{}", cm.round() as i64)
    } else {
        commands::fmt_num(cm, 1)
    }
}

// ── Remembered between uses ─────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HoleKind {
    Door,
    Window,
}

#[derive(Clone, Copy, Debug)]
struct Last {
    kind: HoleKind,
    w: f64,
    h: f64,
    d: f64,
}

static LAST: Mutex<Option<Last>> = Mutex::new(None);
static RESUME: Mutex<Option<HoleTool>> = Mutex::new(None);

fn take_resume() -> Option<HoleTool> {
    RESUME.lock().unwrap_or_else(|e| e.into_inner()).take()
}

fn hand_over(tool: HoleTool, verb: &str) -> CommandStep {
    *RESUME.lock().unwrap_or_else(|e| e.into_inner()) = Some(tool);
    tools::deferred(verb, String::new())
}

// ── The interactive tool ────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq)]
enum Phase {
    /// Door or window, width, height.
    Ask,
    /// A notice: now click the corner.
    CornerInfo,
    /// Click the corner the distance is measured from.
    Corner,
    /// A notice: now click the side.
    SideInfo,
    /// Click the side of that corner the opening is on.
    Side,
    /// Distance from the corner and wall thickness.
    Dist,
}

/// The steps of the "Doors and windows" button. A click hands the tool to a
/// command (the host shows a tool's window only when it starts or after an
/// answer, never after a click), which starts it again with its window.
#[derive(Clone)]
pub struct HoleTool {
    rings: Vec<Ring>,
    /// Metres per drawing unit.
    m: f64,
    phase: Phase,
    kind: HoleKind,
    w: f64,
    h: f64,
    d: f64,
    t: f64,
    /// The thickness found on the drawing for the chosen side.
    found: Option<f64>,
    corner: Option<(usize, usize)>,
    toward: Option<usize>,
    /// Into the wall, from the clicked side (found when the side is picked).
    n: Option<P2>,
    /// Where the corner was clicked: the host can report one click twice
    /// (as an object pick and as a point), which must not choose the side.
    corner_click: Option<P2>,
    msg: Option<String>,
}

impl HoleTool {
    pub fn new(rings: Vec<Ring>, m: f64) -> Self {
        let last = LAST.lock().unwrap_or_else(|e| e.into_inner()).unwrap_or(Last { kind: HoleKind::Window, w: 1.2 / m, h: 1.4 / m, d: 1.0 / m });
        HoleTool { rings, m, phase: Phase::Ask, kind: last.kind, w: last.w, h: last.h, d: last.d, t: 0.3 / m, found: None, corner: None, toward: None, n: None, corner_click: None, msg: None }
    }

    /// Back to the corner step after an opening that could not be placed,
    /// on the drawing as it is now.
    fn again(mut self, rings: Vec<Ring>, msg: Option<String>) -> Self {
        self.rings = rings;
        self.phase = Phase::Corner;
        self.corner = None;
        self.toward = None;
        self.found = None;
        self.n = None;
        self.msg = msg;
        self
    }

    fn nearest_corner(&self, p: P2) -> Option<(usize, usize)> {
        let mut best: Option<((usize, usize), f64)> = None;
        for (r, ring) in self.rings.iter().enumerate() {
            for (i, q) in ring.pts.iter().enumerate() {
                let d = solve::dist(*q, p);
                if best.map_or(true, |(_, b)| d < b) {
                    best = Some(((r, i), d));
                }
            }
        }
        best.map(|(c, _)| c)
    }

    fn nearest_side(&self, p: P2) -> Option<usize> {
        let (r, i) = self.corner?;
        let ring = &self.rings[r];
        let c = ring.pts[i];
        ring.neighbours(i).into_iter().min_by(|&a, &b| {
            let da = solve::dist(p, tools::seg_foot(p, c, ring.pts[a]));
            let db = solve::dist(p, tools::seg_foot(p, c, ring.pts[b]));
            da.total_cmp(&db)
        })
    }

    fn cross_at(&self, p: P2, z: f64) -> Vec<Vec<[f64; 3]>> {
        let s = 0.12 / self.m;
        vec![vec![[p[0] - s, p[1] - s, z], [p[0] + s, p[1] + s, z]], vec![[p[0] - s, p[1] + s, z], [p[0] + s, p[1] - s, z]]]
    }

    fn kind_name(&self) -> &'static str {
        match self.kind {
            HoleKind::Door => tr!("porta", "door", "puerta", "porte", "kapı", "porta", "drzwi"),
            HoleKind::Window => tr!("finestra", "window", "ventana", "fenêtre", "pencere", "janela", "okno"),
        }
    }
}

fn unit_name(m: f64) -> &'static str {
    if (m - 0.01).abs() < 1e-12 {
        "cm"
    } else if (m - 0.001).abs() < 1e-12 {
        "mm"
    } else {
        "m"
    }
}

impl InteractiveCommand for HoleTool {
    fn prompt(&self) -> String {
        let p = match self.phase {
            Phase::Ask | Phase::Dist | Phase::CornerInfo | Phase::SideInfo => tr!(
                "Porte e finestre: rispondi nella finestra",
                "Doors and windows: answer in the window",
                "Puertas y ventanas: responda en la ventana",
                "Portes et fenêtres : répondez dans la fenêtre",
                "Kapılar ve pencereler: pencerede yanıtlayın",
                "Portas e janelas: responda na janela",
                "Drzwi i okna: odpowiedz w oknie",
            )
            .to_string(),
            Phase::Corner => {
                let k = self.kind_name();
                let (w, h) = (commands::fmt_num(self.w, 2), commands::fmt_num(self.h, 2));
                tr!(
                    format!("Porte e finestre ({k} {w} × {h}): clicca lo spigolo da cui misuri la distanza, sul perimetro esterno o su un vano. Invio per finire"),
                    format!("Doors and windows ({k} {w} × {h}): click the corner you measure the distance from, on the outer perimeter or on a room. Enter to finish"),
                    format!("Puertas y ventanas ({k} {w} × {h}): haga clic en la esquina desde la que mide la distancia, en el perímetro exterior o en una estancia. Intro para terminar"),
                    format!("Portes et fenêtres ({k} {w} × {h}) : cliquez sur le coin depuis lequel vous mesurez la distance, sur le périmètre extérieur ou sur une pièce. Entrée pour terminer"),
                    format!("Kapılar ve pencereler ({k} {w} × {h}): uzaklığı ölçtüğünüz köşeye tıklayın, dış çevrede veya bir odada. Bitirmek için Enter"),
                    format!("Portas e janelas ({k} {w} × {h}): clique no canto a partir do qual mede a distância, no perímetro exterior ou num compartimento. Enter para terminar"),
                    format!("Drzwi i okna ({k} {w} × {h}): kliknij narożnik, od którego mierzysz odległość, na obrysie zewnętrznym lub w pomieszczeniu. Enter, aby zakończyć"),
                )
            }
            Phase::Side => tr!(
                "Porte e finestre: clicca il lato dello spigolo su cui sta il foro (in verde quello più vicino al cursore). Invio per finire",
                "Doors and windows: click the side of the corner the opening is on (the one nearest the cursor in green). Enter to finish",
                "Puertas y ventanas: haga clic en el lado de la esquina donde está el hueco (en verde el más cercano al cursor). Intro para terminar",
                "Portes et fenêtres : cliquez sur le côté du coin où se trouve l'ouverture (en vert le plus proche du curseur). Entrée pour terminer",
                "Kapılar ve pencereler: açıklığın bulunduğu köşe kenarına tıklayın (imlece en yakın olan yeşil). Bitirmek için Enter",
                "Portas e janelas: clique no lado do canto onde fica o vão (a verde o mais próximo do cursor). Enter para terminar",
                "Drzwi i okna: kliknij bok narożnika, na którym jest otwór (najbliższy kursorowi na zielono). Enter, aby zakończyć",
            )
            .to_string(),
        };
        match &self.msg {
            Some(m) => format!("{m}  |  {p}"),
            None => p,
        }
    }

    fn needs_object_pick(&self) -> bool {
        matches!(self.phase, Phase::Corner | Phase::Side | Phase::CornerInfo | Phase::SideInfo)
    }

    fn on_object_pick(&mut self, _handle: Handle, pt: [f64; 3]) -> CommandStep {
        self.on_point(pt)
    }

    fn on_point(&mut self, pt: [f64; 3]) -> CommandStep {
        let p = [pt[0], pt[1]];
        // A notice that did not show must not hold the clicks.
        if self.phase == Phase::CornerInfo {
            self.phase = Phase::Corner;
        } else if self.phase == Phase::SideInfo {
            self.phase = Phase::Side;
        }
        match self.phase {
            Phase::Corner => {
                match self.nearest_corner(p) {
                    // A notice for the side: a click shows no window, so the
                    // tool goes through a command that starts it again.
                    Some(c) => {
                        let mut next = self.clone();
                        next.corner = Some(c);
                        next.corner_click = Some(p);
                        next.phase = Phase::SideInfo;
                        next.msg = None;
                        *self = next.clone();
                        hand_over(next, "RV_FOROSPIGOLO")
                    }
                    None => CommandStep::NeedPoint,
                }
            }
            Phase::Side if self.corner_click.is_some_and(|q| solve::dist(q, p) < 1e-9) => CommandStep::NeedPoint,
            Phase::Side => match self.nearest_side(p) {
                Some(s) => {
                    let mut next = self.clone();
                    next.toward = Some(s);
                    next.phase = Phase::Dist;
                    next.msg = None;
                    hand_over(next, "RV_FOROLATO")
                }
                None => CommandStep::NeedPoint,
            },
            _ => CommandStep::NeedPoint,
        }
    }

    fn on_cursor_move(&mut self, pt: [f64; 3]) -> Vec<PreviewWire> {
        let p = [pt[0], pt[1]];
        let mut w = Vec::new();
        match self.phase {
            Phase::Corner => {
                if let Some((r, i)) = self.nearest_corner(p) {
                    let ring = &self.rings[r];
                    let mut pts: Vec<[f64; 3]> = ring.pts.iter().map(|q| [q[0], q[1], ring.z]).collect();
                    if ring.closed {
                        pts.push(pts[0]);
                    }
                    w.push(PreviewWire::with_color(pts, ORANGE));
                    for c in self.cross_at(ring.pts[i], ring.z) {
                        w.push(PreviewWire::with_color(c, GREEN));
                    }
                }
            }
            Phase::Side | Phase::SideInfo => {
                if let Some((r, i)) = self.corner {
                    let ring = &self.rings[r];
                    let c = ring.pts[i];
                    let hot = self.nearest_side(p);
                    for s in ring.neighbours(i) {
                        let q = ring.pts[s];
                        let col = if Some(s) == hot { GREEN } else { ORANGE };
                        w.push(PreviewWire::with_color(vec![[c[0], c[1], ring.z], [q[0], q[1], ring.z]], col));
                    }
                    for x in self.cross_at(c, ring.z) {
                        w.push(PreviewWire::with_color(x, GREEN));
                    }
                }
            }
            _ => {}
        }
        w
    }

    fn modal_dialog(&self) -> Option<DialogDef> {
        let u = unit_name(self.m);
        match self.phase {
            Phase::Ask => Some({
                let mut def = DialogDef::new(tr!("Porte e finestre", "Doors and windows", "Puertas y ventanas", "Portes et fenêtres", "Kapılar ve pencereler", "Portas e janelas", "Drzwi i okna"));
                if let Some(m) = &self.msg {
                    def = def.with_control(tools::info(format!("{m}\n\n{}", tr!(
                        "Per un'altra porta o finestra correggi i dati e premi Avanti; Annulla per finire.",
                        "For another door or window correct the data and press Next; Cancel to finish.",
                        "Para otra puerta o ventana corrija los datos y pulse Siguiente; Cancelar para terminar.",
                        "Pour une autre porte ou fenêtre corrigez les données et appuyez sur Suivant ; Annuler pour terminer.",
                        "Başka bir kapı veya pencere için verileri düzeltip İleri'ye basın; bitirmek için İptal.",
                        "Para outra porta ou janela corrija os dados e prima Seguinte; Cancelar para terminar.",
                        "Aby dodać kolejne drzwi lub okno, popraw dane i naciśnij Dalej; Anuluj, aby zakończyć.",
                    ))));
                }
                def
                    .with_control(tools::info(tr!(
                        "Scegli il tipo e scrivi larghezza e altezza del foro. Poi sul disegno clicca lo spigolo da cui misuri la distanza (perimetro esterno o vano), il lato su cui sta il foro, e scrivi la distanza. Il muro viene interrotto e la quota larghezza/altezza va fuori dal perimetro dell'edificio. Solo disegno: un vano interrotto non entra più nel calcolo.",
                        "Choose the kind and type the opening's width and height. Then on the drawing click the corner you measure the distance from (outer perimeter or room), the side the opening is on, and type the distance. The wall is cut and the width/height label goes outside the building's perimeter. Drawing only: an opened room no longer takes part in the calculation.",
                        "Elija el tipo y escriba ancho y alto del hueco. Luego, en el dibujo, haga clic en la esquina desde la que mide la distancia (perímetro exterior o estancia), en el lado donde está el hueco, y escriba la distancia. El muro se interrumpe y la cota ancho/alto va fuera del perímetro del edificio. Solo dibujo: una estancia interrumpida ya no entra en el cálculo.",
                        "Choisissez le type et saisissez largeur et hauteur de l'ouverture. Puis sur le dessin cliquez sur le coin depuis lequel vous mesurez la distance (périmètre extérieur ou pièce), sur le côté où se trouve l'ouverture, et saisissez la distance. Le mur est interrompu et la cote largeur/hauteur va à l'extérieur du périmètre du bâtiment. Dessin seul : une pièce interrompue n'entre plus dans le calcul.",
                        "Türü seçin, açıklığın genişlik ve yüksekliğini yazın. Sonra çizimde uzaklığı ölçtüğünüz köşeye (dış çevre veya oda), açıklığın bulunduğu kenara tıklayın ve uzaklığı yazın. Duvar kesilir ve genişlik/yükseklik ölçüsü binanın çevresinin dışına konur. Yalnızca çizim: kesilen oda artık hesaba girmez.",
                        "Escolha o tipo e escreva a largura e a altura do vão. Depois, no desenho, clique no canto a partir do qual mede a distância (perímetro exterior ou compartimento), no lado onde fica o vão, e escreva a distância. A parede é interrompida e a cota largura/altura vai para fora do perímetro do edifício. Só desenho: um compartimento interrompido já não entra no cálculo.",
                        "Wybierz rodzaj i wpisz szerokość i wysokość otworu. Potem na rysunku kliknij narożnik, od którego mierzysz odległość (obrys zewnętrzny lub pomieszczenie), bok, na którym jest otwór, i wpisz odległość. Ściana zostaje przerwana, a wymiar szerokość/wysokość trafia na zewnątrz obrysu budynku. Tylko rysunek: przerwane pomieszczenie nie wchodzi już do obliczeń.",
                    ).to_string()))
                    .with_control(DialogControl::Choice {
                        id: "kind".into(),
                        label: tr!("Tipo", "Kind", "Tipo", "Type", "Tür", "Tipo", "Rodzaj").into(),
                        options: vec![
                            tr!("Finestra", "Window", "Ventana", "Fenêtre", "Pencere", "Janela", "Okno").into(),
                            tr!("Porta", "Door", "Puerta", "Porte", "Kapı", "Porta", "Drzwi").into(),
                        ],
                        default_index: if self.kind == HoleKind::Door { 1 } else { 0 },
                    })
                    .with_control(tools::number("w", &format!("{} ({u})", tr!("Larghezza", "Width", "Ancho", "Largeur", "Genişlik", "Largura", "Szerokość")), self.w))
                    .with_control(tools::number("h", &format!("{} ({u})", tr!("Altezza", "Height", "Alto", "Hauteur", "Yükseklik", "Altura", "Wysokość")), self.h))
                    .submit_label(tr!("Avanti ▸", "Next ▸", "Siguiente ▸", "Suivant ▸", "İleri ▸", "Seguinte ▸", "Dalej ▸"))
                    .cancel_label(tr!("Annulla", "Cancel", "Cancelar", "Annuler", "İptal", "Cancelar", "Anuluj"))
                    .width(520.0)
            }),
            Phase::CornerInfo => Some(
                DialogDef::new(tr!("Porte e finestre — spigolo", "Doors and windows — corner", "Puertas y ventanas — esquina", "Portes et fenêtres — coin", "Kapılar ve pencereler — köşe", "Portas e janelas — canto", "Drzwi i okna — narożnik"))
                    .with_control(tools::info(tr!(
                        "Ora clicca nel disegno lo spigolo da cui misuri la distanza: un vertice del perimetro esterno o di un vano. Lo spigolo più vicino al cursore è segnato con una crocetta verde.",
                        "Now click on the drawing the corner you measure the distance from: a vertex of the outer perimeter or of a room. The corner nearest the cursor is marked with a green cross.",
                        "Ahora haga clic en el dibujo en la esquina desde la que mide la distancia: un vértice del perímetro exterior o de una estancia. La esquina más cercana al cursor se marca con una cruz verde.",
                        "Cliquez maintenant sur le dessin le coin depuis lequel vous mesurez la distance : un sommet du périmètre extérieur ou d'une pièce. Le coin le plus proche du curseur est marqué d'une croix verte.",
                        "Şimdi çizimde uzaklığı ölçtüğünüz köşeye tıklayın: dış çevrenin veya bir odanın bir köşesi. İmlece en yakın köşe yeşil bir çarpıyla işaretlenir.",
                        "Agora clique no desenho no canto a partir do qual mede a distância: um vértice do perímetro exterior ou de um compartimento. O canto mais próximo do cursor fica marcado com uma cruz verde.",
                        "Teraz kliknij na rysunku narożnik, od którego mierzysz odległość: wierzchołek obrysu zewnętrznego lub pomieszczenia. Narożnik najbliższy kursorowi jest oznaczony zielonym krzyżykiem.",
                    ).to_string()))
                    .submit_label("OK")
                    .cancel_label(tr!("Annulla", "Cancel", "Cancelar", "Annuler", "İptal", "Cancelar", "Anuluj"))
                    .width(460.0),
            ),
            Phase::SideInfo => {
                let k = self.kind_name();
                Some(
                    DialogDef::new(tr!("Porte e finestre — lato", "Doors and windows — side", "Puertas y ventanas — lado", "Portes et fenêtres — côté", "Kapılar ve pencereler — kenar", "Portas e janelas — lado", "Drzwi i okna — bok"))
                        .with_control(tools::info(tr!(
                            format!("Spigolo scelto. Ora clicca il tratto della polilinea su cui va la {k}: uno dei due lati che partono dallo spigolo (quello più vicino al cursore è verde)."),
                            format!("Corner chosen. Now click the stretch of the polyline the {k} goes on: one of the two sides starting from the corner (the one nearest the cursor is green)."),
                            format!("Esquina elegida. Ahora haga clic en el tramo de la polilínea donde va la {k}: uno de los dos lados que parten de la esquina (el más cercano al cursor está en verde)."),
                            format!("Coin choisi. Cliquez maintenant le tronçon de la polyligne où va la {k} : l'un des deux côtés partant du coin (le plus proche du curseur est en vert)."),
                            format!("Köşe seçildi. Şimdi {k} için çoklu çizginin parçasına tıklayın: köşeden çıkan iki kenardan biri (imlece en yakın olan yeşil)."),
                            format!("Canto escolhido. Agora clique no troço da polilinha onde vai a {k}: um dos dois lados que partem do canto (o mais próximo do cursor está a verde)."),
                            format!("Narożnik wybrany. Teraz kliknij odcinek polilinii, na którym ma być {k}: jeden z dwóch boków wychodzących z narożnika (najbliższy kursorowi jest zielony)."),
                        )))
                        .submit_label("OK")
                        .cancel_label(tr!("Annulla", "Cancel", "Cancelar", "Annuler", "İptal", "Cancelar", "Anuluj"))
                        .width(460.0),
                )
            }
            Phase::Dist => {
                let (r, i) = self.corner?;
                let s = self.toward?;
                let ring = &self.rings[r];
                let len = commands::fmt_num(solve::dist(ring.pts[i], ring.pts[s]), 2);
                let k = self.kind_name();
                let found = match self.found {
                    Some(t) => {
                        let t = commands::fmt_num(t, 2);
                        tr!(
                            format!("Spessore del muro trovato sul disegno: {t}."),
                            format!("Wall thickness found on the drawing: {t}."),
                            format!("Espesor del muro encontrado en el dibujo: {t}."),
                            format!("Épaisseur du mur trouvée sur le dessin : {t}."),
                            format!("Çizimde bulunan duvar kalınlığı: {t}."),
                            format!("Espessura da parede encontrada no desenho: {t}."),
                            format!("Grubość ściany znaleziona na rysunku: {t}."),
                        )
                    }
                    None => tr!(
                        "Nessuna linea parallela trovata dall'altra parte del muro: scrivi lo spessore (verrà interrotto solo il lato scelto).",
                        "No parallel line found on the other side of the wall: type the thickness (only the chosen side will be cut).",
                        "No se encontró ninguna línea paralela al otro lado del muro: escriba el espesor (solo se interrumpirá el lado elegido).",
                        "Aucune ligne parallèle trouvée de l'autre côté du mur : saisissez l'épaisseur (seul le côté choisi sera interrompu).",
                        "Duvarın diğer tarafında paralel çizgi bulunamadı: kalınlığı yazın (yalnızca seçilen kenar kesilecek).",
                        "Nenhuma linha paralela encontrada do outro lado da parede: escreva a espessura (só o lado escolhido será interrompido).",
                        "Nie znaleziono równoległej linii po drugiej stronie ściany: wpisz grubość (przerwany zostanie tylko wybrany bok).",
                    )
                    .to_string(),
                };
                Some(
                    DialogDef::new(tr!(
                        format!("Distanza dallo spigolo — {k}"),
                        format!("Distance from the corner — {k}"),
                        format!("Distancia desde la esquina — {k}"),
                        format!("Distance depuis le coin — {k}"),
                        format!("Köşeden uzaklık — {k}"),
                        format!("Distância a partir do canto — {k}"),
                        format!("Odległość od narożnika — {k}"),
                    ))
                    .with_control(tools::info(tr!(
                        format!("Lato lungo {len}. Scrivi la distanza dallo spigolo cliccato al bordo del foro (la mazzetta), misurata lungo questo lato."),
                        format!("Side {len} long. Type the distance from the clicked corner to the edge of the opening (the jamb), measured along this side."),
                        format!("Lado de {len}. Escriba la distancia desde la esquina pulsada hasta el borde del hueco (la jamba), medida a lo largo de este lado."),
                        format!("Côté long de {len}. Saisissez la distance du coin cliqué au bord de l'ouverture (le tableau), mesurée le long de ce côté."),
                        format!("Kenar uzunluğu {len}. Tıklanan köşeden açıklığın kenarına (söveye) olan uzaklığı, bu kenar boyunca ölçülmüş olarak yazın."),
                        format!("Lado com {len}. Escreva a distância do canto clicado até à borda do vão (a ombreira), medida ao longo deste lado."),
                        format!("Bok o długości {len}. Wpisz odległość od klikniętego narożnika do krawędzi otworu (ościeża), mierzoną wzdłuż tego boku."),
                    )))
                    .with_control(tools::number("d", &format!("{} ({u})", tr!("Distanza dallo spigolo", "Distance from the corner", "Distancia desde la esquina", "Distance depuis le coin", "Köşeden uzaklık", "Distância a partir do canto", "Odległość od narożnika")), self.d))
                    .with_control(tools::info(found))
                    .with_control(tools::number("t", &format!("{} ({u})", tr!("Spessore del muro", "Wall thickness", "Espesor del muro", "Épaisseur du mur", "Duvar kalınlığı", "Espessura da parede", "Grubość ściany")), self.found.unwrap_or(self.t)))
                    .submit_label(tr!("Inserisci", "Insert", "Insertar", "Insérer", "Ekle", "Inserir", "Wstaw"))
                    .cancel_label(tr!("Annulla", "Cancel", "Cancelar", "Annuler", "İptal", "Cancelar", "Anuluj"))
                    .width(520.0),
                )
            }
            _ => None,
        }
    }

    fn on_dialog_response(&mut self, response: DialogResponse) -> CommandStep {
        match (self.phase, response) {
            (Phase::Ask, DialogResponse::Submitted(m)) => {
                self.kind = if tools::get_choice(&m, "kind", 0) == 1 { HoleKind::Door } else { HoleKind::Window };
                if let Some(w) = tools::get_num(&m, "w").filter(|v| *v > 0.0) {
                    self.w = w;
                }
                if let Some(h) = tools::get_num(&m, "h").filter(|v| *v > 0.0) {
                    self.h = h;
                }
                self.phase = Phase::CornerInfo;
                self.msg = None;
                // The host shows a tool's window when the tool starts: start
                // it again for the notice.
                hand_over(self.clone(), "RV_FOROSPIGOLO")
            }
            (Phase::Ask, _) => CommandStep::Cancel,
            (Phase::CornerInfo, DialogResponse::Submitted(_)) => {
                self.phase = Phase::Corner;
                CommandStep::NeedPoint
            }
            (Phase::SideInfo, DialogResponse::Submitted(_)) => {
                self.phase = Phase::Side;
                CommandStep::NeedPoint
            }
            (Phase::CornerInfo | Phase::SideInfo, _) => CommandStep::Cancel,
            (Phase::Dist, DialogResponse::Submitted(m)) => {
                if let Some(d) = tools::get_num(&m, "d").filter(|v| *v >= 0.0) {
                    self.d = d;
                }
                if let Some(t) = tools::get_num(&m, "t").filter(|v| *v > 0.0) {
                    self.t = t;
                }
                *LAST.lock().unwrap_or_else(|e| e.into_inner()) = Some(Last { kind: self.kind, w: self.w, h: self.h, d: self.d });
                hand_over(self.clone(), "RV_FOROAPPLY")
            }
            (Phase::Dist, _) => {
                self.phase = Phase::Corner;
                self.corner = None;
                self.toward = None;
                CommandStep::NeedPoint
            }
            _ => CommandStep::NeedPoint,
        }
    }

    fn on_enter(&mut self) -> CommandStep {
        match self.phase {
            Phase::Ask | Phase::Dist | Phase::CornerInfo | Phase::SideInfo => CommandStep::NeedPoint,
            _ => CommandStep::Done,
        }
    }
}

// ── Host side ───────────────────────────────────────────────────────────────

fn excluded(layer: &str) -> bool {
    [LAYER_ROOM, LAYER_NOTES, LAYER_OPENINGS, LAYER_LABELS].iter().any(|l| layer.eq_ignore_ascii_case(l))
}

fn verts_of(p: &codec::entities::LwPolyline) -> Vec<(P2, f64)> {
    let mut v: Vec<(P2, f64)> = p.vertices.iter().map(|x| ([x.location.x, x.location.y], x.bulge)).collect();
    v.dedup_by(|b, a| solve::dist(a.0, b.0) < 1e-9);
    v
}

/// Whether a polyline closes: flagged, or its last vertex on its first.
fn closed_verts(p: &codec::entities::LwPolyline) -> (Vec<(P2, f64)>, bool) {
    let mut v = verts_of(p);
    let mut closed = p.is_closed;
    if v.len() > 3 && solve::dist(v[0].0, v[v.len() - 1].0) < 1e-9 {
        v.pop();
        closed = true;
    }
    (v, closed)
}

/// The polylines whose corners can be picked: three corners or more, on the
/// drawing's own layers.
pub fn rings(host: &dyn HostApi) -> Vec<Ring> {
    host.document()
        .entities()
        .filter(|e| !excluded(&e.common().layer))
        .filter_map(|e| {
            let EntityType::LwPolyline(p) = e else { return None };
            if p.normal.z < 0.999_999 {
                return None;
            }
            let (v, closed) = closed_verts(p);
            (v.len() >= 3).then(|| Ring { handle: e.common().handle, pts: v.into_iter().map(|x| x.0).collect(), closed, z: p.elevation })
        })
        .collect()
}

/// Every straight piece of line that can be a wall face.
fn faces(host: &dyn HostApi) -> Vec<Face> {
    let mut out = Vec::new();
    for e in host.document().entities() {
        if excluded(&e.common().layer) {
            continue;
        }
        let h = e.common().handle;
        match e {
            EntityType::LwPolyline(p) if p.normal.z >= 0.999_999 => {
                let (v, closed) = closed_verts(p);
                let n = v.len();
                let segs = if closed { n } else { n.saturating_sub(1) };
                for s in 0..segs {
                    if v[s].1.abs() < 1e-9 {
                        out.push(Face { handle: h, seg: s, a: v[s].0, b: v[(s + 1) % n].0 });
                    }
                }
            }
            EntityType::Line(l) => out.push(Face { handle: h, seg: 0, a: [l.start.x, l.start.y], b: [l.end.x, l.end.y] }),
            _ => {}
        }
    }
    out
}

fn live(host: &dyn HostApi, h: Handle) -> Option<EntityType> {
    match host.entities_batch(&[h]).ok().and_then(|v| v.into_iter().next().flatten()) {
        Some(e) => Some(e),
        None => host.document().get_entity(h).cloned(),
    }
}

/// Metres per drawing unit, from the size of the drawing.
pub fn metres_per_unit(rings: &[Ring]) -> f64 {
    let longest = rings
        .iter()
        .flat_map(|r| (0..r.pts.len()).map(move |i| solve::dist(r.pts[i], r.pts[(i + 1) % r.pts.len()])))
        .fold(0.0f64, f64::max);
    survey::unit_factor(survey::guess_unit(longest))
}

fn no_polylines() -> String {
    tr!(
        "Porte e finestre: il disegno non ha polilinee di vani o del perimetro.",
        "Doors and windows: the drawing has no room or perimeter polylines.",
        "Puertas y ventanas: el dibujo no tiene polilíneas de estancias ni del perímetro.",
        "Portes et fenêtres : le dessin n'a pas de polylignes de pièces ni du périmètre.",
        "Kapılar ve pencereler: çizimde oda veya çevre çoklu çizgisi yok.",
        "Portas e janelas: o desenho não tem polilinhas de compartimentos nem do perímetro.",
        "Drzwi i okna: rysunek nie ma polilinii pomieszczeń ani obrysu.",
    )
    .to_string()
}

/// `RV_FORO`: start the tool.
pub fn start(host: &mut dyn HostApi) {
    let rs = rings(host);
    if rs.is_empty() {
        host.push_error(&no_polylines());
        return;
    }
    let m = metres_per_unit(&rs);
    host.start_interactive(Box::new(HoleTool::new(rs, m)));
}

/// The chosen side: corner, other end, the wall's inward direction, and
/// whether the clicked polyline is the building's outline.
fn side_of(tool: &HoleTool) -> Option<(P2, P2)> {
    let (r, i) = tool.corner?;
    let s = tool.toward?;
    let ring = tool.rings.get(r)?;
    Some((ring.pts[i], ring.pts[s]))
}

/// The building's outline: the fixed perimeter when chosen, otherwise the
/// largest polyline.
pub fn outline(rings: &[Ring], perimeter: Option<Handle>) -> Option<&Ring> {
    rings
        .iter()
        .find(|r| Some(r.handle) == perimeter)
        .or_else(|| rings.iter().max_by(|a, b| a.area().total_cmp(&b.area())))
}

/// Which way the wall lies from side c–d of `ring`: the side where the
/// other face of the wall is drawn. When both sides (or neither) have one,
/// into the building from its outline and out of a room otherwise.
pub fn wall_direction(ring: &Ring, rings: &[Ring], faces: &[Face], c: P2, d: P2, perimeter: Option<Handle>, m: f64) -> P2 {
    let inw = inward(ring, c, d);
    let out = mul(inw, -1.0);
    let found = |n: P2| ray_hits(faces, mid(c, d), n, 0.01 / m, 1.2 / m, 0.002 / m).first().map(|h| h.1);
    match (found(inw), found(out)) {
        (Some(_), None) => inw,
        (None, Some(_)) => out,
        _ => {
            let outer = is_outer(ring, rings, perimeter) || outline(rings, perimeter).is_some_and(|o| o.handle == ring.handle);
            if outer { inw } else { out }
        }
    }
}

fn perimeter_handle() -> Option<Handle> {
    commands::perimeter().and_then(|(h, _)| commands::parse_handle(&h))
}

/// `RV_FOROSPIGOLO`: start the tool again with its next notice (before the
/// corner, after the corner).
pub fn corner_picked(host: &mut dyn HostApi) {
    if let Some(tool) = take_resume() {
        host.start_interactive(Box::new(tool));
    }
}

/// `RV_FOROLATO`: a side was clicked; find the wall's thickness on the
/// drawing and ask the distance.
pub fn side_picked(host: &mut dyn HostApi) {
    let Some(mut tool) = take_resume() else { return };
    if let (Some((c, d)), Some((r, _))) = (side_of(&tool), tool.corner) {
        let fs = faces(host);
        let m = tool.m;
        let n = wall_direction(&tool.rings[r], &tool.rings, &fs, c, d, perimeter_handle(), m);
        tool.n = Some(n);
        tool.found = ray_hits(&fs, mid(c, d), n, 0.01 / m, 1.2 / m, 0.002 / m).first().map(|h| h.1);
    }
    host.start_interactive(Box::new(tool));
}

/// Open the line `h` between p and q (both on one of its straight
/// segments). Returns whether it was done.
fn open_line(host: &mut dyn HostApi, h: Handle, p: P2, q: P2, tol: f64) -> bool {
    let Some(e) = live(host, h) else { return false };
    match e {
        EntityType::LwPolyline(pl) => {
            let (v, closed) = closed_verts(&pl);
            let Some(s) = segment_holding(&v, closed, p, q, tol) else { return false };
            let pieces = cut(&v, closed, s, p, q);
            let mut it = pieces.into_iter();
            let Some(first) = it.next() else {
                return host.remove_entity(h);
            };
            let mut a = pl.clone();
            a.vertices = first.iter().map(|(pt, b)| {
                let mut x = LwVertex::new(Vector2::new(pt[0], pt[1]));
                x.bulge = *b;
                x
            }).collect();
            a.is_closed = false;
            let base = a.clone();
            host.update_entity(EntityType::LwPolyline(a));
            for rest in it {
                let mut b = base.clone();
                b.common.handle = Handle::NULL;
                b.vertices = rest.iter().map(|(pt, bu)| {
                    let mut x = LwVertex::new(Vector2::new(pt[0], pt[1]));
                    x.bulge = *bu;
                    x
                }).collect();
                host.add_entity(EntityType::LwPolyline(b));
            }
            true
        }
        EntityType::Line(l) => {
            let (a, b) = ([l.start.x, l.start.y], [l.end.x, l.end.y]);
            let u = unit(a, b);
            let (p1, p2) = if dot(sub(p, a), u) <= dot(sub(q, a), u) { (p, q) } else { (q, p) };
            let mut first = l.clone();
            first.end = Vector3::new(p1[0], p1[1], l.end.z);
            let mut second = l.clone();
            second.common.handle = Handle::NULL;
            second.start = Vector3::new(p2[0], p2[1], l.start.z);
            let keep1 = solve::dist(a, p1) > tol;
            let keep2 = solve::dist(p2, b) > tol;
            match (keep1, keep2) {
                (true, true) => {
                    host.update_entity(EntityType::Line(first));
                    host.add_entity(EntityType::Line(second));
                }
                (true, false) => {
                    host.update_entity(EntityType::Line(first));
                }
                (false, true) => {
                    let mut s = second;
                    s.common.handle = h;
                    host.update_entity(EntityType::Line(s));
                }
                (false, false) => {
                    host.remove_entity(h);
                }
            }
            true
        }
        _ => false,
    }
}

/// Empty the hole in the hatches that fill the wall there: the hole's
/// outline becomes one more loop (an island) of each hatch around it.
fn hole_in_hatches(host: &mut dyn HostApi, corners: &[P2; 4]) -> usize {
    let centre = mul(corners.iter().fold([0.0, 0.0], |s, p| add(s, *p)), 0.25);
    let hits: Vec<Handle> = host
        .document()
        .entities()
        .filter_map(|e| {
            let EntityType::Hatch(hatch) = e else { return None };
            let inside = commands::hatch_loops(hatch).iter().filter(|l| commands::point_in_polygon(l, centre)).count();
            (inside % 2 == 1).then(|| e.common().handle)
        })
        .collect();
    let mut done = 0;
    for h in hits {
        let Some(EntityType::Hatch(mut hatch)) = live(host, h) else { continue };
        let mut path = BoundaryPath::new();
        path.add_edge(BoundaryEdge::Polyline(PolylineEdge::new(corners.iter().map(|p| Vector2::new(p[0], p[1])).collect(), true)));
        hatch.paths.push(path);
        hatch.is_associative = false;
        if host.update_entity(EntityType::Hatch(hatch)) {
            done += 1;
        }
    }
    done
}

fn coloured(mut e: EntityType, aci: i16) -> EntityType {
    e.common_mut().color = codec::types::Color::from_index(aci);
    e
}

/// `RV_FOROAPPLY`: cut the wall and draw the opening; then the tool starts
/// again for the next one.
pub fn apply(host: &mut dyn HostApi) {
    let Some(tool) = take_resume() else { return };
    // Done: the next opening starts from the button again (a window opened
    // by the plugin on its own can end up out of sight and hold the command).
    match apply_inner(host, &tool) {
        Ok(m) => host.push_info(&m),
        Err(e) => {
            host.push_error(&e);
            let rs = rings(host);
            host.start_interactive(Box::new(tool.again(rs, Some(format!("⚠ {e}")))));
        }
    }
}

fn apply_inner(host: &mut dyn HostApi, tool: &HoleTool) -> Result<String, String> {
    let (c, d) = side_of(tool).ok_or_else(no_polylines)?;
    let (r, _) = tool.corner.ok_or_else(no_polylines)?;
    let ring = tool.rings[r].clone();
    let m = tool.m;
    let n = match tool.n {
        Some(n) => n,
        None => wall_direction(&ring, &tool.rings, &faces(host), c, d, perimeter_handle(), m),
    };
    let tol = 0.002 / m;
    let u = unit(c, d);
    let j = add(c, mul(u, tool.d));
    let k = add(c, mul(u, tool.d + tool.w));
    let fs = faces(host);
    // The opening must lie on one straight piece of the clicked line.
    let on_line = live(host, ring.handle).and_then(|e| match e {
        EntityType::LwPolyline(p) => {
            let (v, closed) = closed_verts(&p);
            segment_holding(&v, closed, j, k, tol)
        }
        _ => None,
    });
    if on_line.is_none() {
        return Err(tr!(
            "il foro non sta tutto su un tratto dritto del lato scelto (esce dal lato o cade su un foro già fatto): controlla distanza e larghezza",
            "the opening does not lie wholly on a straight part of the chosen side (it runs past the side or onto an earlier opening): check distance and width",
            "el hueco no cabe entero en un tramo recto del lado elegido (sale del lado o cae sobre un hueco ya hecho): revise distancia y ancho",
            "l'ouverture ne tient pas entièrement sur une partie droite du côté choisi (elle dépasse le côté ou tombe sur une ouverture déjà faite) : vérifiez distance et largeur",
            "açıklık seçilen kenarın düz bir parçasına tamamen sığmıyor (kenarı aşıyor veya önceki bir açıklığa denk geliyor): uzaklığı ve genişliği kontrol edin",
            "o vão não cabe inteiro num troço reto do lado escolhido (sai do lado ou cai sobre um vão já feito): verifique distância e largura",
            "otwór nie mieści się w całości na prostym odcinku wybranego boku (wychodzi poza bok lub trafia na wcześniejszy otwór): sprawdź odległość i szerokość",
        )
        .to_string());
    }
    // The other face of the wall, square to the hole's jambs or not.
    let far = far_face(&fs, j, k, n, tool.t, 0.01 / m, 1.2 / m, tol * 5.0);
    let (dj, dk) = far.map(|(a, _, b, _)| (a, b)).unwrap_or((tool.t, tool.t));
    let (j2, k2) = (add(j, mul(n, dj)), add(k, mul(n, dk)));
    let layer = live(host, ring.handle).map(|e| e.common().layer.clone()).unwrap_or_default();

    host.begin_undo_group("RV_FORO");
    // Every line on the clicked face and on the far face, where the opening is.
    let mut near = faces_holding(&fs, j, k, tol);
    if !near.contains(&ring.handle) {
        near.push(ring.handle);
    }
    for h in near {
        open_line(host, h, j, k, tol);
    }
    let mut cut_far = false;
    if let Some((_, fj, _, fk)) = far {
        if fs[fj].handle == fs[fk].handle {
            for h in faces_holding(&fs, j2, k2, tol * 5.0) {
                cut_far |= open_line(host, h, j2, k2, tol * 5.0);
            }
        }
    }
    // Jambs across the wall, as wall lines.
    let mut ents = vec![commands::pline(&[j, j2], ring.z, false, &layer), commands::pline(&[k, k2], ring.z, false, &layer)];
    hole_in_hatches(host, &[j, k, k2, j2]);
    commands::ensure_layer(host, LAYER_OPENINGS, 4);
    // Which face is the inside of the building: the window sits against it.
    let ol = outline(&tool.rings, perimeter_handle());
    let (bj, bk, e) = label_side(j, k, j2, k2, n, ol, m);
    let outside_is_near = ol.is_some_and(|o| !o.contains(add(mid(j, k), mul(n, -0.05 / m))));
    if tool.kind == HoleKind::Window {
        // Doors have no frame: just the hole and, outside, the bracket.
        for l in window_lines(j, k, j2, k2, outside_is_near || ol.is_none(), m) {
            ents.push(coloured(commands::pline(&l, ring.z, false, LAYER_OPENINGS), ACI_FRAME));
        }
    }
    // Width over height, outside the building.
    let th = (0.13 * tool.w).clamp(0.08 / m, 0.2 / m);
    let g = 0.35 * th;
    ents.push(coloured(commands::pline(&[bj, add(bj, mul(e, g)), add(bk, mul(e, g)), bk], ring.z, false, LAYER_OPENINGS), ACI_BRACKET));
    let (wt, ht) = (cm_text(tool.w, m), cm_text(tool.h, m));
    let len = wt.chars().count().max(ht.chars().count()) as f64 * 0.72 * th;
    let centre = add(mid(bj, bk), mul(e, g + 0.7 * th + len / 2.0));
    let ang = commands::readable_angle([0.0, 0.0], e);
    let along = [ang.cos(), ang.sin()];
    let up = left(along);
    let half = len / 2.0 + 0.15 * th;
    ents.push(coloured(commands::pline(&[sub(centre, mul(along, half)), add(centre, mul(along, half))], ring.z, false, LAYER_OPENINGS), ACI_RULE));
    for (txt, side) in [(wt, 1.0), (ht, -1.0)] {
        let mut x = commands::mtext(&txt, add(centre, mul(up, side * 0.68 * th)), ring.z, th, AttachmentPoint::MiddleCenter, LAYER_OPENINGS);
        if let EntityType::MText(mt) = &mut x {
            mt.rotation = ang;
        }
        ents.push(coloured(x, ACI_TEXT));
    }
    host.add_entities(ents);
    host.end_undo_group();
    host.set_dirty();

    let k_name = tool.kind_name();
    let (w, h) = (commands::fmt_num(tool.w, 2), commands::fmt_num(tool.h, 2));
    let mut out = tr!(
        format!("Porte e finestre: {k_name} {w} × {h} inserita (Ctrl+Z per annullare)."),
        format!("Doors and windows: {k_name} {w} × {h} inserted (Ctrl+Z to undo)."),
        format!("Puertas y ventanas: {k_name} {w} × {h} insertada (Ctrl+Z para deshacer)."),
        format!("Portes et fenêtres : {k_name} {w} × {h} insérée (Ctrl+Z pour annuler)."),
        format!("Kapılar ve pencereler: {k_name} {w} × {h} eklendi (geri almak için Ctrl+Z)."),
        format!("Portas e janelas: {k_name} {w} × {h} inserida (Ctrl+Z para anular)."),
        format!("Drzwi i okna: {k_name} {w} × {h} wstawione (Ctrl+Z, aby cofnąć)."),
    );
    if !cut_far {
        out.push(' ');
        out.push_str(tr!(
            "La linea dall'altra parte del muro non è stata trovata: interrotto solo il lato scelto.",
            "The line on the other side of the wall was not found: only the chosen side was cut.",
            "No se encontró la línea al otro lado del muro: solo se interrumpió el lado elegido.",
            "La ligne de l'autre côté du mur n'a pas été trouvée : seul le côté choisi a été interrompu.",
            "Duvarın diğer tarafındaki çizgi bulunamadı: yalnızca seçilen kenar kesildi.",
            "A linha do outro lado da parede não foi encontrada: só o lado escolhido foi interrompido.",
            "Nie znaleziono linii po drugiej stronie ściany: przerwano tylko wybrany bok.",
        ));
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ring(h: u64, pts: &[P2]) -> Ring {
        Ring { handle: Handle::new(h), pts: pts.to_vec(), closed: true, z: 0.0 }
    }

    #[test]
    fn cut_closed_and_open() {
        let sq: Vec<(P2, f64)> = vec![([0.0, 0.0], 0.0), ([4.0, 0.0], 0.0), ([4.0, 3.0], 0.0), ([0.0, 3.0], 0.0)];
        let s = segment_holding(&sq, true, [1.0, 0.0], [2.0, 0.0], 1e-6).unwrap();
        assert_eq!(s, 0);
        let p = cut(&sq, true, s, [2.0, 0.0], [1.0, 0.0]);
        assert_eq!(p.len(), 1);
        let pts: Vec<P2> = p[0].iter().map(|x| x.0).collect();
        assert_eq!(pts, vec![[2.0, 0.0], [4.0, 0.0], [4.0, 3.0], [0.0, 3.0], [0.0, 0.0], [1.0, 0.0]]);
        // The closing side.
        let s = segment_holding(&sq, true, [0.0, 2.0], [0.0, 1.0], 1e-6).unwrap();
        assert_eq!(s, 3);
        let pts: Vec<P2> = cut(&sq, true, s, [0.0, 2.0], [0.0, 1.0])[0].iter().map(|x| x.0).collect();
        assert_eq!(pts, vec![[0.0, 1.0], [0.0, 0.0], [4.0, 0.0], [4.0, 3.0], [0.0, 3.0], [0.0, 2.0]]);
        // An open polyline splits in two.
        let open: Vec<(P2, f64)> = pts.iter().map(|p| (*p, 0.0)).collect();
        let s = segment_holding(&open, false, [1.0, 0.0], [3.0, 0.0], 1e-6).unwrap();
        let parts = cut(&open, false, s, [1.0, 0.0], [3.0, 0.0]);
        assert_eq!(parts.len(), 2);
        assert_eq!(parts[0].last().unwrap().0, [1.0, 0.0]);
        assert_eq!(parts[1][0].0, [3.0, 0.0]);
        // Not on one straight piece: none.
        assert!(segment_holding(&sq, true, [3.0, 0.0], [5.0, 0.0], 1e-6).is_none());
    }

    #[test]
    fn opposite_face_and_sides() {
        // Outer perimeter 10×8 with a 0.3 thick wall: the room inside.
        let outer = ring(1, &[[0.0, 0.0], [10.0, 0.0], [10.0, 8.0], [0.0, 8.0]]);
        let room = ring(2, &[[0.3, 0.3], [9.7, 0.3], [9.7, 7.7], [0.3, 7.7]]);
        let rings = vec![outer.clone(), room.clone()];
        assert!(is_outer(&outer, &rings, None));
        assert!(!is_outer(&room, &rings, None));
        let n = inward(&outer, [0.0, 0.0], [10.0, 0.0]);
        assert!((n[1] - 1.0).abs() < 1e-9);
        let faces = vec![
            Face { handle: Handle::new(1), seg: 0, a: [0.0, 0.0], b: [10.0, 0.0] },
            Face { handle: Handle::new(2), seg: 0, a: [0.3, 0.3], b: [9.7, 0.3] },
            Face { handle: Handle::new(2), seg: 1, a: [9.7, 0.3], b: [9.7, 7.7] },
        ];
        let (x, t) = find_opposite(&faces, [2.0, 0.0], [3.0, 0.0], n, 0.01, 1.2, true, Some((0.3, 0.05))).unwrap();
        assert_eq!(x, 1);
        assert!((t - 0.3).abs() < 1e-9);
        // From the room, outwards.
        let nr = mul(inward(&room, [9.7, 0.3], [0.3, 0.3]), -1.0);
        assert!((nr[1] + 1.0).abs() < 1e-9);
        let (x, _) = find_opposite(&faces, [9.7, 0.3], [0.3, 0.3], nr, 0.01, 1.2, false, None).unwrap();
        assert_eq!(x, 0);
        // The label goes outside the building in both cases.
        let (bj, _, e) = label_side([2.0, 0.3], [3.0, 0.3], [2.0, 0.0], [3.0, 0.0], nr, Some(&outer), 1.0);
        assert!((bj[1]).abs() < 1e-9 && (e[1] + 1.0).abs() < 1e-9);
        let (bj, _, e) = label_side([2.0, 0.0], [3.0, 0.0], [2.0, 0.3], [3.0, 0.3], n, Some(&outer), 1.0);
        // A slanted outer face: the jambs reach it at their own lengths.
        let slant = vec![Face { handle: Handle::new(7), seg: 0, a: [0.0, -0.3], b: [10.0, -0.4] }];
        let (dj, _, dk, _) = far_face(&slant, [2.0, 0.0], [3.0, 0.0], [0.0, -1.0], 0.3, 0.01, 1.2, 0.002).unwrap();
        assert!((dj - 0.32).abs() < 1e-9 && (dk - 0.33).abs() < 1e-9, "{dj} {dk}");
        assert!(bj[1].abs() < 1e-9 && (e[1] + 1.0).abs() < 1e-9);
        // The wall's side: where its other face is, even with no room drawn.
        let only = vec![outer.clone()];
        let offs = vec![faces[0].clone(), Face { handle: Handle::new(9), seg: 0, a: [0.3, 0.3], b: [9.7, 0.3] }];
        let w = wall_direction(&outer, &only, &offs, [0.0, 0.0], [10.0, 0.0], None, 1.0);
        assert!((w[1] - 1.0).abs() < 1e-9, "{w:?}");
    }

    #[test]
    fn corners_and_labels() {
        let r = Ring { handle: Handle::new(1), pts: vec![[0.0, 0.0], [1.0, 0.0], [1.0, 1.0]], closed: false, z: 0.0 };
        assert_eq!(r.neighbours(0), vec![1]);
        assert_eq!(r.neighbours(1), vec![0, 2]);
        assert_eq!(r.neighbours(2), vec![1]);
        assert_eq!(cm_text(1.1, 1.0), "110");
        assert_eq!(cm_text(120.0, 0.01), "120");
        let w = window_lines([0.0, 0.0], [1.2, 0.0], [0.0, 0.3], [1.2, 0.3], true, 1.0);
        assert!(w.len() >= 6);
    }
}
