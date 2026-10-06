use std::f32::consts::TAU;

use bevy::prelude::*;

use crate::houses::Dims;
use crate::mesh::MeshBuilder;
use crate::util::*;
use crate::{FRONT, LOT};

const LEAF_COLORS: [[f32; 3]; 6] = [
    [0.55, 0.20, 0.04],
    [0.48, 0.08, 0.05],
    [0.62, 0.42, 0.07],
    [0.36, 0.17, 0.06],
    [0.28, 0.12, 0.04],
    [0.70, 0.30, 0.05],
];

pub fn leaf_card(b: &mut MeshBuilder, c: Vec3, u: Vec3, v: Vec3, col: Rgba) {
    let n = u.cross(v).normalize_or_zero();
    for s in [1.0_f32, -1.0] {
        b.quad_out(c - u, c + v, c + u, c - v, n * s, col);
    }
}

pub fn ground_leaf(b: &mut MeshBuilder, rng: &mut GameRng, p: Vec3) {
    let pc = LEAF_COLORS[rng.pick(LEAF_COLORS.len())];
    let k = rng.range(0.75, 1.2);
    let col = scale_col(lin(pc[0], pc[1], pc[2]), k);
    let col2 = scale_col(col, 0.8);
    let a = rng.range(0.0, TAU);
    let len = rng.range(0.07, 0.18);
    let dir = Vec3::new(a.cos(), 0.0, a.sin());
    let perp = Vec3::new(-dir.z, 0.0, dir.x);
    let lift = 0.02 * rng.f32();
    b.quad_c(
        [
            p + dir * (len * 0.5),
            p + perp * (len * 0.32) + Vec3::Y * lift,
            p - dir * (len * 0.5),
            p - perp * (len * 0.32),
        ],
        [col, col2, col, col2],
        Vec3::Y,
    );
}

pub fn rock(b: &mut MeshBuilder, rng: &mut GameRng, p: Vec3, r: f32) {
    let g = rng.range(0.22, 0.42);
    let dark = lin(g * 0.6, g * 0.6, g * 0.62);
    let light = lin(g * 1.1, g * 1.08, g * 1.04);
    b.blob(
        p + Vec3::Y * (r * 0.25),
        r,
        0.6,
        dark,
        light,
        3,
        6,
        rng.range(0.0, 30.0),
    );
}

pub fn twig(b: &mut MeshBuilder, rng: &mut GameRng, p: Vec3, len: f32) {
    let a = rng.range(0.0, TAU);
    let d = Vec3::new(a.cos(), 0.0, a.sin());
    let c = lin(0.14, 0.10, 0.06);
    let to = p + d * len + Vec3::Y * rng.range(0.0, 0.05);
    b.tube(p, to, 0.012 + len * 0.01, 0.006, 4, c, scale_col(c, 0.8));
    if len > 0.5 {
        let m = p + d * len * 0.55;
        let sign = if rng.chance(0.5) { -1.0 } else { 1.0 };
        let a2 = a + rng.range(0.4, 0.9) * sign;
        let d2 = Vec3::new(a2.cos(), 0.0, a2.sin());
        b.tube(
            m,
            m + d2 * len * 0.35 + Vec3::Y * 0.03,
            0.008,
            0.004,
            4,
            c,
            c,
        );
    }
}

pub fn mushroom(b: &mut MeshBuilder, rng: &mut GameRng, p: Vec3) {
    let h = rng.range(0.04, 0.1);
    let stem = lin(0.75, 0.70, 0.60);
    b.tube(p, p + Vec3::Y * h, 0.009, 0.006, 5, stem, stem);
    let reddish = rng.chance(0.35);
    let (dark, light) = if reddish {
        (lin(0.40, 0.08, 0.05), lin(0.62, 0.16, 0.08))
    } else {
        (lin(0.30, 0.17, 0.10), lin(0.52, 0.34, 0.20))
    };
    b.blob(
        p + Vec3::Y * h,
        h * 0.7 + 0.02,
        0.5,
        dark,
        light,
        3,
        6,
        rng.range(0.0, 20.0),
    );
}

