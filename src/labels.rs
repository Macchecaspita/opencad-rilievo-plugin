//! Where to write the corner letters so they never cover each other.
//!
//! Each letter first tries its natural place, just inside the corner on the
//! bisector. When that spot is taken — short sides, a small step in the
//! wall, corners close together — it tries places further away and turned
//! around the corner, and keeps the first one free from other letters, the
//! room outlines and the other corners. A letter moved away from its corner
//! gets a leader line back to it, so it is always clear which corner it
//! names. Host-free, unit-tested.

use crate::solve::{self, P2};

/// One placed text.
#[derive(Clone, Debug, PartialEq)]
pub struct Placed {
    pub text: String,
    pub at: P2,
    pub height: f64,
    /// Leader from the corner to the text, when the text was moved away.
    pub leader: Option<(P2, P2)>,
}

/// Approximate radius of the circle around a text of `chars` characters.
fn radius(chars: usize, h: f64) -> f64 {
    // CAD fonts are wide: about one text height per character.
    let hw = 0.5 * h * chars.max(1) as f64 + 0.2 * h;
    hw.max(0.6 * h)
}

pub fn seg_dist(p: P2, a: P2, b: P2) -> f64 {
    let u = [b[0] - a[0], b[1] - a[1]];
    let l2 = u[0] * u[0] + u[1] * u[1];
    let t = if l2 > 0.0 { (((p[0] - a[0]) * u[0] + (p[1] - a[1]) * u[1]) / l2).clamp(0.0, 1.0) } else { 0.0 };
    solve::dist(p, [a[0] + t * u[0], a[1] + t * u[1]])
}

/// Unit inward bisector at corner `i` of `pts`.
fn inward(pts: &[P2], i: usize) -> P2 {
    let n = pts.len();
    let ccw = solve::signed_area(pts) > 0.0;
    let (a, b, c) = (pts[(i + n - 1) % n], pts[i], pts[(i + 1) % n]);
    let nrm = |p: P2, q: P2| {
        let (dx, dy) = (q[0] - p[0], q[1] - p[1]);
        let l = dx.hypot(dy).max(1e-12);
        if ccw { [-dy / l, dx / l] } else { [dy / l, -dx / l] }
    };
    let (n1, n2) = (nrm(a, b), nrm(b, c));
    let d = [n1[0] + n2[0], n1[1] + n2[1]];
    let l = d[0].hypot(d[1]);
    if l < 1e-9 { n1 } else { [d[0] / l, d[1] / l] }
}

fn rotate(v: P2, deg: f64) -> P2 {
    let (s, c) = deg.to_radians().sin_cos();
    [v[0] * c - v[1] * s, v[0] * s + v[1] * c]
}

/// Place the letters of every corner of every room. `names[r][i]` is the
/// text of corner `i` of room `r`; `extra` are texts already fixed (room
/// titles) that letters must avoid.
pub fn place(rooms: &[Vec<P2>], names: &[Vec<String>], h: f64, extra: &[(String, P2, f64)]) -> Vec<Placed> {
    let edges: Vec<(P2, P2)> = rooms
        .iter()
        .flat_map(|r| (0..r.len()).map(move |i| (r[i], r[(i + 1) % r.len()])))
        .collect();
    let corners: Vec<P2> = rooms.iter().flatten().copied().collect();
    // Taken circles: (centre, radius).
    let mut taken: Vec<(P2, f64)> = extra.iter().map(|(t, p, hh)| (*p, radius(t.chars().count(), *hh))).collect();

    let dists = [1.3, 2.3, 3.3, 4.4, 5.6, 7.0];
    let angles = [0.0, 25.0, -25.0, 50.0, -50.0, 75.0, -75.0, 105.0, -105.0, 135.0, -135.0, 180.0];

    let mut out = Vec::new();
    for (r, room) in rooms.iter().enumerate() {
        for (i, &v) in room.iter().enumerate() {
            let text = names[r][i].clone();
            let rad = radius(text.chars().count(), h);
            let dir = inward(room, i);
            let free = |c: P2, taken: &Vec<(P2, f64)>| {
                taken.iter().all(|(p, rr)| solve::dist(*p, c) >= rr + rad)
                    && edges.iter().all(|(a, b)| seg_dist(c, *a, *b) >= rad * 0.85)
                    && corners.iter().all(|q| solve::dist(*q, c) >= rad * 0.9)
            };
            let mut best: Option<(P2, bool)> = None;
            'search: for &d in &dists {
                for &ang in &angles {
                    let u = rotate(dir, ang);
                    let c = [v[0] + u[0] * h * d, v[1] + u[1] * h * d];
                    if free(c, &taken) {
                        best = Some((c, d > dists[0] || ang != 0.0));
                        break 'search;
                    }
                }
            }
            // Nowhere free: natural place, with a leader so it still points.
            let (at, moved) = best.unwrap_or(([v[0] + dir[0] * h * dists[0], v[1] + dir[1] * h * dists[0]], true));
            let leader = moved.then(|| {
                let (dx, dy) = (at[0] - v[0], at[1] - v[1]);
                let l = dx.hypot(dy).max(1e-12);
                let u = [dx / l, dy / l];
                let start = [v[0] + u[0] * h * 0.15, v[1] + u[1] * h * 0.15];
                let end = [at[0] - u[0] * rad * 0.85, at[1] - u[1] * rad * 0.85];
                (start, end)
            });
            taken.push((at, rad));
            out.push(Placed { text, at, height: h, leader });
        }
    }
    out
}

