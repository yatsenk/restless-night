use std::f32::consts::TAU;

use bevy::prelude::*;

use crate::mesh::MeshBuilder;
use crate::util::{Rgba, fbm, hash2, lerp_col, lin, scale_col, vnoise};
use crate::{BLOCK_MAX, BLOCK_MIN, CHUNK, CURB_W, LOT, ROAD_W, WALK_IN, WALK_W};

type Rect = (f32, f32, f32, f32);

fn smooth(a: f32, b: f32, x: f32) -> f32 {
    let t = ((x - a) / (b - a)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

fn in_rects(x: f32, z: f32, rects: &[Rect]) -> bool {
    rects
        .iter()
        .any(|r| x >= r.0 && x <= r.2 && z >= r.1 && z <= r.3)
}

fn lawn_set(v: f32) -> bool {
    (v >= ROAD_W + CURB_W && v < WALK_IN)
        || (v >= BLOCK_MIN && v <= BLOCK_MAX)
        || (v >= BLOCK_MAX + WALK_W && v < CHUNK - CURB_W)
}

fn walk_set(v: f32) -> bool {
    (v >= WALK_IN && v < BLOCK_MIN) || (v >= BLOCK_MAX && v < BLOCK_MAX + WALK_W)
}

fn lot_rects() -> (Vec<Rect>, Vec<Rect>) {
    let mut houses = Vec::new();
    let mut paths = Vec::new();
    for ix in 0..2 {
        for iz in 0..2 {
            let cx = BLOCK_MIN + LOT * (ix as f32 + 0.5);
            let cz = BLOCK_MIN + LOT * (iz as f32 + 0.5);
            if ix == 0 {
                houses.push((cx - 11.8, cz - 8.4, cx + 5.5, cz + 8.4));
                paths.push((cx - LOT * 0.5, cz - 1.1, cx - 11.4, cz + 1.1));
            } else {
                houses.push((cx - 5.5, cz - 8.4, cx + 11.8, cz + 8.4));
                paths.push((cx + 11.4, cz - 1.1, cx + LOT * 0.5, cz + 1.1));
            }
        }
    }
    (houses, paths)
}

fn lawn_height(x: f32, z: f32, s: u32) -> f32 {
    (fbm(x * 0.3, z * 0.3, s + 3) - 0.5) * 0.09
}

fn lawn_color(x: f32, z: f32, s: u32) -> Rgba {
    let big = fbm(x * 0.07, z * 0.07, s + 1);
    let mid = fbm(x * 0.35, z * 0.35, s + 2);
    let fine = vnoise(x * 2.2, z * 2.2, s + 3);
    let dirt_m = fbm(x * 0.18, z * 0.18, s + 4);
    let lush = lin(0.07, 0.10, 0.04);
    let green = lin(0.12, 0.15, 0.06);
    let dry = lin(0.26, 0.21, 0.09);
    let dirt = lin(0.16, 0.11, 0.07);
    let mut c = lerp_col(lush, green, mid);
    c = lerp_col(c, dry, smooth(0.42, 0.7, big));
    c = lerp_col(c, dirt, smooth(0.62, 0.78, dirt_m));
    scale_col(c, 0.85 + 0.3 * fine)
}

fn asphalt_color(x: f32, z: f32, s: u32) -> Rgba {
    let n = fbm(x * 0.5, z * 0.5, s + 21);
    scale_col(lin(0.075, 0.075, 0.085), 0.8 + 0.4 * n)
}

fn surface_y(x: f32, z: f32, s: u32, paths: &[Rect]) -> f32 {
    if x < ROAD_W || z < ROAD_W {
        return 0.03;
    }
    if x < ROAD_W + CURB_W || x > CHUNK - CURB_W || z < ROAD_W + CURB_W || z > CHUNK - CURB_W {
        return 0.15;
    }
    let far = BLOCK_MAX + WALK_W;
    if (walk_set(x) && z >= WALK_IN && z < far) || (walk_set(z) && x >= WALK_IN && x < far) {
        return 0.12;
    }
    if in_rects(x, z, paths) {
        return 0.06;
    }
    lawn_height(x, z, s)
}

fn flat_quad(b: &mut MeshBuilder, x0: f32, z0: f32, x1: f32, z1: f32, y: f32, col: Rgba) {
    b.quad_out(
        Vec3::new(x0, y, z0),
        Vec3::new(x1, y, z0),
        Vec3::new(x1, y, z1),
        Vec3::new(x0, y, z1),
        Vec3::Y,
        col,
    );
}

fn line_quad(b: &mut MeshBuilder, p0: Vec2, p1: Vec2, w: f32, y: f32, col: Rgba) {
    let d = (p1 - p0).normalize_or_zero();
    let n = Vec2::new(-d.y, d.x) * (w * 0.5);
    b.quad_out(
        Vec3::new(p0.x - n.x, y, p0.y - n.y),
        Vec3::new(p0.x + n.x, y, p0.y + n.y),
        Vec3::new(p1.x + n.x, y, p1.y + n.y),
        Vec3::new(p1.x - n.x, y, p1.y - n.y),
        Vec3::Y,
        col,
    );
}

fn tuft(b: &mut MeshBuilder, p: Vec3, sid: i32, base: Rgba, tip: Rgba) {
    for blade in 0..3 {
        let a = hash2(sid, blade, 5) * TAU;
        let lean = hash2(sid, blade, 6) * 0.14;
        let h = 0.14 + 0.24 * hash2(sid, blade, 7);
        let dir = Vec3::new(a.cos(), 0.0, a.sin());
        let perp = Vec3::new(-dir.z, 0.0, dir.x);
        let root = p + dir * 0.05;
        let l = root - perp * 0.03;
        let r = root + perp * 0.03;
        let t = root + dir * lean + Vec3::Y * h;
        b.tri3c(l, r, t, base, base, tip);
        b.tri3c(r, l, t, base, base, tip);
    }
}

fn tile_run(b: &mut MeshBuilder, rect: Rect, along_z: bool, s: u32) {
    let (x0, z0, x1, z1) = rect;
    let walk = lin(0.30, 0.29, 0.28);
    let len = if along_z { z1 - z0 } else { x1 - x0 };
    let n = (len / 2.0).ceil() as i32;
    let key = (x0 * 7.0) as i32 + (z0 * 3.0) as i32;
    for i in 0..n {
        let a0 = i as f32 * 2.0;
        let a1 = (a0 + 2.0).min(len);
        let (tx0, tz0, tx1, tz1) = if along_z {
            (x0, z0 + a0, x1, z0 + a1)
        } else {
            (x0 + a0, z0, x0 + a1, z1)
        };
        let hgt = 0.115 + 0.01 * hash2(i, key, s + 30);
        let k = 0.8 + 0.4 * hash2(i, key, s + 31);
        b.cuboid(
            Vec3::new(tx0 + 0.02, 0.0, tz0 + 0.02),
            Vec3::new(tx1 - 0.02, hgt, tz1 - 0.02),
            scale_col(walk, k),
        );
    }
}

fn curb_run(b: &mut MeshBuilder, rect: Rect, along_z: bool, s: u32) {
    let (x0, z0, x1, z1) = rect;
    let curb = lin(0.34, 0.33, 0.31);
    let len = if along_z { z1 - z0 } else { x1 - x0 };
    let n = (len / 2.0).ceil() as i32;
    let key = (x0 * 5.0) as i32 + (z0 * 11.0) as i32;
    for i in 0..n {
        let a0 = i as f32 * 2.0;
        let a1 = (a0 + 2.0).min(len);
        let (tx0, tz0, tx1, tz1) = if along_z {
            (x0, z0 + a0, x1, z0 + a1)
        } else {
            (x0 + a0, z0, x0 + a1, z1)
        };
        let k = 0.8 + 0.35 * hash2(i, key, s + 40);
        b.cuboid(
            Vec3::new(tx0, 0.0, tz0),
            Vec3::new(tx1, 0.15, tz1),
            scale_col(curb, k),
        );
    }
}

pub fn build_street(variant: u32) -> Mesh {
    let s = variant;
    let mut b = MeshBuilder::default();
    let (houses, paths) = lot_rects();
    let n = CHUNK as i32;
    let stride = (n + 1) as usize;

    let mut vid = vec![u32::MAX; stride * stride];
    for gx in 0..n {
        for gz in 0..n {
            let x0 = gx as f32;
            let z0 = gz as f32;
            if x0 + 1.0 <= ROAD_W || z0 + 1.0 <= ROAD_W {
                continue;
            }
            let mut ids = [0u32; 4];
            for (k, (cx, cz)) in [(gx, gz), (gx + 1, gz), (gx + 1, gz + 1), (gx, gz + 1)]
                .iter()
                .enumerate()
            {
                let slot = *cz as usize * stride + *cx as usize;
                if vid[slot] == u32::MAX {
                    let fx = *cx as f32;
                    let fz = *cz as f32;
                    vid[slot] = b.vert(
                        Vec3::new(fx, lawn_height(fx, fz, s), fz),
                        lawn_color(fx, fz, s),
                    );
                }
                ids[k] = vid[slot];
            }
            b.tri(ids[0], ids[3], ids[2]);
            b.tri(ids[0], ids[2], ids[1]);
        }
    }

    let acell = 2.0;
    let ac = (CHUNK / acell) as i32;
    let rc = (ROAD_W / acell) as i32;
    let mut road_cells = Vec::new();
    for i in 0..ac {
        for j in 0..rc {
            road_cells.push((i, j));
        }
    }
    for i in 0..rc {
        for j in rc..ac {
            road_cells.push((i, j));
        }
    }
    for (i, j) in road_cells {
        let x0 = i as f32 * acell;
        let z0 = j as f32 * acell;
        let x1 = x0 + acell;
        let z1 = z0 + acell;
        b.quad_c(
            [
                Vec3::new(x0, 0.03, z0),
                Vec3::new(x1, 0.03, z0),
                Vec3::new(x1, 0.03, z1),
                Vec3::new(x0, 0.03, z1),
            ],
            [
                asphalt_color(x0, z0, s),
                asphalt_color(x1, z0, s),
                asphalt_color(x1, z1, s),
                asphalt_color(x0, z1, s),
            ],
            Vec3::Y,
        );
    }

    let asphalt = lin(0.075, 0.075, 0.085);
    for i in 0..16 {
        let on_x = hash2(i, 1, s + 50) < 0.5;
        let (rx, rz) = if on_x {
            (
                hash2(i, 2, s + 50) * (CHUNK - 4.0),
                hash2(i, 3, s + 50) * (ROAD_W - 2.0),
            )
        } else {
            (
                hash2(i, 2, s + 51) * (ROAD_W - 2.0),
                ROAD_W + hash2(i, 3, s + 51) * (CHUNK - ROAD_W - 4.0),
            )
        };
        let w = 1.4 + 1.2 * hash2(i, 4, s + 50);
        let d = 0.9 + 0.7 * hash2(i, 5, s + 50);
        let (w, d) = if on_x { (w, d) } else { (d, w) };
        flat_quad(
            &mut b,
            rx,
            rz,
            rx + w,
            rz + d,
            0.032,
            scale_col(asphalt, 0.75),
        );
    }
    for i in 0..44 {
        let on_x = hash2(i, 6, s + 52) < 0.5;
        let mut p = if on_x {
            Vec2::new(
                hash2(i, 7, s + 52) * CHUNK,
                0.3 + hash2(i, 8, s + 52) * (ROAD_W - 0.6),
            )
        } else {
            Vec2::new(
                0.3 + hash2(i, 7, s + 53) * (ROAD_W - 0.6),
                ROAD_W + hash2(i, 8, s + 53) * (CHUNK - ROAD_W),
            )
        };
        let mut a = hash2(i, 9, s + 52) * TAU;
        for k in 0..5 {
            let len = 0.4 + 0.6 * hash2(i, 10 + k, s + 52);
            let q = p + Vec2::new(a.cos(), a.sin()) * len;
            line_quad(&mut b, p, q, 0.035, 0.034, scale_col(asphalt, 0.4));
            p = q;
            a += (hash2(i, 30 + k, s + 52) - 0.5) * 1.4;
        }
    }

    let far = BLOCK_MAX + WALK_W;
    let gap = (WALK_IN, BLOCK_MIN);
    let edge = CHUNK - CURB_W;
    curb_run(&mut b, (ROAD_W, ROAD_W, ROAD_W + CURB_W, gap.0), true, s);
    curb_run(&mut b, (ROAD_W, gap.1, ROAD_W + CURB_W, CHUNK), true, s);
    curb_run(&mut b, (edge, ROAD_W, CHUNK, gap.0), true, s);
    curb_run(&mut b, (edge, gap.1, CHUNK, CHUNK), true, s);
    curb_run(
        &mut b,
        (ROAD_W + CURB_W, ROAD_W, gap.0, ROAD_W + CURB_W),
        false,
        s,
    );
    curb_run(&mut b, (gap.1, ROAD_W, edge, ROAD_W + CURB_W), false, s);
    curb_run(&mut b, (ROAD_W + CURB_W, edge, gap.0, CHUNK), false, s);
    curb_run(&mut b, (gap.1, edge, edge, CHUNK), false, s);

    tile_run(&mut b, (WALK_IN, WALK_IN, BLOCK_MIN, far), true, s);
    tile_run(&mut b, (BLOCK_MAX, WALK_IN, far, far), true, s);
    tile_run(&mut b, (BLOCK_MIN, WALK_IN, BLOCK_MAX, BLOCK_MIN), false, s);
    tile_run(&mut b, (BLOCK_MIN, BLOCK_MAX, BLOCK_MAX, far), false, s);

    tile_run(&mut b, (ROAD_W + CURB_W, gap.0, WALK_IN, gap.1), false, s);
    tile_run(&mut b, (far, gap.0, edge, gap.1), false, s);
    tile_run(&mut b, (gap.0, ROAD_W + CURB_W, gap.1, WALK_IN), true, s);
    tile_run(&mut b, (gap.0, far, gap.1, edge), true, s);

    let white = lin(0.70, 0.70, 0.68);
    for i in 0..9 {
        let c = 0.95 + i as f32;
        flat_quad(
            &mut b,
            c - 0.25,
            gap.0 + 0.1,
            c + 0.25,
            gap.1 - 0.1,
            0.035,
            white,
        );
        flat_quad(
            &mut b,
            gap.0 + 0.1,
            c - 0.25,
            gap.1 - 0.1,
            c + 0.25,
            0.035,
            white,
        );
    }
    let yellow = lin(0.72, 0.60, 0.18);
    let mut t = 20.0;
    while t + 3.0 <= CHUNK {
        flat_quad(&mut b, 4.9, t, 5.1, t + 3.0, 0.036, yellow);
        flat_quad(&mut b, t, 4.9, t + 3.0, 5.1, 0.036, yellow);
        t += 6.0;
    }
    for (x, z) in [(42.0_f32, 2.6_f32), (2.6, 55.0), (66.0, 7.4)] {
        b.disc(Vec3::new(x, 0.038, z), 0.5, 12, true, lin(0.12, 0.12, 0.13));
        b.disc(Vec3::new(x, 0.04, z), 0.38, 12, true, lin(0.18, 0.18, 0.19));
    }

    for gx in 0..n {
        for gz in 0..n {
            for k in 0..2 {
                let fx = gx as f32 + hash2(gx * 2 + k, gz, s + 60);
                let fz = gz as f32 + hash2(gx, gz * 2 + k, s + 61);
                if !lawn_set(fx) || !lawn_set(fz) || in_rects(fx, fz, &houses) {
                    continue;
                }
                if in_rects(fx, fz, &paths) {
                    continue;
                }
                let lushness = fbm(fx * 0.07, fz * 0.07, s + 1);
                let dens = 0.75 - 0.5 * smooth(0.55, 0.8, lushness);
                if hash2(gx * 2 + k, gz * 3 + 7, s + 62) > dens {
                    continue;
                }
                let c = lawn_color(fx, fz, s);
                let base = scale_col(c, 0.7);
                let tip = lerp_col(scale_col(c, 1.5), lin(0.45, 0.38, 0.18), 0.4);
                tuft(
                    &mut b,
                    Vec3::new(fx, lawn_height(fx, fz, s), fz),
                    gx * 131 + gz * 7 + k,
                    base,
                    tip,
                );
            }
        }
    }

    let palette: [[f32; 3]; 6] = [
        [0.55, 0.20, 0.04],
        [0.48, 0.08, 0.05],
        [0.62, 0.42, 0.07],
        [0.36, 0.17, 0.06],
        [0.28, 0.12, 0.04],
        [0.70, 0.30, 0.05],
    ];
    for i in 0..4600 {
        let x = hash2(i, 1, s + 70) * CHUNK;
        let z = hash2(i, 2, s + 71) * CHUNK;
        let drift = fbm(x * 0.09, z * 0.09, s + 90);
        let p = 0.1 + 0.9 * smooth(0.45, 0.72, drift);
        if hash2(i, 3, s + 72) > p || in_rects(x, z, &houses) {
            continue;
        }
        let y = surface_y(x, z, s, &paths) + 0.012;
        let pc = palette[(hash2(i, 4, s + 73) * 6.0) as usize % 6];
        let k = 0.75 + 0.45 * hash2(i, 5, s + 74);
        let col = scale_col(lin(pc[0], pc[1], pc[2]), k);
        let col2 = scale_col(col, 0.8);
        let a = hash2(i, 6, s + 75) * TAU;
        let len = 0.09 + 0.12 * hash2(i, 7, s + 76);
        let dir = Vec3::new(a.cos(), 0.0, a.sin());
        let perp = Vec3::new(-dir.z, 0.0, dir.x);
        let c = Vec3::new(x, y, z);
        let lift = 0.02 * hash2(i, 8, s + 77);
        b.quad_c(
            [
                c + dir * (len * 0.5),
                c + perp * (len * 0.3) + Vec3::Y * lift,
                c - dir * (len * 0.5),
                c - perp * (len * 0.3),
            ],
            [col, col2, col, col2],
            Vec3::Y,
        );
    }

    b.build()
}
