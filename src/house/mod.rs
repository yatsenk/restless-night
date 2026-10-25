use crate::sim::collision::{Ramp, Solid, World};
use crate::sim::doors::{DoorSpec, LEAF_HALF, LIMIT};
use crate::house::geom::{Builder, Mat, MeshData, SKIP_NX, SKIP_NY, SKIP_NZ, SKIP_PX, SKIP_PY, SKIP_PZ};
use crate::sim::math::{v3, V3};

pub mod geom;
pub mod models;
mod plan;
mod shell;
mod stairs;

pub const EXT: f32 = 0.30;
pub const HALF: f32 = 0.08;
pub const GROUND: f32 = -0.36;
pub const FOUNDATION_Y: f32 = 0.12;
pub const CEIL0: f32 = 2.7;
pub const UPPER: f32 = 3.0;
pub const TOP: f32 = 5.4;

pub const BAY_X1: f32 = 3.5;
pub const MX0: f32 = 3.66;
pub const MX1: f32 = 15.2;
pub const MZ1: f32 = 10.3;
pub const G_Z0: f32 = -1.0;
pub const G_Z1: f32 = 5.4;
pub const U_Z0: f32 = 5.7;
pub const P_X1: f32 = 7.7;
pub const P_Z0: f32 = -0.3;
pub const P_Z1: f32 = 1.5;
pub const H_Z0: f32 = 1.8;
pub const OUT_X0: f32 = -EXT;
pub const OUT_X1: f32 = MX1 + EXT;
pub const OUT_Z0: f32 = G_Z0 - EXT;
pub const OUT_Z1: f32 = MZ1 + EXT;

pub const S_X0: f32 = MX0;
pub const S_X1: f32 = 4.91;
pub const S_Z0: f32 = 5.0;
pub const RISERS: usize = 16;
pub const TREAD: f32 = 0.26;
pub const RISE: f32 = UPPER / RISERS as f32;
pub const S_END: f32 = S_Z0 + (RISERS as f32 - 1.0) * TREAD;
pub const HOLE_Z0: f32 = S_Z0 + 1.5 * TREAD;
pub const RAIL_H: f32 = 0.92;

pub const B_FLOOR: f32 = -2.7;
pub const B_CEIL: f32 = -0.25;
pub const B_STEPS: usize = 14;
pub const B_RISE: f32 = -B_FLOOR / B_STEPS as f32;
pub const B_TREAD: f32 = 0.25;
pub const B_ZT: f32 = 4.3;
pub const B_ZE: f32 = B_ZT - (B_STEPS as f32 - 1.0) * B_TREAD;
pub const B_HZ0: f32 = B_ZE - 0.3;
pub const B_X1: f32 = 1.0;
pub const BX1: f32 = 7.4;
pub const BZ1: f32 = MZ1;
pub const B_Z0: f32 = 0.0;

pub const LAWN_R: f32 = 60.0;
pub const SIDEWALK_Z0: f32 = -8.4;
pub const SIDEWALK_Z1: f32 = -7.0;
pub const LOT_X0: f32 = -5.5;
pub const LOT_X1: f32 = 19.5;
pub const LOT_Z0: f32 = -8.4;
pub const LOT_Z1: f32 = 20.6;
pub const DRIVE_X0: f32 = 0.15;
pub const DRIVE_X1: f32 = 3.35;
pub const PATH_X0: f32 = 4.9;
pub const PATH_X1: f32 = 6.0;
pub const STEP_X0: f32 = 4.5;
pub const STEP_X1: f32 = 6.5;

const DOOR_H: f32 = 2.3;
const WIN_SILL: f32 = 0.9;
const WIN_HEAD: f32 = 2.15;
const SKIRT_H: f32 = 0.12;
const SKIRT_P: f32 = 0.02;
const CROWN_H: f32 = 0.08;
const CROWN_P: f32 = 0.05;
const CASE_W: f32 = 0.075;
const CASE_P: f32 = 0.015;
const LEAF_W: f32 = 0.92;
const OPEN_W: f32 = 0.94;