/// A block of text lines (room name, area) fitted inside a room.
#[derive(Clone, Debug, PartialEq)]
pub struct Fitted {
    /// Centre of each line, in order.
    pub at: Vec<P2>,
    /// Height of each line.
    pub heights: Vec<f64>,
    /// Turned by 90° (reading upwards) to fit a long narrow room.
    pub vertical: bool,
}

/// Width of a line of `chars` characters at height `h` in the CAD's fonts.
fn text_width(chars: usize, h: f64) -> f64 {
    0.95 * h * chars.max(1) as f64
}

fn seg_cross(p: P2, q: P2, a: P2, b: P2) -> bool {
    let o = |a: P2, b: P2, c: P2| (b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0]);
    let (d1, d2, d3, d4) = (o(a, b, p), o(a, b, q), o(p, q, a), o(p, q, b));
    d1 * d2 < 0.0 && d3 * d4 < 0.0
}

fn inside(poly: &[P2], p: P2) -> bool {
    let mut c = false;
    let n = poly.len();
    let mut j = n - 1;
    for i in 0..n {
        let (a, b) = (poly[i], poly[j]);
        if (a[1] > p[1]) != (b[1] > p[1]) && p[0] < (b[0] - a[0]) * (p[1] - a[1]) / (b[1] - a[1]) + a[0] {
            c = !c;
        }
        j = i;
    }
    c
}

/// Whether the rectangle [x0,x1]×[y0,y1] lies inside the polygon, touching
/// none of its sides.
fn box_inside(poly: &[P2], x0: f64, y0: f64, x1: f64, y1: f64) -> bool {
    let corners = [[x0, y0], [x1, y0], [x1, y1], [x0, y1]];
    if !corners.iter().all(|c| inside(poly, *c)) {
        return false;
    }
    if poly.iter().any(|v| v[0] > x0 && v[0] < x1 && v[1] > y0 && v[1] < y1) {
        return false;
    }
    let n = poly.len();
    (0..n).all(|i| {
        let (a, b) = (poly[i], poly[(i + 1) % n]);
        (0..4).all(|k| !seg_cross(corners[k], corners[(k + 1) % 4], a, b))
    })
}

