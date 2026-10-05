use std::f32::consts::{PI, TAU};

use bevy::prelude::*;

use crate::mesh::MeshBuilder;
use crate::util::{GameRng, Rgba, lerp_col, lin, scale_col};

const LEAF_TONES: [([f32; 3], [f32; 3]); 4] = [
    ([0.40, 0.12, 0.02], [0.80, 0.38, 0.05]),
    ([0.32, 0.04, 0.03], [0.65, 0.16, 0.05]),
    ([0.42, 0.28, 0.04], [0.80, 0.60, 0.12]),
    ([0.25, 0.10, 0.04], [0.55, 0.27, 0.07]),
];

pub fn build_gravestone(kind: u32, detail: bool) -> Mesh {
    let mut b = MeshBuilder::default();
    let stone = lin(0.34, 0.35, 0.36);
    let dark = lin(0.12, 0.13, 0.13);
    let moss = lin(0.17, 0.26, 0.12);
    if !detail {
        b.cuboid(
            Vec3::new(-0.45, 0.0, -0.1),
            Vec3::new(0.45, 1.3, 0.1),
            stone,
        );
        return b.build();
    }
    b.cuboid(
        Vec3::new(-0.65, 0.0, -0.3),
        Vec3::new(0.65, 0.18, 0.3),
        scale_col(stone, 0.8),
    );
    match kind {
        0 => {
            b.cuboid(
                Vec3::new(-0.45, 0.18, -0.1),
                Vec3::new(0.45, 1.1, 0.1),
                stone,
            );
            let steps = 12;
            for i in 0..steps {
                let a0 = i as f32 / steps as f32 * PI;
                let a1 = (i + 1) as f32 / steps as f32 * PI;
                let p0 = Vec2::new(a0.cos() * 0.45, 1.1 + a0.sin() * 0.4);
                let p1 = Vec2::new(a1.cos() * 0.45, 1.1 + a1.sin() * 0.4);
                b.tri_out(
                    Vec3::new(0.0, 1.1, 0.1),
                    Vec3::new(p0.x, p0.y, 0.1),
                    Vec3::new(p1.x, p1.y, 0.1),
                    Vec3::Z,
                    stone,
                );
                b.tri_out(
                    Vec3::new(0.0, 1.1, -0.1),
                    Vec3::new(p0.x, p0.y, -0.1),
                    Vec3::new(p1.x, p1.y, -0.1),
                    -Vec3::Z,
                    stone,
                );
                b.quad_out(
                    Vec3::new(p0.x, p0.y, -0.1),
                    Vec3::new(p1.x, p1.y, -0.1),
                    Vec3::new(p1.x, p1.y, 0.1),
                    Vec3::new(p0.x, p0.y, 0.1),
                    Vec3::new((p0.x + p1.x) * 0.5, p0.y - 1.1, 0.0),
                    stone,
                );
            }
        }
        1 => {
            b.cuboid(
                Vec3::new(-0.22, 0.18, -0.1),
                Vec3::new(0.22, 1.5, 0.1),
                stone,
            );
            b.cuboid(
                Vec3::new(-0.55, 0.95, -0.1),
                Vec3::new(0.55, 1.28, 0.1),
                stone,
            );
        }
        _ => {
            b.cuboid(
                Vec3::new(-0.38, 0.18, -0.16),
                Vec3::new(0.38, 1.4, 0.16),
                stone,
            );
            b.cuboid(
                Vec3::new(-0.32, 1.4, -0.12),
                Vec3::new(0.32, 1.52, 0.12),
                stone,
            );
            b.pyramid(Vec3::new(0.0, 1.52, 0.0), 0.28, 0.1, 0.35, stone);
        }
    }
    for i in 0..3 {
        let w = 0.28 - i as f32 * 0.06;
        b.cuboid(
            Vec3::new(-w, 0.55 + i as f32 * 0.17, 0.1),
            Vec3::new(w, 0.6 + i as f32 * 0.17, 0.115),
            dark,
        );
    }
    b.cuboid(
        Vec3::new(-0.45, 0.18, 0.0),
        Vec3::new(-0.2, 0.7, 0.12),
        moss,
    );
    b.build()
}

