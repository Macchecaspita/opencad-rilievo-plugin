//! Rooms fitted to the walls: onto the inner face of the outer walls once
//! their thicknesses are given (button 2), and one against the other across
//! a partition (button "Partition thicknesses", `RV_TRAMEZZE`).
//!
//! The side of a room near the inner face of an outer wall (the line drawn
//! at the wall's thickness from the perimeter) and nearly parallel to it is
//! moved onto that line; its corners are found again where it meets the
//! sides next to it. For a partition, a side of the room to move is
//! clicked: the facing side of the room next door stays put and the clicked
//! side becomes parallel to it, at the partition's thickness.

use std::sync::Mutex;

use ocs_plugin_api::dialog::{DialogDef, DialogResponse};
use ocs_plugin_api::host::codec::{self, Handle};
use ocs_plugin_api::host::{CommandStep, HostApi, InteractiveCommand, PreviewWire};

use codec::entities::AttachmentPoint;
use codec::{EntityType, Vector2};

use crate::commands;
use crate::holes::{self, Ring};
use crate::solve::{self, P2};
use crate::tools;

/// Layer of the partitions given with their thickness: the gap between
/// the two rooms as a closed outline and the thickness written in it. The
/// measures window reads them back as walls between rooms.
pub const LAYER_PART: &str = "RILIEVO_TRAMEZZE";

const GREEN: [f32; 4] = [0.45, 0.85, 0.45, 1.0];
const ORANGE: [f32; 4] = [0.95, 0.55, 0.2, 1.0];

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

/// A straight line: a point on it and its unit direction.
pub type Line = (P2, P2);

fn meet(l1: Line, l2: Line) -> Option<P2> {
    let c = cross(l1.1, l2.1);
    // Under about 1,5°: as good as parallel.
    if c.abs() < 0.025 {
        return None;
    }
    let t = cross(sub(l2.0, l1.0), l2.1) / c;
    Some(add(l1.0, mul(l1.1, t)))
}

fn foot_on(l: Line, p: P2) -> P2 {
    add(l.0, mul(l.1, dot(sub(p, l.0), l.1)))
}

/// The unit normal of side i of the closed polygon `pts`, towards its inside.
pub fn inward_of(pts: &[P2], i: usize) -> P2 {
    let n = pts.len();
    let (a, b) = (pts[i], pts[(i + 1) % n]);
    let u = unit(a, b);
    let l = solve::dist(a, b);
    let probe = add(mid(a, b), mul(left(u), (l * 1e-3).max(1e-6)));
    if commands::point_in_polygon(pts, probe) { left(u) } else { mul(left(u), -1.0) }
}

/// The corners of `pts` once some sides lie on new lines (`moved[i]` for
/// side i, from corner i to corner i+1): each corner where its two sides
/// meet. A corner between two sides that are (nearly) parallel, or whose
/// meeting point would land farther than `max_move`, drops square onto the
/// moved side instead.
pub fn rebuild(pts: &[P2], moved: &[Option<Line>], max_move: f64) -> Vec<P2> {
    let n = pts.len();
    let side = |i: usize| moved[i].unwrap_or((pts[i], unit(pts[i], pts[(i + 1) % n])));
    (0..n)
        .map(|i| {
            let prev = (i + n - 1) % n;
            let Some(on) = moved[i].or(moved[prev]) else { return pts[i] };
            match meet(side(prev), side(i)) {
                Some(p) if solve::dist(p, pts[i]) <= max_move => p,
                _ => foot_on(on, pts[i]),
            }
        })
        .collect()
}

/// The inner face of an outer wall: the line along one side of the
/// perimeter at the wall's thickness, `nin` pointing into the building.
#[derive(Clone, Copy, Debug)]
pub struct Target {
    pub a: P2,
    pub b: P2,
    pub nin: P2,
    /// How far a room's side may be from the line and still go onto it.
    pub tol: f64,
}

