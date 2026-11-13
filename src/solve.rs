//! The survey maths, free of any host type so it can be unit-tested alone.
//!
//! A room is a closed polygon of `n` vertices. In the field one measures its
//! sides and some diagonals with a tape or a laser meter; every measure is a
//! distance between two vertices. Fixing vertex 0 and the direction of the
//! side 0→1 removes the free movement on the sheet (2 shifts + 1 rotation),
//! so the shape has `2n − 3` unknowns and needs at least that many distances:
//! the `n` sides plus `n − 3` diagonals. More measures than that are a
//! check: the shape is then found by least squares and every measure gets
//! its residual (computed − measured), as in a Pregeo adjustment.
//!
//! The solution starts from the drawn shape (scaled to the measures), so
//! the result keeps the drawn arrangement — a measured triangle is never
//! mirrored onto the wrong side of its base.

pub type P2 = [f64; 2];

/// One measured distance between vertices `i` and `j`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Obs {
    pub i: usize,
    pub j: usize,
    pub d: f64,
}

#[derive(Clone, Debug)]
pub struct Solution {
    /// The adjusted vertices, in the drawn order.
    pub pts: Vec<P2>,
    /// Computed − measured, one per observation, in the given order.
    pub residuals: Vec<f64>,
    /// Measures beyond the minimum (0: no check is possible).
    pub redundancy: usize,
    /// Standard deviation of a single measure, when `redundancy > 0`.
    pub sigma0: Option<f64>,
    pub iterations: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub enum SolveError {
    /// Fewer than 3 vertices.
    TooFewPoints,
    /// A measure joins a vertex to itself, names a missing vertex or is not
    /// a positive number (index into the observations).
    BadObs(usize),
    /// Fewer measures than unknowns.
    NotEnough { needed: usize, have: usize },
    /// Enough measures, but some part of the shape can still move: the
    /// diagonals do not brace every part of the room.
    NotRigid,
    /// The measures contradict each other too much to find a shape (for
    /// example a diagonal longer than two sides together).
    NoConvergence,
}

/// Name of vertex `i`: A … Z, then AA, AB, …
pub fn label(i: usize) -> String {
    let mut n = i;
    let mut s = Vec::new();
    loop {
        s.push((b'A' + (n % 26) as u8) as char);
        if n < 26 {
            break;
        }
        n = n / 26 - 1;
    }
    s.iter().rev().collect()
}

/// Index of a vertex name (`"A"` → 0, `"AA"` → 26); case-insensitive.
pub fn parse_label(s: &str) -> Option<usize> {
    let s = s.trim();
    if s.is_empty() || !s.chars().all(|c| c.is_ascii_alphabetic()) {
        return None;
    }
    let mut n: usize = 0;
    for c in s.chars() {
        let d = (c.to_ascii_uppercase() as u8 - b'A') as usize;
        n = n.checked_mul(26)?.checked_add(d + 1)?;
    }
    Some(n - 1)
}

pub fn dist(a: P2, b: P2) -> f64 {
    (b[0] - a[0]).hypot(b[1] - a[1])
}

pub fn signed_area(p: &[P2]) -> f64 {
    let n = p.len();
    (0..n).map(|i| {
        let (a, b) = (p[i], p[(i + 1) % n]);
        a[0] * b[1] - b[0] * a[1]
    }).sum::<f64>() / 2.0
}

/// Minimum number of measures for `n` vertices.
pub fn needed(n: usize) -> usize {
    (2 * n).saturating_sub(3)
}

/// Unknown layout: x[0] = distance of vertex 1 from vertex 0 along the drawn
/// direction; then (x, y) of vertices 2 … n−1.
fn unpack(x: &[f64], p0: P2, u: P2, n: usize) -> Vec<P2> {
    let mut pts = Vec::with_capacity(n);
    pts.push(p0);
    pts.push([p0[0] + x[0] * u[0], p0[1] + x[0] * u[1]]);
    for k in 2..n {
        pts.push([x[2 * k - 3], x[2 * k - 2]]);
    }
    pts
}

/// Solve the dense system `a · x = b` (Gauss, partial pivoting). `None` when
/// a pivot is negligible against the matrix scale.
fn gauss(mut a: Vec<Vec<f64>>, mut b: Vec<f64>) -> Option<Vec<f64>> {
    let n = b.len();
    let scale = a.iter().flatten().fold(0.0f64, |m, v| m.max(v.abs())).max(1e-300);
    for c in 0..n {
        let p = (c..n).max_by(|&i, &j| a[i][c].abs().total_cmp(&a[j][c].abs()))?;
        if a[p][c].abs() < scale * 1e-10 {
            return None;
        }
        a.swap(c, p);
        b.swap(c, p);
        for r in c + 1..n {
            let f = a[r][c] / a[c][c];
            if f != 0.0 {
                for k in c..n {
                    a[r][k] -= f * a[c][k];
                }
                b[r] -= f * b[c];
            }
        }
    }
    let mut x = vec![0.0; n];
    for r in (0..n).rev() {
        let s: f64 = (r + 1..n).map(|k| a[r][k] * x[k]).sum();
        x[r] = (b[r] - s) / a[r][r];
    }
    Some(x)
}

/// Adjust the drawn polygon `drawn` to the measured distances `obs`.
pub fn solve(drawn: &[P2], obs: &[Obs]) -> Result<Solution, SolveError> {
    let n = drawn.len();
    if n < 3 {
        return Err(SolveError::TooFewPoints);
    }
    for (k, o) in obs.iter().enumerate() {
        if o.i >= n || o.j >= n || o.i == o.j || !(o.d.is_finite() && o.d > 0.0) {
            return Err(SolveError::BadObs(k));
        }
    }
    let unknowns = needed(n);
    if obs.len() < unknowns {
        return Err(SolveError::NotEnough { needed: unknowns, have: obs.len() });
    }

    // Start: the drawn shape scaled about vertex 0 to the measures' size.
    let (sum_m, sum_d) = obs.iter().fold((0.0, 0.0), |(m, d), o| (m + o.d, d + dist(drawn[o.i], drawn[o.j])));
    let s = if sum_d > 0.0 { sum_m / sum_d } else { 1.0 };
    let p0 = drawn[0];
    let l01 = dist(drawn[0], drawn[1]);
    if l01 <= 0.0 {
        return Err(SolveError::NotRigid);
    }
    let u = [(drawn[1][0] - p0[0]) / l01, (drawn[1][1] - p0[1]) / l01];
    let mut x = vec![0.0; unknowns];
    x[0] = l01 * s;
    for k in 2..n {
        x[2 * k - 3] = p0[0] + (drawn[k][0] - p0[0]) * s;
        x[2 * k - 2] = p0[1] + (drawn[k][1] - p0[1]) * s;
    }
    let size = sum_m / obs.len() as f64;

    // Column of a vertex's coordinates in the unknown vector.
    let cols = |v: usize| -> Option<(usize, usize)> {
        if v >= 2 { Some((2 * v - 3, 2 * v - 2)) } else { None }
    };

    let mut iterations = 0;
    let mut converged = false;
    let mut last_rank_ok = true;
    for it in 1..=100 {
        iterations = it;
        let pts = unpack(&x, p0, u, n);
        let mut nm = vec![vec![0.0; unknowns]; unknowns];
        let mut rhs = vec![0.0; unknowns];
        for o in obs {
            let (a, b) = (pts[o.i], pts[o.j]);
            let f = dist(a, b).max(size * 1e-9);
            let g = [(b[0] - a[0]) / f, (b[1] - a[1]) / f];
            // Row of the Jacobian, sparse: (column, value).
            let mut row: Vec<(usize, f64)> = Vec::with_capacity(4);
            for (v, sign) in [(o.i, -1.0), (o.j, 1.0)] {
                match (v, cols(v)) {
                    (_, Some((cx, cy))) => {
                        row.push((cx, sign * g[0]));
                        row.push((cy, sign * g[1]));
                    }
                    (1, None) => row.push((0, sign * (g[0] * u[0] + g[1] * u[1]))),
                    _ => {}
                }
            }
            let w = o.d - f; // misclosure
            for &(c1, v1) in &row {
                rhs[c1] += v1 * w;
                for &(c2, v2) in &row {
                    nm[c1][c2] += v1 * v2;
                }
            }
        }
        let Some(dx) = gauss(nm, rhs) else {
            last_rank_ok = false;
            break;
        };
        // Damp very large steps so a poor start cannot fold the shape.
        let big = dx.iter().fold(0.0f64, |m, v| m.max(v.abs()));
        let k = if big > size { size / big } else { 1.0 };
        for (xi, di) in x.iter_mut().zip(&dx) {
            *xi += k * di;
        }
        if big < size * 1e-12 + 1e-12 {
            converged = true;
            break;
        }
    }
    if !last_rank_ok {
        return Err(SolveError::NotRigid);
    }
    if !converged {
        return Err(SolveError::NoConvergence);
    }

    let pts = unpack(&x, p0, u, n);
    let residuals: Vec<f64> = obs.iter().map(|o| dist(pts[o.i], pts[o.j]) - o.d).collect();
    let redundancy = obs.len() - unknowns;
    let sigma0 = (redundancy > 0)
        .then(|| (residuals.iter().map(|v| v * v).sum::<f64>() / redundancy as f64).sqrt());
    Ok(Solution { pts, residuals, redundancy, sigma0, iterations })
}

/// Whether the segment p–q crosses the segment a–b at an inner point.
fn segments_cross(p: P2, q: P2, a: P2, b: P2) -> bool {
    let o = |a: P2, b: P2, c: P2| (b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0]);
    let (d1, d2, d3, d4) = (o(a, b, p), o(a, b, q), o(p, q, a), o(p, q, b));
    d1 * d2 < 0.0 && d3 * d4 < 0.0
}

