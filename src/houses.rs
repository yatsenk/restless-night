use bevy::prelude::*;

use crate::mesh::MeshBuilder;
use crate::util::{GameRng, Rgba, lin, scale_col};

pub const FLOOR_H: f32 = 3.0;
pub const BASE_H: f32 = 0.5;

const WALL_COLORS: [[f32; 3]; 8] = [
    [0.60, 0.55, 0.46],
    [0.40, 0.45, 0.42],
    [0.52, 0.30, 0.24],
    [0.64, 0.62, 0.58],
    [0.30, 0.33, 0.40],
    [0.47, 0.42, 0.30],
    [0.55, 0.50, 0.38],
    [0.28, 0.36, 0.33],
];

const ROOF_COLORS: [[f32; 3]; 4] = [
    [0.22, 0.13, 0.11],
    [0.17, 0.18, 0.20],
    [0.27, 0.16, 0.09],
    [0.12, 0.14, 0.13],
];

const TRIM_COLORS: [[f32; 3]; 3] = [[0.80, 0.78, 0.72], [0.55, 0.52, 0.46], [0.18, 0.17, 0.16]];

const DOOR_COLORS: [[f32; 3]; 5] = [
    [0.30, 0.07, 0.05],
    [0.07, 0.17, 0.12],
    [0.05, 0.05, 0.06],
    [0.28, 0.17, 0.08],
    [0.12, 0.14, 0.26],
];

#[derive(Clone, Copy)]
pub struct Dims {
    pub w: f32,
    pub d: f32,
    pub h: f32,
    pub rh: f32,
    pub bay_z: Option<f32>,
    pub wall: Rgba,
    pub roof: Rgba,
}

pub struct HouseMesh {
    pub body: Mesh,
    pub glass: Mesh,
}

struct Style {
    floors: u32,
    dims: Dims,
    wall: Rgba,
    roof: Rgba,
    trim: Rgba,
    door: Rgba,
    shutter: Option<Rgba>,
    bay: Option<f32>,
    chimney: bool,
    boxes: bool,
    stone: Rgba,
    brick: Rgba,
    wood: Rgba,
}

fn style_from(seed: u64) -> Style {
    let mut rng = GameRng::seeded(seed);
    let floors = if rng.chance(0.5) { 1 } else { 2 };
    let w = rng.range(12.0, 15.0);
    let d = rng.range(9.5, 11.5);
    let rh = rng.range(2.8, 3.8);
    let wc = WALL_COLORS[rng.pick(WALL_COLORS.len())];
    let rc = ROOF_COLORS[rng.pick(ROOF_COLORS.len())];
    let tc = TRIM_COLORS[rng.pick(TRIM_COLORS.len())];
    let dc = DOOR_COLORS[rng.pick(DOOR_COLORS.len())];
    let shutter = if rng.chance(0.55) {
        let s = DOOR_COLORS[rng.pick(DOOR_COLORS.len())];
        Some(lin(s[0], s[1], s[2]))
    } else {
        None
    };
    let bay = if rng.chance(0.4) {
        Some(if rng.chance(0.5) { 1.0 } else { -1.0 })
    } else {
        None
    };
    let chimney = rng.chance(0.8);
    let boxes = rng.chance(0.45);
    Style {
        floors,
        dims: Dims {
            w,
            d,
            h: floors as f32 * FLOOR_H,
            rh,
            bay_z: bay.map(|side| w * 0.5 * 0.6 * side),
            wall: lin(wc[0], wc[1], wc[2]),
            roof: lin(rc[0], rc[1], rc[2]),
        },
        wall: lin(wc[0], wc[1], wc[2]),
        roof: lin(rc[0], rc[1], rc[2]),
        trim: lin(tc[0], tc[1], tc[2]),
        door: lin(dc[0], dc[1], dc[2]),
        shutter,
        bay,
        chimney,
        boxes,
        stone: lin(0.24, 0.23, 0.22),
        brick: lin(0.38, 0.15, 0.11),
        wood: lin(0.18, 0.11, 0.06),
    }
}