/// Sides of the room `pts` that lie along an inner face (nearly parallel,
/// within its reach, side by side with it and with the room on the inside
/// of the building), moved onto it. `None` when no side moves.
pub fn snap_room(pts: &[P2], targets: &[Target], max_move: f64) -> Option<(Vec<P2>, usize)> {
    let n = pts.len();
    let sin_max = (8.0f64).to_radians().sin();
    let mut moved: Vec<Option<Line>> = vec![None; n];
    let mut count = 0;
    for i in 0..n {
        let (a, b) = (pts[i], pts[(i + 1) % n]);
        let l = solve::dist(a, b);
        if l < 1e-9 {
            continue;
        }
        let u = unit(a, b);
        let room_in = inward_of(pts, i);
        let mut best: Option<(f64, Line, f64)> = None;
        for t in targets {
            let lt = solve::dist(t.a, t.b);
            if lt < 1e-9 {
                continue;
            }
            let ut = unit(t.a, t.b);
            if cross(u, ut).abs() > sin_max || dot(room_in, t.nin) < 0.9 {
                continue;
            }
            let d = dot(sub(mid(a, b), t.a), t.nin);
            if d.abs() > t.tol {
                continue;
            }
            let (sa, sb) = (dot(sub(a, t.a), ut), dot(sub(b, t.a), ut));
            let overlap = sa.max(sb).min(lt) - sa.min(sb).max(0.0);
            if overlap < 0.25 * l.min(lt) {
                continue;
            }
            if best.map_or(true, |(bd, _, _)| d.abs() < bd) {
                best = Some((d.abs(), (t.a, ut), d));
            }
        }
        if let Some((_, line, d)) = best {
            moved[i] = Some(line);
            if d.abs() > 1e-9 || cross(u, line.1).abs() > 1e-9 {
                count += 1;
            }
        }
    }
    if count == 0 {
        return None;
    }
    Some((rebuild(pts, &moved, max_move), count))
}

/// The facing side of the room next door, across side i of `pts`: nearly
/// parallel, side by side with it, on the outer side of the room within
/// `reach`. Its ring, its ends and the gap between the two.
pub fn facing(pts: &[P2], i: usize, others: &[&Ring], reach: f64) -> Option<(Handle, P2, P2, f64)> {
    let n = pts.len();
    let (a, b) = (pts[i], pts[(i + 1) % n]);
    let l = solve::dist(a, b);
    if l < 1e-9 {
        return None;
    }
    let u = unit(a, b);
    let out = mul(inward_of(pts, i), -1.0);
    let sin_max = (15.0f64).to_radians().sin();
    let mut best: Option<(Handle, P2, P2, f64)> = None;
    for r in others {
        let m = r.pts.len();
        for s in 0..m {
            let (c, d) = (r.pts[s], r.pts[(s + 1) % m]);
            let lc = solve::dist(c, d);
            if lc < 1e-9 {
                continue;
            }
            let uc = unit(c, d);
            if cross(u, uc).abs() > sin_max {
                continue;
            }
            let (sc, sd) = (dot(sub(c, a), u), dot(sub(d, a), u));
            let overlap = sc.max(sd).min(l) - sc.min(sd).max(0.0);
            if overlap < 0.2 * l.min(lc) {
                continue;
            }
            // Measured square to the facing side, from the middle of the
            // stretch the two sides share.
            let m0 = add(a, mul(u, (sc.min(sd).max(0.0) + sc.max(sd).min(l)) / 2.0));
            let gap = dot(sub(foot_on((c, uc), m0), m0), out);
            if gap <= 1e-9 || gap > reach {
                continue;
            }
            if best.map_or(true, |x| gap < x.3) {
                best = Some((r.handle, c, d, gap));
            }
        }
    }
    best
}

/// Side i of `pts` moved parallel to the facing side c–d, `t` from it on
/// the room's own side.
pub fn against(pts: &[P2], i: usize, c: P2, d: P2, t: f64, max_move: f64) -> Vec<P2> {
    let out = mul(inward_of(pts, i), -1.0);
    let uc = unit(c, d);
    let mut nb = left(uc);
    if dot(nb, out) > 0.0 {
        nb = mul(nb, -1.0);
    }
    let mut moved = vec![None; pts.len()];
    moved[i] = Some((add(c, mul(nb, t)), uc));
    rebuild(pts, &moved, max_move)
}

/// Whether the reshaped room is still a sensible polygon: same turning
/// sense and not much smaller or larger.
pub fn sane(old: &[P2], new: &[P2]) -> bool {
    let (a0, a1) = (solve::signed_area(old), solve::signed_area(new));
    new.iter().all(|p| p[0].is_finite() && p[1].is_finite()) && a0 * a1 > 0.0 && (a1 / a0) > 0.5 && (a1 / a0) < 2.0
}