pub fn gravestone_collider(kind: u32) -> Vec3 {
    match kind {
        0 => Vec3::new(0.9, 1.5, 0.3),
        1 => Vec3::new(1.1, 1.5, 0.3),
        _ => Vec3::new(0.76, 1.9, 0.34),
    }
}

pub fn build_pumpkin(
    radius: f32,
    ribs: f32,
    c_dark: Rgba,
    c_light: Rgba,
    rings: u32,
    segs: u32,
) -> Mesh {
    let mut b = MeshBuilder::default();
    b.surface(rings, segs, |theta, phi| {
        let d = Vec3::new(
            theta.sin() * phi.cos(),
            theta.cos(),
            theta.sin() * phi.sin(),
        );
        let rib = (phi * ribs).cos();
        let rr = radius * (1.0 + 0.10 * rib * theta.sin());
        let mut y = d.y * rr * 0.82;
        if theta < 0.35 {
            y -= (0.35 - theta) / 0.35 * radius * 0.12;
        } else if theta > PI - 0.3 {
            y += (theta - (PI - 0.3)) / 0.3 * radius * 0.08;
        }
        let p = Vec3::new(d.x * rr, y, d.z * rr);
        (p, lerp_col(c_dark, c_light, rib * 0.5 + 0.5))
    });
    b.tube(
        Vec3::new(0.0, radius * 0.66, 0.0),
        Vec3::new(radius * 0.12, radius * 1.1, radius * 0.05),
        radius * 0.14,
        radius * 0.09,
        if segs > 16 { 8 } else { 5 },
        lin(0.20, 0.27, 0.08),
        lin(0.30, 0.22, 0.08),
    );
    b.build()
}

pub fn build_pumpkin_face(radius: f32) -> Mesh {
    let mut b = MeshBuilder::default();
    let col = lin(1.0, 1.0, 1.0);
    let r = radius;
    let proj = |y: f32, z: f32| -> Vec3 {
        let yy = y * r;
        let zz = z * r;
        let v = 1.0 - (yy / (0.82 * r)).powi(2) - (zz / r).powi(2);
        let x = -(r * v.max(0.02).sqrt()) * 1.0 - 0.012;
        Vec3::new(x, yy, zz)
    };
    let out = -Vec3::X;
    for sg in [-1.0_f32, 1.0] {
        b.tri_out(
            proj(0.2, sg * 0.18),
            proj(0.2, sg * 0.5),
            proj(0.55, sg * 0.34),
            out,
            col,
        );
    }
    b.tri_out(
        proj(0.05, 0.0),
        proj(0.18, -0.08),
        proj(0.18, 0.08),
        out,
        col,
    );
    let teeth = 6;
    let z0 = -0.55;
    let z1 = 0.55;
    for i in 0..teeth {
        let a = z0 + (z1 - z0) * i as f32 / teeth as f32;
        let c = z0 + (z1 - z0) * (i + 1) as f32 / teeth as f32;
        let (top, bot) = if i % 2 == 0 {
            (-0.15, -0.5)
        } else {
            (-0.15, -0.38)
        };
        b.quad_out(
            proj(top, a),
            proj(top, c),
            proj(bot, c),
            proj(bot, a),
            out,
            col,
        );
    }
    b.build()
}

pub fn build_bush(seed: f32, detail: bool) -> Mesh {
    let mut b = MeshBuilder::default();
    let (rings, segs) = if detail { (7, 12) } else { (4, 6) };
    let dark = lin(0.10, 0.12, 0.05);
    let light = lin(0.30, 0.28, 0.08);
    b.blob(
        Vec3::new(0.0, 0.55, 0.0),
        0.8,
        0.75,
        dark,
        light,
        rings,
        segs,
        seed,
    );
    if detail {
        b.blob(
            Vec3::new(0.6, 0.4, 0.2),
            0.5,
            0.8,
            dark,
            lerp_col(light, lin(0.5, 0.25, 0.05), 0.5),
            rings,
            segs,
            seed + 2.0,
        );
        b.blob(
            Vec3::new(-0.5, 0.4, -0.3),
            0.55,
            0.8,
            dark,
            light,
            rings,
            segs,
            seed + 4.0,
        );
    }
    b.build()
}