#[allow(clippy::too_many_arguments)]
fn window(
    b: &mut MeshBuilder,
    g: &mut MeshBuilder,
    c: Vec3,
    n: Vec3,
    t: Vec3,
    ww: f32,
    wh: f32,
    detail: bool,
    trim: Rgba,
    shutter: Option<Rgba>,
    boxed: bool,
) {
    let white = lin(1.0, 1.0, 1.0);
    if !detail {
        g.obox(
            c + n * 0.02,
            t,
            n,
            Vec3::new(ww * 0.5, wh * 0.5, 0.02),
            white,
        );
        return;
    }
    g.obox(
        c + n * 0.03,
        t,
        n,
        Vec3::new(ww * 0.5, wh * 0.5, 0.02),
        white,
    );
    let hw = ww * 0.5;
    let hh = wh * 0.5;
    b.obox(
        c + Vec3::Y * (hh + 0.07) + n * 0.06,
        t,
        n,
        Vec3::new(hw + 0.14, 0.07, 0.07),
        trim,
    );
    b.obox(
        c - Vec3::Y * (hh + 0.07) + n * 0.09,
        t,
        n,
        Vec3::new(hw + 0.24, 0.07, 0.12),
        trim,
    );
    b.obox(
        c + Vec3::Y * (hh + 0.22) + n * 0.07,
        t,
        n,
        Vec3::new(hw + 0.22, 0.07, 0.09),
        scale_col(trim, 0.85),
    );
    for s in [-1.0_f32, 1.0] {
        b.obox(
            c + t * (s * (hw + 0.07)) + n * 0.06,
            t,
            n,
            Vec3::new(0.07, hh, 0.07),
            trim,
        );
    }
    b.obox(c + n * 0.05, t, n, Vec3::new(0.03, hh, 0.045), trim);
    for k in [-1.0_f32, 1.0] {
        b.obox(
            c + Vec3::Y * (wh * 0.167 * k) + n * 0.05,
            t,
            n,
            Vec3::new(hw, 0.025, 0.045),
            trim,
        );
    }
    b.obox(
        c - Vec3::Y * (hh - 0.04) + n * 0.04,
        t,
        n,
        Vec3::new(hw, 0.04, 0.055),
        scale_col(trim, 0.8),
    );
    if boxed {
        let wood = lin(0.20, 0.12, 0.07);
        let bc = c - Vec3::Y * (hh + 0.3) + n * 0.22;
        b.obox(bc, t, n, Vec3::new(hw + 0.1, 0.13, 0.15), wood);
        let dry = lin(0.38, 0.26, 0.09);
        for i in 0..7 {
            let u = -hw + 0.15 + i as f32 * (2.0 * hw - 0.3) / 6.0;
            let hgt = 0.25 + 0.12 * ((i * 5 % 4) as f32);
            b.obox(
                bc + t * u + Vec3::Y * (0.13 + hgt * 0.5) + n * ((i % 3) as f32 * 0.03 - 0.03),
                t,
                n,
                Vec3::new(0.02, hgt * 0.5, 0.02),
                dry,
            );
        }
    }
    if let Some(sc) = shutter {
        let sw = ww * 0.28;
        for s in [-1.0_f32, 1.0] {
            let sc_pos = c + t * (s * (hw + 0.2 + sw * 0.5)) + n * 0.05;
            b.obox(sc_pos, t, n, Vec3::new(sw * 0.5, hh + 0.08, 0.04), sc);
            for k in 0..5 {
                let y = -hh + 0.2 + k as f32 * (wh - 0.1) / 4.5;
                b.obox(
                    sc_pos + Vec3::Y * y + n * 0.03,
                    t,
                    n,
                    Vec3::new(sw * 0.44, 0.03, 0.025),
                    scale_col(sc, 0.7),
                );
            }
        }
    }
}

fn door(b: &mut MeshBuilder, g: &mut MeshBuilder, x: f32, s: &Style, detail: bool) {
    let n = -Vec3::X;
    let t = Vec3::Z;
    let c = Vec3::new(x, BASE_H + 1.15, 0.0);
    if !detail {
        b.obox(c + n * 0.04, t, n, Vec3::new(0.65, 1.15, 0.05), s.door);
        return;
    }
    let gold = lin(0.75, 0.6, 0.2);
    b.obox(c + n * 0.03, t, n, Vec3::new(0.6, 1.1, 0.05), s.door);
    b.obox(
        c + Vec3::Y * 1.2 + n * 0.06,
        t,
        n,
        Vec3::new(0.82, 0.1, 0.09),
        s.trim,
    );
    for sg in [-1.0_f32, 1.0] {
        b.obox(
            c + t * (sg * 0.72) + n * 0.05,
            t,
            n,
            Vec3::new(0.11, 1.15, 0.08),
            s.trim,
        );
    }
    for (dy, h) in [(-0.5_f32, 0.28_f32), (0.4, 0.28)] {
        for sg in [-1.0_f32, 1.0] {
            b.obox(
                c + t * (sg * 0.3) + Vec3::Y * dy + n * 0.075,
                t,
                n,
                Vec3::new(0.2, h, 0.02),
                scale_col(s.door, 0.7),
            );
        }
    }
    g.obox(
        c + Vec3::Y * 0.55 + n * 0.085,
        t,
        n,
        Vec3::new(0.5, 0.2, 0.012),
        lin(1.0, 1.0, 1.0),
    );
    b.obox(
        c + t * 0.42 + Vec3::Y * -0.1 + n * 0.12,
        t,
        n,
        Vec3::new(0.05, 0.05, 0.05),
        gold,
    );
}

