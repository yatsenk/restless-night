use crate::collision::{Ramp, Solid, World};
use crate::geom::{Builder, Mat, MeshData, SKIP_NX, SKIP_NY, SKIP_NZ, SKIP_PX, SKIP_PY, SKIP_PZ};
use crate::math::{V3, v3};
use crate::models;

pub const W: f32 = 16.0;
pub const D: f32 = 12.0;
pub const EXT: f32 = 0.30;
pub const HALF: f32 = 0.08;
pub const GROUND: f32 = -0.15;
pub const CEIL0: f32 = 3.0;
pub const UPPER: f32 = 3.3;
pub const TOP: f32 = 6.3;
pub const PX1: f32 = 6.5;
pub const PX2: f32 = 9.5;
pub const PX3: f32 = 12.75;
pub const PZ: f32 = 6.2;
pub const STAIR_X0: f32 = 7.82;
pub const STAIR_X1: f32 = PX2 - HALF;
pub const STAIR_Z0: f32 = 3.3;
pub const RISERS: usize = 18;
pub const TREAD: f32 = 0.28;
pub const RISE: f32 = UPPER / RISERS as f32;
pub const STAIR_END: f32 = STAIR_Z0 + (RISERS as f32 - 1.0) * TREAD;
pub const HOLE_Z0: f32 = 4.55;
pub const RAIL_X0: f32 = STAIR_X0 + 0.01;
pub const RAIL_H: f32 = 0.9;
const DOOR_H: f32 = 2.15;
const WIN_SILL: f32 = 0.9;
const WIN_HEAD: f32 = 2.25;
const SKIRT_H: f32 = 0.12;
const SKIRT_P: f32 = 0.02;
const CROWN_H: f32 = 0.09;
const CROWN_P: f32 = 0.06;
const CASE_W: f32 = 0.085;
const CASE_P: f32 = 0.015;

#[derive(Clone, Copy, PartialEq, Debug)]
enum Axis {
    X,
    Z,
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Kind {
    Door,
    Passage,
    ExtDoor,
    Window,
    ModelWindow,
}

#[derive(Clone, Copy, Debug)]
struct Op {
    a0: f32,
    a1: f32,
    y0: f32,
    y1: f32,
    kind: Kind,
}

impl Op {
    fn open(&self) -> bool {
        matches!(self.kind, Kind::Door | Kind::Passage)
    }
}

#[derive(Clone, Copy, Debug)]
struct Room {
    x0: f32,
    x1: f32,
    z0: f32,
    z1: f32,
    level: u8,
    floor: Mat,
    wall: Mat,
    ceil: Mat,
}

impl Room {
    fn contains(&self, x: f32, z: f32) -> bool {
        x >= self.x0 && x <= self.x1 && z >= self.z0 && z <= self.z1
    }

    fn floor_y(&self) -> f32 {
        if self.level == 0 { 0.0 } else { UPPER }
    }

