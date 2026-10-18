use crate::collision::{Ramp, Solid, World};
use crate::doors::{DoorSpec, LEAF_HALF, LIMIT};
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
pub const CLOSET_X: f32 = 1.5;
pub const CLOSET_Z: f32 = 5.9;
pub const CLOSET_IN: f32 = CLOSET_X - HALF;
pub const B_FLOOR: f32 = -2.8;
pub const B_CEIL: f32 = -0.25;
pub const B_Z0: f32 = 6.4;
pub const B_TREAD: f32 = 0.27;
pub const B_STEPS: usize = 14;
pub const B_RISE: f32 = 2.8 / B_STEPS as f32;
pub const B_END: f32 = B_Z0 + (B_STEPS as f32 - 1.0) * B_TREAD;
pub const B_ROOM_Z0: f32 = 2.0;
pub const TW_X: f32 = 13.5;
pub const TW_Z: f32 = 9.3;
pub const KX: f32 = 10.9;
pub const KZ: f32 = 5.4;
pub const GARAGE_X0: f32 = 1.0;
pub const GARAGE_X1: f32 = 5.4;
const DOOR_H: f32 = 2.15;
const WIN_SILL: f32 = 0.9;
const WIN_HEAD: f32 = 2.25;
const SKIRT_H: f32 = 0.12;
const SKIRT_P: f32 = 0.02;
const CROWN_H: f32 = 0.09;
const CROWN_P: f32 = 0.06;
const CASE_W: f32 = 0.085;
const CASE_P: f32 = 0.015;
const LEAF_W: f32 = 0.93;

#[derive(Clone, Copy, PartialEq, Debug)]
enum Axis {
    X,
    Z,
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Kind {
    Door,
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

    fn door_like(&self) -> bool {
        matches!(self.kind, Kind::Door | Kind::ExtDoor | Kind::GarageDoor)
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
    hole_floor: bool,
    hole_ceil: bool,
    shaft: bool,
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
            hole_floor: false,
            hole_ceil: false,
            shaft: false,
        }
    }

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
pub struct LightSpec {
    pub pos: V3,
    pub color: [f32; 3],
    pub lumens: f32,
    pub range: f32,
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
    pub doors: Vec<DoorSpec>,
    pub leaf_wood: MeshData,
    pub leaf_metal: MeshData,
    pub spawn: V3,
    pub spawn_yaw: f32,
}

fn rooms() -> Vec<Room> {
    let tz = TW_Z - HALF;
    let mut v = Vec::new();
    let mut garage = Room::new(0.0, PX1 - HALF, 0.0, D, 0, Mat::Concrete, Mat::Plaster);
    garage.shaft = true;
    v.push(garage);
    let mut hall = Room::new(PX1 + HALF, PX2 - HALF, 0.0, tz, 0, Mat::Floor, Mat::Plaster);
    hall.hole_ceil = true;
    v.push(hall);
    v.push(Room::new(
        PX2 - HALF,
        PX2 + HALF,
        0.0,
        STAIR_Z0 - 0.1,
        0,
        Mat::Floor,
        Mat::Plaster,
    ));
    v.push(Room::new(
        PX2 - HALF,
        PX2 + HALF,
        STAIR_END,
        tz,
        0,
        Mat::Floor,
        Mat::Plaster,
    ));
    v.push(Room::new(
        PX2 + HALF,
        KX,
        0.0,
        tz,
        0,
        Mat::Floor,
        Mat::Plaster,
    ));
    v.push(Room::new(KX, W, 0.0, KZ, 0, Mat::Tile, Mat::Plaster));
    v.push(Room::new(KX, W, KZ, tz, 0, Mat::Floor, Mat::Plaster));
    v.push(Room::new(
        PX1 + HALF,
        TW_X - HALF,
        tz,
        D,
        0,
        Mat::Floor,
        Mat::Plaster,
    ));
    v.push(Room::new(
        TW_X + HALF,
        W,
        TW_Z + HALF,
        D,
        0,
        Mat::Tile,
        Mat::Tile,
    ));
    v.push(Room::new(
        0.0,
        PX1 - HALF,
        0.0,
        PZ - HALF,
        1,
        Mat::Floor,
        Mat::Paper,
    ));
    v.push(Room::new(
        0.0,
        PX1 - HALF,
        PZ + HALF,
        D,
        1,
        Mat::Floor,
        Mat::Paper,
    ));
    let mut up = Room::new(PX1 + HALF, PX2 - HALF, 0.0, D, 1, Mat::Floor, Mat::Plaster);
    up.hole_floor = true;
    v.push(up);
    v.push(Room::new(PX2 + HALF, W, 0.0, D, 1, Mat::Floor, Mat::Paper));
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
        room_at(rs, px, pz, level).is_some()
    }

