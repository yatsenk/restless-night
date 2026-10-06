use bevy::prelude::*;
use std::f32::consts::{PI, TAU};

use bevy::prelude::*;

use crate::houses::Dims;
use crate::mesh::MeshBuilder;
use crate::util::*;
use crate::{
    BLOCK_MIN, CHUNK, CURB_W, FRONT, LAMP_LIT_CHANCE, LOT, ROAD_W, STRIP_W, WINDOW_LIT_CHANCE,
};

const GROUND_Y: f32 = -0.25;
const ROAD_Y: f32 = -0.2;

fn tree(b: &mut MeshBuilder, base: Vec3, scale: f32, leafy: bool, tone: f32) {
    let h = 8.3 * scale;
    let trunk = lin(0.08, 0.06, 0.045);
    b.tube(
        base,
        base + Vec3::Y * h * 0.75,
        0.4 * scale,
        0.14 * scale,
        5,
        trunk,
        trunk,
    );
    if leafy {
        let dark = lerp_col(lin(0.26, 0.10, 0.04), lin(0.40, 0.14, 0.04), tone);
        let light = lerp_col(lin(0.52, 0.26, 0.06), lin(0.74, 0.40, 0.08), tone);
        b.blob(
            base + Vec3::Y * h * 0.72,
            h * 0.27,
            0.85,
            dark,
            light,
            4,
            7,
            tone * 9.0,
        );
        b.blob(
            base + Vec3::new(h * 0.13, h * 0.55, h * 0.05),
            h * 0.2,
            0.85,
            dark,
            light,
            3,
            6,
            tone * 5.0,
        );
        b.blob(
            base + Vec3::new(-h * 0.12, h * 0.52, -h * 0.07),
            h * 0.2,
            0.85,
            dark,
            light,
            3,
            6,
            tone * 3.0,
        );
    } else {
        let col = lin(0.06, 0.05, 0.04);
        b.pyramid(base + Vec3::Y * h * 0.4, h * 0.2, h * 0.2, h * 0.68, col);
    }
}

fn house(
    b: &mut MeshBuilder,
    g: &mut MeshBuilder,
    v: &Dims,
    p: &dyn Fn(f32, f32, f32) -> Vec3,
    rot: Quat,
    lit: bool,
) {
    let Dims { w, d, h, rh, .. } = *v;
    let hx = d * 0.5 + 0.7;
    let hz = w * 0.5 + 0.55;
    b.obox(
        p(0.0, h * 0.5, 0.0),
        rot * Vec3::X,
        rot * Vec3::Z,
        Vec3::new(d * 0.5, h * 0.5, w * 0.5),
        v.wall,
    );
    for s in [-1.0_f32, 1.0] {
        b.quad_out(
            p(s * hx, h, -hz),
            p(s * hx, h, hz),
            p(0.0, h + rh, hz),
            p(0.0, h + rh, -hz),
            rot * Vec3::new(s * rh, hx, 0.0),
            v.roof,
        );
        b.tri_out(
            p(-d * 0.5, h, s * w * 0.5),
            p(d * 0.5, h, s * w * 0.5),
            p(0.0, h + rh * 0.9, s * w * 0.5),
            rot * Vec3::new(0.0, 0.0, s),
            v.wall,
        );
    }
    if lit {
        let white = lin(1.0, 1.0, 1.0);
        for s in [-1.0_f32, 1.0] {
            let x = -d * 0.5 - 0.04;
            let zc = s * w * 0.25;
            g.quad_out(
                p(x, 1.5, zc - 0.5),
                p(x, 1.5, zc + 0.5),
                p(x, 2.6, zc + 0.5),
                p(x, 2.6, zc - 0.5),
                rot * Vec3::NEG_X,
                white,
            );
        }
    }
}