// ── Host side ───────────────────────────────────────────────────────────────

/// Put the corners `new` in place of `old` on the polyline `h` (bulges and
/// everything else kept).
pub fn move_corners(host: &mut dyn HostApi, h: Handle, old: &[P2], new: &[P2]) -> bool {
    let Some(EntityType::LwPolyline(mut p)) = holes::live(host, h) else { return false };
    let mut hit = false;
    for v in p.vertices.iter_mut() {
        let q = [v.location.x, v.location.y];
        if let Some(i) = old.iter().position(|o| solve::dist(*o, q) < 1e-9) {
            v.location = Vector2::new(new[i][0], new[i][1]);
            hit = true;
        }
    }
    hit && host.update_entity(EntityType::LwPolyline(p))
}

/// After the outer walls' thicknesses: every room inside the perimeter
/// with sides along an inner face gets them onto it. Returns the sides and
/// rooms moved.
pub fn snap_to_perimeter(host: &mut dyn HostApi, perim: Handle, pts: &[P2], thick: &[Option<f64>]) -> (usize, usize) {
    let n = pts.len();
    let longest = (0..n).map(|k| solve::dist(pts[k], pts[(k + 1) % n])).fold(0.0f64, f64::max);
    let m = crate::survey::unit_factor(crate::survey::guess_unit(longest));
    let mut targets = Vec::new();
    for (k, t) in thick.iter().enumerate() {
        let Some(t) = t else { continue };
        // A wall towards the outside (negative): the perimeter is its inner face.
        let on = t.max(0.0);
        let (a, b) = commands::offset_side(pts, k, on);
        let (a1, _) = commands::offset_side(pts, k, on + 1.0);
        let nin = unit(a, a1);
        targets.push(Target { a, b, nin, tol: (0.5 / m).max(1.2 * t.abs()) });
    }
    if targets.is_empty() {
        return (0, 0);
    }
    let max_move = 1.5 / m + thick.iter().flatten().fold(0.0f64, |x, t| x.max(t.abs()));
    let rooms: Vec<commands::Room> = commands::rooms(host)
        .into_iter()
        .filter(|r| r.handle != perim && !r.had_arcs)
        .filter(|r| r.pts.iter().all(|p| commands::point_in_polygon(pts, *p) || inside_by(pts, *p, max_move)))
        .collect();
    let mut sides = 0;
    let mut done = 0;
    let mut open = false;
    for r in rooms {
        let Some((new, c)) = snap_room(&r.pts, &targets, max_move) else { continue };
        if !sane(&r.pts, &new) {
            continue;
        }
        if !open {
            host.begin_undo_group("RV_SPESSORI");
            open = true;
        }
        if move_corners(host, r.handle, &r.pts, &new) {
            sides += c;
            done += 1;
        }
    }
    if open {
        host.end_undo_group();
        host.set_dirty();
    }
    (sides, done)
}

/// Whether `p`, outside the polygon, is within `d` of its boundary (a room
/// drawn a little past the perimeter).
fn inside_by(pts: &[P2], p: P2, d: f64) -> bool {
    let n = pts.len();
    (0..n).any(|k| solve::dist(p, tools::seg_foot(p, pts[k], pts[(k + 1) % n])) <= d)
}

/// The partitions written on the drawing: each gap outline and the
/// thickness written inside it.
pub fn marks(host: &dyn HostApi) -> Vec<(Vec<P2>, f64)> {
    let mut loops = Vec::new();
    let mut texts = Vec::new();
    for e in host.document().entities() {
        if !e.common().layer.eq_ignore_ascii_case(LAYER_PART) {
            continue;
        }
        match e {
            EntityType::LwPolyline(p) if p.vertices.len() >= 3 => loops.push((e.common().handle, p.vertices.iter().map(|v| [v.location.x, v.location.y]).collect::<Vec<P2>>())),
            EntityType::MText(m) => {
                if let Some(t) = commands::parse_num(m.value.trim()).filter(|v| *v > 0.0) {
                    texts.push(([m.insertion_point.x, m.insertion_point.y], t));
                }
            }
            _ => {}
        }
    }
    loops
        .into_iter()
        .filter_map(|(_, l)| texts.iter().find(|(p, _)| commands::point_in_polygon(&l, *p)).map(|(_, t)| (l, *t)))
        .collect()
}

