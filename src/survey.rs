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
        Survey { rooms, point_of, points, names }
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
                            format!("side {name}: \"{cell}\" is not a valid measure")
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
                    format!("write corners as room number and letter ({}); for a wall thickness, a side like 1B-1C and a corner", self.ranges())
                )
            } else {
                tr!(
                    format!("scrivi i due vertici con le loro lettere ({})", self.ranges()),
                    format!("write the two corners by letter ({})", self.ranges())
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
                        format!("row {r} of the other measures (\"{a}\" and \"{b}\"): {hint}")
                    ))
                }
            };
            let d = match parse_num(&m) {
                Some(d) if d > 0.0 => d,
                _ => {
                    return Err(tr!(
                        format!("{name}: \"{m}\" non è una misura valida"),
                        format!("{name}: \"{m}\" is not a valid measure")
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
        Ok(Parsed { measures: out, openings })
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
                        format!("constraint on row {r}: for corner {at} type the angle in degrees (for example 90)")
                    ));
                };
                if deg > 180.0 {
                    deg = 360.0 - deg;
                }
                if (deg - 180.0).abs() < 1e-9 {
                    return Err(tr!(
                        format!("vincolo alla riga {r}: un angolo piatto (180°) non serve; se {at} non è uno spigolo, toglilo dalla polilinea"),
                        format!("constraint on row {r}: a straight angle (180°) is not needed; if {at} is not a corner, remove it from the polyline")
                    ));
                }
                let n = self.rooms[room].len();
                let ids = &self.point_of[room];
                out.push(Measure {
                    meas: Meas::Angle { a: ids[(i + n - 1) % n], b: ids[i], c: ids[(i + 1) % n], rad: deg.to_radians() },
                    name: format!("{} {}", tr!("ang.", "ang."), self.corner(room, i)),
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
                let vertical = if ["o", "or", "orizz", "orizzontale", "h", "horizontal", "oriz"].contains(&low) || low.starts_with("oriz") || low.starts_with("hor") {
                    false
                } else if ["v", "vert", "verticale", "vertical"].contains(&low) || low.starts_with("vert") {
                    true
                } else {
                    return Err(tr!(
                        format!("vincolo alla riga {r}: per il lato {at} scrivi orizzontale oppure verticale"),
                        format!("constraint on row {r}: for side {at} type horizontal or vertical")
                    ));
                };
                out.push(Measure {
                    meas: Meas::Level { a: self.point_of[room][i], b: self.point_of[room][j], vertical },
                    name: format!(
                        "{}–{} {}",
                        self.corner(room, i),
                        self.corner(room, j),
                        if vertical { tr!("vert.", "vert.") } else { tr!("orizz.", "horiz.") }
                    ),
                    kind: Kind::Level,
                });
            } else {
                return Err(tr!(
                    format!("vincolo alla riga {r}: \"{at}\" non è uno spigolo né un lato ({})", self.ranges()),
                    format!("constraint on row {r}: \"{at}\" is neither a corner nor a side ({})", self.ranges())
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
                    format!("door or window on row {r}: \"{}\" is not a room side (write two neighbouring corners, for example {}-{})", cell(1), self.corner(0, 0), self.corner(0, 1))
                ));
            };
            let t = cell(2).to_lowercase();
            let kind = if t.is_empty() || t.starts_with('p') || t.starts_with('d') {
                OpKind::Door
            } else if t.starts_with('f') || t.starts_with('w') {
                OpKind::Window
            } else if t.starts_with('a') || t.starts_with('o') || t.starts_with('v') {
                OpKind::Opening
            } else {
                return Err(tr!(
                    format!("riga {r} di porte e finestre: tipo \"{}\" sconosciuto (porta, finestra o apertura)", cell(2)),
                    format!("row {r} of doors and windows: unknown kind \"{}\" (door, window or opening)", cell(2))
                ));
            };
            let s0 = parse_num(&cell(3)).filter(|v| *v >= 0.0);
            let w = parse_num(&cell(4)).filter(|v| *v > 0.0);
            let (Some(s0), Some(w)) = (s0, w) else {
                return Err(tr!(
                    format!("{name}: scrivi la distanza dallo spigolo {} e la larghezza", self.corner(room, i)),
                    format!("{name}: type the distance from corner {} and the width", self.corner(room, i))
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
                    format!("the name {} is used too often: a shared door is written once for each of the two rooms", ops[x].name)
                ));
            }
            let (o1, o2) = (same[0], same[1]);
            let Some(t) = ops[o1].thick.or(ops[o2].thick) else {
                return Err(tr!(
                    format!("la porta {} è comune a due vani: scrivi lo spessore del muro", ops[x].name),
                    format!("door {} is shared by two rooms: type the wall thickness", ops[x].name)
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
            let nm = |n: usize| format!("{} {} ({n})", tr!("porta", "door"), p.name);
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

fn corner_name(multi: bool, r: usize, i: usize) -> String {
    if multi {
        format!("{}{}", r + 1, label(i))
    } else {
        label(i)
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
        let saved = Saved { sides, others, h: 0.2, tol: 0.02, dims: true, table: true, csv: false, csv_path: String::new(), constraints: vec![], openings: vec![], unit: 0, fill: true };
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
        };
        let e = s.parse(&saved, &num).unwrap_err();
        assert!(e.contains("1") && e.contains("\"A\""), "{e}");
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
        };
        assert!(s.parse(&saved, &num).unwrap_err().contains("P1"));
    }
}