pub fn weed(b: &mut MeshBuilder, rng: &mut GameRng, p: Vec3, h: f32, base: Rgba, tip: Rgba) {
    for _ in 0..6 {
        let a = rng.range(0.0, TAU);
        let dir = Vec3::new(a.cos(), 0.0, a.sin());
        let perp = Vec3::new(-dir.z, 0.0, dir.x);
        let lean = rng.range(0.0, 0.2);
        let hh = h * rng.range(0.6, 1.2);
        let root = p + dir * 0.03;
        let l = root - perp * 0.02;
        let r = root + perp * 0.02;
        let t = root + dir * lean + Vec3::Y * hh;
        b.tri3c(l, r, t, base, base, tip);
        b.tri3c(r, l, t, base, base, tip);
    }
}

fn vine(b: &mut MeshBuilder, rng: &mut GameRng, start: Vec3, out: Vec3, along: Vec3, height: f32) {
    let bark = lin(0.12, 0.09, 0.05);
    let segs = (height / 0.4) as i32;
    let mut p = start;
    for _ in 0..segs {
        let next = p + Vec3::Y * 0.4 + along * rng.range(-0.15, 0.15);
        b.tube(p, next, 0.02, 0.016, 4, bark, bark);
        for _ in 0..3 {
            let c =
                next + out * 0.04 + along * rng.range(-0.18, 0.18) + Vec3::Y * rng.range(-0.2, 0.2);
            let phi = rng.range(0.0, TAU);
            let u = (along * phi.cos() + Vec3::Y * phi.sin()) * 0.09;
            let v = out.cross(u).normalize_or_zero() * 0.055;
            let col = if rng.chance(0.3) {
                scale_col(lin(0.12, 0.16, 0.05), rng.range(0.8, 1.2))
            } else {
                let pc = LEAF_COLORS[rng.pick(LEAF_COLORS.len())];
                scale_col(lin(pc[0], pc[1], pc[2]), rng.range(0.7, 1.1))
            };
            leaf_card(b, c, u, v, col);
        }
        p = next;
    }
}

