//! The survey maths, free of any host type so it can be unit-tested alone.
//!
//! The unknowns are plan points (the corners of one or more rooms). Every
//! measure ties some of them together:
//!
//! * [`Meas::Dist`] — a distance between two points, taken with a tape or a
//!   laser meter: a side, a diagonal, or a measure from one room into the
//!   next through a door;
//! * [`Meas::Perp`] — the distance of a point from the line through two
//!   others: the thickness of a wall, from a side of one room to a corner of
//!   the room behind it.
//!
//! Points tied together by measures form a *group*. In each group the
//! first point stays where it was drawn and the direction to the second one
//! is kept, which removes the free movement on the sheet (2 shifts + 1
//! rotation): a group of `m` points has `2m − 3` unknowns and needs at
//! least that many measures. A room of `n` corners needs its `n` sides and
//! `n − 3` diagonals; every further room joined to it needs its own measures
//! plus at least 3 linking ones. Extra measures are checks: the shape is
//! then found by least squares and every measure gets its residual
//! (computed − measured), as in a Pregeo adjustment.
//!
//! The solution starts from the drawn shape (scaled to the measures), so
//! the result keeps the drawn arrangement — a measured triangle is never
//! mirrored onto the wrong side of its base.

pub type P2 = [f64; 2];

/// One measured distance between points `i` and `j` (single-room helper).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Obs {
    pub i: usize,
    pub j: usize,
    pub d: f64,
}

/// A plan point used by a measure: corner `a`, or the point at distance
/// `s` from corner `a` along the side towards corner `b` (a door jamb).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct At {
    pub a: usize,
    pub b: usize,
    pub s: f64,
}

impl At {
    pub fn corner(i: usize) -> Self {
        At { a: i, b: i, s: 0.0 }
    }

    pub fn eval(&self, pts: &[P2]) -> P2 {
        let a = pts[self.a];
        if self.s == 0.0 || self.a == self.b {
            return a;
        }
        let b = pts[self.b];
        let l = dist(a, b).max(1e-300);
        [a[0] + (b[0] - a[0]) * self.s / l, a[1] + (b[1] - a[1]) * self.s / l]
    }
}

/// A measure (or a constraint) between plan points.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Meas {
    /// Distance between points `i` and `j`.
    Dist { i: usize, j: usize, d: f64 },
    /// Distance of point `p` from the line through `a` and `b` (a wall
    /// thickness).
    Perp { p: usize, a: usize, b: usize, d: f64 },
    /// Angle at corner `b` between the sides towards `a` and `c`, in
    /// radians, 0 … π (a reflex corner of 270° is the same as 90°).
    Angle { a: usize, b: usize, c: usize, rad: f64 },
    /// Side `a`–`b` horizontal (or vertical): the difference of y (or x)
    /// of its ends is 0.
    Level { a: usize, b: usize, vertical: bool },
    /// Distance between two points that may lie along sides.
    DistAt { p: At, q: At, d: f64 },
    /// Distance of a point that may lie along a side from the line `a`–`b`.
    PerpAt { p: At, a: usize, b: usize, d: f64 },
    /// How far `q` is from `p` along the direction `a`→`b` (signed): 0
    /// says two door jambs face each other square across the wall.
    Along { p: At, q: At, a: usize, b: usize, d: f64 },
}

fn angle_at(a: P2, b: P2, c: P2) -> f64 {
    let u = [a[0] - b[0], a[1] - b[1]];
    let v = [c[0] - b[0], c[1] - b[1]];
    (u[0] * v[1] - u[1] * v[0]).abs().atan2(u[0] * v[0] + u[1] * v[1])
}

impl Meas {
    /// The measured (or imposed) value.
    pub fn value(&self) -> f64 {
        match *self {
            Meas::Dist { d, .. } | Meas::Perp { d, .. } | Meas::DistAt { d, .. } | Meas::PerpAt { d, .. } | Meas::Along { d, .. } => d,
            Meas::Angle { rad, .. } => rad,
            Meas::Level { .. } => 0.0,
        }
    }

    pub fn points(&self) -> Vec<usize> {
        let mut v = match *self {
            Meas::Dist { i, j, .. } => vec![i, j],
            Meas::Perp { p, a, b, .. } => vec![p, a, b],
            Meas::Angle { a, b, c, .. } => vec![a, b, c],
            Meas::Level { a, b, .. } => vec![a, b],
            Meas::DistAt { p, q, .. } => vec![p.a, p.b, q.a, q.b],
            Meas::PerpAt { p, a, b, .. } => vec![p.a, p.b, a, b],
            Meas::Along { p, q, a, b, .. } => vec![p.a, p.b, q.a, q.b, a, b],
        };
        v.sort();
        v.dedup();
        v
    }