pub fn build_lamp_pole() -> Mesh {
    let mut b = MeshBuilder::default();
    let iron = lin(0.07, 0.07, 0.08);
    b.lathe(
        &[
            (0.28, 0.0),
            (0.24, 0.15),
            (0.14, 0.3),
            (0.1, 0.6),
            (0.07, 1.2),
            (0.06, 4.2),
        ],
        8,
        iron,
        iron,
    );
    b.tube(
        Vec3::new(0.0, 4.2, 0.0),
        Vec3::new(0.0, 4.5, 0.0),
        0.12,
        0.08,
        8,
        iron,
        iron,
    );
    b.cuboid(
        Vec3::new(-0.26, 4.5, -0.26),
        Vec3::new(0.26, 4.58, 0.26),
        iron,
    );
    b.pyramid(Vec3::new(0.0, 5.02, 0.0), 0.34, 0.34, 0.3, iron);
    for sx in [-1.0_f32, 1.0] {
        for sz in [-1.0_f32, 1.0] {
            b.cuboid(
                Vec3::new(sx * 0.25 - 0.025, 4.58, sz * 0.25 - 0.025),
                Vec3::new(sx * 0.25 + 0.025, 5.02, sz * 0.25 + 0.025),
                iron,
            );
        }
    }
    b.build()
}

pub fn build_lamp_glass() -> Mesh {
    let mut b = MeshBuilder::default();
    let c = lin(1.0, 1.0, 1.0);
    b.cuboid(
        Vec3::new(-0.22, 4.58, -0.22),
        Vec3::new(0.22, 5.02, 0.22),
        c,
    );
    b.build()
}

pub fn build_mailbox() -> Mesh {
    let mut b = MeshBuilder::default();
    let wood = lin(0.20, 0.13, 0.08);
    let metal = lin(0.10, 0.12, 0.16);
    b.cuboid(
        Vec3::new(-0.06, 0.0, -0.06),
        Vec3::new(0.06, 1.15, 0.06),
        wood,
    );
    b.cuboid(Vec3::new(-0.3, 1.15, -0.2), Vec3::new(0.3, 1.2, 0.2), wood);
    b.cuboid(
        Vec3::new(-0.3, 1.2, -0.17),
        Vec3::new(0.3, 1.5, 0.17),
        metal,
    );
    b.cuboid(
        Vec3::new(0.3, 1.25, 0.1),
        Vec3::new(0.34, 1.55, 0.14),
        lin(0.6, 0.1, 0.08),
    );
    b.build()
}

pub fn build_ghost() -> Mesh {
    let mut b = MeshBuilder::default();
    let top = lin(0.88, 0.9, 0.95);
    let bottom = lin(0.55, 0.58, 0.66);
    let profile = [
        (0.55, 0.0),
        (0.5, 0.25),
        (0.42, 0.7),
        (0.34, 1.1),
        (0.3, 1.4),
        (0.34, 1.62),
        (0.3, 1.82),
        (0.18, 1.95),
        (0.02, 2.0),
    ];
    b.lathe(&profile, 14, bottom, top);
    let dark = lin(0.02, 0.02, 0.03);
    for sg in [-1.0_f32, 1.0] {
        b.obox(
            Vec3::new(-0.3, 1.66, sg * 0.12),
            Vec3::Z,
            -Vec3::X,
            Vec3::new(0.05, 0.1, 0.02),
            dark,
        );
    }
    b.obox(
        Vec3::new(-0.3, 1.4, 0.0),
        Vec3::Z,
        -Vec3::X,
        Vec3::new(0.08, 0.1, 0.02),
        dark,
    );
    b.build()
}

pub fn build_hay(detail: bool) -> Mesh {
    let mut b = MeshBuilder::default();
    let hay = lin(0.62, 0.50, 0.18);
    b.cuboid(Vec3::new(-0.6, 0.0, -0.45), Vec3::new(0.6, 0.8, 0.45), hay);
    if detail {
        let mut jr = GameRng::seeded(77);
        for i in 0..14 {
            let x = -0.55 + i as f32 * 0.085;
            b.cuboid(
                Vec3::new(x, 0.02, -0.455),
                Vec3::new(x + 0.03, 0.78, 0.455),
                scale_col(hay, 0.8 + 0.2 * jr.f32()),
            );
        }
        for x in [-0.32_f32, 0.32] {
            b.cuboid(
                Vec3::new(x - 0.02, -0.01, -0.47),
                Vec3::new(x + 0.02, 0.81, 0.47),
                lin(0.25, 0.18, 0.08),
            );
        }
    }
    b.build()
}