    fn ceil_y(&self) -> f32 {
        if self.level == 0 { CEIL0 } else { TOP }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Flicker {
    Steady,
    Breathe,
    Sputter,
    Fluorescent,
}

#[derive(Clone, Copy, Debug)]
pub struct LightSpec {
    pub pos: V3,
    pub color: [f32; 3],
    pub lumens: f32,
    pub range: f32,
    pub flicker: Flicker,
    pub name: &'static str,
}

#[derive(Clone, Debug)]
pub struct Slot {
    pub name: &'static str,
    pub need: &'static str,
    pub min: V3,
    pub max: V3,
}

pub type QuadRec = crate::geom::LoggedQuad;

pub struct House {
    pub meshes: Vec<MeshData>,
    pub world: World,
    pub lights: Vec<LightSpec>,
    pub slots: Vec<Slot>,
    pub spawn: V3,
    pub spawn_yaw: f32,
}

fn rooms() -> Vec<Room> {
    let mut v = Vec::new();
    let xs = [(0.0, PX1 - HALF), (PX1 + HALF, PX2 - HALF), (PX2 + HALF, W)];
    for level in 0..2u8 {
        let l0 = level == 0;
        v.push(Room {
            x0: xs[0].0,
            x1: xs[0].1,
            z0: 0.0,
            z1: PZ - HALF,
            level,
            floor: Mat::Floor,
            wall: Mat::Paper,
            ceil: Mat::Plaster,
        });
        v.push(Room {
            x0: xs[0].0,
            x1: xs[0].1,
            z0: PZ + HALF,
            z1: D,
            level,
            floor: Mat::Floor,
            wall: Mat::Paper,
            ceil: Mat::Plaster,
        });
        v.push(Room {
            x0: xs[1].0,
            x1: xs[1].1,
            z0: 0.0,
            z1: D,
            level,
            floor: Mat::Floor,
            wall: Mat::Plaster,
            ceil: Mat::Plaster,
        });
        v.push(Room {
            x0: xs[2].0,
            x1: W,
            z0: 0.0,
            z1: PZ - HALF,
            level,
            floor: Mat::Floor,
            wall: if l0 { Mat::Plaster } else { Mat::Paper },
            ceil: Mat::Plaster,
        });
        v.push(Room {
            x0: xs[2].0,
            x1: PX3 - HALF,
            z0: PZ + HALF,
            z1: D,
            level,
            floor: Mat::Tile,
            wall: Mat::Tile,
            ceil: Mat::Plaster,
        });
        v.push(Room {
            x0: PX3 + HALF,
            x1: W,
            z0: PZ + HALF,
            z1: D,
            level,
            floor: if l0 { Mat::Concrete } else { Mat::Floor },
            wall: if l0 { Mat::Plaster } else { Mat::Paper },
            ceil: Mat::Plaster,
        });
    }
    v
}

fn room_at(rs: &[Room], x: f32, z: f32, level: u8) -> Option<Room> {
    rs.iter()
        .copied()
        .find(|r| r.level == level && r.contains(x, z))
}

fn level_of(y: f32) -> Option<u8> {
    if y > 0.0 && y < CEIL0 {
        Some(0)
    } else if y > UPPER && y < TOP {
        Some(1)
    } else {
        None
    }
}

fn in_hole_z(z: f32) -> bool {
    z > HOLE_Z0 && z < STAIR_END
}

struct Wall {
    axis: Axis,
    lo: f32,
    hi: f32,
    a0: f32,
    a1: f32,
    y0: f32,
    y1: f32,
    ops: Vec<Op>,
    breaks: Vec<f32>,
    cap: bool,
    exterior: bool,
}

impl Wall {
    fn center(&self) -> f32 {
        (self.lo + self.hi) * 0.5
    }

    fn pt(&self, a: f32, across: f32) -> (f32, f32) {
        match self.axis {
            Axis::X => (a, across),
            Axis::Z => (across, a),
        }
    }

    fn face_mat(&self, rs: &[Room], a: f32, y: f32, hi_side: bool) -> Option<Mat> {
        let across = if hi_side {
            self.hi + 0.05
        } else {
            self.lo - 0.05
        };
        if self.exterior {
            let (px, pz) = self.pt(a, across);
            let inside = px > 0.0 && px < W && pz > 0.0 && pz < D;
            if !inside {
                return Some(if y < 0.5 { Mat::Concrete } else { Mat::Stucco });
            }
        }
        let (px, pz) = self.pt(a, across);
        match level_of(y) {
            Some(l) => room_at(rs, px, pz, l).map(|r| r.wall),
            None => {
                let near_hole = self.axis == Axis::Z
                    && (self.center() - PX2).abs() < 0.01
                    && !hi_side
                    && in_hole_z(a)
                    && y > CEIL0
                    && y < UPPER;
                if near_hole { Some(Mat::Plaster) } else { None }
            }
        }
    }
}

fn dedup(mut v: Vec<f32>) -> Vec<f32> {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mut out: Vec<f32> = Vec::new();
    for x in v {
        if out.last().map_or(true, |l| (x - l).abs() > 1e-4) {
            out.push(x);
        }
    }
    out
}

fn emit_rect(
    b: &mut Builder,
    axis: Axis,
    m: Mat,
    across: f32,
    a0: f32,
    a1: f32,
    y0: f32,
    y1: f32,
    dir: f32,
) {
    match axis {
        Axis::X => b.fz(m, across, a0, a1, y0, y1, dir),
        Axis::Z => b.fx(m, across, y0, y1, a0, a1, dir),
    }
}

fn build_wall(b: &mut Builder, rs: &[Room], w: &Wall) {
    let mut xs = vec![w.a0, w.a1];
    let mut ys = vec![w.y0, w.y1, 0.0, 0.5, CEIL0, UPPER, TOP];
    for o in &w.ops {
        xs.push(o.a0);
        xs.push(o.a1);
        ys.push(o.y0);
        ys.push(o.y1);
    }
    for &k in &w.breaks {
        xs.push(k);
    }
    if w.axis == Axis::Z && (w.center() - PX2).abs() < 0.01 {
        xs.push(HOLE_Z0);
        xs.push(STAIR_END);
    }
    let xs: Vec<f32> = dedup(xs)
        .into_iter()
        .filter(|&x| x >= w.a0 - 1e-4 && x <= w.a1 + 1e-4)
        .collect();
    let ys: Vec<f32> = dedup(ys)
        .into_iter()
        .filter(|&y| y >= w.y0 - 1e-4 && y <= w.y1 + 1e-4)
        .collect();
    for hi_side in [false, true] {
        let across = if hi_side { w.hi } else { w.lo };
        let dir = if hi_side { 1.0 } else { -1.0 };
        for i in 0..xs.len() - 1 {
            let ca = (xs[i] + xs[i + 1]) * 0.5;
            for j in 0..ys.len() - 1 {
                let cy = (ys[j] + ys[j + 1]) * 0.5;
                if w.ops
                    .iter()
                    .any(|o| ca > o.a0 && ca < o.a1 && cy > o.y0 && cy < o.y1)
                {
                    continue;
                }
                if let Some(m) = w.face_mat(rs, ca, cy, hi_side) {
                    emit_rect(
                        b,
                        w.axis,
                        m,
                        across,
                        xs[i],
                        xs[i + 1],
                        ys[j],
                        ys[j + 1],
                        dir,
                    );
                }
            }
        }
    }
    for o in &w.ops {
        let (a0, a1) = (o.a0, o.a1);
        match w.axis {
            Axis::X => {
                b.fx(Mat::Trim, a0, o.y0, o.y1, w.lo, w.hi, 1.0);
                b.fx(Mat::Trim, a1, o.y0, o.y1, w.lo, w.hi, -1.0);
                b.fy(Mat::Trim, o.y1, a0, a1, w.lo, w.hi, -1.0);
                if o.y0 > 0.01 && (o.y0 - UPPER).abs() > 0.01 {
                    b.fy(Mat::Trim, o.y0, a0, a1, w.lo, w.hi, 1.0);
                }
            }
            Axis::Z => {
                b.fz(Mat::Trim, a0, w.lo, w.hi, o.y0, o.y1, 1.0);
                b.fz(Mat::Trim, a1, w.lo, w.hi, o.y0, o.y1, -1.0);
                b.fy(Mat::Trim, o.y1, w.lo, w.hi, a0, a1, -1.0);
                if o.y0 > 0.01 && (o.y0 - UPPER).abs() > 0.01 {
                    b.fy(Mat::Trim, o.y0, w.lo, w.hi, a0, a1, 1.0);
                }
            }
        }
    }
    if w.cap {
        match w.axis {
            Axis::Z => {
                b.fz(Mat::Stucco, w.a0, w.lo, w.hi, w.y0, w.y1, -1.0);
                b.fz(Mat::Stucco, w.a1, w.lo, w.hi, w.y0, w.y1, 1.0);
            }
            Axis::X => {
                b.fx(Mat::Stucco, w.a0, w.y0, w.y1, w.lo, w.hi, -1.0);
                b.fx(Mat::Stucco, w.a1, w.y0, w.y1, w.lo, w.hi, 1.0);
            }
        }
    }
}

fn box_at(axis: Axis, across0: f32, across1: f32, a0: f32, a1: f32, y0: f32, y1: f32) -> (V3, V3) {
    match axis {
        Axis::X => (v3(a0, y0, across0), v3(a1, y1, across1)),
        Axis::Z => (v3(across0, y0, a0), v3(across1, y1, a1)),
    }
}

fn casing(b: &mut Builder, w: &Wall, o: &Op, hi_side: bool) {
    let face = if hi_side { w.hi } else { w.lo };
    let (c0, c1) = if hi_side {
        (face, face + CASE_P)
    } else {
        (face - CASE_P, face)
    };
    let door_like = matches!(o.kind, Kind::Door | Kind::Passage | Kind::ExtDoor);
    let cw = CASE_W;
    let (l0, l1) = (o.a0 - cw, o.a0);
    let (r0, r1) = (o.a1, o.a1 + cw);
    let ybot = o.y0;
    let (mn, mx) = box_at(w.axis, c0, c1, l0, l1, ybot, o.y1);
    b.cuboid(Mat::Trim, mn, mx, 0);
    let (mn, mx) = box_at(w.axis, c0, c1, r0, r1, ybot, o.y1);
    b.cuboid(Mat::Trim, mn, mx, 0);
    let (mn, mx) = box_at(w.axis, c0, c1, l0, r1, o.y1, o.y1 + cw);
    b.cuboid(Mat::Trim, mn, mx, 0);
    if !door_like {
        let (s0, s1) = if hi_side {
            (face, face + 0.06)
        } else {
            (face - 0.06, face)
        };
        let (mn, mx) = box_at(w.axis, s0, s1, l0 - 0.02, r1 + 0.02, o.y0 - 0.04, o.y0);
        b.cuboid(Mat::Trim, mn, mx, 0);
    }
}

fn window_frame(b: &mut Builder, w: &Wall, o: &Op) {
    let mid = w.center();
    let d = 0.04;
    let (a0, a1) = (o.a0 - 0.02, o.a1 + 0.02);
    let (y0, y1) = (o.y0 - 0.02, o.y1 + 0.02);
    let t = 0.07;
    let rail = |b: &mut Builder, ya: f32, yb: f32| {
        let (mn, mx) = box_at(w.axis, mid - d, mid + d, a0, a1, ya, yb);
        b.cuboid(Mat::Frame, mn, mx, 0);
    };
    rail(b, y0, y0 + t);
    rail(b, y1 - t, y1);
    let ia0 = a0;
    let ia1 = a1;
    let cols = (((o.a1 - o.a0) / 0.7).round() as usize).max(1);
    let inner_w = (ia1 - ia0) - 2.0 * t;
    let mut stile_pos = vec![ia0];
    for c in 1..cols {
        stile_pos.push(ia0 + t + inner_w * c as f32 / cols as f32 - 0.025 + 0.025);
    }
    let ym0 = y0 + t;
    let ym1 = y1 - t;
    let stile = |b: &mut Builder, s0: f32, s1: f32| {
        let (mn, mx) = box_at(w.axis, mid - d, mid + d, s0, s1, ym0, ym1);
        b.cuboid(Mat::Frame, mn, mx, SKIP_NY | SKIP_PY);
    };
    stile(b, ia0, ia0 + t);
    stile(b, ia1 - t, ia1);
    let mut seams: Vec<(f32, f32)> = vec![(ia0, ia0 + t)];
    for c in 1..cols {
        let cc = ia0 + t + inner_w * c as f32 / cols as f32;
        stile(b, cc - 0.025, cc + 0.025);
        seams.push((cc - 0.025, cc + 0.025));
    }
    seams.push((ia1 - t, ia1));
    let _ = stile_pos;
    if (o.y1 - o.y0) > 1.1 {
        let ymid = (ym0 + ym1) * 0.5;
        for k in 0..seams.len() - 1 {
            let s0 = seams[k].1;
            let s1 = seams[k + 1].0;
            let (mn, mx) = box_at(w.axis, mid - d, mid + d, s0, s1, ymid - 0.025, ymid + 0.025);
            let skip = match w.axis {
                Axis::X => SKIP_NX | SKIP_PX,
                Axis::Z => SKIP_NZ | SKIP_PZ,
            };
            b.cuboid(Mat::Frame, mn, mx, skip);
        }
    }
    let g0 = o.a0 - 0.005;
    let g1 = o.a1 + 0.005;
    match w.axis {
        Axis::X => {
            b.fz(Mat::Glass, mid, g0, g1, o.y0 - 0.005, o.y1 + 0.005, 1.0);
            b.fz(Mat::Glass, mid, g0, g1, o.y0 - 0.005, o.y1 + 0.005, -1.0);
        }
        Axis::Z => {
            b.fx(Mat::Glass, mid, o.y0 - 0.005, o.y1 + 0.005, g0, g1, 1.0);
            b.fx(Mat::Glass, mid, o.y0 - 0.005, o.y1 + 0.005, g0, g1, -1.0);
        }
    }
}

fn exterior_door(b: &mut Builder, w: &Wall, o: &Op) {
    let mid = w.center();
    let (mn, mx) = box_at(
        w.axis,
        mid - 0.04,
        mid + 0.04,
        o.a0 + 0.01,
        o.a1 - 0.01,
        o.y0,
        o.y1 - 0.01,
    );
    b.cuboid(Mat::Door, mn, mx, 0);
    let (mn, mx) = box_at(
        w.axis,
        mid - 0.06,
        mid + 0.06,
        o.a1 - 0.2,
        o.a1 - 0.17,
        1.0,
        1.1,
    );
    b.cuboid(Mat::Frame, mn, mx, 0);
}

fn wall_solids(w: &Wall, out: &mut Vec<Solid>) {
    let mut cuts: Vec<(f32, f32, f32)> = Vec::new();
    for o in &w.ops {
        if o.open() {
            cuts.push((o.a0, o.a1, o.y1));
        }
    }
    cuts.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    let mut cur = w.a0;
    let push = |a0: f32, a1: f32, y0: f32, y1: f32, out: &mut Vec<Solid>| {
        if a1 - a0 < 1e-4 || y1 - y0 < 1e-4 {
            return;
        }
        let (mn, mx) = box_at(w.axis, w.lo, w.hi, a0, a1, y0, y1);
        out.push(Solid { min: mn, max: mx });
    };
    for (c0, c1, head) in cuts {
        push(cur, c0, w.y0, w.y1, out);
        let mut lintels: Vec<(f32, f32)> = vec![(head, w.y1)];
        for o in &w.ops {
            if o.open() && o.a0 == c0 && o.y0 > 0.5 {
                lintels.push((CEIL0, o.y0));
            }
        }
        let upper_open = w.ops.iter().find(|o| o.open() && o.a0 == c0 && o.y0 > 0.5);
        if let Some(u) = upper_open {
            push(c0, c1, head, UPPER.min(u.y0), out);
            push(c0, c1, u.y1, w.y1, out);
        } else {
            push(c0, c1, head, w.y1, out);
        }
        cur = c1;
    }
    push(cur, w.a1, w.y0, w.y1, out);
}

fn walls() -> Vec<Wall> {
    let win = |c: f32, hw: f32, base: f32| Op {
        a0: c - hw,
        a1: c + hw,
        y0: base + WIN_SILL,
        y1: base + WIN_HEAD,
        kind: Kind::Window,
    };
    let small = |c: f32, hw: f32, base: f32| Op {
        a0: c - hw,
        a1: c + hw,
        y0: base + 1.4,
        y1: base + 2.1,
        kind: Kind::Window,
    };
    let door = |c: f32, base: f32| Op {
        a0: c - 0.475,
        a1: c + 0.475,
        y0: base,
        y1: base + DOOR_H,
        kind: Kind::Door,
    };
    let ext = |c: f32| Op {
        a0: c - 0.55,
        a1: c + 0.55,
        y0: 0.0,
        y1: 2.2,
        kind: Kind::ExtDoor,
    };
    let kitchen_win = Op {
        a0: 13.925,
        a1: 15.135,
        y0: 1.075,
        y1: 2.345,
        kind: Kind::ModelWindow,
    };
    let u = UPPER;
    let mut v = Vec::new();
    v.push(Wall {
        axis: Axis::X,
        lo: -EXT,
        hi: 0.0,
        a0: 0.0,
        a1: W,
        y0: GROUND,
        y1: TOP,
        ops: vec![
            win(2.0, 0.65, 0.0),
            win(4.7, 0.65, 0.0),
            ext(8.0),
            win(11.3, 0.65, 0.0),
            kitchen_win,
            win(2.2, 0.65, u),
            win(8.0, 0.55, u),
            win(12.0, 0.65, u),
            win(14.0, 0.65, u),
        ],
        breaks: vec![PX1 - HALF, PX1 + HALF, PX2 - HALF, PX2 + HALF],
        cap: false,
        exterior: true,
    });
    v.push(Wall {
        axis: Axis::X,
        lo: D,
        hi: D + EXT,
        a0: 0.0,
        a1: W,
        y0: GROUND,
        y1: TOP,
        ops: vec![
            win(3.2, 0.65, 0.0),
            ext(8.0),
            small(11.1, 0.35, 0.0),
            win(14.4, 0.4, 0.0),
            win(3.2, 0.65, u),
            win(8.0, 0.55, u),
            small(11.1, 0.35, u),
            win(14.4, 0.4, u),
        ],
        breaks: vec![
            PX1 - HALF,
            PX1 + HALF,
            PX2 - HALF,
            PX2 + HALF,
            PX3 - HALF,
            PX3 + HALF,
        ],
        cap: false,
        exterior: true,
    });
    v.push(Wall {
        axis: Axis::Z,
        lo: -EXT,
        hi: 0.0,
        a0: -EXT,
        a1: D + EXT,
        y0: GROUND,
        y1: TOP,
        ops: vec![
            win(1.9, 0.7, 0.0),
            win(9.0, 0.7, 0.0),
            win(1.9, 0.7, u),
            win(9.0, 0.7, u),
        ],
        breaks: vec![0.0, D, PZ - HALF, PZ + HALF],
        cap: true,
        exterior: true,
    });
    v.push(Wall {
        axis: Axis::Z,
        lo: W,
        hi: W + EXT,
        a0: -EXT,
        a1: D + EXT,
        y0: GROUND,
        y1: TOP,
        ops: vec![
            win(5.1, 0.6, 0.0),
            Op {
                a0: 8.6,
                a1: 9.4,
                y0: 1.2,
                y1: 2.2,
                kind: Kind::Window,
            },
            win(2.05, 0.65, u),
            Op {
                a0: 8.6,
                a1: 9.6,
                y0: u + 1.2,
                y1: u + 2.2,
                kind: Kind::Window,
            },
        ],
        breaks: vec![0.0, D, PZ - HALF, PZ + HALF],
        cap: true,
        exterior: true,
    });
    v.push(Wall {
        axis: Axis::Z,
        lo: PX1 - HALF,
        hi: PX1 + HALF,
        a0: -0.03,
        a1: D + 0.03,
        y0: 0.0,
        y1: TOP,
        ops: vec![
            door(1.825, 0.0),
            door(9.475, 0.0),
            door(1.825, u),
            door(9.475, u),
        ],
        breaks: vec![PZ - HALF, PZ + HALF],
        cap: false,
        exterior: false,
    });
    v.push(Wall {
        axis: Axis::Z,
        lo: PX2 - HALF,
        hi: PX2 + HALF,
        a0: -0.03,
        a1: D + 0.03,
        y0: 0.0,
        y1: TOP,
        ops: vec![
            Op {
                a0: 0.7,
                a1: 2.9,
                y0: 0.0,
                y1: 2.3,
                kind: Kind::Passage,
            },
            door(9.475, 0.0),
            door(1.825, u),
            door(9.475, u),
        ],
        breaks: vec![PZ - HALF, PZ + HALF],
        cap: false,
        exterior: false,
    });
    v.push(Wall {
        axis: Axis::X,
        lo: PZ - HALF,
        hi: PZ + HALF,
        a0: -0.03,
        a1: PX1 - HALF + 0.03,
        y0: 0.0,
        y1: TOP,
        ops: vec![door(5.3, 0.0), door(2.575, u)],
        breaks: vec![],
        cap: false,
        exterior: false,
    });
    v.push(Wall {
        axis: Axis::X,
        lo: PZ - HALF,
        hi: PZ + HALF,
        a0: PX2 + HALF - 0.03,
        a1: W + 0.03,
        y0: 0.0,
        y1: TOP,
        ops: vec![door(13.405, 0.0), door(13.405, u)],
        breaks: vec![PX3 - HALF, PX3 + HALF],
        cap: false,
        exterior: false,
    });
    v.push(Wall {
        axis: Axis::Z,
        lo: PX3 - HALF,
        hi: PX3 + HALF,
        a0: PZ + HALF - 0.03,
        a1: D + 0.03,
        y0: 0.0,
        y1: TOP,
        ops: vec![],
        breaks: vec![],
        cap: false,
        exterior: false,
    });
    v
}

struct DoorRec {
    axis: Axis,
    c: f32,
    a0: f32,
    a1: f32,
    level: u8,
    half: f32,
}

fn door_recs(ws: &[Wall]) -> Vec<DoorRec> {
    let mut v = Vec::new();
    for w in ws {
        for o in &w.ops {
            if matches!(o.kind, Kind::Door | Kind::Passage | Kind::ExtDoor) {
                v.push(DoorRec {
                    axis: w.axis,
                    c: w.center(),
                    a0: o.a0,
                    a1: o.a1,
                    level: if o.y0 > 1.0 { 1 } else { 0 },
                    half: (w.hi - w.lo) * 0.5,
                });
            }
        }
    }
    v
}

fn subtract(span: (f32, f32), cuts: &[(f32, f32)]) -> Vec<(f32, f32)> {
    let mut out = vec![span];
    for &(c0, c1) in cuts {
        let mut next = Vec::new();
        for (s0, s1) in out {
            if c1 <= s0 || c0 >= s1 {
                next.push((s0, s1));
            } else {
                if c0 > s0 {
                    next.push((s0, c0));
                }
                if c1 < s1 {
                    next.push((c1, s1));
                }
            }
        }
        out = next;
    }
    out.into_iter().filter(|(a, b)| b - a > 1e-3).collect()
}

fn room_edges(
    b: &mut Builder,
    r: &Room,
    doors: &[DoorRec],
    mat: Mat,
    y0: f32,
    y1: f32,
    proud: f32,
    shorten: f32,
    skip_hole: bool,
    cut_doors: bool,
) {
    let edges: [(Axis, f32, f32, f32, f32); 4] = [
        (Axis::X, r.z0, r.x0, r.x1, 1.0),
        (Axis::X, r.z1, r.x0, r.x1, -1.0),
        (Axis::Z, r.x0, r.z0, r.z1, 1.0),
        (Axis::Z, r.x1, r.z0, r.z1, -1.0),
    ];
    for (axis, face, a0, a1, inward) in edges {
        let (s0, s1) = if axis == Axis::Z {
            (a0 + shorten, a1 - shorten)
        } else {
            (a0, a1)
        };
        let mut cuts: Vec<(f32, f32)> = Vec::new();
        if cut_doors {
            for d in doors {
                if d.axis == axis && d.level == r.level && (d.c - face).abs() <= d.half + 0.02 {
                    cuts.push((d.a0 - CASE_W, d.a1 + CASE_W));
                }
            }
        }
        if skip_hole && axis == Axis::Z && (face - (PX2 - HALF)).abs() < 0.01 {
            cuts.push((HOLE_Z0 - 0.001, STAIR_END + 0.001));
        }
        for (p0, p1) in subtract((s0, s1), &cuts) {
            let (c0, c1) = if inward > 0.0 {
                (face, face + proud)
            } else {
                (face - proud, face)
            };
            let (mn, mx) = box_at(axis, c0, c1, p0, p1, y0, y1);
            let skip = if y0 == r.floor_y() { SKIP_NY } else { SKIP_PY };
            b.cuboid(mat, mn, mx, skip);
        }
    }
}

pub fn build() -> House {
    build_with(false).0
}

pub fn build_with(record: bool) -> (House, Vec<QuadRec>) {
    let mut b = Builder::new();
    if record {
        b.start_log();
    }
    let rs = rooms();
    let ws = walls();
    let doors = door_recs(&ws);
    let mut world = World::new(GROUND);

    for w in &ws {
        build_wall(&mut b, &rs, w);
        wall_solids(w, &mut world.solids);
        for o in &w.ops {
            match o.kind {
                Kind::Door | Kind::Passage => {
                    casing(&mut b, w, o, false);
                    casing(&mut b, w, o, true);
                    let fy = if o.y0 > 1.0 { UPPER } else { 0.0 };
                    match w.axis {
                        Axis::X => b.fy(Mat::Wood, fy, o.a0, o.a1, w.lo, w.hi, 1.0),
                        Axis::Z => b.fy(Mat::Wood, fy, w.lo, w.hi, o.a0, o.a1, 1.0),
                    }
                }
                Kind::ExtDoor => {
                    casing(&mut b, w, o, false);
                    casing(&mut b, w, o, true);
                    exterior_door(&mut b, w, o);
                    match w.axis {
                        Axis::X => b.fy(Mat::Concrete, 0.0, o.a0, o.a1, w.lo, w.hi, 1.0),
                        Axis::Z => b.fy(Mat::Concrete, 0.0, w.lo, w.hi, o.a0, o.a1, 1.0),
                    }
                }
                Kind::Window => {
                    casing(&mut b, w, o, false);
                    casing(&mut b, w, o, true);
                    window_frame(&mut b, w, o);
                }
                Kind::ModelWindow => {}
            }
        }
    }

    for r in &rs {
        let fy = r.floor_y();
        let cy = r.ceil_y();
        let skip_hole_room = r.x0 > PX1 && r.x1 < PX2 && r.z0 == 0.0 && r.z1 == D;
        if skip_hole_room {
            let xs = [r.x0, STAIR_X0, r.x1];
            let zs = [r.z0, HOLE_Z0, STAIR_END, r.z1];
            b.grid_y(
                r.floor,
                fy,
                &xs,
                &zs,
                if r.level == 1 { Some((1, 1)) } else { None },
                1.0,
            );
            b.grid_y(
                r.ceil,
                cy,
                &xs,
                &zs,
                if r.level == 0 { Some((1, 1)) } else { None },
                -1.0,
            );
        } else {
            b.fy(r.floor, fy, r.x0, r.x1, r.z0, r.z1, 1.0);
            b.fy(r.ceil, cy, r.x0, r.x1, r.z0, r.z1, -1.0);
        }
        let hall = skip_hole_room;
        room_edges(
            &mut b,
            r,
            &doors,
            Mat::Trim,
            fy,
            fy + SKIRT_H,
            SKIRT_P,
            SKIRT_P,
            hall && r.level == 1,
            true,
        );
        room_edges(
            &mut b,
            r,
            &doors,
            Mat::Trim,
            cy - CROWN_H,
            cy,
            CROWN_P,
            CROWN_P,
            hall && r.level == 0,
            false,
        );
    }

    stairs(&mut b);
    hole_rails(&mut b);
    roof(&mut b);
    stoops(&mut b);

    let floor0 = Solid::new([-EXT, GROUND - 1.0, -EXT], [W + EXT, 0.0, D + EXT]);
    world.solids.push(floor0);
    let slab = |x0: f32, x1: f32, z0: f32, z1: f32| Solid::new([x0, CEIL0, z0], [x1, UPPER, z1]);
    world.solids.push(slab(-EXT, STAIR_X0, -EXT, D + EXT));
    world.solids.push(slab(STAIR_X0, STAIR_X1, -EXT, HOLE_Z0));
    world
        .solids
        .push(slab(STAIR_X0, STAIR_X1, STAIR_END, D + EXT));
    world.solids.push(slab(STAIR_X1, W + EXT, -EXT, D + EXT));
    world.ramps.push(Ramp {
        x0: RAIL_X0 + 0.03,
        x1: STAIR_X1,
        z0: STAIR_Z0 - 0.14,
        z1: STAIR_END,
        y0: 0.0,
        y1: UPPER,
    });
    world.solids.push(Solid::new(
        [RAIL_X0 - 0.07, 0.0, STAIR_Z0],
        [RAIL_X0 + 0.03, UPPER, STAIR_END],
    ));
    world.solids.push(Solid::new(
        [RAIL_X0 - 0.07, UPPER, HOLE_Z0],
        [RAIL_X0 + 0.03, UPPER + 1.1, STAIR_END],
    ));
    world.solids.push(Solid::new(
        [STAIR_X0, UPPER, HOLE_Z0 - 0.05],
        [STAIR_X1, UPPER + 1.1, HOLE_Z0 + 0.05],
    ));

    for p in models::placements() {
        for s in p.colliders() {
            world.solids.push(s);
        }
    }

    let lights = lights();
    let slots = slots();
    let log = b.take_log();
    let house = House {
        meshes: b.meshes,
        world,
        lights,
        slots,
        spawn: v3(7.55, 0.0, 1.3),
        spawn_yaw: std::f32::consts::PI,
    };
    (house, log)
}

fn stairs(b: &mut Builder) {
    let (x0, x1) = (STAIR_X0, STAIR_X1);
    for i in 1..=(RISERS - 1) {
        let z0 = STAIR_Z0 + (i as f32 - 1.0) * TREAD;
        let z1 = z0 + TREAD;
        let top = i as f32 * RISE;
        b.fy(Mat::Wood, top, x0, x1, z0, z1, 1.0);
        b.fz(Mat::Wood, z0, x0, x1, (i as f32 - 1.0) * RISE, top, -1.0);
        b.fx(Mat::Wood, x0, 0.0, top, z0, z1, -1.0);
    }
    b.fz(
        Mat::Wood,
        STAIR_END,
        x0,
        x1,
        (RISERS as f32 - 1.0) * RISE,
        UPPER,
        -1.0,
    );
    b.fz(Mat::Plaster, STAIR_END, x0, x1, 0.0, CEIL0, 1.0);
    b.fx(
        Mat::Plaster,
        STAIR_X0,
        CEIL0,
        UPPER,
        HOLE_Z0,
        STAIR_END,
        1.0,
    );
    b.fz(Mat::Plaster, HOLE_Z0, STAIR_X0, STAIR_X1, CEIL0, UPPER, 1.0);

    let surf = |z: f32| RISE * 0.5 + (z - STAIR_Z0) * (RISE / TREAD);
    let k = UPPER + RAIL_H - surf(STAIR_END);
    let rc = RAIL_X0 + 0.09;
    let za = STAIR_Z0 + 0.12;
    let zb = STAIR_END;
    b.beam(
        Mat::Wood,
        v3(rc, surf(za) + k, za),
        v3(rc, surf(zb) + k, zb),
        0.07,
        0.05,
    );
    for i in 1..=(RISERS - 1) {
        let zc = STAIR_Z0 + (i as f32 - 0.5) * TREAD;
        let top = i as f32 * RISE;
        let rail_y = surf(zc) + k;
        b.cuboid(
            Mat::Wood,
            v3(rc - 0.0175, top, zc - 0.0175),
            v3(rc + 0.0175, rail_y - 0.025, zc + 0.0175),
            SKIP_NY | SKIP_PY,
        );
    }
    b.cuboid(
        Mat::Wood,
        v3(rc - 0.05, 0.0, STAIR_Z0 - 0.02),
        v3(rc + 0.05, 1.2, STAIR_Z0 + 0.08),
        SKIP_NY,
    );
}

fn hole_rails(b: &mut Builder) {
    let y = UPPER + RAIL_H;
    let rc = RAIL_X0 + 0.03;
    b.beam(
        Mat::Wood,
        v3(rc, y, HOLE_Z0),
        v3(rc, y, STAIR_END),
        0.07,
        0.05,
    );
    let mut z = HOLE_Z0 + 0.1;
    while z < STAIR_END - 0.05 {
        b.cuboid(
            Mat::Wood,
            v3(rc - 0.0175, UPPER, z - 0.0175),
            v3(rc + 0.0175, y - 0.025, z + 0.0175),
            SKIP_NY | SKIP_PY,
        );
        z += 0.13;
    }
    b.cuboid(
        Mat::Wood,
        v3(rc - 0.05, UPPER, HOLE_Z0 - 0.05),
        v3(rc + 0.05, y + 0.12, HOLE_Z0 + 0.05),
        SKIP_NY,
    );
    let zc = HOLE_Z0 - 0.0;
    b.beam(
        Mat::Wood,
        v3(rc + 0.05, y, zc),
        v3(STAIR_X1, y, zc),
        0.07,
        0.05,
    );
    let mut x = rc + 0.13;
    while x < STAIR_X1 - 0.05 {
        b.cuboid(
            Mat::Wood,
            v3(x - 0.0175, UPPER, zc - 0.0175),
            v3(x + 0.0175, y - 0.025, zc + 0.0175),
            SKIP_NY | SKIP_PY,
        );
        x += 0.13;
    }
}

fn roof(b: &mut Builder) {
    let s = 0.5;
    let t = 0.16;
    let ov = 0.7;
    let x0 = -EXT - ov;
    let x1 = W + EXT + ov;
    let zr = D * 0.5;
    let top = |z: f32| TOP + t + s * (z + EXT);
    let z_e = -EXT - ov;
    let front_top_e = v3(0.0, top(z_e), z_e);
    let _ = front_top_e;
    let n_front = v3(0.0, 1.0, -s);
    let n_back = v3(0.0, 1.0, s);
    b.quad(
        Mat::Roof,
        v3(x0, top(z_e), z_e),
        v3(x1, top(z_e), z_e),
        v3(x1, top(zr), zr),
        v3(x0, top(zr), zr),
        n_front,
    );
    let zb = D - z_e;
    let topb = |z: f32| TOP + t + s * (D - z + EXT);
    b.quad(
        Mat::Roof,
        v3(x0, topb(zb), zb),
        v3(x1, topb(zb), zb),
        v3(x1, topb(zr), zr),
        v3(x0, topb(zr), zr),
        n_back,
    );
    b.quad(
        Mat::Roof,
        v3(x0, top(z_e) - t, z_e),
        v3(x1, top(z_e) - t, z_e),
        v3(x1, top(zr) - t, zr),
        v3(x0, top(zr) - t, zr),
        v3(0.0, -1.0, s),
    );
    b.quad(
        Mat::Roof,
        v3(x0, topb(zb) - t, zb),
        v3(x1, topb(zb) - t, zb),
        v3(x1, topb(zr) - t, zr),
        v3(x0, topb(zr) - t, zr),
        v3(0.0, -1.0, -s),
    );
    b.fz(Mat::Roof, z_e, x0, x1, top(z_e) - t, top(z_e), -1.0);
    b.fz(Mat::Roof, zb, x0, x1, topb(zb) - t, topb(zb), 1.0);
    for (x, dir) in [(x0, -1.0f32), (x1, 1.0f32)] {
        b.quad(
            Mat::Roof,
            v3(x, top(z_e) - t, z_e),
            v3(x, top(z_e), z_e),
            v3(x, top(zr), zr),
            v3(x, top(zr) - t, zr),
            v3(dir, 0.0, 0.0),
        );
        b.quad(
            Mat::Roof,
            v3(x, topb(zb) - t, zb),
            v3(x, topb(zb), zb),
            v3(x, topb(zr), zr),
            v3(x, topb(zr) - t, zr),
            v3(dir, 0.0, 0.0),
        );
    }
    for (x, dir) in [(-EXT, -1.0f32), (W + EXT, 1.0f32)] {
        b.tri(
            Mat::Stucco,
            v3(x, TOP, -EXT),
            v3(x, TOP, D + EXT),
            v3(x, top(zr) - t, zr),
            v3(dir, 0.0, 0.0),
        );
    }
}

fn stoops(b: &mut Builder) {
    b.cuboid(
        Mat::Concrete,
        v3(7.0, GROUND, -1.2),
        v3(9.0, 0.0, -EXT),
        SKIP_NY | SKIP_PZ,
    );
    b.cuboid(
        Mat::Concrete,
        v3(7.0, GROUND, D + EXT),
        v3(9.0, 0.0, D + 1.2),
        SKIP_NY | SKIP_NZ,
    );
}

fn lights() -> Vec<LightSpec> {
    let warm = [1.0, 0.72, 0.42];
    let sick = [0.78, 0.92, 0.72];
    let amber = [1.0, 0.55, 0.25];
    let cold = [0.62, 0.75, 1.0];
    let l = |name: &'static str,
             x: f32,
             y: f32,
             z: f32,
             color: [f32; 3],
             lumens: f32,
             range: f32,
             flicker: Flicker| LightSpec {
        pos: v3(x, y, z),
        color,
        lumens,
        range,
        flicker,
        name,
    };
    vec![
        l(
            "hall_front",
            8.0,
            2.55,
            1.5,
            warm,
            520.0,
            9.0,
            Flicker::Breathe,
        ),
        l(
            "hall_back",
            8.0,
            2.55,
            10.2,
            warm,
            380.0,
            8.0,
            Flicker::Steady,
        ),
        l(
            "living_lamp",
            1.0,
            1.55,
            5.5,
            amber,
            620.0,
            8.5,
            Flicker::Breathe,
        ),
        l(
            "study_lamp",
            2.0,
            1.2,
            10.9,
            warm,
            460.0,
            7.5,
            Flicker::Sputter,
        ),
        l(
            "kitchen_ceiling",
            14.2,
            2.7,
            2.6,
            sick,
            900.0,
            9.0,
            Flicker::Fluorescent,
        ),
        l(
            "dining_pendant",
            11.3,
            2.2,
            3.2,
            amber,
            360.0,
            6.5,
            Flicker::Steady,
        ),
        l(
            "bath_bulb",
            11.1,
            2.3,
            8.8,
            sick,
            520.0,
            6.5,
            Flicker::Fluorescent,
        ),
        l(
            "utility_bulb",
            14.4,
            2.7,
            9.0,
            warm,
            340.0,
            6.0,
            Flicker::Sputter,
        ),
        l(
            "landing_upper",
            8.0,
            5.7,
            10.5,
            warm,
            330.0,
            8.0,
            Flicker::Breathe,
        ),
        l(
            "hall_upper_front",
            8.0,
            5.7,
            1.5,
            warm,
            300.0,
            7.5,
            Flicker::Steady,
        ),
        l(
            "bedroom_a_lamp",
            3.9,
            4.15,
            5.7,
            amber,
            420.0,
            7.0,
            Flicker::Breathe,
        ),
        l(
            "bedroom_b_lamp",
            4.8,
            4.15,
            7.4,
            warm,
            380.0,
            7.0,
            Flicker::Steady,
        ),
        l(
            "bedroom_c_lamp",
            14.0,
            4.15,
            5.4,
            amber,
            360.0,
            7.0,
            Flicker::Sputter,
        ),
        l(
            "room_d_lamp",
            14.4,
            5.5,
            9.0,
            warm,
            280.0,
            6.0,
            Flicker::Steady,
        ),
        l(
            "bath_upper",
            11.1,
            5.6,
            8.8,
            sick,
            420.0,
            6.0,
            Flicker::Fluorescent,
        ),
        l(
            "moon_spill_study",
            3.2,
            2.4,
            11.0,
            cold,
            140.0,
            6.0,
            Flicker::Steady,
        ),
    ]
}

fn slots() -> Vec<Slot> {
    let s = |name: &'static str, need: &'static str, min: [f32; 3], max: [f32; 3]| Slot {
        name,
        need,
        min: v3(min[0], min[1], min[2]),
        max: v3(max[0], max[1], max[2]),
    };
    let u = UPPER;
    vec![
        s(
            "living_coffee_table",
            "coffee table 1.2 x 0.6 m",
            [2.4, 0.0, 3.7],
            [3.9, 0.45, 4.4],
        ),
        s(
            "living_tv_console",
            "TV and console 1.4 x 0.45 m",
            [2.65, 0.0, 0.05],
            [4.05, 1.1, 0.55],
        ),
        s(
            "living_armchair",
            "armchair 0.9 x 0.9 m",
            [0.3, 0.0, 5.0],
            [1.2, 0.95, 5.9],
        ),
        s(
            "living_rug",
            "rug 3 x 2 m",
            [1.8, 0.0, 2.2],
            [4.8, 0.02, 4.4],
        ),
        s(
            "study_desk",
            "desk 1.6 x 0.7 m",
            [2.3, 0.0, 11.3],
            [3.9, 0.78, 12.0],
        ),
        s(
            "study_chair",
            "desk chair",
            [2.7, 0.0, 10.6],
            [3.3, 0.95, 11.2],
        ),
        s(
            "study_bookshelf_west",
            "bookshelf 1.8 m wide, 2 m tall",
            [0.0, 0.0, 6.5],
            [0.4, 2.1, 8.3],
        ),
        s(
            "study_bookshelf_north",
            "bookshelf 1.8 m wide, 2 m tall",
            [0.0, 0.0, 10.1],
            [0.4, 2.1, 11.9],
        ),
        s(
            "hall_shoe_cabinet",
            "shoe cabinet or coat rack",
            [6.6, 0.0, 0.1],
            [7.1, 1.8, 1.3],
        ),
        s(
            "dining_table",
            "dining table 1.8 x 1.0 m",
            [10.5, 0.0, 2.7],
            [12.3, 0.78, 3.7],
        ),
        s(
            "dining_chair_1",
            "dining chair",
            [10.6, 0.0, 1.95],
            [11.1, 0.95, 2.45],
        ),
        s(
            "dining_chair_2",
            "dining chair",
            [11.7, 0.0, 1.95],
            [12.2, 0.95, 2.45],
        ),
        s(
            "dining_chair_3",
            "dining chair",
            [10.6, 0.0, 3.95],
            [11.1, 0.95, 4.45],
        ),
        s(
            "dining_chair_4",
            "dining chair",
            [11.7, 0.0, 3.95],
            [12.2, 0.95, 4.45],
        ),
        s(
            "bath_tub",
            "bathtub 1.7 x 0.75 m",
            [9.6, 0.0, 11.2],
            [11.3, 0.6, 11.98],
        ),
        s("bath_toilet", "toilet", [11.9, 0.0, 8.3], [12.65, 0.8, 9.0]),
        s(
            "bath_sink",
            "sink with mirror",
            [10.3, 0.0, 6.3],
            [11.2, 0.9, 6.9],
        ),
        s(
            "utility_washer",
            "washing machine",
            [15.3, 0.0, 6.3],
            [15.98, 0.9, 7.0],
        ),
        s(
            "utility_shelves",
            "metal shelving",
            [13.0, 0.0, 11.4],
            [14.6, 1.9, 11.98],
        ),
        s(
            "bedroom_a_nightstand",
            "nightstand",
            [3.75, u, 5.55],
            [4.3, u + 0.6, 6.1],
        ),
        s(
            "bedroom_a_wardrobe",
            "wardrobe 1.2 x 0.6 m, 2.1 m tall",
            [0.02, u, 3.4],
            [0.65, u + 2.1, 4.6],
        ),
        s(
            "bedroom_b_bed",
            "double bed 1.8 x 2.1 m",
            [2.6, u, 6.3],
            [4.4, u + 0.6, 8.4],
        ),
        s(
            "bedroom_b_nightstand_1",
            "nightstand",
            [2.0, u, 6.3],
            [2.55, u + 0.6, 6.85],
        ),
        s(
            "bedroom_b_nightstand_2",
            "nightstand",
            [4.45, u, 6.3],
            [5.0, u + 0.6, 6.85],
        ),
        s(
            "bedroom_b_wardrobe",
            "wardrobe 1.6 x 0.6 m",
            [5.7, u, 9.0],
            [6.4, u + 2.1, 10.7],
        ),
        s(
            "bedroom_c_desk",
            "desk 1.2 x 0.6 m",
            [10.0, u, 0.05],
            [11.2, u + 0.76, 0.7],
        ),
        s(
            "bedroom_c_wardrobe",
            "wardrobe 1.2 x 0.6 m",
            [9.6, u, 3.2],
            [10.25, u + 2.1, 4.5],
        ),
        s(
            "room_d_child_bed",
            "small bed or crib",
            [15.0, u, 6.4],
            [15.98, u + 0.6, 8.2],
        ),
        s(
            "room_d_toy_shelf",
            "shelf or toy chest",
            [13.0, u, 11.4],
            [14.2, u + 1.0, 11.98],
        ),
        s(
            "upper_bath_tub",
            "bathtub 1.7 x 0.75 m",
            [9.6, u, 11.2],
            [11.3, u + 0.6, 11.98],
        ),
        s(
            "upper_bath_toilet",
            "toilet",
            [11.9, u, 8.3],
            [12.65, u + 0.8, 9.0],
        ),
        s(
            "upper_bath_sink",
            "sink with mirror",
            [10.3, u, 6.3],
            [11.2, u + 0.9, 6.9],
        ),
        s(
            "upper_hall_bookcase",
            "bookcase or console",
            [6.6, u, 10.2],
            [7.1, u + 1.8, 11.6],
        ),
    ]
}