#[derive(Clone, Copy, PartialEq, Debug)]
enum Axis {
    X,
    Z,
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Kind {
    Door,
    Opening,
    ExtDoor,
    GarageDoor,
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
    hinge_hi: bool,
    lo: f32,
    hi: f32,
}

impl Op {
    fn new(a0: f32, a1: f32, y0: f32, y1: f32, kind: Kind) -> Op {
        Op {
            a0,
            a1,
            y0,
            y1,
            kind,
            hinge_hi: false,
            lo: -LIMIT,
            hi: LIMIT,
        }
    }

    fn open(&self) -> bool {
        self.kind == Kind::Door
    }

    fn passable(&self) -> bool {
        matches!(self.kind, Kind::Door | Kind::Opening)
    }

    fn door_like(&self) -> bool {
        matches!(
            self.kind,
            Kind::Door | Kind::Opening | Kind::ExtDoor | Kind::GarageDoor
        )
    }
}

#[derive(Clone, Copy, Debug)]
struct Hole {
    x0: f32,
    x1: f32,
    z0: f32,
    z1: f32,
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
    hole_floor: Option<Hole>,
    hole_ceil: Option<Hole>,
}

impl Room {
    fn new(x0: f32, x1: f32, z0: f32, z1: f32, level: u8, floor: Mat, wall: Mat) -> Room {
        Room {
            x0,
            x1,
            z0,
            z1,
            level,
            floor,
            wall,
            ceil: Mat::Plaster,
            hole_floor: None,
            hole_ceil: None,
        }
    }

    fn contains(&self, x: f32, z: f32) -> bool {
        x >= self.x0 && x <= self.x1 && z >= self.z0 && z <= self.z1
    }

    fn floor_y(&self) -> f32 {
        if self.level == 0 {
            0.0
        } else {
            UPPER
        }
    }