pub fn build_leaf_pile(seed: f32) -> Mesh {
    let mut b = MeshBuilder::default();
    let a = lin(0.45, 0.15, 0.03);
    let c = lin(0.85, 0.45, 0.08);
    b.blob(Vec3::new(0.0, 0.0, 0.0), 0.9, 0.22, a, c, 5, 10, seed);
    b.build()
}

fn grow_branch(
    b: &mut MeshBuilder,
    rng: &mut GameRng,
    start: Vec3,
    dir: Vec3,
    len: f32,
    r: f32,
    depth: u32,
    sides: u32,
    bark: (Rgba, Rgba),
    tips: &mut Vec<Vec3>,
) {
    let end = start + dir * len;
    let r_end = r * 0.62;
    b.tube(start, end, r, r_end, sides, bark.0, bark.1);
    if depth == 0 {
        tips.push(end);
        return;
    }
    let forks = if depth > 1 { 3 } else { 2 };
    for _ in 0..forks {
        let jitter = Vec3::new(
            rng.range(-1.0, 1.0),
            rng.range(-0.2, 0.8),
            rng.range(-1.0, 1.0),
        );
        let new_dir = (dir + jitter * 0.9).normalize();
        let new_len = len * rng.range(0.62, 0.8);
        grow_branch(
            b,
            rng,
            end,
            new_dir,
            new_len,
            r_end,
            depth - 1,
            sides,
            bark,
            tips,
        );
    }
}

pub fn build_tree(seed: u64, detail: bool, leafy: bool) -> (Mesh, f32, f32) {
    let mut rng = GameRng::seeded(seed);
    let mut b = MeshBuilder::default();
    let mut tips: Vec<Vec3> = Vec::new();
    let bark_dark = lin(0.07, 0.055, 0.045);
    let bark_light = lin(0.17, 0.14, 0.11);
    let sides = if detail { 8 } else { 5 };
    let depth = if detail { 3 } else { 1 };

    let h = rng.range(7.0, 9.5);
    let r0 = rng.range(0.3, 0.42);
    let mid = Vec3::new(rng.range(-0.4, 0.4), h * 0.5, rng.range(-0.4, 0.4));
    let top = mid + Vec3::new(rng.range(-0.5, 0.5), h * 0.5, rng.range(-0.5, 0.5));
    b.tube(
        Vec3::new(0.0, -0.3, 0.0),
        mid,
        r0 * 1.3,
        r0 * 0.8,
        sides,
        bark_dark,
        bark_light,
    );
    b.tube(mid, top, r0 * 0.8, r0 * 0.3, sides, bark_light, bark_dark);
    let roots = if detail { 6 } else { 3 };
    for i in 0..roots {
        let a = i as f32 / roots as f32 * TAU + rng.range(-0.3, 0.3);
        let (s, co) = a.sin_cos();
        b.tube(
            Vec3::new(co * r0 * 0.7, 0.6, s * r0 * 0.7),
            Vec3::new(co * r0 * 2.4, -0.1, s * r0 * 2.4),
            r0 * 0.38,
            r0 * 0.12,
            5,
            bark_dark,
            bark_dark,
        );
    }
    let boughs = 5;
    for i in 0..boughs {
        let a = i as f32 / boughs as f32 * TAU + rng.range(-0.4, 0.4);
        let start = mid.lerp(top, rng.range(0.0, 0.9));
        let dir = Vec3::new(a.cos(), rng.range(0.4, 1.0), a.sin()).normalize();
        let len = rng.range(1.8, 2.5);
        grow_branch(
            &mut b,
            &mut rng,
            start,
            dir,
            len,
            r0 * 0.45,
            depth,
            sides.min(6),
            (bark_light, bark_dark),
            &mut tips,
        );
    }
    grow_branch(
        &mut b,
        &mut rng,
        top,
        Vec3::Y,
        1.8,
        r0 * 0.3,
        depth,
        sides.min(6),
        (bark_light, bark_dark),
        &mut tips,
    );
    if leafy {
        let (d, l) = LEAF_TONES[(seed % LEAF_TONES.len() as u64) as usize];
        let dark = lin(d[0], d[1], d[2]);
        let light = lin(l[0], l[1], l[2]);
        let (rings, segs) = if detail { (8, 13) } else { (4, 7) };
        let max_tips = if detail { 14 } else { 6 };
        let step = (tips.len() / max_tips).max(1);
        for (i, tip) in tips.iter().enumerate().step_by(step) {
            let radius = rng.range(1.2, 1.8);
            b.blob(
                *tip + Vec3::Y * radius * 0.3,
                radius,
                0.75,
                dark,
                light,
                rings,
                segs,
                i as f32 * 1.7 + (seed % 97) as f32,
            );
        }
        b.blob(
            top + Vec3::Y * 1.0,
            rng.range(1.6, 2.1),
            0.8,
            dark,
            light,
            rings,
            segs,
            (seed % 53) as f32,
        );
    }
    (b.build(), r0, 4.5)
}