pub fn build_far_chunk(c: IVec2, houses: &[Dims], tree_leafy: &[bool]) -> (Mesh, Mesh) {
    let mut b = MeshBuilder::default();
    let mut g = MeshBuilder::default();
    let white = lin(1.0, 1.0, 1.0);

    b.quad_out(
        Vec3::new(0.0, GROUND_Y, 0.0),
        Vec3::new(CHUNK, GROUND_Y, 0.0),
        Vec3::new(CHUNK, GROUND_Y, CHUNK),
        Vec3::new(0.0, GROUND_Y, CHUNK),
        Vec3::Y,
        lin(0.09, 0.12, 0.06),
    );
    let asphalt = lin(0.07, 0.07, 0.08);
    b.quad_out(
        Vec3::new(0.0, ROAD_Y, 0.0),
        Vec3::new(CHUNK, ROAD_Y, 0.0),
        Vec3::new(CHUNK, ROAD_Y, ROAD_W),
        Vec3::new(0.0, ROAD_Y, ROAD_W),
        Vec3::Y,
        asphalt,
    );
    b.quad_out(
        Vec3::new(0.0, ROAD_Y, 0.0),
        Vec3::new(ROAD_W, ROAD_Y, 0.0),
        Vec3::new(ROAD_W, ROAD_Y, CHUNK),
        Vec3::new(0.0, ROAD_Y, CHUNK),
        Vec3::Y,
        asphalt,
    );

    for idx in 0..4u32 {
        let mut rng =
            GameRng::seeded(chunk_seed(c) ^ (idx as u64 + 1).wrapping_mul(0x9E37_79B9_7F4A_7C15));
        let ix = (idx % 2) as f32;
        let iz = (idx / 2) as f32;
        let center = Vec3::new(
            BLOCK_MIN + LOT * (ix + 0.5),
            0.0,
            BLOCK_MIN + LOT * (iz + 0.5),
        );
        let yaw = if ix < 0.5 { 0.0 } else { PI };
        let rot = Quat::from_rotation_y(yaw);
        let v = houses[rng.pick(houses.len())];
        let lit = rng.chance(WINDOW_LIT_CHANCE);
        let hx_c = -LOT * 0.5 + FRONT + v.d * 0.5;
        let place = move |x: f32, y: f32, z: f32| center + rot * Vec3::new(hx_c + x, y, z);
        house(&mut b, &mut g, &v, &place, rot, lit);

        let mut tr = GameRng::seeded(
            chunk_seed(c) ^ 0x7733_AA11 ^ (idx as u64 + 1).wrapping_mul(0xC2B2_AE3D_27D4_EB4F),
        );
        for _ in 0..(1 + tr.pick(3)) {
            let x = tr.range(hx_c + v.d * 0.5 + 2.0, LOT * 0.5 - 2.0);
            let side = if tr.chance(0.5) { -1.0 } else { 1.0 };
            let z = side * tr.range(2.0, LOT * 0.5 - 2.5);
            let base = center + rot * Vec3::new(x, 0.0, z);
            let leafy = tree_leafy[tr.pick(tree_leafy.len())];
            let s = tr.range(0.9, 1.2);
            tree(&mut b, base, s, leafy, tr.f32());
        }
    }

    let mut rng = GameRng::seeded(chunk_seed(c));
    let near = ROAD_W + CURB_W + STRIP_W * 0.5;
    let far = CHUNK - CURB_W - STRIP_W * 0.5;
    let lamp = |g: &mut MeshBuilder, x: f32, z: f32, lit: bool| {
        if lit {
            g.cuboid(
                Vec3::new(x - 0.3, 4.3, z - 0.3),
                Vec3::new(x + 0.3, 4.9, z + 0.3),
                white,
            );
        }
    };
    for k in 0..2 {
        for edge in [near, far] {
            let t = 26.0 + k as f32 * 30.0 + rng.range(-3.0, 3.0);
            let lit = rng.chance(LAMP_LIT_CHANCE);
            lamp(&mut g, edge, t, lit);
            let t = 26.0 + k as f32 * 30.0 + rng.range(-3.0, 3.0);
            let lit = rng.chance(LAMP_LIT_CHANCE);
            lamp(&mut g, t, edge, lit);
        }
    }
    for k in 0..3 {
        for edge in [near, far] {
            for along_z in [true, false] {
                if rng.chance(0.8) {
                    let t = 18.0 + k as f32 * 26.0 + rng.range(-2.0, 2.0);
                    let idx = rng.pick(tree_leafy.len());
                    let s = rng.range(0.9, 1.25);
                    let _yaw = rng.range(0.0, TAU);
                    let base = if along_z {
                        Vec3::new(edge, 0.0, t)
                    } else {
                        Vec3::new(t, 0.0, edge)
                    };
                    tree(&mut b, base, s, tree_leafy[idx], 0.5);
                }
            }
        }
    }

    (b.build(), g.build())
}
