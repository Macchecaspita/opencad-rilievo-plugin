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

use crate::solve::{self, label, parse_label, Meas, P2};

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
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Side,
    Diagonal,
    Link,
    Wall,
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

    /// A corner name → global point. `"1A"`, `"1 a"`, `"1.A"`; a bare
    /// letter only when there is a single room.
    pub fn parse_point(&self, s: &str) -> Option<usize> {
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
        self.point_of.get(r)?.get(i).copied()
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
    pub fn parse(&self, saved: &Saved, parse_num: &dyn Fn(&str) -> Option<f64>) -> Result<Vec<Measure>, String> {
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
            };
            out.push(Measure { meas, name, kind });
        }
        Ok(out)
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
        let saved = Saved { sides, others, h: 0.2, tol: 0.02, dims: true, table: true, csv: false, csv_path: String::new() };
        let m = s.parse(&saved, &num).unwrap();
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
        };
        let e = s.parse(&saved, &num).unwrap_err();
        assert!(e.contains("1") && e.contains("\"A\""), "{e}");
    }
}