    /// Indices in range, distinct where they must be, value sensible.
    fn valid(&self, n: usize) -> bool {
        let inr = self.points().iter().all(|&p| p < n);
        let ok = match *self {
            Meas::Dist { i, j, d } => i != j && d > 0.0,
            Meas::Perp { p, a, b, d } => p != a && p != b && a != b && d > 0.0,
            Meas::Angle { a, b, c, rad } => a != b && b != c && a != c && rad > 0.0 && rad <= std::f64::consts::PI + 1e-12,
            Meas::Level { a, b, .. } => a != b,
            Meas::DistAt { d, .. } => d > 0.0,
            Meas::PerpAt { a, b, d, .. } => a != b && d > 0.0,
            Meas::Along { a, b, .. } => a != b,
        };
        inr && ok && self.value().is_finite()
    }

    /// Whether the measure ties the drawing to the sheet's axes (so the
    /// group may not keep the drawn direction of its first side).
    pub fn orients(&self) -> bool {
        matches!(self, Meas::Level { .. })
    }

    /// Weight factor turning the value into a length: angles count as the
    /// sideways shift they make over `size`.
    pub fn scale(&self, size: f64) -> f64 {
        match self {
            Meas::Angle { .. } => size,
            _ => 1.0,
        }
    }

    /// The value the measure has on `pts`.
    pub fn compute(&self, pts: &[P2]) -> f64 {
        match *self {
            Meas::Dist { i, j, .. } => dist(pts[i], pts[j]),
            Meas::Perp { p, a, b, .. } => perp_dist(pts[p], pts[a], pts[b]),
            Meas::Angle { a, b, c, .. } => angle_at(pts[a], pts[b], pts[c]),
            Meas::Level { a, b, vertical } => {
                if vertical { pts[b][0] - pts[a][0] } else { pts[b][1] - pts[a][1] }
            }
            Meas::DistAt { p, q, .. } => dist(p.eval(pts), q.eval(pts)),
            Meas::PerpAt { p, a, b, .. } => perp_dist(p.eval(pts), pts[a], pts[b]),
            Meas::Along { p, q, a, b, .. } => {
                let (pp, qq) = (p.eval(pts), q.eval(pts));
                let u = [pts[b][0] - pts[a][0], pts[b][1] - pts[a][1]];
                let l = u[0].hypot(u[1]).max(1e-300);
                ((qq[0] - pp[0]) * u[0] + (qq[1] - pp[1]) * u[1]) / l
            }
        }
    }