    fn face_mat(&self, rs: &[Room], a: f32, y: f32, hi_side: bool) -> Option<Mat> {
        let across = if hi_side {
            self.hi + 0.05
        } else {
            self.lo - 0.05
        };
        let (px, pz) = self.pt(a, across);
        if self.exterior {
            let inside = px > 0.0 && px < W && pz > 0.0 && pz < D;
            if !inside {
                return Some(if y < 0.5 { Mat::Concrete } else { Mat::Stucco });
            }
        }
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
    let door_like = o.door_like();
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
    let mut cuts: Vec<(f32, f32, f32, f32)> = Vec::new();
    for o in &w.ops {
        if o.open() {
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

fn wall_xs(w: &Wall) -> Vec<f32> {
    let mut xs = vec![w.a0, w.a1];
    for o in &w.ops {
        xs.push(o.a0);
        xs.push(o.a1);
    }
    for &k in &w.breaks {
        xs.push(k);
    }
    if w.axis == Axis::Z && (w.center() - PX2).abs() < 0.01 {
        xs.push(HOLE_Z0);
        xs.push(STAIR_END);
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
    let mut ys = vec![w.y0, w.y1, 0.0, 0.5, CEIL0, UPPER, TOP];
    for o in &w.ops {
        ys.push(o.y0);
        ys.push(o.y1);
    }
    let xs = wall_xs(w);
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

fn door_op(a0: f32, a1: f32, base: f32) -> Op {
    Op::new(a0, a1, base, base + DOOR_H, Kind::Door)
}

fn walls() -> Vec<Wall> {
    let win = |c: f32, hw: f32, base: f32| {
        Op::new(
            c - hw,
            c + hw,
            base + WIN_SILL,
            base + WIN_HEAD,
            Kind::Window,
        )
    };
    let small =
        |c: f32, hw: f32, base: f32| Op::new(c - hw, c + hw, base + 1.4, base + 2.1, Kind::Window);
    let door = |c: f32, base: f32| door_op(c - 0.475, c + 0.475, base);
    let ext = |c: f32| Op::new(c - 0.55, c + 0.55, 0.0, 2.2, Kind::ExtDoor);
    let kitchen_win = Op::new(13.925, 15.135, 1.075, 2.345, Kind::ModelWindow);
    let garage = Op::new(GARAGE_X0, GARAGE_X1, 0.0, 2.35, Kind::GarageDoor);
    let u = UPPER;
    let tz = TW_Z - HALF;
    let mut v = Vec::new();

    let mut front = Wall::new(Axis::X, -EXT, 0.0, 0.0, W, GROUND, TOP);
    front.ops = vec![
        garage,
        ext(8.0),
        win(11.4, 0.65, 0.0),
        kitchen_win,
        win(2.2, 0.65, u),
        win(8.0, 0.55, u),
        win(12.0, 0.65, u),
        win(14.0, 0.65, u),
    ];
    front.breaks = vec![PX1 - HALF, PX1 + HALF, PX2 - HALF, PX2 + HALF];
    front.exterior = true;
    v.push(front);

    let mut back = Wall::new(Axis::X, D, D + EXT, 0.0, W, GROUND, TOP);
    back.ops = vec![
        win(3.4, 0.65, 0.0),
        win(8.6, 0.65, 0.0),
        small(14.8, 0.35, 0.0),
        win(3.4, 0.65, u),
        win(8.0, 0.55, u),
        win(11.4, 0.65, u),
        win(14.4, 0.65, u),
    ];
    back.breaks = vec![
        CLOSET_IN,
        CLOSET_X + HALF,
        PX1 - HALF,
        PX1 + HALF,
        PX2 - HALF,
        PX2 + HALF,
        TW_X - HALF,
        TW_X + HALF,
    ];
    back.exterior = true;
    v.push(back);

    let mut west = Wall::new(Axis::Z, -EXT, 0.0, -EXT, D + EXT, GROUND, TOP);
    west.ops = vec![small(2.8, 0.5, 0.0), win(1.9, 0.7, u), win(9.0, 0.7, u)];
    west.breaks = vec![
        0.0,
        D,
        CLOSET_Z - HALF,
        CLOSET_Z + HALF,
        PZ - HALF,
        PZ + HALF,
    ];
    west.cap_lo = true;
    west.cap_hi = true;
    west.cap_mat = Mat::Stucco;
    west.exterior = true;
    v.push(west);

    let mut east = Wall::new(Axis::Z, W, W + EXT, -EXT, D + EXT, GROUND, TOP);
    east.ops = vec![
        win(4.9, 0.6, 0.0),
        win(2.05, 0.65, u),
        win(6.0, 0.65, u),
        win(10.0, 0.65, u),
    ];
    east.breaks = vec![0.0, D, tz, TW_Z + HALF];
    east.cap_lo = true;
    east.cap_hi = true;
    east.cap_mat = Mat::Stucco;
    east.exterior = true;
    v.push(east);

    let mut pg = Wall::new(Axis::Z, PX1 - HALF, PX1 + HALF, -0.03, D + 0.03, 0.0, CEIL0);
    pg.ops = vec![door(10.4, 0.0)];
    v.push(pg);

    let mut pg_up = Wall::new(Axis::Z, PX1 - HALF, PX1 + HALF, -0.03, D + 0.03, UPPER, TOP);
    pg_up.ops = vec![door(1.825, u), door(9.475, u)];
    pg_up.breaks = vec![PZ - HALF, PZ + HALF];
    v.push(pg_up);

    let mut stair_wall = Wall::new(
        Axis::Z,
        PX2 - HALF,
        PX2 + HALF,
        STAIR_Z0 - 0.1,
        STAIR_END,
        0.0,
        UPPER,
    );
    stair_wall.cap_lo = true;
    stair_wall.cap_hi = true;
    v.push(stair_wall);

    let mut px2_up = Wall::new(Axis::Z, PX2 - HALF, PX2 + HALF, -0.03, D + 0.03, UPPER, TOP);
    px2_up.ops = vec![door(9.475, u)];
    v.push(px2_up);

    let pz_up = Wall::new(
        Axis::X,
        PZ - HALF,
        PZ + HALF,
        -0.03,
        PX1 - HALF + 0.03,
        UPPER,
        TOP,
    );
    v.push(pz_up);

    let mut closet_front = Wall::new(
        Axis::X,
        CLOSET_Z - HALF,
        CLOSET_Z + HALF,
        -0.03,
        CLOSET_X + HALF,
        0.0,
        CEIL0,
    );
    let mut cd = door_op(0.235, 1.185, 0.0);
    cd.hi = 0.0;
    closet_front.ops = vec![cd];
    closet_front.breaks = vec![CLOSET_IN];
    closet_front.cap_hi = true;
    v.push(closet_front);

    let closet_side = Wall::new(
        Axis::Z,
        CLOSET_IN,
        CLOSET_X + HALF,
        CLOSET_Z + HALF,
        D + 0.03,
        0.0,
        CEIL0,
    );
    v.push(closet_side);

    let mut toilet_south = Wall::new(Axis::X, tz, TW_Z + HALF, TW_X - HALF, W + 0.03, 0.0, CEIL0);
    let mut td = door_op(14.325, 15.275, 0.0);
    td.hinge_hi = true;
    toilet_south.ops = vec![td];
    toilet_south.breaks = vec![TW_X + HALF];
    toilet_south.cap_lo = true;
    v.push(toilet_south);

    let toilet_west = Wall::new(
        Axis::Z,
        TW_X - HALF,
        TW_X + HALF,
        TW_Z + HALF,
        D + 0.03,
        0.0,
        CEIL0,
    );
    v.push(toilet_west);
    v
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
    let hole_here = w.axis == Axis::Z && (w.center() - PX2).abs() < 0.01 && !hi_side;
    if hole_here && ((crown && level == 0) || (!crown && level == 1)) {
        cuts.push((HOLE_Z0 - 0.001, STAIR_END + 0.001));
    }
    if hole_here && !crown && level == 0 {
        cuts.push((STAIR_Z0 - 0.12, STAIR_END + 0.001));
    }
    cuts
}

fn trims(b: &mut Builder, rs: &[Room], ws: &[Wall]) {
    for w in ws {
        let mut xs = wall_xs(w);
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
                for crown in [false, true] {
                    if crown && ceil_y > w.y1 + 1e-3 {
                        continue;
                    }
                    let (y0, y1, proud) = if crown {
                        (ceil_y - CROWN_H, ceil_y, CROWN_P)
                    } else {
                        (floor_y, floor_y + SKIRT_H, SKIRT_P)
                    };
                    let cuts = trim_cuts(w, level, probe, hi_side, crown);
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

fn basement(b: &mut Builder) {
    let bx1 = PX1 - HALF;
    let (z0, z1) = (B_ROOM_Z0, D);
    let sx = CLOSET_X + HALF;
    b.fy(Mat::Concrete, B_FLOOR, 0.0, bx1, z0, z1, 1.0);
    b.grid_y(
        Mat::Concrete,
        B_CEIL,
        &[0.0, CLOSET_IN, bx1],
        &[z0, B_Z0, B_END, z1],
        Some((0, 1)),
        -1.0,
    );
    b.fx(Mat::Concrete, 0.0, B_FLOOR, B_CEIL, z0, z1, 1.0);
    b.fx(Mat::Concrete, bx1, B_FLOOR, B_CEIL, z0, z1, -1.0);
    b.fz(Mat::Concrete, z0, 0.0, bx1, B_FLOOR, B_CEIL, 1.0);
    b.fz(Mat::Concrete, z1, 0.0, bx1, B_FLOOR, B_CEIL, -1.0);

    for i in 1..B_STEPS {
        let za = B_Z0 + (i as f32 - 1.0) * B_TREAD;
        let zb = za + B_TREAD;
        let top = -(i as f32) * B_RISE;
        b.fy(Mat::Wood, top, 0.0, CLOSET_IN, za, zb, 1.0);
        b.fz(Mat::Wood, za, 0.0, CLOSET_IN, top, top + B_RISE, 1.0);
        b.fx(Mat::Concrete, CLOSET_IN, top, 0.0, za, zb, -1.0);
    }
    let last = -((B_STEPS - 1) as f32) * B_RISE;
    b.fz(Mat::Wood, B_END, 0.0, CLOSET_IN, B_FLOOR, last, 1.0);
    b.fz(Mat::Concrete, B_Z0, 0.0, sx, B_FLOOR, B_CEIL, -1.0);
    b.fx(Mat::Concrete, sx, B_FLOOR, B_CEIL, B_Z0, B_END, 1.0);
    b.fz(Mat::Concrete, B_END, CLOSET_IN, sx, B_FLOOR, B_CEIL, 1.0);
    b.fz(Mat::Concrete, B_END, 0.0, CLOSET_IN, B_CEIL, 0.0, -1.0);
    b.fx(Mat::Concrete, 0.0, B_CEIL, 0.0, B_Z0, B_END, 1.0);
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
    let mut world = World::new(B_FLOOR - 1.0);

    for w in &ws {
        build_wall(&mut b, &rs, &ws, w);
        wall_solids(w, &mut world.solids);
        for o in &w.ops {
            let fy = if o.y0 > 1.0 && o.kind == Kind::Door {
                UPPER
            } else {
                0.0
            };
            let sill = |b: &mut Builder, m: Mat| match w.axis {
                Axis::X => b.fy(m, fy, o.a0, o.a1, w.lo, w.hi, 1.0),
                Axis::Z => b.fy(m, fy, w.lo, w.hi, o.a0, o.a1, 1.0),
            };
            match o.kind {
                Kind::Door => {
                    casing(&mut b, w, o, false);
                    casing(&mut b, w, o, true);
                    if (w.center() - CLOSET_Z).abs() > 0.01 {
                        sill(&mut b, Mat::Wood);
                    }
                }
                Kind::ExtDoor => {
                    casing(&mut b, w, o, false);
                    casing(&mut b, w, o, true);
                    exterior_door(&mut b, w, o);
                    sill(&mut b, Mat::Concrete);
                }
                Kind::GarageDoor => {
                    casing(&mut b, w, o, false);
                    casing(&mut b, w, o, true);
                    garage_door(&mut b, w, o);
                    sill(&mut b, Mat::Concrete);
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
        let stair_hole = r.hole_floor || r.hole_ceil;
        let (xs, zs): (Vec<f32>, Vec<f32>) = if stair_hole {
            (
                vec![r.x0, STAIR_X0, r.x1],
                vec![r.z0, HOLE_Z0, STAIR_END, r.z1],
            )
        } else if r.shaft {
            (vec![r.x0, CLOSET_IN, r.x1], vec![r.z0, B_Z0, B_END, r.z1])
        } else {
            (vec![r.x0, r.x1], vec![r.z0, r.z1])
        };
        if stair_hole {
            b.grid_y(
                r.floor,
                fy,
                &xs,
                &zs,
                if r.hole_floor { Some((1, 1)) } else { None },
                1.0,
            );
            b.grid_y(
                r.ceil,
                cy,
                &xs,
                &zs,
                if r.hole_ceil { Some((1, 1)) } else { None },
                -1.0,
            );
        } else if r.shaft {
            b.grid_y(r.floor, fy, &xs, &zs, Some((0, 1)), 1.0);
            b.fy(r.ceil, cy, r.x0, r.x1, r.z0, r.z1, -1.0);
        } else {
            b.fy(r.floor, fy, r.x0, r.x1, r.z0, r.z1, 1.0);
            b.fy(r.ceil, cy, r.x0, r.x1, r.z0, r.z1, -1.0);
        }
    }
    trims(&mut b, &rs, &ws);

    stairs(&mut b);
    hole_rails(&mut b);
    basement(&mut b);
    roof(&mut b);

    let bx1 = PX1 - HALF;
    let deep = B_FLOOR - 1.0;
    let sx = CLOSET_X + HALF;
    let sol = |w: &mut World, min: [f32; 3], max: [f32; 3]| w.solids.push(Solid::new(min, max));
    sol(&mut world, [-EXT, deep, -EXT], [W + EXT, 0.0, B_ROOM_Z0]);
    sol(&mut world, [bx1, deep, B_ROOM_Z0], [W + EXT, 0.0, D + EXT]);
    sol(&mut world, [-EXT, deep, B_ROOM_Z0], [0.0, 0.0, D + EXT]);
    sol(&mut world, [0.0, deep, D], [bx1, 0.0, D + EXT]);
    sol(&mut world, [0.0, deep, B_ROOM_Z0], [bx1, B_FLOOR, D]);
    sol(&mut world, [0.0, B_CEIL, B_ROOM_Z0], [bx1, 0.0, B_Z0]);
    sol(&mut world, [0.0, B_CEIL, B_END], [bx1, 0.0, D]);
    sol(&mut world, [CLOSET_IN, B_CEIL, B_Z0], [bx1, 0.0, B_END]);
    sol(&mut world, [CLOSET_IN, B_FLOOR, B_Z0], [sx, 0.0, B_END]);
    let slope = B_RISE / B_TREAD;
    let rz0 = B_Z0 - 0.06;
    world.ramps.push(Ramp {
        x0: 0.0,
        x1: CLOSET_IN,
        z0: rz0,
        z1: B_END,
        y0: 0.0,
        y1: -slope * (B_END - rz0),
    });

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

    let doors = door_specs(&ws);
    let (leaf_wood, leaf_metal) = door_leaf();
    let lights = lights();
    let slots = slots();
    let log = b.take_log();
    let house = House {
        meshes: b.meshes,
        world,
        lights,
        slots,
        doors,
        leaf_wood,
        leaf_metal,
        spawn: v3(7.55, 0.0, 1.3),
        spawn_yaw: std::f32::consts::PI,
    };
    (house, log)
}

fn lights() -> Vec<LightSpec> {
    let warm = [1.0, 0.72, 0.42];
    let sick = [0.78, 0.92, 0.72];
    let amber = [1.0, 0.55, 0.25];
    let l =
        |name: &'static str, x: f32, y: f32, z: f32, color: [f32; 3], lumens: f32, range: f32| {
            LightSpec {
                pos: v3(x, y, z),
                color,
                lumens,
                range,
                name,
            }
        };
    vec![
        l("hall_front", 8.0, 2.55, 1.8, warm, 520.0, 9.0),
        l("hall_back", 9.0, 2.6, 10.6, warm, 400.0, 8.0),
        l("living_lamp", 10.2, 2.0, 7.0, amber, 520.0, 8.0),
        l("kitchen_ceiling", 13.2, 2.7, 2.7, sick, 900.0, 9.0),
        l("dining_pendant", 13.0, 2.2, 7.2, amber, 360.0, 6.5),
        l("toilet_bulb", 14.8, 2.4, 10.7, sick, 480.0, 6.0),
        l("garage_front", 3.2, 2.7, 3.0, sick, 520.0, 9.0),
        l("garage_back", 3.6, 2.7, 9.0, warm, 380.0, 8.0),
        l("closet_bulb", 0.7, 2.6, 9.0, warm, 220.0, 4.0),
        l("basement_a", 3.4, -0.7, 4.0, sick, 620.0, 9.0),
        l("basement_b", 3.4, -0.7, 9.0, warm, 520.0, 9.0),
        l("landing_upper", 8.0, 5.7, 10.5, warm, 330.0, 8.0),
        l("hall_upper_front", 8.0, 5.7, 2.0, warm, 300.0, 7.5),
        l("bedroom_a_lamp", 3.9, 4.15, 5.7, amber, 420.0, 7.0),
        l("room_b_bulb", 3.2, 5.7, 9.0, warm, 240.0, 7.0),
        l("room_c_bulb", 12.8, 5.7, 6.0, warm, 300.0, 9.0),
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
    let f = B_FLOOR;
    vec![
        s(
            "kitchen_table",
            "table 1.8 x 1.0 m",
            [11.6, 0.0, 2.4],
            [13.4, 0.78, 3.4],
        ),
        s(
            "kitchen_chair_1",
            "chair",
            [11.7, 0.0, 1.7],
            [12.2, 0.95, 2.2],
        ),
        s(
            "kitchen_chair_2",
            "chair",
            [12.8, 0.0, 1.7],
            [13.3, 0.95, 2.2],
        ),
        s(
            "kitchen_chair_3",
            "chair",
            [11.7, 0.0, 3.6],
            [12.2, 0.95, 4.1],
        ),
        s(
            "kitchen_chair_4",
            "chair",
            [12.8, 0.0, 3.6],
            [13.3, 0.95, 4.1],
        ),
        s(
            "living_coffee_table",
            "coffee table 1.2 x 0.6 m",
            [11.0, 0.0, 6.4],
            [11.9, 0.45, 7.8],
        ),
        s(
            "living_tv_console",
            "TV and console 1.4 x 0.45 m",
            [15.5, 0.0, 6.4],
            [15.98, 1.1, 7.8],
        ),
        s(
            "living_rug",
            "rug 2 x 2.4 m",
            [10.8, 0.0, 5.9],
            [12.6, 0.02, 8.3],
        ),
        s(
            "hall_coat_rack",
            "coat rack or shoe cabinet",
            [6.6, 0.0, 3.0],
            [7.0, 1.8, 4.0],
        ),
        s(
            "corridor_bookshelf",
            "bookshelf 1.8 m wide",
            [10.0, 0.0, 11.5],
            [11.8, 2.1, 11.98],
        ),
        s("toilet_wc", "toilet", [14.4, 0.0, 11.1], [15.2, 0.8, 11.98]),
        s(
            "toilet_sink",
            "sink with mirror",
            [13.6, 0.0, 10.1],
            [14.0, 0.9, 10.9],
        ),
        s(
            "garage_workbench",
            "workbench",
            [0.1, 0.0, 0.1],
            [0.75, 0.9, 2.2],
        ),
        s(
            "garage_shelves",
            "metal shelving",
            [5.9, 0.0, 3.0],
            [6.38, 1.9, 5.0],
        ),
        s(
            "basement_shelves",
            "metal shelving",
            [0.1, f, 2.2],
            [0.6, f + 1.9, 5.8],
        ),
        s(
            "basement_crates",
            "crates or boxes",
            [3.0, f, 2.1],
            [4.4, f + 0.8, 3.0],
        ),
        s(
            "basement_boiler",
            "boiler or water tank",
            [4.8, f, 10.8],
            [6.3, f + 1.6, 11.9],
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
            "upper_hall_bookcase",
            "bookcase or console",
            [6.6, u, 10.2],
            [7.1, u + 1.8, 11.6],
        ),
    ]
}