pub fn build_house_detail(seed: u64, dims: Dims) -> Mesh {
    let mut rng = GameRng::seeded(seed ^ 0xD37A_11E5);
    let mut b = MeshBuilder::default();
    let Dims { w, d, h, bay_z, .. } = dims;
    let hx_c = -LOT * 0.5 + FRONT + d * 0.5;
    let xf = hx_c - d * 0.5;
    let xb = hx_c + d * 0.5 + 0.1;
    let hw = w * 0.5 + 0.08;
    let half = LOT * 0.5 - 0.6;
    let y0 = 0.05;
    let near_bay = |z: f32| bay_z.map(|bz| (z - bz).abs() < 1.4).unwrap_or(false);
    let blocked = |x: f32, z: f32| -> bool {
        (x > xf - 0.5 && x < xb + 0.5 && z.abs() < hw + 0.5)
            || (x > xf - 4.4 && x < xf + 0.1 && z.abs() < 2.9)
            || (x < xf - 3.8 && z.abs() < 1.3)
    };
    let spot = |rng: &mut GameRng| -> (f32, f32) {
        for _ in 0..30 {
            let x = rng.range(-half, half);
            let z = rng.range(-half, half);
            if !blocked(x, z) {
                return (x, z);
            }
        }
        (-half, half)
    };

    let weed_base = lin(0.12, 0.12, 0.05);
    let weed_tip = lin(0.45, 0.38, 0.18);

    for _ in 0..70 {
        let x = xf - 0.15 - rng.f32().powi(2) * 0.9;
        let z = rng.range(-hw, hw);
        if z.abs() < 2.9 || near_bay(z) {
            continue;
        }
        ground_leaf(&mut b, &mut rng, Vec3::new(x, y0, z));
    }
    for _ in 0..70 {
        let x = xb + 0.1 + rng.f32().powi(2) * 0.9;
        let z = rng.range(-hw, hw);
        ground_leaf(&mut b, &mut rng, Vec3::new(x, y0, z));
    }
    for sg in [-1.0_f32, 1.0] {
        for _ in 0..60 {
            let x = rng.range(xf, xb);
            let z = sg * (hw + 0.1 + rng.f32().powi(2) * 0.9);
            ground_leaf(&mut b, &mut rng, Vec3::new(x, y0, z));
        }
        for _ in 0..9 {
            let x = rng.range(xf, xb);
            let z = sg * (hw + rng.range(0.05, 0.3));
            let hgt = rng.range(0.2, 0.5);
            weed(
                &mut b,
                &mut rng,
                Vec3::new(x, y0, z),
                hgt,
                weed_base,
                weed_tip,
            );
        }
    }
    for _ in 0..10 {
        let x = xb + rng.range(0.05, 0.3);
        let z = rng.range(-hw, hw);
        let hgt = rng.range(0.2, 0.5);
        weed(
            &mut b,
            &mut rng,
            Vec3::new(x, y0, z),
            hgt,
            weed_base,
            weed_tip,
        );
    }

    let soil = lin(0.07, 0.05, 0.035);
    let bed_end = w * 0.5 - 0.5;
    for sg in [-1.0_f32, 1.0] {
        let (z0, z1) = if sg < 0.0 {
            (-bed_end, -2.9)
        } else {
            (2.9, bed_end)
        };
        if z1 - z0 < 0.5 {
            continue;
        }
        b.quad_out(
            Vec3::new(xf - 0.8, 0.045, z0),
            Vec3::new(xf - 0.08, 0.045, z0),
            Vec3::new(xf - 0.08, 0.045, z1),
            Vec3::new(xf - 0.8, 0.045, z1),
            Vec3::Y,
            soil,
        );
        for _ in 0..9 {
            let z = rng.range(z0 + 0.1, z1 - 0.1);
            if near_bay(z) {
                continue;
            }
            let x = xf - rng.range(0.2, 0.7);
            let hgt = rng.range(0.3, 0.8);
            let top = Vec3::new(
                x + rng.range(-0.08, 0.08),
                0.045 + hgt,
                z + rng.range(-0.08, 0.08),
            );
            let stalk = lin(0.20, 0.15, 0.07);
            b.tube(Vec3::new(x, 0.045, z), top, 0.008, 0.004, 4, stalk, stalk);
            b.blob(
                top,
                0.035,
                0.8,
                lin(0.20, 0.10, 0.06),
                lin(0.35, 0.22, 0.12),
                3,
                5,
                rng.range(0.0, 10.0),
            );
        }
    }

    for _ in 0..4 {
        let z = rng.range(-hw + 1.0, hw - 1.0);
        let height = rng.range(2.0, (h - 0.3).max(2.5));
        vine(
            &mut b,
            &mut rng,
            Vec3::new(xb - 0.05, 0.0, z),
            Vec3::X,
            Vec3::Z,
            height,
        );
    }
    for sg in [-1.0_f32, 1.0] {
        for _ in 0..2 {
            let x = rng.range(xf + 1.0, xb - 1.0);
            let height = rng.range(1.8, (h - 0.5).max(2.4));
            vine(
                &mut b,
                &mut rng,
                Vec3::new(x, 0.0, sg * (hw - 0.05)),
                Vec3::Z * sg,
                Vec3::X,
                height,
            );
        }
    }

    b.quad_out(
        Vec3::new(xf - 1.0, 0.565, -0.5),
        Vec3::new(xf - 0.3, 0.565, -0.5),
        Vec3::new(xf - 0.3, 0.565, 0.5),
        Vec3::new(xf - 1.0, 0.565, 0.5),
        Vec3::Y,
        lin(0.18, 0.12, 0.07),
    );
    for _ in 0..18 {
        let x = rng.range(xf - 2.4, xf - 0.2);
        let z = rng.range(-2.2, 2.2);
        ground_leaf(&mut b, &mut rng, Vec3::new(x, 0.56, z));
    }

    for _ in 0..45 {
        let (x, z) = spot(&mut rng);
        let r = rng.range(0.04, 0.16);
        rock(&mut b, &mut rng, Vec3::new(x, y0 - 0.02, z), r);
    }
    for _ in 0..24 {
        let (x, z) = spot(&mut rng);
        let len = rng.range(0.3, 0.9);
        twig(&mut b, &mut rng, Vec3::new(x, y0, z), len);
    }
    for _ in 0..6 {
        let (cx, cz) = spot(&mut rng);
        for _ in 0..3 {
            let x = cx + rng.range(-0.2, 0.2);
            let z = cz + rng.range(-0.2, 0.2);
            mushroom(&mut b, &mut rng, Vec3::new(x, y0, z));
        }
    }
    for _ in 0..7 {
        let (x, z) = spot(&mut rng);
        let r = rng.range(0.5, 1.3);
        b.disc(Vec3::new(x, 0.052, z), r, 10, true, lin(0.14, 0.10, 0.07));
    }
    for _ in 0..20 {
        let (x, z) = spot(&mut rng);
        let hgt = rng.range(0.2, 0.5);
        weed(
            &mut b,
            &mut rng,
            Vec3::new(x, y0, z),
            hgt,
            weed_base,
            weed_tip,
        );
    }

    b.build()
}