    /// Value and partial derivatives `(point, [d/dx, d/dy])`.
    fn linearize(&self, pts: &[P2], size: f64) -> (f64, Vec<(usize, [f64; 2])>) {
        let tiny = size * 1e-9;
        match *self {
            Meas::Dist { i, j, .. } => {
                let (a, b) = (pts[i], pts[j]);
                let f = dist(a, b).max(tiny);
                let g = [(b[0] - a[0]) / f, (b[1] - a[1]) / f];
                (f, vec![(i, [-g[0], -g[1]]), (j, g)])
            }
            Meas::Perp { p, a, b, .. } => {
                let (pp, pa, pb) = (pts[p], pts[a], pts[b]);
                let u = [pb[0] - pa[0], pb[1] - pa[1]];
                let l = (u[0] * u[0] + u[1] * u[1]).sqrt().max(tiny);
                let c = u[0] * (pp[1] - pa[1]) - u[1] * (pp[0] - pa[0]);
                let s = if c < 0.0 { -1.0 } else { 1.0 };
                let f = c.abs() / l;
                let dc_p = [-u[1], u[0]];
                let dc_a = [-(pp[1] - pb[1]), pp[0] - pb[0]];
                let dc_b = [pp[1] - pa[1], -(pp[0] - pa[0])];
                let dl_a = [-u[0] / l, -u[1] / l];
                let dl_b = [u[0] / l, u[1] / l];
                let g = |dc: [f64; 2], dl: [f64; 2]| [s * dc[0] / l - f * dl[0] / l, s * dc[1] / l - f * dl[1] / l];
                (f, vec![(p, g(dc_p, [0.0, 0.0])), (a, g(dc_a, dl_a)), (b, g(dc_b, dl_b))])
            }
            Meas::Level { a, b, vertical } => {
                let f = self.compute(pts);
                let (ga, gb) = if vertical { ([-1.0, 0.0], [1.0, 0.0]) } else { ([0.0, -1.0], [0.0, 1.0]) };
                (f, vec![(a, ga), (b, gb)])
            }
            _ => {
                // Central differences: exact enough, and the same code for
                // every composite measure.
                let f = self.compute(pts);
                let h = size.max(1e-12) * 1e-6;
                let mut q = pts.to_vec();
                let parts = self
                    .points()
                    .into_iter()
                    .map(|p| {
                        let mut g = [0.0; 2];
                        for (c, gc) in g.iter_mut().enumerate() {
                            let x = q[p][c];
                            q[p][c] = x + h;
                            let up = self.compute(&q);
                            q[p][c] = x - h;
                            let dn = self.compute(&q);
                            q[p][c] = x;
                            *gc = (up - dn) / (2.0 * h);
                        }
                        (p, g)
                    })
                    .collect();
                (f, parts)
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct Solution {
    /// The adjusted points, in the given order.
    pub pts: Vec<P2>,
    /// Computed − measured, one per measure, in the given order (angles in
    /// radians).
    pub residuals: Vec<f64>,
    /// The residuals as lengths (angles times the room size): what the
    /// tolerance is checked against.
    pub scaled: Vec<f64>,
    /// Measures beyond the minimum, over all groups (0: no check).
    pub redundancy: usize,
    /// Standard deviation of a single measure, when `redundancy > 0`.
    pub sigma0: Option<f64>,
    /// Number of independent groups solved.
    pub groups: usize,
    pub iterations: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub enum SolveError {
    /// Fewer than 3 points.
    TooFewPoints,
    /// A measure names a missing point, joins a point to itself or is not a
    /// positive number (index into the measures).
    BadObs(usize),
    /// Fewer measures than unknowns in a group (its points listed).
    NotEnough { needed: usize, have: usize, group: Vec<usize> },
    /// Enough measures, but part of the group can still move: the measures
    /// do not brace it (its points listed).
    NotRigid { group: Vec<usize> },
    /// The measures contradict each other too much to find a shape (for
    /// example a diagonal longer than two sides together).
    NoConvergence,
}

/// Name of corner `i`: A … Z, then AA, AB, …
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

/// Index of a corner name (`"A"` → 0, `"AA"` → 26); case-insensitive.
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

/// Distance of `p` from the line through `a` and `b`.
pub fn perp_dist(p: P2, a: P2, b: P2) -> f64 {
    let u = [b[0] - a[0], b[1] - a[1]];
    let l = u[0].hypot(u[1]);
    if l <= 0.0 {
        return dist(p, a);
    }
    (u[0] * (p[1] - a[1]) - u[1] * (p[0] - a[0])).abs() / l
}

/// Foot of the perpendicular from `p` on the line through `a` and `b`.
pub fn foot(p: P2, a: P2, b: P2) -> P2 {
    let u = [b[0] - a[0], b[1] - a[1]];
    let l2 = u[0] * u[0] + u[1] * u[1];
    if l2 <= 0.0 {
        return a;
    }
    let t = ((p[0] - a[0]) * u[0] + (p[1] - a[1]) * u[1]) / l2;
    [a[0] + t * u[0], a[1] + t * u[1]]
}

pub fn signed_area(p: &[P2]) -> f64 {
    let n = p.len();
    (0..n)
        .map(|i| {
            let (a, b) = (p[i], p[(i + 1) % n]);
            a[0] * b[1] - b[0] * a[1]
        })
        .sum::<f64>()
        / 2.0
}

/// Minimum number of measures for a group of `n` points.
pub fn needed(n: usize) -> usize {
    (2 * n).saturating_sub(3)
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

/// Groups of points tied together by measures (union–find). Each group is
/// sorted; groups are ordered by their first point.
pub fn groups(n: usize, meas: &[Meas]) -> Vec<Vec<usize>> {
    let mut parent: Vec<usize> = (0..n).collect();
    fn find(p: &mut Vec<usize>, x: usize) -> usize {
        let mut r = x;
        while p[r] != r {
            r = p[r];
        }
        let mut y = x;
        while p[y] != r {
            let next = p[y];
            p[y] = r;
            y = next;
        }
        r
    }
    for m in meas {
        let pts = m.points();
        for w in pts.windows(2) {
            let (a, b) = (find(&mut parent, w[0]), find(&mut parent, w[1]));
            if a != b {
                parent[a.max(b)] = a.min(b);
            }
        }
    }
    let mut out: Vec<Vec<usize>> = Vec::new();
    let mut index = std::collections::HashMap::new();
    for i in 0..n {
        let r = find(&mut parent, i);
        let k = *index.entry(r).or_insert_with(|| {
            out.push(Vec::new());
            out.len() - 1
        });
        out[k].push(i);
    }
    out
}

/// Single room: adjust the drawn polygon to measured distances.
pub fn solve(drawn: &[P2], obs: &[Obs]) -> Result<Solution, SolveError> {
    let meas: Vec<Meas> = obs.iter().map(|o| Meas::Dist { i: o.i, j: o.j, d: o.d }).collect();
    solve_network(drawn, &meas, &[])
}

/// Unknowns of a group of `m` points: `2m − 3`, or `2m − 2` when some
/// measure ties it to the sheet's axes (then only its first point is held).
fn unknowns_of(m: usize, orients: bool) -> usize {
    if orients { (2 * m).saturating_sub(2) } else { needed(m) }
}

/// Adjust the drawn points `init` to the measures. `datum_hint` may name,
/// for a group, the pair (fixed point, direction point) to use; otherwise
/// the group's first point is fixed and the direction to the first point it
/// is measured to is kept.
pub fn solve_network(init: &[P2], meas: &[Meas], datum_hint: &[(usize, usize)]) -> Result<Solution, SolveError> {
    let n = init.len();
    if n < 3 {
        return Err(SolveError::TooFewPoints);
    }
    for (k, m) in meas.iter().enumerate() {
        if !m.valid(n) {
            return Err(SolveError::BadObs(k));
        }
    }

    let mut pts = init.to_vec();
    let mut iterations = 0;
    let mut redundancy = 0;
    let all_groups = groups(n, meas);
    for group in &all_groups {
        let local: Vec<usize> = meas
            .iter()
            .enumerate()
            .filter(|(_, m)| group.binary_search(&m.points()[0]).is_ok())
            .map(|(k, _)| k)
            .collect();
        let gm: Vec<Meas> = local.iter().map(|&k| meas[k]).collect();
        let orients = gm.iter().any(Meas::orients);
        let unknowns = unknowns_of(group.len(), orients);
        if group.len() < 3 || local.len() < unknowns {
            return Err(SolveError::NotEnough { needed: unknowns.max(3), have: local.len(), group: group.clone() });
        }
        let (p0, p1) = datum_hint
            .iter()
            .copied()
            .find(|(a, b)| group.contains(a) && group.contains(b) && a != b)
            .unwrap_or_else(|| {
                let p0 = group[0];
                let p1 = gm
                    .iter()
                    .filter_map(|m| match *m {
                        Meas::Dist { i, j, .. } if i == p0 => Some(j),
                        Meas::Dist { i, j, .. } if j == p0 => Some(i),
                        _ => None,
                    })
                    .min()
                    .unwrap_or(group[1]);
                (p0, p1)
            });
        let it = solve_group(&mut pts, group, &gm, p0, p1, orients)?;
        iterations = iterations.max(it);
        redundancy += local.len() - unknowns;
    }

    let size = mean_size(meas);
    let residuals: Vec<f64> = meas.iter().map(|m| m.compute(&pts) - m.value()).collect();
    let scaled: Vec<f64> = meas.iter().zip(&residuals).map(|(m, v)| v * m.scale(size)).collect();
    let sigma0 = (redundancy > 0).then(|| (scaled.iter().map(|v| v * v).sum::<f64>() / redundancy as f64).sqrt());
    Ok(Solution { pts, residuals, scaled, redundancy, sigma0, groups: all_groups.len(), iterations })
}

/// Mean length of the length measures (the scale of the room).
fn mean_size(meas: &[Meas]) -> f64 {
    let l: Vec<f64> = meas
        .iter()
        .filter(|m| matches!(m, Meas::Dist { .. } | Meas::DistAt { .. }))
        .map(|m| m.value())
        .collect();
    if l.is_empty() { 1.0 } else { l.iter().sum::<f64>() / l.len() as f64 }
}

/// Gauss–Newton on one group; writes the result into `pts`.
fn solve_group(pts: &mut [P2], group: &[usize], meas: &[Meas], p0: usize, p1: usize, orients: bool) -> Result<usize, SolveError> {
    let m = group.len();
    let unknowns = unknowns_of(m, orients);
    let fixed = pts[p0];
    let l01 = dist(pts[p0], pts[p1]);
    if l01 <= 0.0 {
        return Err(SolveError::NotRigid { group: group.to_vec() });
    }
    let u = [(pts[p1][0] - fixed[0]) / l01, (pts[p1][1] - fixed[1]) / l01];

    // Start: the drawn group scaled about p0 to the measures' size.
    let lengths: Vec<&Meas> = meas.iter().filter(|x| matches!(x, Meas::Dist { .. } | Meas::DistAt { .. })).collect();
    let (sum_m, sum_d) = lengths.iter().fold((0.0, 0.0), |(a, b), x| (a + x.value(), b + x.compute(pts)));
    let s = if sum_d > 0.0 && sum_m > 0.0 { sum_m / sum_d } else { 1.0 };
    for &p in group {
        pts[p] = [fixed[0] + (pts[p][0] - fixed[0]) * s, fixed[1] + (pts[p][1] - fixed[1]) * s];
    }
    let size = mean_size(meas).max(1e-12);

    // Unknown columns. Held direction: p1 → one (distance along u). Free
    // rotation: p1 → two like every other point.
    let mut col: std::collections::HashMap<usize, usize> = std::collections::HashMap::new();
    let mut next = if orients { 0 } else { 1 };
    for &p in group {
        if p != p0 && (orients || p != p1) {
            col.insert(p, next);
            next += 2;
        }
    }
    debug_assert_eq!(next, unknowns);

    let mut last_ok = true;
    let mut converged = false;
    let mut iterations = 0;
    for it in 1..=200 {
        iterations = it;
        let mut nm = vec![vec![0.0; unknowns]; unknowns];
        let mut rhs = vec![0.0; unknowns];
        for x in meas {
            let (f, parts) = x.linearize(pts, size);
            let w8 = x.scale(size);
            let mut row: Vec<(usize, f64)> = Vec::with_capacity(8);
            for (p, g) in parts {
                if p == p0 {
                    continue;
                }
                if p == p1 && !orients {
                    row.push((0, w8 * (g[0] * u[0] + g[1] * u[1])));
                } else if let Some(&c) = col.get(&p) {
                    row.push((c, w8 * g[0]));
                    row.push((c + 1, w8 * g[1]));
                }
            }
            let w = w8 * (x.value() - f);
            for &(c1, v1) in &row {
                rhs[c1] += v1 * w;
                for &(c2, v2) in &row {
                    nm[c1][c2] += v1 * v2;
                }
            }
        }
        let Some(dx) = gauss(nm, rhs) else {
            last_ok = false;
            break;
        };
        let big = dx.iter().fold(0.0f64, |a, v| a.max(v.abs()));
        let k = if big > size { size / big } else { 1.0 };
        if !orients {
            let t = dist(fixed, pts[p1]) + k * dx[0];
            pts[p1] = [fixed[0] + t * u[0], fixed[1] + t * u[1]];
        }
        for (&p, &c) in &col {
            pts[p][0] += k * dx[c];
            pts[p][1] += k * dx[c + 1];
        }
        if big < size * 1e-10 {
            converged = true;
            break;
        }
    }
    if !last_ok {
        return Err(SolveError::NotRigid { group: group.to_vec() });
    }
    if !converged {
        return Err(SolveError::NoConvergence);
    }
    Ok(iterations)
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
        let truth: Vec<P2> = vec![[0.0, 0.0], [5.12, 0.0], [5.20, 4.03], [-0.07, 3.98]];
        let drawn: Vec<P2> = vec![[100.0, 50.0], [105.0, 50.0], [105.0, 54.0], [100.0, 54.0]];
        let mut pairs = sides(4);
        pairs.push((0, 2));
        let sol = solve(&drawn, &obs_of(&truth, &pairs)).unwrap();
        assert_eq!(sol.redundancy, 0);
        for (k, &(i, j)) in pairs.iter().enumerate() {
            assert!((dist(sol.pts[i], sol.pts[j]) - dist(truth[i], truth[j])).abs() < 1e-9, "{k}");
            assert!(sol.residuals[k].abs() < 1e-9);
        }
        assert_eq!(sol.pts[0], drawn[0]);
        assert!((sol.pts[1][1] - 50.0).abs() < 1e-12);
        assert!(signed_area(&sol.pts) > 0.0);
    }

    #[test]
    fn redundant_measures_give_residuals() {
        let truth: Vec<P2> = vec![[0.0, 0.0], [4.0, 0.0], [4.0, 3.0], [0.0, 3.0]];
        let mut pairs = sides(4);
        pairs.push((0, 2));
        pairs.push((1, 3));
        let mut obs = obs_of(&truth, &pairs);
        obs[5].d += 0.02;
        let sol = solve(&truth, &obs).unwrap();
        assert_eq!(sol.redundancy, 1);
        let s0 = sol.sigma0.unwrap();
        assert!(s0 > 0.001 && s0 < 0.05, "{s0}");
        let worst = sol.residuals.iter().enumerate().max_by(|a, b| a.1.abs().total_cmp(&b.1.abs())).unwrap().0;
        assert!(worst == 4 || worst == 5);
    }

    #[test]
    fn l_shaped_room() {
        let truth: Vec<P2> = vec![[0.0, 0.0], [6.0, 0.0], [6.0, 2.5], [3.1, 2.5], [3.1, 5.0], [0.0, 5.05]];
        let drawn: Vec<P2> = vec![[0.0, 0.0], [6.0, 0.0], [6.0, 2.0], [3.0, 2.0], [3.0, 5.0], [0.0, 5.0]];
        let diags = suggested_diagonals(&drawn);
        assert_eq!(diags.len(), 3);
        assert!(!diags.contains(&(2, 4)));
        let mut pairs = sides(6);
        pairs.extend(&diags);
        let sol = solve(&drawn, &obs_of(&truth, &pairs)).unwrap();
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
        assert_eq!(e, SolveError::NotEnough { needed: 5, have: 4, group: vec![0, 1, 2, 3] });
    }

    #[test]
    fn unbraced_room_is_not_rigid() {
        let p: Vec<P2> = vec![[0.0, 0.0], [4.0, 0.0], [5.0, 3.0], [2.0, 5.0], [-1.0, 3.0]];
        let mut pairs = sides(5);
        pairs.push((0, 2));
        pairs.push((0, 2));
        let e = solve(&p, &obs_of(&p, &pairs)).unwrap_err();
        assert!(matches!(e, SolveError::NotRigid { .. }));
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

    #[test]
    fn perp_derivatives_match_numeric() {
        let pts: Vec<P2> = vec![[0.3, 2.1], [0.0, 0.0], [4.0, 0.5]];
        let m = Meas::Perp { p: 0, a: 1, b: 2, d: 1.0 };
        let (f, parts) = m.linearize(&pts, 1.0);
        assert!((f - perp_dist(pts[0], pts[1], pts[2])).abs() < 1e-12);
        let h = 1e-6;
        for (p, g) in parts {
            for c in 0..2 {
                let mut q = pts.clone();
                q[p][c] += h;
                let up = m.compute(&q);
                q[p][c] -= 2.0 * h;
                let dn = m.compute(&q);
                let num = (up - dn) / (2.0 * h);
                assert!((num - g[c]).abs() < 1e-6, "point {p} coord {c}: {num} vs {}", g[c]);
            }
        }
    }

    /// Two rooms side by side with a 0.30 wall between them, joined by the
    /// wall thickness and two measures through the door.
    fn two_rooms() -> (Vec<P2>, Vec<P2>) {
        // Room 1: 0..4 × 0..3 (slightly skewed); room 2 behind the wall at x = 4.30.
        let truth: Vec<P2> = vec![
            [0.0, 0.0], [4.0, 0.0], [4.02, 3.0], [0.0, 3.01],
            [4.32, 0.0], [7.5, 0.05], [7.48, 3.02], [4.34, 3.0],
        ];
        // Drawn by eye: square, wrong sizes, wall too thin.
        let drawn: Vec<P2> = vec![
            [0.0, 0.0], [4.0, 0.0], [4.0, 3.0], [0.0, 3.0],
            [4.2, 0.0], [7.0, 0.0], [7.0, 3.0], [4.2, 3.0],
        ];
        (truth, drawn)
    }

    fn room_meas(truth: &[P2], off: usize) -> Vec<Meas> {
        let mut v: Vec<Meas> = (0..4)
            .map(|i| {
                let (a, b) = (off + i, off + (i + 1) % 4);
                Meas::Dist { i: a, j: b, d: dist(truth[a], truth[b]) }
            })
            .collect();
        v.push(Meas::Dist { i: off, j: off + 2, d: dist(truth[off], truth[off + 2]) });
        v
    }

    #[test]
    fn two_rooms_joined_by_wall_and_door() {
        let (truth, drawn) = two_rooms();
        let mut meas = room_meas(&truth, 0);
        meas.extend(room_meas(&truth, 4));
        // Wall thickness: corner 2A (4) from side 1B–1C (1,2).
        meas.push(Meas::Perp { p: 4, a: 1, b: 2, d: perp_dist(truth[4], truth[1], truth[2]) });
        // Through the door: 1B–2D and 1C–2A.
        meas.push(Meas::Dist { i: 1, j: 7, d: dist(truth[1], truth[7]) });
        meas.push(Meas::Dist { i: 2, j: 4, d: dist(truth[2], truth[4]) });
        let sol = solve_network(&drawn, &meas, &[(0, 1)]).unwrap();
        assert_eq!(sol.groups, 1);
        assert_eq!(sol.redundancy, 0);
        for (a, b) in sol.pts.iter().zip(&truth) {
            assert!(dist(*a, *b) < 1e-7, "{a:?} {b:?}");
        }
    }

    #[test]
    fn two_rooms_with_a_check_measure() {
        let (truth, drawn) = two_rooms();
        let mut meas = room_meas(&truth, 0);
        meas.extend(room_meas(&truth, 4));
        meas.push(Meas::Perp { p: 4, a: 1, b: 2, d: perp_dist(truth[4], truth[1], truth[2]) });
        meas.push(Meas::Perp { p: 7, a: 1, b: 2, d: perp_dist(truth[7], truth[1], truth[2]) + 0.01 });
        meas.push(Meas::Dist { i: 1, j: 7, d: dist(truth[1], truth[7]) });
        meas.push(Meas::Dist { i: 2, j: 4, d: dist(truth[2], truth[4]) });
        let sol = solve_network(&drawn, &meas, &[(0, 1)]).unwrap();
        assert_eq!(sol.redundancy, 1);
        assert!(sol.sigma0.unwrap() > 1e-4, "{:?} {:?}", sol.sigma0, sol.residuals);
    }

    #[test]
    fn rooms_not_joined_are_solved_apart() {
        let (truth, drawn) = two_rooms();
        let mut meas = room_meas(&truth, 0);
        meas.extend(room_meas(&truth, 4));
        let sol = solve_network(&drawn, &meas, &[(0, 1), (4, 5)]).unwrap();
        assert_eq!(sol.groups, 2);
        // Room 2 keeps its own corner A where drawn.
        assert_eq!(sol.pts[4], drawn[4]);
    }

    #[test]
    fn two_links_are_not_enough() {
        let (truth, drawn) = two_rooms();
        let mut meas = room_meas(&truth, 0);
        meas.extend(room_meas(&truth, 4));
        meas.push(Meas::Perp { p: 4, a: 1, b: 2, d: perp_dist(truth[4], truth[1], truth[2]) });
        meas.push(Meas::Dist { i: 2, j: 4, d: dist(truth[2], truth[4]) });
        let e = solve_network(&drawn, &meas, &[(0, 1)]).unwrap_err();
        assert!(matches!(e, SolveError::NotEnough { needed: 13, have: 12, .. }), "{e:?}");
    }
    #[test]
    fn right_angle_replaces_the_diagonal() {
        let truth: Vec<P2> = vec![[0.0, 0.0], [5.0, 0.0], [5.0, 4.0], [0.0, 4.0]];
        let drawn: Vec<P2> = vec![[0.0, 0.0], [5.3, 0.2], [5.1, 4.4], [-0.2, 3.9]];
        let mut meas: Vec<Meas> = sides(4).iter().map(|&(i, j)| Meas::Dist { i, j, d: dist(truth[i], truth[j]) }).collect();
        meas.push(Meas::Angle { a: 3, b: 0, c: 1, rad: std::f64::consts::FRAC_PI_2 });
        let sol = solve_network(&drawn, &meas, &[(0, 1)]).unwrap();
        assert_eq!(sol.redundancy, 0);
        assert!((dist(sol.pts[0], sol.pts[2]) - 41f64.sqrt()).abs() < 1e-8);
    }

    #[test]
    fn reflex_corner_angle() {
        // L-room: the inner corner (3) is 270°, given as 90°.
        let truth: Vec<P2> = vec![[0.0, 0.0], [6.0, 0.0], [6.0, 2.5], [3.0, 2.5], [3.0, 5.0], [0.0, 5.0]];
        let drawn: Vec<P2> = vec![[0.0, 0.0], [6.2, 0.1], [6.0, 2.2], [3.3, 2.6], [2.9, 5.2], [0.1, 4.8]];
        let mut meas: Vec<Meas> = sides(6).iter().map(|&(i, j)| Meas::Dist { i, j, d: dist(truth[i], truth[j]) }).collect();
        let r = std::f64::consts::FRAC_PI_2;
        meas.push(Meas::Angle { a: 5, b: 0, c: 1, rad: r });
        meas.push(Meas::Angle { a: 2, b: 3, c: 4, rad: r });
        meas.push(Meas::Angle { a: 1, b: 2, c: 3, rad: r });
        let sol = solve_network(&drawn, &meas, &[(0, 1)]).unwrap();
        // Same shape as the truth (the drawn A–B direction is kept).
        for i in 0..6 {
            for j in i + 1..6 {
                assert!((dist(sol.pts[i], sol.pts[j]) - dist(truth[i], truth[j])).abs() < 1e-6, "{i}-{j}");
            }
        }
    }

    #[test]
    fn horizontal_side_frees_the_rotation() {
        // Drawn tilted by 5°; side A–B declared horizontal.
        let truth: Vec<P2> = vec![[0.0, 0.0], [5.0, 0.0], [5.0, 4.0], [0.0, 4.0]];
        let t = 5f64.to_radians();
        let drawn: Vec<P2> = truth.iter().map(|p| [p[0] * t.cos() - p[1] * t.sin(), p[0] * t.sin() + p[1] * t.cos()]).collect();
        let mut meas: Vec<Meas> = sides(4).iter().map(|&(i, j)| Meas::Dist { i, j, d: dist(truth[i], truth[j]) }).collect();
        meas.push(Meas::Dist { i: 0, j: 2, d: dist(truth[0], truth[2]) });
        meas.push(Meas::Level { a: 0, b: 1, vertical: false });
        let sol = solve_network(&drawn, &meas, &[(0, 1)]).unwrap();
        assert_eq!(sol.redundancy, 0);
        assert!((sol.pts[1][1] - sol.pts[0][1]).abs() < 1e-9);
        assert!((sol.pts[3][0] - sol.pts[0][0]).abs() < 1e-9, "D straight above A");
    }

    #[test]
    fn rooms_linked_through_a_shared_door() {
        // Room 1: 0..4 × 0..3; room 2 behind a 0.30 wall at x = 4.30.
        // Door 0.90 wide: in room 1 on side B–C (1→2) at 1.00 from B;
        // in room 2 on side D–A (7→4) at 1.10 from D (so it faces it).
        let (truth, drawn) = two_rooms();
        let mut meas = room_meas(&truth, 0);
        meas.extend(room_meas(&truth, 4));
        let (b, c, d2, a2) = (1, 2, 7, 4);
        let j1 = At { a: b, b: c, s: 1.0 };
        let j2 = At { a: d2, b: a2, s: 1.1 };
        let k2 = At { a: d2, b: a2, s: 2.0 };
        meas.push(Meas::PerpAt { p: j2, a: b, b: c, d: perp_dist(j2.eval(&truth), truth[b], truth[c]) });
        meas.push(Meas::PerpAt { p: k2, a: b, b: c, d: perp_dist(k2.eval(&truth), truth[b], truth[c]) });
        meas.push(Meas::Along { p: j1, q: k2, a: b, b: c, d: {
            let (pp, qq) = (j1.eval(&truth), k2.eval(&truth));
            let u = [truth[c][0] - truth[b][0], truth[c][1] - truth[b][1]];
            ((qq[0] - pp[0]) * u[0] + (qq[1] - pp[1]) * u[1]) / u[0].hypot(u[1])
        } });
        let sol = solve_network(&drawn, &meas, &[(0, 1), (4, 5)]).unwrap();
        assert_eq!(sol.groups, 1);
        for (a, b) in sol.pts.iter().zip(&truth) {
            assert!(dist(*a, *b) < 1e-6, "{a:?} {b:?}");
        }
    }
}