/// Fit text lines `(chars, relative height)` inside `poly`, with a free
/// margin of half a line all round, as large as possible up to `h_max`: the
/// block goes as near the room's centre as it fits, turned upright in a
/// long narrow room when that fits better. When nothing fits even small,
/// the smallest size at the centre.
pub fn fit_block(poly: &[P2], lines: &[(usize, f64)], h_max: f64, centre: P2) -> Fitted {
    let (bx0, bx1) = poly.iter().fold((f64::MAX, f64::MIN), |a, q| (a.0.min(q[0]), a.1.max(q[0])));
    let (by0, by1) = poly.iter().fold((f64::MAX, f64::MIN), |a, q| (a.0.min(q[1]), a.1.max(q[1])));
    // Candidate centres: the given centre first, then a grid, nearest first.
    let g = 24;
    let mut cand: Vec<P2> = vec![centre];
    for i in 0..=g {
        for j in 0..=g {
            cand.push([bx0 + (bx1 - bx0) * i as f64 / g as f64, by0 + (by1 - by0) * j as f64 / g as f64]);
        }
    }
    cand.sort_by(|a, b| solve::dist(*a, centre).total_cmp(&solve::dist(*b, centre)));
    let layout = |h: f64, vertical: bool, c: P2| -> Fitted {
        let heights: Vec<f64> = lines.iter().map(|l| l.1 * h).collect();
        let gap = 0.45 * h;
        let total: f64 = heights.iter().sum::<f64>() + gap * (lines.len().saturating_sub(1)) as f64;
        let mut off = total / 2.0;
        let mut at = Vec::new();
        for hh in &heights {
            let mid = off - hh / 2.0;
            // Lines stack downwards; upright text stacks to the right.
            at.push(if vertical { [c[0] + mid, c[1]] } else { [c[0], c[1] + mid] });
            off -= hh + gap;
        }
        if vertical {
            // Reading upwards, the first line is on the left.
            for p in at.iter_mut() {
                p[0] = 2.0 * c[0] - p[0];
            }
        }
        Fitted { at, heights, vertical }
    };
    let mut h = h_max;
    for _ in 0..16 {
        let heights: Vec<f64> = lines.iter().map(|l| l.1 * h).collect();
        let w = lines.iter().zip(&heights).map(|(l, hh)| text_width(l.0, *hh)).fold(0.0f64, f64::max);
        let tall = heights.iter().sum::<f64>() + 0.45 * h * (lines.len().saturating_sub(1)) as f64;
        let m = 0.5 * h;
        for vertical in [false, true] {
            let (hw, hh) = if vertical { (tall / 2.0 + m, w / 2.0 + m) } else { (w / 2.0 + m, tall / 2.0 + m) };
            if let Some(c) = cand.iter().find(|c| box_inside(poly, c[0] - hw, c[1] - hh, c[0] + hw, c[1] + hh)) {
                return layout(h, vertical, *c);
            }
        }
        h *= 0.85;
    }
    layout(h / 0.85, false, centre)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn names(rooms: &[Vec<P2>]) -> Vec<Vec<String>> {
        rooms.iter().map(|r| (0..r.len()).map(solve::label).collect()).collect()
    }

    fn no_overlap(p: &[Placed]) {
        for (a, x) in p.iter().enumerate() {
            for y in &p[a + 1..] {
                let need = radius(x.text.len(), x.height) + radius(y.text.len(), y.height);
                assert!(solve::dist(x.at, y.at) >= need * 0.999, "{} and {} overlap", x.text, y.text);
            }
        }
    }

    #[test]
    fn plain_room_keeps_letters_at_corners() {
        let room = vec![vec![[0.0, 0.0], [5.0, 0.0], [5.0, 4.0], [0.0, 4.0]]];
        let p = place(&room, &names(&room), 0.2, &[]);
        assert!(p.iter().all(|x| x.leader.is_none()));
        no_overlap(&p);
    }

    #[test]
    fn small_step_spreads_letters_with_leaders() {
        // The room of the screenshot: a 10 cm step and a short chamfer.
        let room = vec![vec![
            [29.0, 30.0], [10.0, 30.0], [10.0, 10.0], [21.0, 10.0], [21.0, 10.25],
            [21.3, 10.25], [21.3, 10.5], [26.0, 10.5], [29.6, 14.0],
        ]];
        let h = 1.0;
        let p = place(&room, &names(&room), h, &[]);
        no_overlap(&p);
        assert!(p.iter().any(|x| x.leader.is_some()));
        // Each moved letter's leader starts at its own corner.
        for (x, v) in p.iter().zip(&room[0]) {
            if let Some((s, _)) = x.leader {
                assert!(solve::dist(s, *v) < h * 0.2);
            }
        }
    }

    #[test]
    fn two_rooms_do_not_mix_letters() {
        let rooms = vec![
            vec![[0.0, 0.0], [4.0, 0.0], [4.0, 3.0], [0.0, 3.0]],
            vec![[4.3, 0.0], [7.0, 0.0], [7.0, 3.0], [4.3, 3.0]],
        ];
        let n: Vec<Vec<String>> = rooms.iter().enumerate().map(|(r, x)| (0..x.len()).map(|i| format!("{}{}", r + 1, solve::label(i))).collect()).collect();
        let p = place(&rooms, &n, 0.2, &[]);
        no_overlap(&p);
    }
    #[test]
    fn titles_fit_inside_with_margin() {
        // A wide room: full size, horizontal.
        let wide = vec![[0.0, 0.0], [6.0, 0.0], [6.0, 4.0], [0.0, 4.0]];
        let f = fit_block(&wide, &[(6, 1.4), (8, 1.1)], 0.3, [3.0, 2.0]);
        assert!(!f.vertical && (f.heights[0] - 0.42).abs() < 1e-9);
        assert!(f.at[0][1] > f.at[1][1], "name above the area");
        // A long narrow corridor 1.0 wide: upright, still inside.
        let narrow = vec![[0.0, 0.0], [1.0, 0.0], [1.0, 9.0], [0.0, 9.0]];
        let f = fit_block(&narrow, &[(6, 1.4), (8, 1.1)], 0.4, [0.5, 4.5]);
        assert!(f.vertical, "{f:?}");
        for p in &f.at {
            assert!(p[0] > 0.0 && p[0] < 1.0);
        }
        // A tiny room: the text shrinks until it fits with its margin.
        let tiny = vec![[0.0, 0.0], [1.2, 0.0], [1.2, 0.9], [0.0, 0.9]];
        let f = fit_block(&tiny, &[(6, 1.4)], 0.5, [0.6, 0.45]);
        let w = text_width(6, f.heights[0]);
        assert!(w + f.heights[0] / 1.4 <= 1.2 + 1e-9 || f.vertical, "{f:?}");
        // An L: the block moves into the part where it fits.
        let l = vec![[0.0, 0.0], [6.0, 0.0], [6.0, 1.5], [1.5, 1.5], [1.5, 6.0], [0.0, 6.0]];
        let f = fit_block(&l, &[(6, 1.4)], 0.25, [2.0, 2.0]);
        let hw = text_width(6, f.heights[0]) / 2.0;
        let c = f.at[0];
        assert!(inside(&l, [c[0] - hw, c[1]]) && inside(&l, [c[0] + hw, c[1]]));
    }
}