/// Write partition `t` in the gap `quad`, replacing any partition written
/// in the same gap before.
fn write_mark(host: &mut dyn HostApi, quad: &[P2; 4], z: f64, t: f64) {
    commands::ensure_layer(host, LAYER_PART, 8);
    let centre = [(quad[0][0] + quad[1][0] + quad[2][0] + quad[3][0]) / 4.0, (quad[0][1] + quad[1][1] + quad[2][1] + quad[3][1]) / 4.0];
    let mut old = Vec::new();
    for e in host.document().entities() {
        if !e.common().layer.eq_ignore_ascii_case(LAYER_PART) {
            continue;
        }
        let hit = match e {
            EntityType::LwPolyline(p) => {
                let l: Vec<P2> = p.vertices.iter().map(|v| [v.location.x, v.location.y]).collect();
                l.len() >= 3 && (commands::point_in_polygon(&l, centre) || commands::point_in_polygon(quad, mid(l[0], l[2])))
            }
            EntityType::MText(m) => commands::point_in_polygon(quad, [m.insertion_point.x, m.insertion_point.y]),
            _ => false,
        };
        if hit {
            old.push(e.common().handle);
        }
    }
    for h in old {
        host.remove_entity(h);
    }
    host.add_entity(commands::pline(quad, z, true, LAYER_PART));
    let len = solve::dist(quad[0], quad[1]);
    let h = (0.6 * t).min(len / 3.0).max(1e-6);
    let mut e = commands::mtext(&commands::fmt_num(t, 2), centre, z, h, AttachmentPoint::MiddleCenter, LAYER_PART);
    if let EntityType::MText(m) = &mut e {
        m.rotation = commands::readable_angle(quad[0], quad[1]);
    }
    host.add_entity(e);
}

// ── The partition tool ──────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq)]
enum Phase {
    /// Click the side of the room that moves.
    Pick,
    /// The partition's thickness.
    Thick,
}

/// What was clicked: ring, side, and the facing side with the gap.
#[derive(Clone, Copy, Debug)]
struct Picked {
    ring: usize,
    side: usize,
    other: Handle,
    c: P2,
    d: P2,
    gap: f64,
}

#[derive(Clone)]
pub struct PartitionTool {
    rings: Vec<Ring>,
    perim: Option<Handle>,
    /// Metres per drawing unit.
    m: f64,
    phase: Phase,
    picked: Option<Picked>,
    t: f64,
    msg: Option<String>,
}

static LAST_T: Mutex<Option<f64>> = Mutex::new(None);
static RESUME: Mutex<Option<PartitionTool>> = Mutex::new(None);

fn take_resume() -> Option<PartitionTool> {
    RESUME.lock().unwrap_or_else(|e| e.into_inner()).take()
}

fn hand_over(tool: PartitionTool, verb: &str) -> CommandStep {
    *RESUME.lock().unwrap_or_else(|e| e.into_inner()) = Some(tool);
    tools::deferred(verb, String::new())
}

fn room_rings(host: &dyn HostApi, perim: Option<Handle>) -> Vec<Ring> {
    holes::rings(host).into_iter().filter(|r| r.closed && Some(r.handle) != perim).collect()
}

impl PartitionTool {
    pub fn new(rings: Vec<Ring>, perim: Option<Handle>, m: f64, msg: Option<String>) -> Self {
        let t = LAST_T.lock().unwrap_or_else(|e| e.into_inner()).unwrap_or(0.10 / m);
        PartitionTool { rings, perim, m, phase: Phase::Pick, picked: None, t, msg }
    }

    fn reach(&self) -> f64 {
        0.6 / self.m
    }

    fn nearest_side(&self, p: P2) -> Option<(usize, usize)> {
        let mut best: Option<((usize, usize), f64)> = None;
        for (r, ring) in self.rings.iter().enumerate() {
            let n = ring.pts.len();
            for i in 0..n {
                let d = solve::dist(p, tools::seg_foot(p, ring.pts[i], ring.pts[(i + 1) % n]));
                if best.map_or(true, |(_, b)| d < b) {
                    best = Some(((r, i), d));
                }
            }
        }
        best.map(|(s, _)| s)
    }