pub fn build_car(color: [f32; 3], detail: bool) -> Mesh {
    let mut b = MeshBuilder::default();
    let body = lin(color[0], color[1], color[2]);
    let dark = lin(0.03, 0.035, 0.04);
    let glass = lin(0.07, 0.09, 0.11);
    let tire = lin(0.02, 0.02, 0.02);
    let chrome = lin(0.55, 0.55, 0.55);
    b.cuboid(
        Vec3::new(-2.2, 0.38, -0.92),
        Vec3::new(2.2, 1.0, 0.92),
        body,
    );
    b.cuboid(
        Vec3::new(-1.0, 1.0, -0.84),
        Vec3::new(1.15, 1.62, 0.84),
        scale_col(body, 0.95),
    );
    b.cuboid(
        Vec3::new(-2.25, 0.3, -0.95),
        Vec3::new(-2.15, 0.6, 0.95),
        chrome,
    );
    b.cuboid(
        Vec3::new(2.15, 0.3, -0.95),
        Vec3::new(2.25, 0.6, 0.95),
        chrome,
    );
    b.cuboid(
        Vec3::new(-1.0, 1.62, -0.8),
        Vec3::new(1.1, 1.67, 0.8),
        scale_col(body, 0.8),
    );
    for sz in [-1.0_f32, 1.0] {
        let za = sz * 0.83;
        let zb = sz * 0.87;
        b.cuboid(
            Vec3::new(-0.9, 1.05, za.min(zb)),
            Vec3::new(1.0, 1.55, za.max(zb)),
            glass,
        );
    }
    b.cuboid(
        Vec3::new(-1.03, 1.05, -0.78),
        Vec3::new(-0.98, 1.55, 0.78),
        glass,
    );
    b.cuboid(
        Vec3::new(1.12, 1.05, -0.78),
        Vec3::new(1.17, 1.55, 0.78),
        glass,
    );
    let sides = if detail { 12 } else { 6 };
    for sx in [-1.35_f32, 1.4] {
        for sz in [-0.88_f32, 0.88] {
            b.tube(
                Vec3::new(sx, 0.36, sz - 0.12),
                Vec3::new(sx, 0.36, sz + 0.12),
                0.36,
                0.36,
                sides,
                tire,
                tire,
            );
            if detail {
                b.tube(
                    Vec3::new(sx, 0.36, sz - 0.13 * sz.signum() + 0.0),
                    Vec3::new(sx, 0.36, sz + 0.125 * sz.signum()),
                    0.2,
                    0.2,
                    8,
                    chrome,
                    chrome,
                );
            }
        }
    }
    if detail {
        for sz in [-0.65_f32, 0.65] {
            b.cuboid(
                Vec3::new(-2.22, 0.65, sz - 0.18),
                Vec3::new(-2.16, 0.82, sz + 0.18),
                lin(0.95, 0.9, 0.6),
            );
            b.cuboid(
                Vec3::new(2.16, 0.65, sz - 0.18),
                Vec3::new(2.22, 0.82, sz + 0.18),
                lin(0.5, 0.05, 0.04),
            );
        }
        b.cuboid(
            Vec3::new(-2.2, 0.38, -0.92),
            Vec3::new(2.2, 0.48, 0.92),
            dark,
        );
    }
    b.build()
}
