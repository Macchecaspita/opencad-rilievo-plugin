//! One survey: one or more rooms, the names of their corners, and the
//! measures typed in the dialog. Host-free, so it is unit-tested alone.
//!
//! With one room the corners are A, B, C …; with several, each name starts
//! with the room's number: 1A, 1B … 2A, 2B … Corners drawn exactly on top of
//! each other (rooms drawn touching, with no wall between them) are one and
//! the same point.
//!
//! The dialog has two tables:
//!
//! * **Lati** — one row per side of every room: `[side, drawn, measured]`;
//! * **Altre misure** — `[from, to, drawn, measured]`. Two corners make a
//!   distance: a diagonal when both are in the same room, a link when they
//!   are in different rooms (a measure through a door). A side and a corner
//!   (`1B-1C` and `2A`) make a wall thickness: the distance of the corner
//!   from the line of the side.
//! * **Vincoli** — `[corner or side, constraint]`: an angle at a corner
//!   (`C`, `90`) or a side horizontal / vertical (`A-B`, `orizzontale`).
//!   Each counts as one measure.
//! * **Porte e finestre** — `[name, side, kind, distance, width, wall]`:
//!   an opening on a side, at a distance from the side's first corner. Two
//!   doors with the same name in two rooms are the same door seen from both
//!   sides: with the wall thickness they link the two rooms by themselves.
//! * **Muri esterni** — `[side, thickness]`, with an outer outline surveyed
//!   exactly (total station): its corners are held where they are, and the
//!   room side keeps the given distance (the outer wall's thickness) from
//!   the nearest outline side. Two measures per row, one per end.

use crate::solve::{self, label, parse_label, At, Meas, P2};

/// What the user typed for a survey (kept for the session, so a failed
/// calculation or a re-run opens the dialog with the same values).
#[derive(Clone, Debug, PartialEq)]
pub struct Saved {
    /// Rows `[side, drawn, measured]`, room after room, in corner order.
    pub sides: Vec<Vec<String>>,
    /// Rows `[from, to, drawn, measured]`.
    pub others: Vec<Vec<String>>,
    pub h: f64,
    pub tol: f64,
    pub dims: bool,
    pub table: bool,
    /// Write the measures book (CSV) after the calculation.
    pub csv: bool,
    pub csv_path: String,
    /// Rows `[corner or side, constraint]`.
    pub constraints: Vec<Vec<String>>,
    /// Rows `[name, side, kind, distance, width, wall thickness]`.
    pub openings: Vec<Vec<String>>,
    /// Drawing unit: 0 metres, 1 centimetres, 2 millimetres.
    pub unit: usize,
    /// When the measures are not enough, finish the shape from the drawing.
    pub fill: bool,
    /// Move the original polylines to the result straight away.
    pub replace: bool,
    /// Handle (hex) of the outer outline surveyed exactly, held fixed; empty
    /// when there is none.
    pub outer: String,
    /// Thickness of the outer walls, for the rows that leave it empty.
    pub outer_t: String,
    /// Rows `[room side, thickness]`: sides along the outer outline.
    pub outer_walls: Vec<Vec<String>>,
    /// Rows `[side, facing side of another room, thickness]`: solid walls
    /// between rooms (found from the hatch drawn in them); the thickness is
    /// measured at a door.
    pub inner_walls: Vec<Vec<String>>,
}

/// Metres per drawing unit, for the areas.
pub fn unit_factor(unit: usize) -> f64 {
    match unit {
        1 => 0.01,
        2 => 0.001,
        _ => 1.0,
    }
}