    fn facing_of(&self, r: usize, i: usize) -> Option<(Handle, P2, P2, f64)> {
        let me = self.rings[r].handle;
        let others: Vec<&Ring> = self.rings.iter().filter(|x| x.handle != me).collect();
        facing(&self.rings[r].pts, i, &others, self.reach())
    }

    fn max_move(&self) -> f64 {
        1.5 / self.m
    }
}

fn title() -> &'static str {
    tr!("Spessori tramezze", "Partition thicknesses", "Espesores de tabiques", "Épaisseurs des cloisons", "Bölme duvar kalınlıkları", "Espessuras dos tabiques", "Grubości ścianek działowych")
}

impl InteractiveCommand for PartitionTool {
    fn prompt(&self) -> String {
        let p = match self.phase {
            Phase::Pick => tr!(
                "Spessori tramezze: clicca il lato del vano da spostare (in verde; in arancione il lato del vano accanto, che resta fermo). Invio per finire",
                "Partition thicknesses: click the side of the room to move (in green; in orange the side of the room next door, which stays put). Enter to finish",
                "Espesores de tabiques: haga clic en el lado de la estancia que se moverá (en verde; en naranja el lado de la estancia contigua, que queda fijo). Intro para terminar",
                "Épaisseurs des cloisons : cliquez sur le côté de la pièce à déplacer (en vert ; en orange le côté de la pièce voisine, qui reste fixe). Entrée pour terminer",
                "Bölme duvar kalınlıkları: taşınacak odanın kenarına tıklayın (yeşil; turuncu olan yan odanın kenarı yerinde kalır). Bitirmek için Enter",
                "Espessuras dos tabiques: clique no lado do compartimento a mover (a verde; a laranja o lado do compartimento ao lado, que fica parado). Enter para terminar",
                "Grubości ścianek działowych: kliknij bok pomieszczenia do przesunięcia (na zielono; na pomarańczowo bok sąsiedniego pomieszczenia, który zostaje na miejscu). Enter, aby zakończyć",
            )
            .to_string(),
            Phase::Thick => tr!(
                "Spessori tramezze: rispondi nella finestra",
                "Partition thicknesses: answer in the window",
                "Espesores de tabiques: responda en la ventana",
                "Épaisseurs des cloisons : répondez dans la fenêtre",
                "Bölme duvar kalınlıkları: pencerede yanıtlayın",
                "Espessuras dos tabiques: responda na janela",
                "Grubości ścianek działowych: odpowiedz w oknie",
            )
            .to_string(),
        };
        match &self.msg {
            Some(m) => format!("{m}  |  {p}"),
            None => p,
        }
    }

    fn needs_object_pick(&self) -> bool {
        self.phase == Phase::Pick
    }

    fn on_object_pick(&mut self, _handle: Handle, pt: [f64; 3]) -> CommandStep {
        self.on_point(pt)
    }

    fn on_point(&mut self, pt: [f64; 3]) -> CommandStep {
        if self.phase != Phase::Pick {
            return CommandStep::NeedPoint;
        }
        let Some((r, i)) = self.nearest_side([pt[0], pt[1]]) else { return CommandStep::NeedPoint };
        match self.facing_of(r, i) {
            Some((other, c, d, gap)) => {
                let mut next = self.clone();
                next.picked = Some(Picked { ring: r, side: i, other, c, d, gap });
                next.phase = Phase::Thick;
                next.msg = None;
                hand_over(next, "RV_TRAMEZZELATO")
            }
            None => {
                let reach = commands::fmt_num(self.reach(), 2);
                self.msg = Some(tr!(
                    format!("⚠ Di fronte a quel lato non c'è un vano parallelo entro {reach}"),
                    format!("⚠ No parallel room faces that side within {reach}"),
                    format!("⚠ Frente a ese lado no hay ninguna estancia paralela a menos de {reach}"),
                    format!("⚠ Aucune pièce parallèle en face de ce côté à moins de {reach}"),
                    format!("⚠ O kenarın karşısında {reach} içinde paralel oda yok"),
                    format!("⚠ Em frente a esse lado não há nenhum compartimento paralelo a menos de {reach}"),
                    format!("⚠ Naprzeciw tego boku nie ma równoległego pomieszczenia w odległości {reach}"),
                ));
                CommandStep::NeedPoint
            }
        }
    }