    fn ceil_y(&self) -> f32 {
        if self.level == 0 {
            CEIL0
        } else {
            TOP
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct LightSpec {
    pub pos: V3,
    pub color: [f32; 3],
    pub lumens: f32,
    pub range: f32,
    pub name: &'static str,
    pub kind: LightKind,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LightKind {
    Practical,
    Window,
}

#[derive(Clone, Debug)]
pub struct Slot {
    pub name: &'static str,
    pub need: &'static str,
    pub min: V3,
    pub max: V3,
}

pub type QuadRec = crate::house::geom::LoggedQuad;

pub struct House {
    pub meshes: Vec<MeshData>,
    pub world: World,
    pub lights: Vec<LightSpec>,
    pub slots: Vec<Slot>,
    pub doors: Vec<DoorSpec>,
    pub leaf_wood: MeshData,
    pub leaf_metal: MeshData,
    pub spawn: V3,
    pub spawn_yaw: f32,
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

fn in_hole(h: &Hole, x: f32, z: f32) -> bool {
    x > h.x0 && x < h.x1 && z > h.z0 && z < h.z1
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
    cap_lo: bool,
    cap_hi: bool,
    cap_mat: Mat,
    exterior: bool,
}

impl Wall {
    fn new(axis: Axis, lo: f32, hi: f32, a0: f32, a1: f32, y0: f32, y1: f32) -> Wall {
        Wall {
            axis,
            lo,
            hi,
            a0,
            a1,
            y0,
            y1,
            ops: Vec::new(),
            breaks: Vec::new(),
            cap_lo: false,
            cap_hi: false,
            cap_mat: Mat::Plaster,
            exterior: false,
        }
    }

    fn ext(axis: Axis, lo: f32, hi: f32, a0: f32, a1: f32, y0: f32, y1: f32) -> Wall {
        let mut w = Wall::new(axis, lo, hi, a0, a1, y0, y1);
        w.exterior = true;
        w.cap_mat = Mat::Siding;
        w
    }

    fn center(&self) -> f32 {
        (self.lo + self.hi) * 0.5
    }

    fn pt(&self, a: f32, across: f32) -> (f32, f32) {
        match self.axis {
            Axis::X => (a, across),
            Axis::Z => (across, a),
        }
    }

    fn room_side(&self, rs: &[Room], a: f32, level: u8, hi_side: bool) -> bool {
        let across = if hi_side {
            self.hi + 0.05
        } else {
            self.lo - 0.05
        };
        let (px, pz) = self.pt(a, across);
        room_at(rs, px, pz, level).map_or(false, |r| r.wall != Mat::Siding)
    }

    fn face_mat(&self, rs: &[Room], a: f32, y: f32, hi_side: bool) -> Option<Mat> {
        let across = if hi_side {
            self.hi + 0.05
        } else {
            self.lo - 0.05
        };
        let (px, pz) = self.pt(a, across);
        if self.exterior {
            let room = if y <= 0.0 {
                room_at(rs, px, pz, 0)
            } else if y < CEIL0 {
                room_at(rs, px, pz, 0)
            } else if y <= UPPER {
                room_at(rs, px, pz, 1).or_else(|| room_at(rs, px, pz, 0))
            } else {
                room_at(rs, px, pz, 1)
            };
            return match room {
                Some(r) => {
                    if y <= 0.0 {
                        None
                    } else {
                        Some(r.wall)
                    }
                }
                None => Some(if y < FOUNDATION_Y {
                    Mat::Concrete
                } else {
                    Mat::Siding
                }),
            };
        }
        level_of(y).and_then(|l| room_at(rs, px, pz, l).map(|r| r.wall))
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

fn box_at(axis: Axis, across0: f32, across1: f32, a0: f32, a1: f32, y0: f32, y1: f32) -> (V3, V3) {
    match axis {
        Axis::X => (v3(a0, y0, across0), v3(a1, y1, across1)),
        Axis::Z => (v3(across0, y0, a0), v3(across1, y1, a1)),
    }
}

fn casing(b: &mut Builder, w: &Wall, o: &Op, hi_side: bool, cm: Mat) {
    let face = if hi_side { w.hi } else { w.lo };
    let (c0, c1) = if hi_side {
        (face, face + CASE_P)
    } else {
        (face - CASE_P, face)
    };
    let door_like = o.door_like();
    let cw = CASE_W;
    let (l0, l1) = (o.a0 - cw, o.a0);
    let (r0, r1) = (o.a1, o.a1 + cw);
    let ybot = o.y0;
    let (mn, mx) = box_at(w.axis, c0, c1, l0, l1, ybot, o.y1);
    b.cuboid(cm, mn, mx, 0);
    let (mn, mx) = box_at(w.axis, c0, c1, r0, r1, ybot, o.y1);
    b.cuboid(cm, mn, mx, 0);
    let (mn, mx) = box_at(w.axis, c0, c1, l0, r1, o.y1, o.y1 + cw);
    b.cuboid(cm, mn, mx, 0);
    if !door_like {
        let (s0, s1) = if hi_side {
            (face, face + 0.06)
        } else {
            (face - 0.06, face)
        };
        let (mn, mx) = box_at(w.axis, s0, s1, l0 - 0.02, r1 + 0.02, o.y0 - 0.04, o.y0);
        b.cuboid(cm, mn, mx, 0);
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
    let mut cuts: Vec<(f32, f32, f32, f32)> = Vec::new();
    for o in &w.ops {
        if o.passable() {
            cuts.push((o.a0, o.a1, o.y0, o.y1));
        }
    }
    cuts.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    let push = |a0: f32, a1: f32, y0: f32, y1: f32, out: &mut Vec<Solid>| {
        if a1 - a0 < 1e-4 || y1 - y0 < 1e-4 {
            return;
        }
        let (mn, mx) = box_at(w.axis, w.lo, w.hi, a0, a1, y0, y1);
        out.push(Solid { min: mn, max: mx });
    };
    let mut cur = w.a0;
    for (c0, c1, y0, y1) in cuts {
        push(cur, c0, w.y0, w.y1, out);
        push(c0, c1, w.y0, y0, out);
        push(c0, c1, y1, w.y1, out);
        cur = c1;
    }
    push(cur, w.a1, w.y0, w.y1, out);
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

fn wall_xs(ws: &[Wall], w: &Wall) -> Vec<f32> {
    let mut xs = vec![w.a0, w.a1];
    for o in &w.ops {
        xs.push(o.a0);
        xs.push(o.a1);
    }
    for &k in &w.breaks {
        xs.push(k);
    }
    for v in ws {
        if v.axis == w.axis {
            continue;
        }
        if v.a1 >= w.lo - 1e-4 && v.a0 <= w.hi + 1e-4 {
            xs.push(v.lo);
            xs.push(v.hi);
        }
    }
    dedup(xs)
        .into_iter()
        .filter(|&x| x >= w.a0 - 1e-4 && x <= w.a1 + 1e-4)
        .collect()
}

fn buried(ws: &[Wall], me: &Wall, across: f32, a: f32, y: f32, dir: f32) -> bool {
    let (px, pz) = me.pt(a, across + dir * 0.01);
    ws.iter().any(|v| {
        if std::ptr::eq(v, me) {
            return false;
        }
        let (al, ac) = match v.axis {
            Axis::X => (px, pz),
            Axis::Z => (pz, px),
        };
        al > v.a0 && al < v.a1 && ac > v.lo && ac < v.hi && y > v.y0 && y < v.y1
    })
}

fn build_wall(b: &mut Builder, rs: &[Room], ws: &[Wall], w: &Wall) {
    let mut ys = vec![w.y0, w.y1, 0.0, FOUNDATION_Y, CEIL0, UPPER, TOP];
    for o in &w.ops {
        ys.push(o.y0);
        ys.push(o.y1);
    }
    let xs = wall_xs(ws, w);
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
                if buried(ws, w, across, ca, cy, dir) {
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
        let rm = if o.passable() { Mat::Trim } else { Mat::Frame };
        match w.axis {
            Axis::X => {
                b.fx(rm, a0, o.y0, o.y1, w.lo, w.hi, 1.0);
                b.fx(rm, a1, o.y0, o.y1, w.lo, w.hi, -1.0);
                b.fy(rm, o.y1, a0, a1, w.lo, w.hi, -1.0);
                if o.y0 > 0.01 && (o.y0 - UPPER).abs() > 0.01 {
                    b.fy(rm, o.y0, a0, a1, w.lo, w.hi, 1.0);
                }
            }
            Axis::Z => {
                b.fz(rm, a0, w.lo, w.hi, o.y0, o.y1, 1.0);
                b.fz(rm, a1, w.lo, w.hi, o.y0, o.y1, -1.0);
                b.fy(rm, o.y1, w.lo, w.hi, a0, a1, -1.0);
                if o.y0 > 0.01 && (o.y0 - UPPER).abs() > 0.01 {
                    b.fy(rm, o.y0, w.lo, w.hi, a0, a1, 1.0);
                }
            }
        }
    }
    let cap_top = if (w.y1 - UPPER).abs() < 1e-4 {
        CEIL0
    } else {
        w.y1
    };
    if w.cap_lo {
        match w.axis {
            Axis::Z => b.fz(w.cap_mat, w.a0, w.lo, w.hi, w.y0, cap_top, -1.0),
            Axis::X => b.fx(w.cap_mat, w.a0, w.y0, cap_top, w.lo, w.hi, -1.0),
        }
    }
    if w.cap_hi {
        match w.axis {
            Axis::Z => b.fz(w.cap_mat, w.a1, w.lo, w.hi, w.y0, cap_top, 1.0),
            Axis::X => b.fx(w.cap_mat, w.a1, w.y0, cap_top, w.lo, w.hi, 1.0),
        }
    }
}

fn garage_door(b: &mut Builder, w: &Wall, o: &Op) {
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
    b.cuboid(Mat::Frame, mn, mx, 0);
    let rows = 4;
    for k in 1..=rows {
        let y = o.y0 + (o.y1 - o.y0) * k as f32 / (rows + 1) as f32;
        let (mn, mx) = box_at(
            w.axis,
            mid - 0.05,
            mid + 0.05,
            o.a0 + 0.03,
            o.a1 - 0.03,
            y - 0.02,
            y + 0.02,
        );
        b.cuboid(Mat::Trim, mn, mx, 0);
    }
    let (mn, mx) = box_at(
        w.axis,
        mid - 0.05,
        mid + 0.05,
        (o.a0 + o.a1) * 0.5 - 0.1,
        (o.a0 + o.a1) * 0.5 + 0.1,
        1.0,
        1.08,
    );
    b.cuboid(Mat::Trim, mn, mx, 0);
}



fn party_wall(w: &Wall) -> bool {
    w.axis == Axis::Z && (w.center() - (BAY_X1 + MX0) * 0.5).abs() < 0.01
}

fn trim_cuts(w: &Wall, level: u8, probe: f32, hi_side: bool, crown: bool) -> Vec<(f32, f32)> {
    let mut cuts = Vec::new();
    if !crown {
        for o in &w.ops {
            if o.door_like() && o.y0 < probe && o.y1 > probe {
                cuts.push((o.a0 - CASE_W, o.a1 + CASE_W));
            }
        }
    }
    let hole_here = party_wall(w) && hi_side;
    if hole_here && ((crown && level == 0) || (!crown && level == 1)) {
        cuts.push((HOLE_Z0 - 0.001, S_END + 0.001));
    }
    if hole_here && !crown && level == 0 {
        cuts.push((S_Z0 - 0.12, S_END + 0.001));
    }
    cuts
}

fn trims(b: &mut Builder, rs: &[Room], ws: &[Wall]) {
    for w in ws {
        let mut xs = wall_xs(ws, w);
        for r in rs {
            let (e0, e1) = match w.axis {
                Axis::X => (r.x0, r.x1),
                Axis::Z => (r.z0, r.z1),
            };
            xs.push(e0);
            xs.push(e1);
        }
        let xs: Vec<f32> = dedup(xs)
            .into_iter()
            .filter(|&x| x >= w.a0 - 1e-4 && x <= w.a1 + 1e-4)
            .collect();
        for level in 0..2u8 {
            let floor_y = if level == 0 { 0.0 } else { UPPER };
            let ceil_y = if level == 0 { CEIL0 } else { TOP };
            let probe = floor_y + 0.5;
            if probe < w.y0 || probe > w.y1 {
                continue;
            }
            for hi_side in [false, true] {
                let mut spans: Vec<(f32, f32)> = Vec::new();
                for i in 0..xs.len() - 1 {
                    let ca = (xs[i] + xs[i + 1]) * 0.5;
                    let face_c = if hi_side { w.hi } else { w.lo };
                    let dir = if hi_side { 1.0 } else { -1.0 };
                    if w.room_side(rs, ca, level, hi_side) && !buried(ws, w, face_c, ca, probe, dir)
                    {
                        match spans.last_mut() {
                            Some(last) if (last.1 - xs[i]).abs() < 1e-4 => last.1 = xs[i + 1],
                            _ => spans.push((xs[i], xs[i + 1])),
                        }
                    }
                }
                let face = if hi_side { w.hi } else { w.lo };
                let mut open_floor: Vec<(f32, f32)> = Vec::new();
                let mut open_ceil: Vec<(f32, f32)> = Vec::new();
                for i in 0..xs.len() - 1 {
                    let ca = (xs[i] + xs[i + 1]) * 0.5;
                    let across = if hi_side { w.hi + 0.05 } else { w.lo - 0.05 };
                    let (px, pz) = w.pt(ca, across);
                    if let Some(r) = room_at(rs, px, pz, level) {
                        if r.hole_floor.map_or(false, |h| in_hole(&h, px, pz)) {
                            open_floor.push((xs[i], xs[i + 1]));
                        }
                        if r.hole_ceil.map_or(false, |h| in_hole(&h, px, pz)) {
                            open_ceil.push((xs[i], xs[i + 1]));
                        }
                    }
                }
                for crown in [false, true] {
                    if crown && ceil_y > w.y1 + 1e-3 {
                        continue;
                    }
                    let (y0, y1, proud) = if crown {
                        (ceil_y - CROWN_H, ceil_y, CROWN_P)
                    } else {
                        (floor_y, floor_y + SKIRT_H, SKIRT_P)
                    };
                    let mut cuts = trim_cuts(w, level, probe, hi_side, crown);
                    cuts.extend(if crown { open_ceil.iter() } else { open_floor.iter() });
                    for &(s0, s1) in &spans {
                        let (s0, s1) = if w.axis == Axis::Z {
                            (s0 + proud, s1 - proud)
                        } else {
                            (s0, s1)
                        };
                        if s1 - s0 < 1e-3 {
                            continue;
                        }
                        for (p0, p1) in subtract((s0, s1), &cuts) {
                            let (c0, c1) = if hi_side {
                                (face, face + proud)
                            } else {
                                (face - proud, face)
                            };
                            let (mn, mx) = box_at(w.axis, c0, c1, p0, p1, y0, y1);
                            let skip = if crown { SKIP_PY } else { SKIP_NY };
                            b.cuboid(Mat::Trim, mn, mx, skip);
                        }
                    }
                }
            }
        }
    }
}

fn door_leaf() -> (MeshData, MeshData) {
    let mut b = Builder::new();
    let len = LEAF_W;
    let h = DOOR_H - 0.02;
    let t = LEAF_HALF;
    b.cuboid(Mat::Door, v3(0.0, 0.0, -t), v3(len, h, t), 0);
    for s in [-1.0f32, 1.0] {
        let (z0, z1) = if s > 0.0 {
            (t, t + 0.008)
        } else {
            (-t - 0.008, -t)
        };
        let skip = if s > 0.0 { SKIP_NZ } else { SKIP_PZ };
        for (y0, y1) in [(0.14, 0.98), (1.12, h - 0.14)] {
            b.cuboid(Mat::Door, v3(0.12, y0, z0), v3(len - 0.12, y1, z1), skip);
        }
        let face = if s > 0.0 { t } else { -t };
        let (r0, r1) = if s > 0.0 {
            (face, face + 0.03)
        } else {
            (face - 0.03, face)
        };
        b.cuboid(
            Mat::Frame,
            v3(len - 0.11, 0.96, r0),
            v3(len - 0.07, 1.04, r1),
            if s > 0.0 { SKIP_NZ } else { SKIP_PZ },
        );
        let (l0, l1) = if s > 0.0 {
            (face + 0.025, face + 0.035)
        } else {
            (face - 0.035, face - 0.025)
        };
        b.cuboid(
            Mat::Frame,
            v3(len - 0.2, 0.99, l0),
            v3(len - 0.07, 1.01, l1),
            0,
        );
    }
    (
        b.meshes[Mat::Door.index()].clone(),
        b.meshes[Mat::Frame.index()].clone(),
    )
}

fn door_specs(ws: &[Wall]) -> Vec<DoorSpec> {
    let mut v = Vec::new();
    for w in ws {
        for o in &w.ops {
            if !o.open() {
                continue;
            }
            let a_h = if o.hinge_hi { o.a1 - 0.01 } else { o.a0 + 0.01 };
            let (hx, hz) = w.pt(a_h, w.center());
            let closed = match (w.axis, o.hinge_hi) {
                (Axis::X, false) => 0.0,
                (Axis::X, true) => std::f32::consts::PI,
                (Axis::Z, false) => std::f32::consts::FRAC_PI_2,
                (Axis::Z, true) => -std::f32::consts::FRAC_PI_2,
            };
            v.push(DoorSpec {
                hx,
                hz,
                closed,
                len: LEAF_W,
                y0: o.y0 + 0.01,
                height: DOOR_H - 0.02,
                lo: o.lo,
                hi: o.hi,
            });
        }
    }
    v
}

fn hole_grid(b: &mut Builder, m: Mat, y: f32, r: &Room, h: &Hole, dir: f32) {
    b.grid_y(
        m,
        y,
        &[r.x0, h.x0, h.x1, r.x1],
        &[r.z0, h.z0, h.z1, r.z1],
        Some((1, 1)),
        dir,
    );
}

fn sol(w: &mut World, x0: f32, y0: f32, z0: f32, x1: f32, y1: f32, z1: f32) {
    w.solids.push(Solid::new([x0, y0, z0], [x1, y1, z1]));
}

fn side_mat(rs: &[Room], w: &Wall, o: &Op, hi_side: bool) -> Mat {
    if o.passable() {
        return Mat::Trim;
    }
    let level = if o.y0 >= UPPER - 0.01 { 1 } else { 0 };
    let across = if hi_side { w.hi + 0.05 } else { w.lo - 0.05 };
    let (px, pz) = w.pt((o.a0 + o.a1) * 0.5, across);
    match room_at(rs, px, pz, level) {
        Some(r) if r.wall != Mat::Siding => Mat::Trim,
        _ => Mat::Frame,
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
    let rs = plan::rooms();
    let ws = plan::walls();
    let mut world = World::new(B_FLOOR - 1.0);

    for w in &ws {
        build_wall(&mut b, &rs, &ws, w);
        wall_solids(w, &mut world.solids);
        for o in &w.ops {
            let fy = if o.y0 >= UPPER - 0.01 { UPPER } else { 0.0 };
            let sill = |b: &mut Builder, m: Mat| match w.axis {
                Axis::X => b.fy(m, fy + 0.004, o.a0, o.a1, w.lo, w.hi, 1.0),
                Axis::Z => b.fy(m, fy + 0.004, w.lo, w.hi, o.a0, o.a1, 1.0),
            };
            match o.kind {
                Kind::Door => {
                    casing(&mut b, w, o, false, side_mat(&rs, w, o, false));
                    casing(&mut b, w, o, true, side_mat(&rs, w, o, true));
                    sill(&mut b, Mat::Wood);
                }
                Kind::Opening => {
                    casing(&mut b, w, o, false, side_mat(&rs, w, o, false));
                    casing(&mut b, w, o, true, side_mat(&rs, w, o, true));
                    sill(&mut b, Mat::Wood);
                }
                Kind::ExtDoor => {
                    casing(&mut b, w, o, false, side_mat(&rs, w, o, false));
                    casing(&mut b, w, o, true, side_mat(&rs, w, o, true));
                    exterior_door(&mut b, w, o);
                    sill(&mut b, Mat::Concrete);
                }
                Kind::GarageDoor => {
                    casing(&mut b, w, o, false, side_mat(&rs, w, o, false));
                    casing(&mut b, w, o, true, side_mat(&rs, w, o, true));
                    garage_door(&mut b, w, o);
                    sill(&mut b, Mat::Concrete);
                }
                Kind::Window => {
                    casing(&mut b, w, o, false, side_mat(&rs, w, o, false));
                    casing(&mut b, w, o, true, side_mat(&rs, w, o, true));
                    window_frame(&mut b, w, o);
                }
                Kind::ModelWindow => {}
            }
        }
    }

    for r in &rs {
        let fy = r.floor_y();
        let cy = r.ceil_y();
        match r.hole_floor {
            Some(h) => hole_grid(&mut b, r.floor, fy, r, &h, 1.0),
            None => b.fy(r.floor, fy, r.x0, r.x1, r.z0, r.z1, 1.0),
        }
        match r.hole_ceil {
            Some(h) => hole_grid(&mut b, r.ceil, cy, r, &h, -1.0),
            None => b.fy(r.ceil, cy, r.x0, r.x1, r.z0, r.z1, -1.0),
        }
    }
    trims(&mut b, &rs, &ws);
    stairs::stairs(&mut b);
    stairs::hole_rails(&mut b);
    shell::basement(&mut b);
    shell::roofs(&mut b);
    shell::porch(&mut b);
    shell::ground(&mut b);

    let deep = B_FLOOR - 1.0;
    let w = &mut world;
    sol(w, OUT_X0, deep, OUT_Z0, MX0, 0.0, 0.0);
    sol(w, OUT_X0, deep, 0.0, 0.0, 0.0, OUT_Z1);
    sol(w, MX0, deep, P_Z0, OUT_X1, 0.0, 0.0);
    sol(w, BX1, deep, 0.0, OUT_X1, 0.0, OUT_Z1);
    sol(w, 0.0, deep, BZ1, BX1, 0.0, OUT_Z1);
    sol(w, B_X1, B_CEIL, 0.0, BX1, 0.0, BZ1);
    sol(w, 0.0, B_CEIL, 0.0, B_X1, 0.0, B_HZ0);
    sol(w, 0.0, B_CEIL, B_ZT, B_X1, 0.0, BZ1);
    sol(w, 0.0, deep, 0.0, BX1, B_FLOOR, BZ1);
    sol(w, B_X1, B_FLOOR, B_ZE, B_X1 + 0.16, 0.0, B_ZT);
    let slope = B_RISE / B_TREAD;
    let bz1 = B_ZT + 0.06;
    w.ramps.push(Ramp {
        x0: 0.0,
        x1: B_X1,
        z0: bz1 - (-B_FLOOR) / slope,
        z1: bz1,
        y0: B_FLOOR,
        y1: 0.0,
    });

    for (x0, x1, z0, z1) in shell::lawn_rects() {
        sol(w, x0, deep, z0, x1, GROUND, z1);
    }
    sol(w, STEP_X0, deep, P_Z0 - 0.6, STEP_X1, GROUND + 0.18, P_Z0);
    sol(w, 2.0, deep, OUT_Z1, 3.7, 0.0, OUT_Z1 + 0.5);
    sol(w, 2.0, deep, OUT_Z1 + 0.5, 3.7, GROUND + 0.17, OUT_Z1 + 1.0);
    w.ramps.push(Ramp {
        x0: DRIVE_X0,
        x1: DRIVE_X1,
        z0: SIDEWALK_Z1,
        z1: G_Z0 - EXT,
        y0: GROUND + 0.01,
        y1: -0.02,
    });
    let wall_h = GROUND + 3.0;
    sol(w, LOT_X0 - 0.5, deep, LOT_Z0 - 0.5, LOT_X0, wall_h, LOT_Z1 + 0.5);
    sol(w, LOT_X1, deep, LOT_Z0 - 0.5, LOT_X1 + 0.5, wall_h, LOT_Z1 + 0.5);
    sol(w, LOT_X0 - 0.5, deep, LOT_Z0 - 0.5, LOT_X1 + 0.5, wall_h, LOT_Z0);
    sol(w, LOT_X0 - 0.5, deep, LOT_Z1, LOT_X1 + 0.5, wall_h, LOT_Z1 + 0.5);

    sol(w, MX0, CEIL0, -EXT, OUT_X1, UPPER, HOLE_Z0);
    sol(w, MX0, CEIL0, S_END, OUT_X1, UPPER, OUT_Z1);
    sol(w, S_X1, CEIL0, HOLE_Z0, OUT_X1, UPPER, S_END);
    let rc = S_X1 - 0.05;
    w.ramps.push(Ramp {
        x0: S_X0,
        x1: rc - 0.05,
        z0: S_Z0 - 0.14,
        z1: S_END,
        y0: 0.0,
        y1: UPPER,
    });
    sol(w, rc - 0.05, 0.0, S_Z0, rc + 0.05, UPPER, S_END);
    sol(w, rc - 0.05, UPPER, HOLE_Z0, rc + 0.05, UPPER + 1.1, S_END);
    sol(w, S_X0, UPPER, HOLE_Z0 - 0.05, S_X1, UPPER + 1.1, HOLE_Z0 + 0.05);

    for p in models::placements() {
        for s in p.colliders() {
            world.solids.push(s);
        }
    }

    let doors = door_specs(&ws);
    let (leaf_wood, leaf_metal) = door_leaf();
    let lights = plan::lights();
    let slots = plan::slots();
    let log = b.take_log();
    let house = House {
        meshes: b.meshes,
        world,
        lights,
        slots,
        doors,
        leaf_wood,
        leaf_metal,
        spawn: v3(5.6, 0.0, 3.0),
        spawn_yaw: std::f32::consts::PI,
    };
    (house, log)
}