/// A guess of the drawing unit from the longest side.
pub fn guess_unit(longest: f64) -> usize {
    if longest <= 60.0 {
        0
    } else if longest <= 6000.0 {
        1
    } else {
        2
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OpKind {
    Door,
    Window,
    Opening,
}

/// A door, window or plain opening on a side.
#[derive(Clone, Debug, PartialEq)]
pub struct Opening {
    pub name: String,
    pub kind: OpKind,
    pub room: usize,
    /// Side, from corner `a` (where the distance is taken from) to `b`.
    pub a: usize,
    pub b: usize,
    pub s: f64,
    pub w: f64,
    pub thick: Option<f64>,
    /// Index of the same door in the other room, when shared.
    pub twin: Option<usize>,
}

/// What the tables say.
#[derive(Clone, Debug, PartialEq)]
pub struct Parsed {
    pub measures: Vec<Measure>,
    pub openings: Vec<Opening>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Side,
    Diagonal,
    Link,
    Wall,
    Angle,
    Level,
    Door,
    /// A room side at the outer wall's thickness from the fixed outline.
    Outer,
}

/// One measure as typed, with its display name.
#[derive(Clone, Debug, PartialEq)]
pub struct Measure {
    pub meas: Meas,
    pub name: String,
    pub kind: Kind,
}

#[derive(Clone, Debug)]
pub struct Survey {
    /// Drawn corners of each room.
    pub rooms: Vec<Vec<P2>>,
    /// Global point of corner `i` of room `r`.
    pub point_of: Vec<Vec<usize>>,
    /// Drawn position of each global point.
    pub points: Vec<P2>,
    /// Display name of each global point (its first room's name).
    pub names: Vec<String>,
    /// Corners of the fixed outer outline: the last `outer` points.
    pub outer: usize,
    /// Hatches of the drawing, each as its boundary loops: a hatch between
    /// two rooms marks a solid wall.
    pub hatches: Vec<Vec<Vec<P2>>>,
    /// Partitions given with "Partition thicknesses": the gap between the
    /// two rooms and the partition's thickness.
    pub partitions: Vec<(Vec<P2>, f64)>,
}

impl Survey {
    pub fn new(rooms: Vec<Vec<P2>>) -> Self {
        let size = rooms
            .iter()
            .flat_map(|r| (0..r.len()).map(move |i| solve::dist(r[i], r[(i + 1) % r.len()])))
            .fold(0.0f64, f64::max)
            .max(1e-9);
        let tol = size * 1e-7;
        let multi = rooms.len() > 1;
        let mut points: Vec<P2> = Vec::new();
        let mut names = Vec::new();
        let mut point_of = Vec::new();
        for (r, room) in rooms.iter().enumerate() {
            let mut ids = Vec::new();
            for (i, &p) in room.iter().enumerate() {
                let id = match points.iter().position(|q| solve::dist(*q, p) <= tol) {
                    Some(id) if !ids.contains(&id) => id,
                    _ => {
                        points.push(p);
                        names.push(corner_name(multi, r, i));
                        points.len() - 1
                    }
                };
                ids.push(id);
            }
            point_of.push(ids);
        }
        Survey { rooms, point_of, points, names, outer: 0, hatches: Vec::new(), partitions: Vec::new() }
    }

    /// Add the outer outline surveyed exactly: its corners (E1, E2 …) are
    /// held fixed by the solver.
    pub fn with_outer(mut self, outline: Vec<P2>) -> Self {
        self.outer = outline.len();
        for (k, p) in outline.into_iter().enumerate() {
            self.points.push(p);
            self.names.push(format!("E{}", k + 1));
        }
        self
    }

    /// The drawing's hatches, as boundary loops.
    pub fn with_hatches(mut self, hatches: Vec<Vec<Vec<P2>>>) -> Self {
        self.hatches = hatches;
        self
    }

    /// The partitions given with their thickness: each a wall between two
    /// rooms, like a hatch, whose thickness is already known.
    pub fn with_partitions(mut self, parts: Vec<(Vec<P2>, f64)>) -> Self {
        for (l, _) in &parts {
            self.hatches.push(vec![l.clone()]);
        }
        self.partitions.extend(parts);
        self
    }

    /// Whether a point lies in a hatched area (inside an odd number of one
    /// hatch's loops: islands are holes).
    pub fn hatched(&self, p: P2) -> bool {
        self.hatches.iter().any(|loops| loops.iter().filter(|l| l.len() >= 3 && inside(l, p)).count() % 2 == 1)
    }

    /// Solid walls between rooms: two sides of different rooms facing each
    /// other (parallel within 10°, overlapping), at most `reach` apart, with
    /// a hatch in the gap between them. One row per pair, the nearest.
    pub fn suggest_inner_walls(&self, reach: f64) -> Vec<Vec<String>> {
        let mut rows = Vec::new();
        let n_rooms = self.rooms.len();
        let side = |r: usize, i: usize| {
            let room = &self.rooms[r];
            (room[i], room[(i + 1) % room.len()])
        };
        for r in 0..n_rooms {
            for i in 0..self.rooms[r].len() {
                let (a, b) = side(r, i);
                let l = solve::dist(a, b);
                if l <= 0.0 {
                    continue;
                }
                let u = [(b[0] - a[0]) / l, (b[1] - a[1]) / l];
                let mut best: Option<(usize, usize, f64, P2)> = None;
                for q in r + 1..n_rooms {
                    for k in 0..self.rooms[q].len() {
                        let (c, d) = side(q, k);
                        let lo = solve::dist(c, d);
                        if lo <= 0.0 {
                            continue;
                        }
                        let v = [(d[0] - c[0]) / lo, (d[1] - c[1]) / lo];
                        if (u[0] * v[1] - u[1] * v[0]).abs() > 10f64.to_radians().sin() {
                            continue;
                        }
                        // Overlap of the two sides along the first one.
                        let t = |x: P2| (x[0] - a[0]) * u[0] + (x[1] - a[1]) * u[1];
                        let (tc, td) = (t(c), t(d));
                        let (lo_t, hi_t) = (tc.min(td).max(0.0), tc.max(td).min(l));
                        if hi_t - lo_t < 0.3 * l.min(lo) {
                            continue;
                        }
                        // Middle of the overlap on the first side, and its
                        // foot on the second: the gap between them.
                        let tm = (lo_t + hi_t) / 2.0;
                        let p = [a[0] + u[0] * tm, a[1] + u[1] * tm];
                        let f = solve::foot(p, c, d);
                        let gap = solve::dist(p, f);
                        if gap <= 0.0 || gap > reach {
                            continue;
                        }
                        let mid = [(p[0] + f[0]) / 2.0, (p[1] + f[1]) / 2.0];
                        if inside(&self.rooms[r], mid) || inside(&self.rooms[q], mid) || !self.hatched(mid) {
                            continue;
                        }
                        if best.is_none_or(|x| gap < x.2) {
                            best = Some((q, k, gap, mid));
                        }
                    }
                }
                if let Some((q, k, _, mid)) = best {
                    let t = self.partitions.iter().find(|(l, _)| l.len() >= 3 && inside(l, mid)).map(|(_, t)| format!("{t:.3}")).unwrap_or_default();
                    let nq = self.rooms[q].len();
                    let n = self.rooms[r].len();
                    rows.push(vec![
                        format!("{}–{}", self.corner(r, i), self.corner(r, (i + 1) % n)),
                        format!("{}–{}", self.corner(q, k), self.corner(q, (k + 1) % nq)),
                        t,
                    ]);
                }
            }
        }
        rows
    }

    /// Which points the solver must hold.
    pub fn fixed(&self) -> Vec<bool> {
        let first = self.points.len() - self.outer;
        (0..self.points.len()).map(|p| p >= first).collect()
    }

    /// The outer outline's corners, as drawn.
    pub fn outline(&self) -> &[P2] {
        &self.points[self.points.len() - self.outer..]
    }

    /// The side of the outer outline that room side `a`–`b` (global points)
    /// runs along: nearly parallel (within 15°), overlapping it, the nearest
    /// one. Returns the outline side as global points and the drawn distance.
    pub fn outer_side_for(&self, a: usize, b: usize) -> Option<(usize, usize, f64)> {
        let m = self.outer;
        if m < 3 {
            return None;
        }
        let first = self.points.len() - m;
        let (pa, pb) = (self.points[a], self.points[b]);
        let l = solve::dist(pa, pb);
        if l <= 0.0 {
            return None;
        }
        let u = [(pb[0] - pa[0]) / l, (pb[1] - pa[1]) / l];
        let mid = [(pa[0] + pb[0]) / 2.0, (pa[1] + pb[1]) / 2.0];
        let mut best: Option<(usize, usize, f64)> = None;
        for k in 0..m {
            let (p, q) = (first + k, first + (k + 1) % m);
            let (pp, pq) = (self.points[p], self.points[q]);
            let lo = solve::dist(pp, pq);
            if lo <= 0.0 {
                continue;
            }
            let v = [(pq[0] - pp[0]) / lo, (pq[1] - pp[1]) / lo];
            if (u[0] * v[1] - u[1] * v[0]).abs() > 15f64.to_radians().sin() {
                continue;
            }
            // The room side's ends, projected on the outline side.
            let t = |x: P2| (x[0] - pp[0]) * v[0] + (x[1] - pp[1]) * v[1];
            let (ta, tb) = (t(pa), t(pb));
            let overlap = ta.max(tb).min(lo) - ta.min(tb).max(0.0);
            if overlap < 0.3 * l.min(lo) {
                continue;
            }
            let d = solve::perp_dist(mid, pp, pq);
            if best.is_none_or(|b| d < b.2) {
                best = Some((p, q, d));
            }
        }
        best
    }

    /// The outer-wall rows from a thickness per side of the outline (`None`:
    /// not given): every room side running along a side of the outline that
    /// has a thickness, within reach of it (four times the thickness, or 5 %
    /// of the outline's longest side) and inside it.
    pub fn outer_rows_with(&self, thick: &[Option<f64>], fmt: &dyn Fn(f64) -> String) -> Vec<Vec<String>> {
        let mut rows = Vec::new();
        let m = self.outer;
        if m < 3 {
            return rows;
        }
        let first = self.points.len() - m;
        let outline = self.outline().to_vec();
        let longest = (0..m).map(|i| solve::dist(outline[i], outline[(i + 1) % m])).fold(0.0f64, f64::max);
        for (r, ids) in self.point_of.iter().enumerate() {
            let n = ids.len();
            for i in 0..n {
                let (a, b) = (ids[i], ids[(i + 1) % n]);
                let mid = [(self.points[a][0] + self.points[b][0]) / 2.0, (self.points[a][1] + self.points[b][1]) / 2.0];
                if !inside(&outline, mid) {
                    continue;
                }
                let Some((p, _, d)) = self.outer_side_for(a, b) else { continue };
                let Some(Some(t)) = thick.get(p - first).copied() else { continue };
                if t > 0.0 && d <= (4.0 * t).max(longest * 0.05) {
                    rows.push(vec![format!("{}–{}", self.corner(r, i), self.corner(r, (i + 1) % n)), fmt(t)]);
                }
            }
        }
        rows
    }

    /// The room sides that run along the outer outline, within `reach` of
    /// it and inside it: the rows proposed for the outer walls.
    pub fn suggest_outer_walls(&self, reach: f64) -> Vec<Vec<String>> {
        let mut rows = Vec::new();
        if self.outer < 3 {
            return rows;
        }
        let outline = self.outline().to_vec();
        for (r, ids) in self.point_of.iter().enumerate() {
            let n = ids.len();
            for i in 0..n {
                let (a, b) = (ids[i], ids[(i + 1) % n]);
                let mid = [(self.points[a][0] + self.points[b][0]) / 2.0, (self.points[a][1] + self.points[b][1]) / 2.0];
                if !inside(&outline, mid) {
                    continue;
                }
                if let Some((_, _, d)) = self.outer_side_for(a, b) {
                    if d <= reach {
                        rows.push(vec![format!("{}–{}", self.corner(r, i), self.corner(r, (i + 1) % n)), String::new()]);
                    }
                }
            }
        }
        rows
    }

    pub fn multi(&self) -> bool {
        self.rooms.len() > 1
    }

    /// Name of corner `i` of room `r` ("A" or "1A").
    pub fn corner(&self, r: usize, i: usize) -> String {
        corner_name(self.multi(), r, i)
    }

    /// The last corner name of each room, for messages ("A … F").
    pub fn ranges(&self) -> String {
        (0..self.rooms.len())
            .map(|r| format!("{} … {}", self.corner(r, 0), self.corner(r, self.rooms[r].len() - 1)))
            .collect::<Vec<_>>()
            .join(", ")
    }

    /// Rooms a global point belongs to.
    pub fn rooms_of(&self, p: usize) -> Vec<usize> {
        (0..self.rooms.len()).filter(|&r| self.point_of[r].contains(&p)).collect()
    }

    /// A corner name → (room, corner). `"1A"`, `"1 a"`, `"1.A"`; a bare
    /// letter only when there is a single room.
    pub fn parse_corner(&self, s: &str) -> Option<(usize, usize)> {
        let t: String = s.chars().filter(|c| c.is_ascii_alphanumeric()).collect();
        let digits: String = t.chars().take_while(|c| c.is_ascii_digit()).collect();
        let letters = &t[digits.len()..];
        let r = if digits.is_empty() {
            if self.multi() {
                return None;
            }
            0
        } else {
            digits.parse::<usize>().ok()?.checked_sub(1)?
        };
        let i = parse_label(letters)?;
        (r < self.rooms.len() && i < self.rooms[r].len()).then_some((r, i))
    }

    /// A corner name → global point.
    pub fn parse_point(&self, s: &str) -> Option<usize> {
        self.parse_corner(s).map(|(r, i)| self.point_of[r][i])
    }

    /// A side written as two corners joined by a dash: `"1B-1C"`, `"B–C"`.
    pub fn parse_side(&self, s: &str) -> Option<(usize, usize)> {
        let parts: Vec<&str> = s.split(['-', '–', '—', '/']).map(str::trim).filter(|x| !x.is_empty()).collect();
        if parts.len() != 2 {
            return None;
        }
        let (a, b) = (self.parse_point(parts[0])?, self.parse_point(parts[1])?);
        (a != b).then_some((a, b))
    }

    /// A side of a room written as two neighbouring corners, in either
    /// order: (room, from corner, to corner).
    pub fn parse_room_side(&self, s: &str) -> Option<(usize, usize, usize)> {
        let parts: Vec<&str> = s.split(['-', '–', '—', '/']).map(str::trim).filter(|x| !x.is_empty()).collect();
        if parts.len() != 2 {
            return None;
        }
        let (r1, i) = self.parse_corner(parts[0])?;
        let (r2, j) = self.parse_corner(parts[1])?;
        let n = self.rooms[r1].len();
        (r1 == r2 && (j == (i + 1) % n || i == (j + 1) % n)).then_some((r1, i, j))
    }

    /// The rows a fresh dialog starts with: every side, and the diagonals
    /// each room needs (drawn values filled in, measured empty).
    pub fn initial(&self, dec: usize, fmt: &dyn Fn(f64, usize) -> String) -> (Vec<Vec<String>>, Vec<Vec<String>>) {
        let mut sides = Vec::new();
        let mut others = Vec::new();
        for (r, room) in self.rooms.iter().enumerate() {
            let n = room.len();
            for i in 0..n {
                let j = (i + 1) % n;
                sides.push(vec![
                    format!("{}–{}", self.corner(r, i), self.corner(r, j)),
                    fmt(solve::dist(room[i], room[j]), dec),
                    String::new(),
                ]);
            }
            for (i, j) in solve::suggested_diagonals(room) {
                others.push(vec![self.corner(r, i), self.corner(r, j), fmt(solve::dist(room[i], room[j]), dec), String::new()]);
            }
        }
        (sides, others)
    }

    /// The drawn value of an "other" row, when its corners are readable.
    pub fn drawn_other(&self, from: &str, to: &str) -> Option<f64> {
        match (self.parse_side(from), self.parse_point(from), self.parse_side(to), self.parse_point(to)) {
            (_, Some(a), _, Some(b)) => Some(solve::dist(self.points[a], self.points[b])),
            (Some((a, b)), None, _, Some(p)) | (_, Some(p), Some((a, b)), None) => {
                Some(solve::perp_dist(self.points[p], self.points[a], self.points[b]))
            }
            _ => None,
        }
    }

    /// Read the tables. Rows with an empty "measured" cell are skipped (a
    /// proposed diagonal not measured). Errors name the row.
    pub fn parse(&self, saved: &Saved, parse_num: &dyn Fn(&str) -> Option<f64>) -> Result<Parsed, String> {
        let mut out = Vec::new();
        let mut row = 0;
        for (r, room) in self.rooms.iter().enumerate() {
            let n = room.len();
            for i in 0..n {
                let j = (i + 1) % n;
                let cells = saved.sides.get(row);
                row += 1;
                let name = format!("{}–{}", self.corner(r, i), self.corner(r, j));
                let cell = cells.and_then(|c| c.get(2)).map(String::as_str).unwrap_or("");
                if cell.trim().is_empty() {
                    continue;
                }
                match parse_num(cell) {
                    Some(d) if d > 0.0 => out.push(Measure {
                        meas: Meas::Dist { i: self.point_of[r][i], j: self.point_of[r][j], d },
                        name,
                        kind: Kind::Side,
                    }),
                    _ => {
                        return Err(tr!(
                            format!("lato {name}: \"{cell}\" non è una misura valida"),
                            format!("side {name}: \"{cell}\" is not a valid measure"),
                            format!("lado {name}: \"{cell}\" no es una medida válida"),
                            format!("côté {name} : \"{cell}\" n'est pas une mesure valable"),
                            format!("kenar {name}: \"{cell}\" geçerli bir ölçü değil"),
                            format!("lado {name}: \"{cell}\" não é uma medida válida"),
                            format!("bok {name}: \"{cell}\" nie jest prawidłowym pomiarem"),
                        ))
                    }
                }
            }
        }
        for (k, cells) in saved.others.iter().enumerate() {
            let cell = |c: usize| cells.get(c).map(|s| s.trim().to_string()).unwrap_or_default();
            let (a, b, m) = (cell(0), cell(1), cell(3));
            if m.is_empty() {
                continue;
            }
            let r = k + 1;
            let hint = if self.multi() {
                tr!(
                    format!("scrivi i vertici con numero del vano e lettera ({}); per uno spessore di muro, un lato come 1B-1C e un vertice", self.ranges()),
                    format!("write corners as room number and letter ({}); for a wall thickness, a side like 1B-1C and a corner", self.ranges()),
                    format!("escriba las esquinas con número de estancia y letra ({}); para un espesor de muro, un lado como 1B-1C y una esquina", self.ranges()),
                    format!("écrivez les angles avec le numéro de pièce et la lettre ({}) ; pour une épaisseur de mur, un côté comme 1B-1C et un angle", self.ranges()),
                    format!("köşeleri oda numarası ve harfle yazın ({}); duvar kalınlığı için 1B-1C gibi bir kenar ve bir köşe", self.ranges()),
                    format!("escreva os cantos com o número do compartimento e a letra ({}); para uma espessura de parede, um lado como 1B-1C e um canto", self.ranges()),
                    format!("wpisz narożniki z numerem pomieszczenia i literą ({}); dla grubości ściany bok, np. 1B-1C, i narożnik", self.ranges()),
                )
            } else {
                tr!(
                    format!("scrivi i due vertici con le loro lettere ({})", self.ranges()),
                    format!("write the two corners by letter ({})", self.ranges()),
                    format!("escriba las dos esquinas con su letra ({})", self.ranges()),
                    format!("écrivez les deux angles par leur lettre ({})", self.ranges()),
                    format!("iki köşeyi harfleriyle yazın ({})", self.ranges()),
                    format!("escreva os dois cantos com a sua letra ({})", self.ranges()),
                    format!("wpisz oba narożniki ich literą ({})", self.ranges()),
                )
            };
            let (meas, name, kind) = match (self.parse_point(&a), self.parse_point(&b), self.parse_side(&a), self.parse_side(&b)) {
                (Some(i), Some(j), _, _) if i != j => {
                    let same = self.rooms_of(i).iter().any(|x| self.rooms_of(j).contains(x));
                    let name = format!("{}–{}", self.names[i.min(j)], self.names[i.max(j)]);
                    (Meas::Dist { i, j, d: 0.0 }, name, if same { Kind::Diagonal } else { Kind::Link })
                }
                (Some(p), None, None, Some((s0, s1))) | (None, Some(p), Some((s0, s1)), None) if p != s0 && p != s1 => {
                    let name = format!("sp. {}–{} / {}", self.names[s0], self.names[s1], self.names[p]);
                    (Meas::Perp { p, a: s0, b: s1, d: 0.0 }, name, Kind::Wall)
                }
                _ => {
                    return Err(tr!(
                        format!("riga {r} delle altre misure (\"{a}\" e \"{b}\"): {hint}"),
                        format!("row {r} of the other measures (\"{a}\" and \"{b}\"): {hint}"),
                        format!("fila {r} de las otras medidas (\"{a}\" y \"{b}\"): {hint}"),
                        format!("ligne {r} des autres mesures (\"{a}\" et \"{b}\") : {hint}"),
                        format!("diğer ölçülerin {r}. satırı (\"{a}\" ve \"{b}\"): {hint}"),
                        format!("linha {r} das outras medidas (\"{a}\" e \"{b}\"): {hint}"),
                        format!("wiersz {r} innych pomiarów (\"{a}\" i \"{b}\"): {hint}"),
                    ))
                }
            };
            let d = match parse_num(&m) {
                Some(d) if d > 0.0 => d,
                _ => {
                    return Err(tr!(
                        format!("{name}: \"{m}\" non è una misura valida"),
                        format!("{name}: \"{m}\" is not a valid measure"),
                        format!("{name}: \"{m}\" no es una medida válida"),
                        format!("{name} : \"{m}\" n'est pas une mesure valable"),
                        format!("{name}: \"{m}\" geçerli bir ölçü değil"),
                        format!("{name}: \"{m}\" não é uma medida válida"),
                        format!("{name}: \"{m}\" nie jest prawidłowym pomiarem"),
                    ))
                }
            };
            let meas = match meas {
                Meas::Dist { i, j, .. } => Meas::Dist { i, j, d },
                Meas::Perp { p, a, b, .. } => Meas::Perp { p, a, b, d },
                other => other,
            };
            out.push(Measure { meas, name, kind });
        }
        self.parse_constraints(saved, parse_num, &mut out)?;
        let openings = self.parse_openings(saved, parse_num, &mut out)?;
        self.parse_outer(saved, parse_num, &mut out)?;
        self.parse_inner(saved, &openings, parse_num, &mut out)?;
        Ok(Parsed { measures: out, openings })
    }

    /// Solid walls between rooms: the second side kept at the wall's
    /// thickness from the first, at both its ends. An empty thickness takes
    /// the one of a shared door in the same wall.
    fn parse_inner(&self, saved: &Saved, openings: &[Opening], parse_num: &dyn Fn(&str) -> Option<f64>, out: &mut Vec<Measure>) -> Result<(), String> {
        for (k, cells) in saved.inner_walls.iter().enumerate() {
            let cell = |c: usize| cells.get(c).map(|s| s.trim().to_string()).unwrap_or_default();
            let (s1, s2, th) = (cell(0), cell(1), cell(2));
            if s1.is_empty() && s2.is_empty() {
                continue;
            }
            let r = k + 1;
            let (Some((r1, i1, j1)), Some((r2, i2, j2))) = (self.parse_room_side(&s1), self.parse_room_side(&s2)) else {
                return Err(tr!(
                    format!("muro tra vani alla riga {r}: scrivi due lati di vani diversi, per esempio 1B-1C e 2D-2A"),
                    format!("wall between rooms on row {r}: write two sides of different rooms, for example 1B-1C and 2D-2A"),
                    format!("muro entre estancias en la fila {r}: escriba dos lados de estancias distintas, por ejemplo 1B-1C y 2D-2A"),
                    format!("mur entre pièces à la ligne {r} : écrivez deux côtés de pièces différentes, par exemple 1B-1C et 2D-2A"),
                    format!("{r}. satırdaki odalar arası duvar: farklı odaların iki kenarını yazın, örneğin 1B-1C ve 2D-2A"),
                    format!("parede entre compartimentos na linha {r}: escreva dois lados de compartimentos diferentes, por exemplo 1B-1C e 2D-2A"),
                    format!("ściana między pomieszczeniami w wierszu {r}: wpisz dwa boki różnych pomieszczeń, np. 1B-1C i 2D-2A"),
                ));
            };
            if r1 == r2 {
                return Err(tr!(
                    format!("muro tra vani alla riga {r}: i due lati sono dello stesso vano"),
                    format!("wall between rooms on row {r}: both sides are of the same room"),
                    format!("muro entre estancias en la fila {r}: los dos lados son de la misma estancia"),
                    format!("mur entre pièces à la ligne {r} : les deux côtés sont de la même pièce"),
                    format!("{r}. satırdaki odalar arası duvar: iki kenar da aynı odaya ait"),
                    format!("parede entre compartimentos na linha {r}: os dois lados são do mesmo compartimento"),
                    format!("ściana między pomieszczeniami w wierszu {r}: oba boki należą do tego samego pomieszczenia"),
                ));
            }
            let (a, b) = (self.point_of[r1][i1], self.point_of[r1][j1]);
            let (c, d) = (self.point_of[r2][i2], self.point_of[r2][j2]);
            let same = |x: &Opening, p: usize, q: usize| (x.a == p && x.b == q) || (x.a == q && x.b == p);
            let door_t = openings
                .iter()
                .filter(|o| same(o, a, b))
                .find(|o| o.twin.is_some_and(|t| same(&openings[t], c, d)))
                .and_then(|o| o.thick);
            let t = if th.is_empty() { door_t } else { parse_num(&th).filter(|v| *v > 0.0) };
            let Some(t) = t else {
                return Err(if th.is_empty() {
                    tr!(
                        format!("muro tra {s1} e {s2}: scrivi lo spessore, misurato in una porta"),
                        format!("wall between {s1} and {s2}: type the thickness, measured at a door"),
                        format!("muro entre {s1} y {s2}: escriba el espesor, medido en una puerta"),
                        format!("mur entre {s1} et {s2} : saisissez l'épaisseur, mesurée dans une porte"),
                        format!("{s1} ile {s2} arasındaki duvar: bir kapıda ölçülen kalınlığı girin"),
                        format!("parede entre {s1} e {s2}: escreva a espessura, medida numa porta"),
                        format!("ściana między {s1} a {s2}: wpisz grubość zmierzoną w drzwiach"),
                    )
                } else {
                    tr!(
                        format!("muro tra {s1} e {s2}: lo spessore \"{th}\" non è una misura valida"),
                        format!("wall between {s1} and {s2}: the thickness \"{th}\" is not a valid measure"),
                        format!("muro entre {s1} y {s2}: el espesor \"{th}\" no es una medida válida"),
                        format!("mur entre {s1} et {s2} : l'épaisseur \"{th}\" n'est pas une mesure valable"),
                        format!("{s1} ile {s2} arasındaki duvar: \"{th}\" kalınlığı geçerli bir ölçü değil"),
                        format!("parede entre {s1} e {s2}: a espessura \"{th}\" não é uma medida válida"),
                        format!("ściana między {s1} a {s2}: grubość \"{th}\" nie jest prawidłowym pomiarem"),
                    )
                });
            };
            let n1 = format!("{}–{}", self.names[a], self.names[b]);
            for p in [c, d] {
                if p == a || p == b {
                    continue;
                }
                out.push(Measure {
                    meas: Meas::Perp { p, a, b, d: t },
                    name: format!("sp. {n1} / {}", self.names[p]),
                    kind: Kind::Wall,
                });
            }
        }
        Ok(())
    }

    /// Outer walls: each room side kept at the wall's thickness from the
    /// fixed outline, at both ends.
    fn parse_outer(&self, saved: &Saved, parse_num: &dyn Fn(&str) -> Option<f64>, out: &mut Vec<Measure>) -> Result<(), String> {
        if self.outer < 3 {
            return Ok(());
        }
        let default_t = parse_num(&saved.outer_t).filter(|v| *v > 0.0);
        for (k, cells) in saved.outer_walls.iter().enumerate() {
            let cell = |c: usize| cells.get(c).map(|s| s.trim().to_string()).unwrap_or_default();
            let (side, th) = (cell(0), cell(1));
            if side.is_empty() {
                continue;
            }
            let r = k + 1;
            let Some((room, i, j)) = self.parse_room_side(&side) else {
                return Err(tr!(
                    format!("muro esterno alla riga {r}: \"{side}\" non è un lato di un vano (scrivi due spigoli vicini, per esempio {}-{})", self.corner(0, 0), self.corner(0, 1)),
                    format!("outer wall on row {r}: \"{side}\" is not a room side (write two neighbouring corners, for example {}-{})", self.corner(0, 0), self.corner(0, 1)),
                    format!("muro exterior en la fila {r}: \"{side}\" no es un lado de estancia (escriba dos esquinas contiguas, por ejemplo {}-{})", self.corner(0, 0), self.corner(0, 1)),
                    format!("mur extérieur à la ligne {r} : \"{side}\" n'est pas un côté de pièce (écrivez deux angles voisins, par exemple {}-{})", self.corner(0, 0), self.corner(0, 1)),
                    format!("{r}. satırdaki dış duvar: \"{side}\" bir oda kenarı değil (iki komşu köşe yazın, örneğin {}-{})", self.corner(0, 0), self.corner(0, 1)),
                    format!("parede exterior na linha {r}: \"{side}\" não é um lado de compartimento (escreva dois cantos vizinhos, por exemplo {}-{})", self.corner(0, 0), self.corner(0, 1)),
                    format!("ściana zewnętrzna w wierszu {r}: \"{side}\" nie jest bokiem pomieszczenia (wpisz dwa sąsiednie narożniki, np. {}-{})", self.corner(0, 0), self.corner(0, 1)),
                ));
            };
            let t = if th.is_empty() { default_t } else { parse_num(&th).filter(|v| *v > 0.0) };
            let Some(t) = t else {
                return Err(if th.is_empty() {
                    tr!(
                        "scrivi lo spessore dei muri esterni".to_string(),
                        "type the thickness of the outer walls".to_string(),
                        "escriba el espesor de los muros exteriores".to_string(),
                        "saisissez l'épaisseur des murs extérieurs".to_string(),
                        "dış duvarların kalınlığını girin".to_string(),
                        "escreva a espessura das paredes exteriores".to_string(),
                        "wpisz grubość ścian zewnętrznych".to_string(),
                    )
                } else {
                    tr!(
                        format!("muro esterno {side}: lo spessore \"{th}\" non è una misura valida"),
                        format!("outer wall {side}: the thickness \"{th}\" is not a valid measure"),
                        format!("muro exterior {side}: el espesor \"{th}\" no es una medida válida"),
                        format!("mur extérieur {side} : l'épaisseur \"{th}\" n'est pas une mesure valable"),
                        format!("dış duvar {side}: \"{th}\" kalınlığı geçerli bir ölçü değil"),
                        format!("parede exterior {side}: a espessura \"{th}\" não é uma medida válida"),
                        format!("ściana zewnętrzna {side}: grubość \"{th}\" nie jest prawidłowym pomiarem"),
                    )
                });
            };
            let (a, b) = (self.point_of[room][i], self.point_of[room][j]);
            let Some((p, q, _)) = self.outer_side_for(a, b) else {
                return Err(tr!(
                    format!("il lato {side} non corre lungo nessun lato del poligono esterno: toglilo dai muri esterni"),
                    format!("side {side} does not run along any side of the outer outline: remove it from the outer walls"),
                    format!("el lado {side} no va a lo largo de ningún lado del polígono exterior: quítelo de los muros exteriores"),
                    format!("le côté {side} ne longe aucun côté du polygone extérieur : retirez-le des murs extérieurs"),
                    format!("{side} kenarı dış poligonun hiçbir kenarı boyunca uzanmıyor: onu dış duvarlardan çıkarın"),
                    format!("o lado {side} não segue nenhum lado do polígono exterior: retire-o das paredes exteriores"),
                    format!("bok {side} nie biegnie wzdłuż żadnego boku obrysu zewnętrznego: usuń go ze ścian zewnętrznych"),
                ));
            };
            let ext = tr!("est.", "ext.", "ext.", "ext.", "dış", "ext.", "zewn.");
            let side_name = format!("{}–{}", self.corner(room, i), self.corner(room, j));
            for c in [a, b] {
                out.push(Measure {
                    meas: Meas::Perp { p: c, a: p, b: q, d: t },
                    name: format!("{ext} {side_name} · {} / {}–{}", self.names[c], self.names[p], self.names[q]),
                    kind: Kind::Outer,
                });
            }
        }
        Ok(())
    }

    fn parse_constraints(&self, saved: &Saved, parse_num: &dyn Fn(&str) -> Option<f64>, out: &mut Vec<Measure>) -> Result<(), String> {
        for (k, cells) in saved.constraints.iter().enumerate() {
            let cell = |c: usize| cells.get(c).map(|s| s.trim().to_string()).unwrap_or_default();
            let (at, what) = (cell(0), cell(1));
            if at.is_empty() && what.is_empty() {
                continue;
            }
            let r = k + 1;
            let low = what.to_lowercase();
            let low = low.trim_end_matches('°').trim();
            if let Some((room, i)) = self.parse_corner(&at) {
                let deg = if low.starts_with("rett") || low == "r" || low.starts_with("right") {
                    Some(90.0)
                } else {
                    parse_num(low)
                };
                let Some(mut deg) = deg.filter(|d| *d > 0.0 && *d < 360.0) else {
                    return Err(tr!(
                        format!("vincolo alla riga {r}: per lo spigolo {at} scrivi l'angolo in gradi (per esempio 90)"),
                        format!("constraint on row {r}: for corner {at} type the angle in degrees (for example 90)"),
                        format!("restricción en la fila {r}: para la esquina {at} escriba el ángulo en grados (por ejemplo 90)"),
                        format!("contrainte à la ligne {r} : pour l'angle {at}, saisissez l'angle en degrés (par exemple 90)"),
                        format!("{r}. satırdaki kısıt: {at} köşesi için açıyı derece olarak girin (örneğin 90)"),
                        format!("restrição na linha {r}: para o canto {at} introduza o ângulo em graus (por exemplo 90)"),
                        format!("warunek w wierszu {r}: dla narożnika {at} wpisz kąt w stopniach (np. 90)"),
                    ));
                };
                if deg > 180.0 {
                    deg = 360.0 - deg;
                }
                if (deg - 180.0).abs() < 1e-9 {
                    return Err(tr!(
                        format!("vincolo alla riga {r}: un angolo piatto (180°) non serve; se {at} non è uno spigolo, toglilo dalla polilinea"),
                        format!("constraint on row {r}: a straight angle (180°) is not needed; if {at} is not a corner, remove it from the polyline"),
                        format!("restricción en la fila {r}: un ángulo llano (180°) no hace falta; si {at} no es una esquina, quítela de la polilínea"),
                        format!("contrainte à la ligne {r} : un angle plat (180°) est inutile ; si {at} n'est pas un angle, retirez-le de la polyligne"),
                        format!("{r}. satırdaki kısıt: düz açı (180°) gereksiz; {at} köşe değilse çoklu çizgiden kaldırın"),
                        format!("restrição na linha {r}: um ângulo raso (180°) não é necessário; se {at} não é um canto, retire-o da polilinha"),
                        format!("warunek w wierszu {r}: kąt półpełny (180°) jest zbędny; jeśli {at} nie jest narożnikiem, usuń go z polilinii"),
                    ));
                }
                let n = self.rooms[room].len();
                let ids = &self.point_of[room];
                out.push(Measure {
                    meas: Meas::Angle { a: ids[(i + n - 1) % n], b: ids[i], c: ids[(i + 1) % n], rad: deg.to_radians() },
                    name: format!("{} {}", tr!("ang.", "ang.", "áng.", "ang.", "açı", "âng.", "kąt"), self.corner(room, i)),
                    kind: Kind::Angle,
                });
            } else if let Some((room, i, j)) = self.parse_room_side(&at).or_else(|| {
                // Any two corners will do for horizontal / vertical.
                let (a, b) = self.parse_side(&at)?;
                let ra = self.rooms_of(a)[0];
                let ia = self.point_of[ra].iter().position(|&p| p == a)?;
                let ib = self.point_of[ra].iter().position(|&p| p == b)?;
                Some((ra, ia, ib))
            }) {
                // Every language of the plugin: orizzontale, horizontal,
                // yatay, poziomy / verticale, vertical, dikey, pionowy.
                let vertical = if ["o", "h", "y"].contains(&low) || ["oriz", "hor", "yat", "poz"].iter().any(|p| low.starts_with(p)) {
                    false
                } else if ["v", "d"].contains(&low) || ["vert", "dik", "pion"].iter().any(|p| low.starts_with(p)) {
                    true
                } else {
                    return Err(tr!(
                        format!("vincolo alla riga {r}: per il lato {at} scrivi orizzontale oppure verticale"),
                        format!("constraint on row {r}: for side {at} type horizontal or vertical"),
                        format!("restricción en la fila {r}: para el lado {at} escriba horizontal o vertical"),
                        format!("contrainte à la ligne {r} : pour le côté {at}, saisissez horizontal ou vertical"),
                        format!("{r}. satırdaki kısıt: {at} kenarı için yatay veya dikey girin"),
                        format!("restrição na linha {r}: para o lado {at} introduza horizontal ou vertical"),
                        format!("warunek w wierszu {r}: dla boku {at} wpisz poziomy lub pionowy"),
                    ));
                };
                out.push(Measure {
                    meas: Meas::Level { a: self.point_of[room][i], b: self.point_of[room][j], vertical },
                    name: format!(
                        "{}–{} {}",
                        self.corner(room, i),
                        self.corner(room, j),
                        if vertical { tr!("vert.", "vert.", "vert.", "vert.", "dikey", "vert.", "pion.") } else { tr!("orizz.", "horiz.", "horiz.", "horiz.", "yatay", "horiz.", "poz.") }
                    ),
                    kind: Kind::Level,
                });
            } else {
                return Err(tr!(
                    format!("vincolo alla riga {r}: \"{at}\" non è uno spigolo né un lato ({})", self.ranges()),
                    format!("constraint on row {r}: \"{at}\" is neither a corner nor a side ({})", self.ranges()),
                    format!("restricción en la fila {r}: \"{at}\" no es ni una esquina ni un lado ({})", self.ranges()),
                    format!("contrainte à la ligne {r} : \"{at}\" n'est ni un angle ni un côté ({})", self.ranges()),
                    format!("{r}. satırdaki kısıt: \"{at}\" ne köşe ne de kenar ({})", self.ranges()),
                    format!("restrição na linha {r}: \"{at}\" não é nem um canto nem um lado ({})", self.ranges()),
                    format!("warunek w wierszu {r}: \"{at}\" nie jest ani narożnikiem, ani bokiem ({})", self.ranges()),
                ));
            }
        }
        Ok(())
    }

    fn parse_openings(&self, saved: &Saved, parse_num: &dyn Fn(&str) -> Option<f64>, out: &mut Vec<Measure>) -> Result<Vec<Opening>, String> {
        let mut ops: Vec<Opening> = Vec::new();
        for (k, cells) in saved.openings.iter().enumerate() {
            let cell = |c: usize| cells.get(c).map(|s| s.trim().to_string()).unwrap_or_default();
            if (0..6).all(|c| cell(c).is_empty()) {
                continue;
            }
            let r = k + 1;
            let name = if cell(0).is_empty() { format!("{r}") } else { cell(0) };
            let Some((room, i, j)) = self.parse_room_side(&cell(1)) else {
                return Err(tr!(
                    format!("porta o finestra alla riga {r}: \"{}\" non è un lato di un vano (scrivi due spigoli vicini, per esempio {}-{})", cell(1), self.corner(0, 0), self.corner(0, 1)),
                    format!("door or window on row {r}: \"{}\" is not a room side (write two neighbouring corners, for example {}-{})", cell(1), self.corner(0, 0), self.corner(0, 1)),
                    format!("puerta o ventana en la fila {r}: \"{}\" no es un lado de estancia (escriba dos esquinas contiguas, por ejemplo {}-{})", cell(1), self.corner(0, 0), self.corner(0, 1)),
                    format!("porte ou fenêtre à la ligne {r} : \"{}\" n'est pas un côté de pièce (écrivez deux angles voisins, par exemple {}-{})", cell(1), self.corner(0, 0), self.corner(0, 1)),
                    format!("{r}. satırdaki kapı veya pencere: \"{}\" bir oda kenarı değil (iki komşu köşe yazın, örneğin {}-{})", cell(1), self.corner(0, 0), self.corner(0, 1)),
                    format!("porta ou janela na linha {r}: \"{}\" não é um lado de compartimento (escreva dois cantos vizinhos, por exemplo {}-{})", cell(1), self.corner(0, 0), self.corner(0, 1)),
                    format!("drzwi lub okno w wierszu {r}: \"{}\" nie jest bokiem pomieszczenia (wpisz dwa sąsiednie narożniki, np. {}-{})", cell(1), self.corner(0, 0), self.corner(0, 1)),
                ));
            };
            let t = cell(2).to_lowercase();
            let kind = if let Some(k) = op_kind(&t) {
                k
            } else {
                return Err(tr!(
                    format!("riga {r} di porte e finestre: tipo \"{}\" sconosciuto (porta, finestra o apertura)", cell(2)),
                    format!("row {r} of doors and windows: unknown kind \"{}\" (door, window or opening)", cell(2)),
                    format!("fila {r} de puertas y ventanas: tipo \"{}\" desconocido (puerta, ventana o abertura)", cell(2)),
                    format!("ligne {r} des portes et fenêtres : type \"{}\" inconnu (porte, fenêtre ou ouverture)", cell(2)),
                    format!("kapı ve pencerelerin {r}. satırı: bilinmeyen tür \"{}\" (kapı, pencere veya açıklık)", cell(2)),
                    format!("linha {r} das portas e janelas: tipo \"{}\" desconhecido (porta, janela ou abertura)", cell(2)),
                    format!("wiersz {r} drzwi i okien: nieznany typ \"{}\" (drzwi, okno lub otwór)", cell(2)),
                ));
            };
            let s0 = parse_num(&cell(3)).filter(|v| *v >= 0.0);
            let w = parse_num(&cell(4)).filter(|v| *v > 0.0);
            let (Some(s0), Some(w)) = (s0, w) else {
                return Err(tr!(
                    format!("{name}: scrivi la distanza dallo spigolo {} e la larghezza", self.corner(room, i)),
                    format!("{name}: type the distance from corner {} and the width", self.corner(room, i)),
                    format!("{name}: escriba la distancia desde la esquina {} y la anchura", self.corner(room, i)),
                    format!("{name} : saisissez la distance depuis l'angle {} et la largeur", self.corner(room, i)),
                    format!("{name}: {} köşesinden uzaklığı ve genişliği girin", self.corner(room, i)),
                    format!("{name}: introduza a distância a partir do canto {} e a largura", self.corner(room, i)),
                    format!("{name}: wpisz odległość od narożnika {} i szerokość", self.corner(room, i)),
                ));
            };
            let thick = parse_num(&cell(5)).filter(|v| *v > 0.0);
            ops.push(Opening { name, kind, room, a: self.point_of[room][i], b: self.point_of[room][j], s: s0, w, thick, twin: None });
        }
        // Same name in two rooms: the same door, seen from both sides.
        for x in 0..ops.len() {
            let same: Vec<usize> = (0..ops.len()).filter(|&y| ops[y].name.eq_ignore_ascii_case(&ops[x].name)).collect();
            if same.len() < 2 || same[0] != x {
                continue;
            }
            if same.len() > 2 || ops[same[0]].room == ops[same[1]].room {
                return Err(tr!(
                    format!("il nome {} è usato più volte: una porta comune va scritta una volta per ciascuno dei due vani", ops[x].name),
                    format!("the name {} is used too often: a shared door is written once for each of the two rooms", ops[x].name),
                    format!("el nombre {} se usa demasiadas veces: una puerta común se escribe una vez para cada una de las dos estancias", ops[x].name),
                    format!("le nom {} est utilisé trop de fois : une porte commune s'écrit une fois pour chacune des deux pièces", ops[x].name),
                    format!("{} adı çok fazla kez kullanılmış: ortak kapı iki odanın her biri için bir kez yazılır", ops[x].name),
                    format!("o nome {} é usado vezes demais: uma porta comum escreve-se uma vez para cada um dos dois compartimentos", ops[x].name),
                    format!("nazwa {} jest użyta zbyt wiele razy: wspólne drzwi wpisuje się raz dla każdego z dwóch pomieszczeń", ops[x].name),
                ));
            }
            let (o1, o2) = (same[0], same[1]);
            let Some(t) = ops[o1].thick.or(ops[o2].thick) else {
                return Err(tr!(
                    format!("la porta {} è comune a due vani: scrivi lo spessore del muro", ops[x].name),
                    format!("door {} is shared by two rooms: type the wall thickness", ops[x].name),
                    format!("la puerta {} es común a dos estancias: escriba el espesor del muro", ops[x].name),
                    format!("la porte {} est commune à deux pièces : saisissez l'épaisseur du mur", ops[x].name),
                    format!("{} kapısı iki odada ortak: duvar kalınlığını girin", ops[x].name),
                    format!("a porta {} é comum a dois compartimentos: introduza a espessura da parede", ops[x].name),
                    format!("drzwi {} są wspólne dla dwóch pomieszczeń: wpisz grubość ściany", ops[x].name),
                ));
            };
            ops[o1].twin = Some(o2);
            ops[o2].twin = Some(o1);
            ops[o1].thick = Some(t);
            ops[o2].thick = Some(t);
            let (p, q) = (&ops[o1], &ops[o2]);
            let j1 = At { a: p.a, b: p.b, s: p.s };
            let k1 = At { a: p.a, b: p.b, s: p.s + p.w };
            let j2 = At { a: q.a, b: q.b, s: q.s };
            let k2 = At { a: q.a, b: q.b, s: q.s + q.w };
            // Which jamb faces which: as drawn.
            let pts = &self.points;
            let cross = solve::dist(j1.eval(pts), k2.eval(pts)) + solve::dist(k1.eval(pts), j2.eval(pts));
            let straight = solve::dist(j1.eval(pts), j2.eval(pts)) + solve::dist(k1.eval(pts), k2.eval(pts));
            let partner = if cross <= straight { k2 } else { j2 };
            let nm = |n: usize| format!("{} {} ({n})", tr!("porta", "door", "puerta", "porte", "kapı", "porta", "drzwi"), p.name);
            out.push(Measure { meas: Meas::PerpAt { p: j2, a: p.a, b: p.b, d: t }, name: nm(1), kind: Kind::Door });
            out.push(Measure { meas: Meas::PerpAt { p: k2, a: p.a, b: p.b, d: t }, name: nm(2), kind: Kind::Door });
            // The facing jambs square across the wall: no shift along it.
            out.push(Measure { meas: Meas::Along { p: j1, q: partner, a: p.a, b: p.b, d: 0.0 }, name: nm(3), kind: Kind::Door });
        }
        Ok(ops)
    }

    /// Stand-in measures taken from the drawing, to finish a shape the real
    /// measures leave free (furniture in the way of a diagonal, a link not
    /// taken): the drawn angle at every corner and, between rooms the real
    /// measures already tie together, their drawn pose (turn and position of
    /// corner A). Used with a low
    /// weight, so where real measures exist they win.
    pub fn stand_ins(&self, real: &[Meas]) -> Vec<Meas> {
        let mut out = Vec::new();
        for ids in &self.point_of {
            let n = ids.len();
            for i in 0..n {
                let (a, b, c) = (ids[(i + n - 1) % n], ids[i], ids[(i + 1) % n]);
                if a == b || b == c || a == c {
                    continue;
                }
                let rad = solve::angle_at(self.points[a], self.points[b], self.points[c]);
                if rad > 1e-6 && rad < std::f64::consts::PI - 1e-6 {
                    out.push(Meas::Angle { a, b, c, rad });
                }
            }
        }
        let groups = solve::groups(self.points.len(), real);
        let group_of = |p: usize| groups.iter().position(|g| g.contains(&p));
        for r in 1..self.rooms.len() {
            let Some(gr) = group_of(self.point_of[r][0]) else { continue };
            let Some(q) = (0..r).find(|&q| group_of(self.point_of[q][0]) == Some(gr)) else { continue };
            // The second room turned as drawn, and its corner A where it was
            // drawn relative to the first room's corner A. Only the pose: no drawn length that could fight the
            // measured sizes.
            let (q0, q1, r0, r1) = (self.point_of[q][0], self.point_of[q][1], self.point_of[r][0], self.point_of[r][1]);
            if q0 == q1 || r0 == r1 || r0 == q0 || r0 == q1 {
                continue;
            }
            let (pq0, pq1, pr0, pr1) = (self.points[q0], self.points[q1], self.points[r0], self.points[r1]);
            let (u, v) = ([pq1[0] - pq0[0], pq1[1] - pq0[1]], [pr1[0] - pr0[0], pr1[1] - pr0[1]]);
            let turn = (u[0] * v[1] - u[1] * v[0]).atan2(u[0] * v[0] + u[1] * v[1]);
            out.push(Meas::Turn { a: q0, b: q1, c: r0, d: r1, rad: turn });
            let l = u[0].hypot(u[1]);
            let along = ((pr0[0] - pq0[0]) * u[0] + (pr0[1] - pq0[1]) * u[1]) / l;
            out.push(Meas::Along { p: At::corner(q0), q: At::corner(r0), a: q0, b: q1, d: along });
            // Second component along the first room's other side at A (never
            // parallel to A–B): together they place corner A.
            let qn = *self.point_of[q].last().unwrap_or(&q0);
            let w = [self.points[qn][0] - pq0[0], self.points[qn][1] - pq0[1]];
            let lw = w[0].hypot(w[1]);
            if qn != q0 && lw > 0.0 {
                let along2 = ((pr0[0] - pq0[0]) * w[0] + (pr0[1] - pq0[1]) * w[1]) / lw;
                out.push(Meas::Along { p: At::corner(q0), q: At::corner(r0), a: q0, b: qn, d: along2 });
            }
        }
        out
    }

    /// Hints for the solver: each room's corner A fixed, A→B kept.
    pub fn datum(&self) -> Vec<(usize, usize)> {
        self.point_of.iter().map(|ids| (ids[0], ids[1])).collect()
    }

    /// Adjusted corners of room `r`.
    pub fn room_points(&self, r: usize, pts: &[P2]) -> Vec<P2> {
        self.point_of[r].iter().map(|&p| pts[p]).collect()
    }

    /// Rooms touched by a group of points, as 1-based numbers.
    pub fn room_numbers(&self, group: &[usize]) -> Vec<usize> {
        let mut v: Vec<usize> = group.iter().flat_map(|&p| self.rooms_of(p)).map(|r| r + 1).collect();
        v.sort();
        v.dedup();
        v
    }
}

/// Whether point `p` lies inside polygon `poly`.
pub fn inside(poly: &[P2], p: P2) -> bool {
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

fn corner_name(multi: bool, r: usize, i: usize) -> String {
    if multi {
        format!("{}{}", r + 1, label(i))
    } else {
        label(i)
    }
}

/// Door, window or opening, from a word in any language of the plugin (or
/// its first letters).
fn op_kind(t: &str) -> Option<OpKind> {
    const WORDS: [(OpKind, &[&str]); 3] = [
        (OpKind::Door, &["porta", "door", "puerta", "porte", "kapı", "kapi", "drzwi"]),
        (OpKind::Window, &["finestra", "window", "ventana", "fenêtre", "fenetre", "pencere", "janela", "okno"]),
        (OpKind::Opening, &["apertura", "opening", "abertura", "ouverture", "açıklık", "aciklik", "otwór", "otwor", "vano"]),
    ];
    let t = t.trim().to_lowercase();
    if t.is_empty() {
        return Some(OpKind::Door);
    }
    // "plain opening", "doğramasız açıklık": any word may carry the kind.
    for word in t.split_whitespace().filter(|w| w.chars().count() >= 2) {
        let hits: Vec<OpKind> = WORDS
            .iter()
            .filter(|(_, ws)| ws.iter().any(|w| w.starts_with(word) || word.starts_with(w)))
            .map(|(k, _)| *k)
            .collect();
        if let [k] = hits[..] {
            return Some(k);
        }
    }
    match t.chars().next()? {
        'p' | 'd' | 'k' => Some(OpKind::Door),
        'f' | 'w' | 'j' => Some(OpKind::Window),
        'a' | 'o' | 'v' => Some(OpKind::Opening),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn num(s: &str) -> Option<f64> {
        s.trim().replace(',', ".").parse().ok()
    }

    fn fmt(v: f64, d: usize) -> String {
        format!("{v:.d$}")
    }

    fn two() -> Survey {
        Survey::new(vec![
            vec![[0.0, 0.0], [4.0, 0.0], [4.0, 3.0], [0.0, 3.0]],
            vec![[4.3, 0.0], [7.0, 0.0], [7.0, 3.0], [4.3, 3.0]],
        ])
    }

    #[test]
    fn solid_walls_from_the_hatch() {
        let rooms = vec![
            vec![[0.0, 0.0], [4.0, 0.0], [4.0, 3.0], [0.0, 3.0]],
            vec![[4.3, 0.0], [7.0, 0.0], [7.0, 3.0], [4.3, 3.0]],
        ];
        // No hatch: nothing proposed.
        assert!(Survey::new(rooms.clone()).suggest_inner_walls(1.2).is_empty());
        let wall = vec![vec![[4.0, -0.3], [4.3, -0.3], [4.3, 3.3], [4.0, 3.3]]];
        let s = Survey::new(rooms).with_hatches(vec![wall]);
        let rows = s.suggest_inner_walls(1.2);
        assert_eq!(rows, vec![vec!["1B–1C".to_string(), "2D–2A".to_string(), String::new()]]);
        // An empty thickness takes the shared door's.
        let (sides, _) = s.initial(3, &fmt);
        let mut saved = Saved {
            sides,
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
            replace: false,
            outer: String::new(),
            outer_t: String::new(),
            outer_walls: vec![],
            inner_walls: rows.clone(),
        };
        let err = s.parse(&saved, &num).unwrap_err();
        assert!(err.contains("1B–1C"), "{err}");
        saved.openings = vec![
            vec!["P1".into(), "1B-1C".into(), "porta".into(), "1".into(), "0,9".into(), "0,3".into()],
            vec!["P1".into(), "2D-2A".into(), "porta".into(), "1,1".into(), "0,9".into(), String::new()],
        ];
        let p = s.parse(&saved, &num).unwrap();
        let walls: Vec<&Measure> = p.measures.iter().filter(|m| m.kind == Kind::Wall).collect();
        assert_eq!(walls.len(), 2);
        assert!(walls.iter().all(|m| (m.meas.value() - 0.3).abs() < 1e-12));
        saved.inner_walls[0][2] = "0,32".into();
        let p = s.parse(&saved, &num).unwrap();
        assert!(p.measures.iter().filter(|m| m.kind == Kind::Wall).all(|m| (m.meas.value() - 0.32).abs() < 1e-12));
    }

    #[test]
    fn rooms_held_at_the_outer_wall_thickness() {
        // Outline surveyed exactly: 10 × 8. Two rooms drawn by eye inside it,
        // the real ones 0,30 from the outer face, 0,10 apart.
        let outline = vec![[0.0, 0.0], [10.0, 0.0], [10.0, 8.0], [0.0, 8.0]];
        let s = Survey::new(vec![
            vec![[0.5, 0.2], [5.1, 0.4], [4.9, 7.5], [0.2, 7.8]],
            vec![[5.3, 0.4], [9.6, 0.1], [9.8, 7.6], [5.2, 7.7]],
        ])
        .with_outer(outline.clone());
        assert_eq!(s.names[8], "E1");
        let walls = s.suggest_outer_walls(1.2);
        let listed: Vec<&str> = walls.iter().map(|r| r[0].as_str()).collect();
        assert_eq!(listed, ["1A–1B", "1C–1D", "1D–1A", "2A–2B", "2B–2C", "2C–2D"]);
        let (sides, _) = s.initial(3, &fmt);
        let mut sides = sides;
        for (row, v) in sides.iter_mut().zip(["4,6", "7,4", "4,6", "7,4", "4,5", "7,4", "4,5", "7,4"]) {
            row[2] = v.into();
        }
        let saved = Saved {
            sides,
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
            replace: false,
            outer: "1".into(),
            outer_t: "0,30".into(),
            outer_walls: walls,
                inner_walls: vec![],
        };
        let p = s.parse(&saved, &num).unwrap();
        assert_eq!(p.measures.iter().filter(|m| m.kind == Kind::Outer).count(), 12);
        let meas: Vec<Meas> = p.measures.iter().map(|m| m.meas).collect();
        let w: Vec<f64> = p.measures.iter().map(|m| if m.kind == Kind::Outer { 30.0 } else { 1.0 }).collect();
        let sol = solve::solve_network_fixed(&s.points, &meas, &w, meas.len(), &s.datum(), &s.fixed()).unwrap();
        let close = |a: P2, b: [f64; 2]| solve::dist(a, b) < 1e-6;
        assert!(close(sol.pts[0], [0.3, 0.3]), "{:?}", sol.pts[0]);
        assert!(close(sol.pts[2], [4.9, 7.7]), "{:?}", sol.pts[2]);
        assert!(close(sol.pts[5], [9.7, 0.3]), "{:?}", sol.pts[5]);
        assert!(close(sol.pts[4], [5.2, 0.3]), "{:?}", sol.pts[4]);
        // The outline has not moved.
        for (k, o) in outline.iter().enumerate() {
            assert_eq!(sol.pts[8 + k], *o);
        }
        // Without a thickness the dialog says what is missing.
        let mut no_t = saved.clone();
        no_t.outer_t = String::new();
        assert!(s.parse(&no_t, &num).is_err());
    }

    #[test]
    fn names_and_parsing() {
        let s = two();
        assert_eq!(s.names[0], "1A");
        assert_eq!(s.names[5], "2B");
        assert_eq!(s.parse_point("2b"), Some(5));
        assert_eq!(s.parse_point(" 1 C "), Some(2));
        assert_eq!(s.parse_point("B"), None, "bare letter is ambiguous with several rooms");
        assert_eq!(s.parse_point("3A"), None);
        assert_eq!(s.parse_side("1B-1C"), Some((1, 2)));
        assert_eq!(s.parse_side("1B–1C"), Some((1, 2)));
        let one = Survey::new(vec![s.rooms[0].clone()]);
        assert_eq!(one.parse_point("c"), Some(2));
        assert_eq!(one.names[3], "D");
    }

    #[test]
    fn touching_rooms_share_corners() {
        let s = Survey::new(vec![
            vec![[0.0, 0.0], [4.0, 0.0], [4.0, 3.0], [0.0, 3.0]],
            vec![[4.0, 0.0], [7.0, 0.0], [7.0, 3.0], [4.0, 3.0]],
        ]);
        assert_eq!(s.points.len(), 6);
        assert_eq!(s.parse_point("2A"), Some(1));
        assert_eq!(s.parse_point("2D"), Some(2));
        assert_eq!(s.rooms_of(1), vec![0, 1]);
    }

    #[test]
    fn tables_round_trip() {
        let s = two();
        let (mut sides, mut others) = s.initial(3, &fmt);
        assert_eq!(sides.len(), 8);
        assert_eq!(others.len(), 2);
        assert!(others[0][0].starts_with('1') && others[1][0].starts_with('2'));
        for r in sides.iter_mut() {
            r[2] = r[1].clone();
        }
        others[0][3] = "5,0".into();
        others.push(vec!["1B-1C".into(), "2A".into(), "".into(), "0,30".into()]);
        others.push(vec!["1C".into(), "2a".into(), "".into(), "3,02".into()]);
        let saved = Saved { sides, others, h: 0.2, tol: 0.02, dims: true, table: true, csv: false, csv_path: String::new(), constraints: vec![], openings: vec![], unit: 0, fill: true, replace: false, outer: String::new(), outer_t: String::new(), outer_walls: vec![], inner_walls: vec![] };
        let m = s.parse(&saved, &num).unwrap().measures;
        assert_eq!(m.len(), 8 + 3);
        assert_eq!(m[8].kind, Kind::Diagonal);
        assert_eq!(m[9].kind, Kind::Wall);
        assert_eq!(m[9].meas, Meas::Perp { p: 4, a: 1, b: 2, d: 0.30 });
        assert_eq!(m[10].kind, Kind::Link);
        assert_eq!(m[10].name, "1C–2A");
        assert!((s.drawn_other("1B-1C", "2A").unwrap() - 0.3).abs() < 1e-12);
    }

    #[test]
    fn errors_name_the_row() {
        let s = two();
        let (sides, _) = s.initial(3, &fmt);
        let saved = Saved {
            sides,
            others: vec![vec!["A".into(), "C".into(), "".into(), "5".into()]],
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
            outer: String::new(),
            outer_t: String::new(),
            outer_walls: vec![],
                inner_walls: vec![],
        };
        let e = s.parse(&saved, &num).unwrap_err();
        assert!(e.contains("1") && e.contains("\"A\""), "{e}");
    }
    #[test]
    fn opening_kinds_in_every_language() {
        use OpKind::*;
        for (t, k) in [
            ("porta", Door), ("door", Door), ("puerta", Door), ("porte", Door), ("kapı", Door), ("drzwi", Door), ("", Door),
            ("finestra", Window), ("window", Window), ("ventana", Window), ("fenêtre", Window), ("pencere", Window), ("janela", Window), ("okno", Window), ("f", Window),
            ("apertura senza serramento", Opening), ("plain opening", Opening), ("abertura sin carpintería", Opening),
            ("ouverture sans menuiserie", Opening), ("doğramasız açıklık", Opening), ("abertura sem caixilharia", Opening), ("otwór bez stolarki", Opening),
        ] {
            assert_eq!(op_kind(t), Some(k), "{t}");
        }
        assert_eq!(op_kind("xyz"), None);
    }

    #[test]
    fn constraints_and_openings() {
        let s = two();
        let (sides, _) = s.initial(3, &fmt);
        let saved = Saved {
            sides: sides.iter().map(|r| vec![r[0].clone(), r[1].clone(), r[1].clone()]).collect(),
            others: vec![],
            h: 0.2,
            tol: 0.02,
            dims: true,
            table: true,
            csv: false,
            csv_path: String::new(),
            constraints: vec![
                vec!["1A".into(), "90".into()],
                vec!["2c".into(), "270°".into()],
                vec!["1A-1B".into(), "orizzontale".into()],
                vec!["2B-2C".into(), "V".into()],
            ],
            openings: vec![
                vec!["P1".into(), "1B-1C".into(), "porta".into(), "1,00".into(), "0,90".into(), "0,30".into()],
                vec!["p1".into(), "2D-2A".into(), "".into(), "1,10".into(), "0,90".into(), "".into()],
                vec!["F1".into(), "1D-1C".into(), "finestra".into(), "0,8".into(), "1,2".into(), "".into()],
            ],
            unit: 0,
            fill: true,
            replace: false,
            outer: String::new(),
            outer_t: String::new(),
            outer_walls: vec![],
                inner_walls: vec![],
        };
        let p = s.parse(&saved, &num).unwrap();
        let kinds: Vec<Kind> = p.measures.iter().map(|m| m.kind).collect();
        assert_eq!(kinds.iter().filter(|k| **k == Kind::Angle).count(), 2);
        assert_eq!(kinds.iter().filter(|k| **k == Kind::Level).count(), 2);
        assert_eq!(kinds.iter().filter(|k| **k == Kind::Door).count(), 3);
        let ang = p.measures.iter().find(|m| m.kind == Kind::Angle && m.name.ends_with("2C")).unwrap();
        assert!(matches!(ang.meas, Meas::Angle { rad, .. } if (rad - std::f64::consts::FRAC_PI_2).abs() < 1e-12));
        assert_eq!(p.openings.len(), 3);
        assert_eq!(p.openings[0].twin, Some(1));
        assert_eq!(p.openings[1].thick, Some(0.30));
        assert_eq!(p.openings[2].kind, OpKind::Window);
        // F1 on side D-C: measured from D.
        assert_eq!(p.openings[2].a, 3);
    }

    #[test]
    fn shared_door_needs_the_wall() {
        let s = two();
        let saved = Saved {
            sides: vec![],
            others: vec![],
            h: 0.2,
            tol: 0.02,
            dims: true,
            table: true,
            csv: false,
            csv_path: String::new(),
            constraints: vec![],
            openings: vec![
                vec!["P1".into(), "1B-1C".into(), "porta".into(), "1".into(), "0,9".into(), "".into()],
                vec!["P1".into(), "2D-2A".into(), "porta".into(), "1".into(), "0,9".into(), "".into()],
            ],
            unit: 0,
            fill: true,
            replace: false,
            outer: String::new(),
            outer_t: String::new(),
            outer_walls: vec![],
                inner_walls: vec![],
        };
        assert!(s.parse(&saved, &num).unwrap_err().contains("P1"));
    }
}