fn porch(b: &mut MeshBuilder, hx: f32, s: &Style, detail: bool) {
    let stone = s.stone;
    b.cuboid(
        Vec3::new(-hx - 2.6, 0.0, -2.4),
        Vec3::new(-hx + 0.05, 0.55, 2.4),
        stone,
    );
    for k in 0..3 {
        let top = 0.55 - 0.18 * (k + 1) as f32;
        let zk = 1.2 + 0.06 * k as f32;
        b.cuboid(
            Vec3::new(-hx - 2.6 - 0.45 * (k + 1) as f32, 0.0, -zk),
            Vec3::new(-hx - 2.55, top, zk),
            scale_col(stone, 1.0 + 0.04 * k as f32),
        );
    }
    b.cuboid(
        Vec3::new(-hx - 2.7, 2.75, -2.5),
        Vec3::new(-hx, 2.92, 2.5),
        s.roof,
    );
    if !detail {
        for sg in [-1.0_f32, 1.0] {
            b.cuboid(
                Vec3::new(-hx - 2.55, 0.0, sg * 2.1 - 0.1),
                Vec3::new(-hx - 2.35, 2.75, sg * 2.1 + 0.1),
                s.trim,
            );
        }
        return;
    }
    b.cuboid(
        Vec3::new(-hx - 2.75, 2.6, -2.55),
        Vec3::new(-hx - 2.65, 2.78, 2.55),
        s.trim,
    );
    for sg in [-1.0_f32, 1.0] {
        let z = sg * 2.1;
        b.cuboid(
            Vec3::new(-hx - 2.5, 0.55, z - 0.14),
            Vec3::new(-hx - 2.22, 2.75, z + 0.14),
            s.trim,
        );
        b.cuboid(
            Vec3::new(-hx - 2.56, 0.55, z - 0.2),
            Vec3::new(-hx - 2.16, 0.75, z + 0.2),
            scale_col(s.trim, 0.9),
        );
        b.cuboid(
            Vec3::new(-hx - 2.56, 2.55, z - 0.2),
            Vec3::new(-hx - 2.16, 2.75, z + 0.2),
            scale_col(s.trim, 0.9),
        );
        b.cuboid(
            Vec3::new(-hx - 2.5, 1.45, z - 0.04),
            Vec3::new(-hx, 1.55, z + 0.04),
            s.trim,
        );
        b.cuboid(
            Vec3::new(-hx - 2.5, 0.55, z - 0.04),
            Vec3::new(-hx, 0.62, z + 0.04),
            s.trim,
        );
        let mut x = -hx - 2.35;
        while x < -hx - 0.1 {
            b.cuboid(
                Vec3::new(x - 0.03, 0.62, z - 0.03),
                Vec3::new(x + 0.03, 1.45, z + 0.03),
                s.trim,
            );
            x += 0.28;
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn roof_side(
    b: &mut MeshBuilder,
    jr: &mut GameRng,
    ea: Vec3,
    eb: Vec3,
    ra: Vec3,
    rb: Vec3,
    out: Vec3,
    col: Rgba,
    rows: u32,
    cols: u32,
) {
    let p = |r: u32, c: u32| -> Vec3 {
        let tc = c as f32 / cols as f32;
        let tr = r as f32 / rows as f32;
        ea.lerp(eb, tc).lerp(ra.lerp(rb, tc), tr)
    };
    for r in 0..rows {
        let rowk = 0.85 + 0.15 * (r as f32 / rows as f32);
        for c in 0..cols {
            let k = (0.82 + 0.3 * jr.f32()) * rowk;
            b.quad_out(
                p(r, c),
                p(r, c + 1),
                p(r + 1, c + 1),
                p(r + 1, c),
                out,
                scale_col(col, k),
            );
        }
    }
}

pub fn build_house(seed: u64, detail: bool) -> (HouseMesh, Dims) {
    let s = style_from(seed);
    let Dims { w, d, h, rh, .. } = s.dims;
    let mut jr = GameRng::seeded(seed ^ 0xA5A5_5A5A);
    let hx = d * 0.5;
    let hz = w * 0.5;
    let ox = 0.7;
    let oz = 0.55;
    let (xa, xb) = (-(hx + ox), hx + ox);
    let (za, zb) = (-(hz + oz), hz + oz);

    let mut b = MeshBuilder::default();
    let mut g = MeshBuilder::default();

    b.cuboid(
        Vec3::new(-hx - 0.08, 0.0, -hz - 0.08),
        Vec3::new(hx + 0.08, BASE_H + 0.05, hz + 0.08),
        s.stone,
    );
    b.cuboid(Vec3::new(-hx, BASE_H, -hz), Vec3::new(hx, h, hz), s.wall);
    b.cuboid(Vec3::new(xa, h - 0.15, za), Vec3::new(xb, h, zb), s.wood);

    let rows = if detail { 7 } else { 1 };
    let cols = if detail { (w as u32) + 2 } else { 1 };
    roof_side(
        &mut b,
        &mut jr,
        Vec3::new(xa, h, za),
        Vec3::new(xa, h, zb),
        Vec3::new(0.0, h + rh, za),
        Vec3::new(0.0, h + rh, zb),
        Vec3::new(-rh, hx + ox, 0.0),
        s.roof,
        rows,
        cols,
    );
    roof_side(
        &mut b,
        &mut jr,
        Vec3::new(xb, h, za),
        Vec3::new(xb, h, zb),
        Vec3::new(0.0, h + rh, za),
        Vec3::new(0.0, h + rh, zb),
        Vec3::new(rh, hx + ox, 0.0),
        s.roof,
        rows,
        cols,
    );
    for sg in [-1.0_f32, 1.0] {
        let z = if sg > 0.0 { zb } else { za };
        b.tri_out(
            Vec3::new(xa, h, z),
            Vec3::new(xb, h, z),
            Vec3::new(0.0, h + rh, z),
            Vec3::new(0.0, 0.0, sg),
            s.wall,
        );
    }

    let wy = |f: u32| BASE_H + 1.45 + f as f32 * FLOOR_H;
    let (ww, wh) = (1.5, 1.6);

    if detail {
        let strip = scale_col(s.wall, 0.8);
        let mut y = BASE_H + 0.3;
        while y < h - 0.1 {
            b.obox(
                Vec3::new(-hx, y, 0.0),
                Vec3::Z,
                -Vec3::X,
                Vec3::new(hz, 0.03, 0.014),
                strip,
            );
            b.obox(
                Vec3::new(hx, y, 0.0),
                Vec3::Z,
                Vec3::X,
                Vec3::new(hz, 0.03, 0.014),
                strip,
            );
            b.obox(
                Vec3::new(0.0, y, hz),
                Vec3::X,
                Vec3::Z,
                Vec3::new(hx, 0.03, 0.014),
                strip,
            );
            b.obox(
                Vec3::new(0.0, y, -hz),
                Vec3::X,
                -Vec3::Z,
                Vec3::new(hx, 0.03, 0.014),
                strip,
            );
            y += 0.28;
        }
        for sx in [-1.0_f32, 1.0] {
            for sz in [-1.0_f32, 1.0] {
                b.cuboid(
                    Vec3::new(sx * hx - 0.1, BASE_H, sz * hz - 0.1),
                    Vec3::new(sx * hx + 0.1, h - 0.15, sz * hz + 0.1),
                    s.trim,
                );
            }
        }
        b.cuboid(
            Vec3::new(-hx - 0.04, h - 0.4, -hz - 0.04),
            Vec3::new(hx + 0.04, h - 0.15, hz + 0.04),
            s.trim,
        );
        for ex in [xa, xb] {
            let sg = if ex < 0.0 { -1.0 } else { 1.0 };
            b.obox(
                Vec3::new(ex - sg * 0.06, h - 0.02, 0.0),
                Vec3::Z,
                Vec3::X,
                Vec3::new(zb, 0.07, 0.08),
                scale_col(s.trim, 0.8),
            );
            let dx0 = sg * hx;
            let dx1 = sg * (hx + 0.1);
            b.cuboid(
                Vec3::new(dx0.min(dx1), BASE_H, hz - 0.5),
                Vec3::new(dx0.max(dx1), h - 0.1, hz - 0.4),
                scale_col(s.trim, 0.7),
            );
        }
        for z in [za, zb] {
            for sx in [-1.0_f32, 1.0] {
                let ex = sx * (hx + ox);
                b.tube(
                    Vec3::new(ex, h, z),
                    Vec3::new(0.0, h + rh, z),
                    0.08,
                    0.08,
                    4,
                    s.trim,
                    s.trim,
                );
            }
        }
        b.obox(
            Vec3::new(0.0, h + rh + 0.04, 0.0),
            Vec3::Z,
            Vec3::X,
            Vec3::new(zb + 0.05, 0.1, 0.18),
            scale_col(s.roof, 0.7),
        );
    }

    for f in 0..s.floors {
        for z in [-hz * 0.6, hz * 0.6] {
            if f == 0 {
                if let Some(side) = s.bay {
                    if z * side > 0.0 {
                        continue;
                    }
                }
            }
            window(
                &mut b,
                &mut g,
                Vec3::new(-hx, wy(f), z),
                -Vec3::X,
                Vec3::Z,
                ww,
                wh,
                detail,
                s.trim,
                s.shutter,
                s.boxes,
            );
        }
        for z in [-hz * 0.5, hz * 0.5] {
            window(
                &mut b,
                &mut g,
                Vec3::new(hx, wy(f), z),
                Vec3::X,
                Vec3::Z,
                ww,
                wh,
                detail,
                s.trim,
                s.shutter,
                false,
            );
        }
        for sg in [-1.0_f32, 1.0] {
            window(
                &mut b,
                &mut g,
                Vec3::new(0.0, wy(f), sg * hz),
                Vec3::new(0.0, 0.0, sg),
                Vec3::X,
                ww,
                wh,
                detail,
                s.trim,
                s.shutter,
                false,
            );
        }
    }

    for sg in [-1.0_f32, 1.0] {
        window(
            &mut b,
            &mut g,
            Vec3::new(0.0, h + rh * 0.38, sg * zb),
            Vec3::new(0.0, 0.0, sg),
            Vec3::X,
            0.9,
            1.0,
            detail,
            s.trim,
            None,
            false,
        );
    }

    if let Some(side) = s.bay {
        let zc = hz * 0.6 * side;
        let depth = 1.0;
        let half = 1.25;
        b.cuboid(
            Vec3::new(-hx - depth, 0.0, zc - half),
            Vec3::new(-hx, BASE_H + 0.1, zc + half),
            s.stone,
        );
        b.cuboid(
            Vec3::new(-hx - depth, BASE_H, zc - half),
            Vec3::new(-hx, FLOOR_H, zc + half),
            s.wall,
        );
        b.cuboid(
            Vec3::new(-hx - depth - 0.15, FLOOR_H, zc - half - 0.15),
            Vec3::new(-hx + 0.05, FLOOR_H + 0.16, zc + half + 0.15),
            s.roof,
        );
        window(
            &mut b,
            &mut g,
            Vec3::new(-hx - depth, wy(0), zc),
            -Vec3::X,
            Vec3::Z,
            1.6,
            wh,
            detail,
            s.trim,
            None,
            false,
        );
        for sg in [-1.0_f32, 1.0] {
            window(
                &mut b,
                &mut g,
                Vec3::new(-hx - depth * 0.5, wy(0), zc + sg * half),
                Vec3::new(0.0, 0.0, sg),
                Vec3::X,
                0.6,
                wh,
                detail,
                s.trim,
                None,
                false,
            );
        }
    }

    porch(&mut b, hx, &s, detail);
    door(&mut b, &mut g, -hx, &s, detail);

    if s.chimney {
        let cx = hx * 0.45;
        let cz = hz * 0.5;
        let top = h + rh + 0.9;
        if detail {
            let mut y = h;
            let mut i = 0;
            while y < top {
                let k = if i % 2 == 0 { 1.0 } else { 0.88 };
                b.cuboid(
                    Vec3::new(cx - 0.5, y, cz - 0.5),
                    Vec3::new(cx + 0.5, y + 0.22, cz + 0.5),
                    scale_col(s.brick, k * (0.9 + 0.2 * jr.f32())),
                );
                y += 0.22;
                i += 1;
            }
            b.cuboid(
                Vec3::new(cx - 0.62, top, cz - 0.62),
                Vec3::new(cx + 0.62, top + 0.15, cz + 0.62),
                s.stone,
            );
            for dz in [-0.2_f32, 0.2] {
                b.tube(
                    Vec3::new(cx, top + 0.15, cz + dz),
                    Vec3::new(cx, top + 0.6, cz + dz),
                    0.14,
                    0.12,
                    8,
                    lin(0.3, 0.12, 0.08),
                    lin(0.3, 0.12, 0.08),
                );
            }
        } else {
            b.cuboid(
                Vec3::new(cx - 0.5, h, cz - 0.5),
                Vec3::new(cx + 0.5, top, cz + 0.5),
                s.brick,
            );
        }
    }

    (
        HouseMesh {
            body: b.build(),
            glass: g.build(),
        },
        s.dims,
    )
}