    fn on_cursor_move(&mut self, pt: [f64; 3]) -> Vec<PreviewWire> {
        let mut w = Vec::new();
        if self.phase != Phase::Pick {
            return w;
        }
        if let Some((r, i)) = self.nearest_side([pt[0], pt[1]]) {
            let ring = &self.rings[r];
            let n = ring.pts.len();
            let (a, b) = (ring.pts[i], ring.pts[(i + 1) % n]);
            w.push(PreviewWire::with_color(vec![[a[0], a[1], ring.z], [b[0], b[1], ring.z]], GREEN));
            if let Some((_, c, d, _)) = self.facing_of(r, i) {
                w.push(PreviewWire::with_color(vec![[c[0], c[1], ring.z], [d[0], d[1], ring.z]], ORANGE));
            }
        }
        w
    }

    fn modal_dialog(&self) -> Option<DialogDef> {
        if self.phase != Phase::Thick {
            return None;
        }
        let p = self.picked?;
        let u = holes::unit_name(self.m);
        let gap = commands::fmt_num(p.gap, 2);
        Some(
            DialogDef::new(title())
                .with_control(tools::info(tr!(
                    format!("Distanza attuale tra i due vani: {gap}. Scrivi lo spessore della tramezza: il lato cliccato diventa parallelo al lato del vano accanto, a questa distanza (gli spigoli si ricalcolano con i lati vicini). Il vano accanto resta fermo."),
                    format!("Present distance between the two rooms: {gap}. Type the partition's thickness: the clicked side becomes parallel to the side of the room next door, at this distance (the corners are found again with the sides next to it). The room next door stays put."),
                    format!("Distancia actual entre las dos estancias: {gap}. Escriba el espesor del tabique: el lado pulsado queda paralelo al lado de la estancia contigua, a esta distancia (las esquinas se recalculan con los lados vecinos). La estancia contigua queda fija."),
                    format!("Distance actuelle entre les deux pièces : {gap}. Saisissez l'épaisseur de la cloison : le côté cliqué devient parallèle au côté de la pièce voisine, à cette distance (les coins sont recalculés avec les côtés voisins). La pièce voisine reste fixe."),
                    format!("İki oda arasındaki mevcut uzaklık: {gap}. Bölme duvarın kalınlığını yazın: tıklanan kenar yan odanın kenarına bu uzaklıkta paralel olur (köşeler komşu kenarlarla yeniden bulunur). Yan oda yerinde kalır."),
                    format!("Distância atual entre os dois compartimentos: {gap}. Escreva a espessura do tabique: o lado clicado fica paralelo ao lado do compartimento ao lado, a esta distância (os cantos são recalculados com os lados vizinhos). O compartimento ao lado fica parado."),
                    format!("Obecna odległość między pomieszczeniami: {gap}. Wpisz grubość ścianki działowej: kliknięty bok staje się równoległy do boku sąsiedniego pomieszczenia, w tej odległości (narożniki są wyznaczane na nowo z sąsiednimi bokami). Sąsiednie pomieszczenie zostaje na miejscu."),
                )))
                .with_control(tools::number("t", &format!("{} ({u})", tr!("Spessore tramezza", "Partition thickness", "Espesor del tabique", "Épaisseur de la cloison", "Bölme duvar kalınlığı", "Espessura do tabique", "Grubość ścianki działowej")), self.t))
                .submit_label(tr!("Applica", "Apply", "Aplicar", "Appliquer", "Uygula", "Aplicar", "Zastosuj"))
                .cancel_label(tr!("Annulla", "Cancel", "Cancelar", "Annuler", "İptal", "Cancelar", "Anuluj"))
                .width(520.0),
        )
    }

    fn on_dialog_response(&mut self, response: DialogResponse) -> CommandStep {
        match (self.phase, response) {
            (Phase::Thick, DialogResponse::Submitted(m)) => {
                if let Some(t) = tools::get_num(&m, "t").filter(|v| *v > 0.0) {
                    self.t = t;
                    *LAST_T.lock().unwrap_or_else(|e| e.into_inner()) = Some(t);
                }
                hand_over(self.clone(), "RV_TRAMEZZEAPPLY")
            }
            (Phase::Thick, _) => {
                self.phase = Phase::Pick;
                self.picked = None;
                CommandStep::NeedPoint
            }
            _ => CommandStep::NeedPoint,
        }
    }