fn point_in_triangle(p: P2, a: P2, b: P2, c: P2) -> bool {
    let o = |a: P2, b: P2, c: P2| (b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0]);
    let (d1, d2, d3) = (o(a, b, p), o(b, c, p), o(c, a, p));
    let neg = d1 < 0.0 || d2 < 0.0 || d3 < 0.0;
    let pos = d1 > 0.0 || d2 > 0.0 || d3 > 0.0;
    !(neg && pos)
}

/// The `n − 3` diagonals to measure: a triangulation of the drawn room by
/// ear clipping, so every diagonal runs inside the room (an L-shaped room
/// gets no diagonal through a wall). Falls back to a fan from vertex 0 when
/// the outline crosses itself.
pub fn suggested_diagonals(p: &[P2]) -> Vec<(usize, usize)> {
    let n = p.len();
    if n < 4 {
        return Vec::new();
    }
    let ccw = signed_area(p) > 0.0;
    let mut idx: Vec<usize> = (0..n).collect();
    let mut out = Vec::new();
    let mut guard = 0;
    while idx.len() > 3 && guard < n * n {
        guard += 1;
        let m = idx.len();
        let mut cut = None;
        // Prefer the ear whose diagonal is shortest: short tape lines.
        let mut best = f64::INFINITY;
        for k in 0..m {
            let (ia, ib, ic) = (idx[(k + m - 1) % m], idx[k], idx[(k + 1) % m]);
            let (a, b, c) = (p[ia], p[ib], p[ic]);
            let cross = (b[0] - a[0]) * (c[1] - b[1]) - (b[1] - a[1]) * (c[0] - b[0]);
            let convex = if ccw { cross > 1e-12 } else { cross < -1e-12 };
            if !convex {
                continue;
            }
            let blocked = idx.iter().any(|&j| j != ia && j != ib && j != ic && point_in_triangle(p[j], a, b, c));
            if blocked {
                continue;
            }
            let crosses = (0..n).any(|e| {
                let (s, t) = (e, (e + 1) % n);
                ![ia, ic].contains(&s) && ![ia, ic].contains(&t) && segments_cross(a, c, p[s], p[t])
            });
            if crosses {
                continue;
            }
            let l = dist(a, c);
            if l < best {
                best = l;
                cut = Some(k);
            }
        }
        let Some(k) = cut else { break };
        let (ia, ic) = (idx[(k + m - 1) % m], idx[(k + 1) % m]);
        out.push((ia.min(ic), ia.max(ic)));
        idx.remove(k);
    }
    if out.len() != n - 3 {
        return (2..n - 1).map(|j| (0, j)).collect();
    }
    out.sort();
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn obs_of(p: &[P2], pairs: &[(usize, usize)]) -> Vec<Obs> {
        pairs.iter().map(|&(i, j)| Obs { i, j, d: dist(p[i], p[j]) }).collect()
    }

    fn sides(n: usize) -> Vec<(usize, usize)> {
        (0..n).map(|i| (i, (i + 1) % n)).collect()
    }

    #[test]
    fn labels_round_trip() {
        assert_eq!(label(0), "A");
        assert_eq!(label(25), "Z");
        assert_eq!(label(26), "AA");
        assert_eq!(label(27), "AB");
        for i in 0..800 {
            assert_eq!(parse_label(&label(i)), Some(i));
        }
        assert_eq!(parse_label("b"), Some(1));
        assert_eq!(parse_label("1"), None);
    }

    #[test]
    fn out_of_square_room_is_recovered() {
        // True room: a slightly skewed quadrilateral, 5.12 × 4.03 or so.
        let truth: Vec<P2> = vec![[0.0, 0.0], [5.12, 0.0], [5.20, 4.03], [-0.07, 3.98]];
        // Drawn by eye as a rectangle, at the wrong size, somewhere else.
        let drawn: Vec<P2> = vec![[100.0, 50.0], [105.0, 50.0], [105.0, 54.0], [100.0, 54.0]];
        let mut pairs = sides(4);
        pairs.push((0, 2));
        let sol = solve(&drawn, &obs_of(&truth, &pairs)).unwrap();
        assert_eq!(sol.redundancy, 0);
        for (k, &(i, j)) in pairs.iter().enumerate() {
            assert!((dist(sol.pts[i], sol.pts[j]) - dist(truth[i], truth[j])).abs() < 1e-9, "{k}");
            assert!(sol.residuals[k].abs() < 1e-9);
        }
        // Vertex A stays put, AB keeps its drawn direction.
        assert_eq!(sol.pts[0], drawn[0]);
        assert!((sol.pts[1][1] - 50.0).abs() < 1e-12);
        // Same orientation as drawn.
        assert!(signed_area(&sol.pts) > 0.0);
    }

    #[test]
    fn redundant_measures_give_residuals() {
        let truth: Vec<P2> = vec![[0.0, 0.0], [4.0, 0.0], [4.0, 3.0], [0.0, 3.0]];
        let mut pairs = sides(4);
        pairs.push((0, 2));
        pairs.push((1, 3));
        let mut obs = obs_of(&truth, &pairs);
        obs[5].d += 0.02; // a 2 cm error on the second diagonal
        let sol = solve(&truth, &obs).unwrap();
        assert_eq!(sol.redundancy, 1);
        let s0 = sol.sigma0.unwrap();
        assert!(s0 > 0.001 && s0 < 0.05, "{s0}");
        // The bad diagonal carries the largest residual.
        let worst = sol.residuals.iter().enumerate().max_by(|a, b| a.1.abs().total_cmp(&b.1.abs())).unwrap().0;
        assert!(worst == 4 || worst == 5);
    }

    #[test]
    fn l_shaped_room() {
        let truth: Vec<P2> = vec![[0.0, 0.0], [6.0, 0.0], [6.0, 2.5], [3.1, 2.5], [3.1, 5.0], [0.0, 5.05]];
        let drawn: Vec<P2> = vec![[0.0, 0.0], [6.0, 0.0], [6.0, 2.0], [3.0, 2.0], [3.0, 5.0], [0.0, 5.0]];
        let diags = suggested_diagonals(&drawn);
        assert_eq!(diags.len(), 3);
        // No diagonal through the reflex corner's outside: (2,4) would cut
        // across the missing quarter.
        assert!(!diags.contains(&(2, 4)));
        let mut pairs = sides(6);
        pairs.extend(&diags);
        let sol = solve(&drawn, &obs_of(&truth, &pairs)).unwrap();
        for &(i, j) in &pairs {
            assert!((dist(sol.pts[i], sol.pts[j]) - dist(truth[i], truth[j])).abs() < 1e-8);
        }
        // Every vertex lands on the truth (same datum: A and AB on the x axis).
        for (a, b) in sol.pts.iter().zip(&truth) {
            assert!(dist(*a, *b) < 1e-7, "{a:?} {b:?}");
        }
    }

    #[test]
    fn clockwise_drawing_is_kept_clockwise() {
        let truth: Vec<P2> = vec![[0.0, 0.0], [0.0, 3.0], [4.1, 3.0], [4.0, 0.0]];
        let mut pairs = sides(4);
        pairs.push((1, 3));
        let sol = solve(&truth, &obs_of(&truth, &pairs)).unwrap();
        assert!(signed_area(&sol.pts) < 0.0);
    }

    #[test]
    fn missing_diagonal_is_reported() {
        let p: Vec<P2> = vec![[0.0, 0.0], [4.0, 0.0], [4.0, 3.0], [0.0, 3.0]];
        let e = solve(&p, &obs_of(&p, &sides(4))).unwrap_err();
        assert_eq!(e, SolveError::NotEnough { needed: 5, have: 4 });
    }

    #[test]
    fn unbraced_room_is_not_rigid() {
        // Pentagon: 5 sides + 2 diagonals, but both diagonals brace the same
        // triangle-free part twice (A–C measured twice) — shape can move.
        let p: Vec<P2> = vec![[0.0, 0.0], [4.0, 0.0], [5.0, 3.0], [2.0, 5.0], [-1.0, 3.0]];
        let mut pairs = sides(5);
        pairs.push((0, 2));
        pairs.push((0, 2));
        let e = solve(&p, &obs_of(&p, &pairs)).unwrap_err();
        assert_eq!(e, SolveError::NotRigid);
    }

    #[test]
    fn bad_observation_is_reported() {
        let p: Vec<P2> = vec![[0.0, 0.0], [4.0, 0.0], [4.0, 3.0]];
        let obs = vec![Obs { i: 0, j: 1, d: 4.0 }, Obs { i: 1, j: 1, d: 3.0 }, Obs { i: 2, j: 0, d: 5.0 }];
        assert_eq!(solve(&p, &obs).unwrap_err(), SolveError::BadObs(1));
    }

    #[test]
    fn triangle_needs_no_diagonal() {
        let truth: Vec<P2> = vec![[0.0, 0.0], [3.0, 0.0], [0.0, 4.0]];
        let drawn: Vec<P2> = vec![[0.0, 0.0], [2.0, 0.0], [0.5, 3.0]];
        assert!(suggested_diagonals(&drawn).is_empty());
        let sol = solve(&drawn, &obs_of(&truth, &sides(3))).unwrap();
        assert!((dist(sol.pts[1], sol.pts[2]) - 5.0).abs() < 1e-9);
    }

    #[test]
    fn many_sided_room_converges() {
        // 10-gon, irregular, drawn with errors of a few percent.
        let truth: Vec<P2> = (0..10)
            .map(|k| {
                let a = k as f64 * std::f64::consts::TAU / 10.0;
                let r = 5.0 + (k % 3) as f64 * 0.7;
                [r * a.cos(), r * a.sin()]
            })
            .collect();
        let drawn: Vec<P2> = truth.iter().enumerate().map(|(k, p)| [p[0] * 1.04 + 0.1 * (k as f64).sin(), p[1] * 0.97]).collect();
        let mut pairs = sides(10);
        pairs.extend(suggested_diagonals(&drawn));
        let sol = solve(&drawn, &obs_of(&truth, &pairs)).unwrap();
        for &(i, j) in &pairs {
            assert!((dist(sol.pts[i], sol.pts[j]) - dist(truth[i], truth[j])).abs() < 1e-7);
        }
    }
}