    fn on_enter(&mut self) -> CommandStep {
        match self.phase {
            Phase::Thick => CommandStep::NeedPoint,
            Phase::Pick => CommandStep::Done,
        }
    }
}

fn perimeter_handle() -> Option<Handle> {
    commands::perimeter().and_then(|(h, _)| commands::parse_handle(&h))
}

/// `RV_TRAMEZZE`: start the tool.
pub fn start(host: &mut dyn HostApi) {
    let perim = perimeter_handle();
    let rs = room_rings(host, perim);
    if rs.len() < 2 {
        host.push_error(tr!(
            "Spessori tramezze: servono almeno due vani (polilinee chiuse).",
            "Partition thicknesses: at least two rooms (closed polylines) are needed.",
            "Espesores de tabiques: se necesitan al menos dos estancias (polilíneas cerradas).",
            "Épaisseurs des cloisons : il faut au moins deux pièces (polylignes fermées).",
            "Bölme duvar kalınlıkları: en az iki oda (kapalı çoklu çizgi) gerekir.",
            "Espessuras dos tabiques: são precisos pelo menos dois compartimentos (polilinhas fechadas).",
            "Grubości ścianek działowych: potrzebne są co najmniej dwa pomieszczenia (zamknięte polilinie).",
        ));
        return;
    }
    let m = holes::metres_per_unit(&rs);
    let _ = host.set_selection(&[]);
    host.start_interactive(Box::new(PartitionTool::new(rs, perim, m, None)));
}

/// `RV_TRAMEZZELATO`: a side was clicked; ask the thickness.
pub fn side_picked(host: &mut dyn HostApi) {
    if let Some(tool) = take_resume() {
        host.start_interactive(Box::new(tool));
    }
}

/// `RV_TRAMEZZEAPPLY`: move the side, then on to the next partition.
pub fn apply(host: &mut dyn HostApi) {
    let Some(tool) = take_resume() else { return };
    let msg = match apply_inner(host, &tool) {
        Ok(m) => {
            host.push_info(&m);
            m
        }
        Err(e) => {
            host.push_error(&e);
            format!("⚠ {e}")
        }
    };
    let rs = room_rings(host, tool.perim);
    let _ = host.set_selection(&[]);
    host.start_interactive(Box::new(PartitionTool::new(rs, tool.perim, tool.m, Some(msg))));
}

fn apply_inner(host: &mut dyn HostApi, tool: &PartitionTool) -> Result<String, String> {
    let gone = || {
        tr!(
            "Spessori tramezze: il vano non è più quello di prima nel disegno.",
            "Partition thicknesses: the room is no longer the same in the drawing.",
            "Espesores de tabiques: la estancia ya no es la misma en el dibujo.",
            "Épaisseurs des cloisons : la pièce n'est plus la même dans le dessin.",
            "Bölme duvar kalınlıkları: oda çizimde artık aynı değil.",
            "Espessuras dos tabiques: o compartimento já não é o mesmo no desenho.",
            "Grubości ścianek działowych: pomieszczenie nie jest już takie samo na rysunku.",
        )
        .to_string()
    };
    let p = tool.picked.ok_or_else(gone)?;
    let ring = tool.rings.get(p.ring).ok_or_else(gone)?;
    let now = room_rings(host, tool.perim);
    if !now.iter().any(|r| r.handle == ring.handle && r.pts == ring.pts) || !now.iter().any(|r| r.handle == p.other) {
        return Err(gone());
    }
    let new = against(&ring.pts, p.side, p.c, p.d, tool.t, tool.max_move());
    if !sane(&ring.pts, &new) {
        return Err(tr!(
            "Spessori tramezze: con questo spessore il vano si deformerebbe troppo; nulla è cambiato.",
            "Partition thicknesses: with this thickness the room would be bent out of shape; nothing changed.",
            "Espesores de tabiques: con este espesor la estancia se deformaría demasiado; no ha cambiado nada.",
            "Épaisseurs des cloisons : avec cette épaisseur la pièce serait trop déformée ; rien n'a changé.",
            "Bölme duvar kalınlıkları: bu kalınlıkla oda fazla bozulur; hiçbir şey değişmedi.",
            "Espessuras dos tabiques: com esta espessura o compartimento ficaria demasiado deformado; nada mudou.",
            "Grubości ścianek działowych: przy tej grubości pomieszczenie zbytnio by się zniekształciło; nic nie zmieniono.",
        )
        .to_string());
    }
    host.begin_undo_group("RV_TRAMEZZE");
    let ok = move_corners(host, ring.handle, &ring.pts, &new);
    if ok {
        let n = new.len();
        let (a, b) = (new[p.side], new[(p.side + 1) % n]);
        let uc = unit(p.c, p.d);
        let fa = foot_on((p.c, uc), a);
        let fb = foot_on((p.c, uc), b);
        write_mark(host, &[a, b, fb, fa], ring.z, tool.t);
    }
    host.end_undo_group();
    if !ok {
        return Err(gone());
    }
    host.set_dirty();
    let t = commands::fmt_num(tool.t, 2);
    let before = commands::fmt_num(p.gap, 2);
    Ok(tr!(
        format!("Tramezza di {t} (prima {before}) — Ctrl+Z per annullare"),
        format!("Partition of {t} (before {before}) — Ctrl+Z to undo"),
        format!("Tabique de {t} (antes {before}) — Ctrl+Z para deshacer"),
        format!("Cloison de {t} (avant {before}) — Ctrl+Z pour annuler"),
        format!("{t} bölme duvar (önce {before}) — geri almak için Ctrl+Z"),
        format!("Tabique de {t} (antes {before}) — Ctrl+Z para anular"),
        format!("Ścianka działowa {t} (wcześniej {before}) — Ctrl+Z, aby cofnąć"),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn close(a: P2, b: P2) -> bool {
        solve::dist(a, b) < 1e-6
    }

    #[test]
    fn room_goes_onto_the_inner_face() {
        // Perimeter 0..10 × 0..8, walls 0,30 on every side: inner face
        // 0,3..9,7 × 0,3..7,7. A room drawn by eye in the corner.
        let perim = vec![[0.0, 0.0], [10.0, 0.0], [10.0, 8.0], [0.0, 8.0]];
        let targets: Vec<Target> = (0..4)
            .map(|k| {
                let (a, b) = commands::offset_side(&perim, k, 0.3);
                let (a1, _) = commands::offset_side(&perim, k, 1.3);
                Target { a, b, nin: unit(a, a1), tol: 0.5 }
            })
            .collect();
        let room = vec![[0.1, 0.45], [4.0, 0.2], [4.0, 4.0], [0.15, 4.0]];
        let (new, c) = snap_room(&room, &targets, 2.0).unwrap();
        assert_eq!(c, 2);
        assert!(close(new[0], [0.3, 0.3]), "{:?}", new[0]);
        assert!(close(new[1], [4.0, 0.3]), "{:?}", new[1]);
        assert!(close(new[2], [4.0, 4.0]));
        assert!(close(new[3], [0.3, 4.0]), "{:?}", new[3]);
    }

    #[test]
    fn partition_moves_the_clicked_side() {
        let a = vec![[0.0, 0.0], [4.0, 0.0], [4.0, 3.0], [0.0, 3.0]];
        let b = Ring { handle: Handle::new(2), pts: vec![[4.2, 0.0], [8.0, 0.0], [8.0, 3.0], [4.25, 3.0]], closed: true, z: 0.0 };
        let (h, c, d, gap) = facing(&a, 1, &[&b], 0.6).unwrap();
        assert_eq!(h, Handle::new(2));
        assert!((gap - 0.225).abs() < 0.01, "{gap}");
        let new = against(&a, 1, c, d, 0.1, 2.0);
        // The side lies 0,10 from the slanted side of b, parallel to it.
        let uc = unit(c, d);
        for p in [new[1], new[2]] {
            assert!((cross(uc, sub(p, c)).abs() - 0.1).abs() < 1e-9);
        }
        assert!(close(new[0], a[0]) && close(new[3], a[3]));
        assert!((new[1][1]).abs() < 1e-9 && (new[2][1] - 3.0).abs() < 1e-9);
    }
}
