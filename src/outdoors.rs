use std::f32::consts::TAU;

use bevy::prelude::*;

use crate::builder::Builder;
use crate::palette::M;

struct Rng(u32);

impl Rng {
    fn unit(&mut self) -> f32 {
        self.0 = self.0.wrapping_mul(1664525).wrapping_add(1013904223);
        (self.0 >> 8) as f32 / 16_777_216.0
    }

    fn range(&mut self, a: f32, b: f32) -> f32 {
        a + (b - a) * self.unit()
    }
}

pub fn build_outdoors(b: &mut Builder) {
    let mut rng = Rng(0x5eed_1234);

    b.gx((-70.0, 70.0), (-1.0, -0.05), (-70.0, 70.0), M::Grass);
    b.collider((-70.0, 70.0), (-3.0, -0.05), (-70.0, 70.0));

    b.gx((-70.0, 70.0), (-0.1, -0.03), (-15.2, -8.8), M::Asphalt);
    b.gx((-70.0, 70.0), (-0.1, -0.01), (-8.8, -7.0), M::Sidewalk);
    b.gx((-70.0, 70.0), (-0.1, -0.01), (-17.0, -15.2), M::Sidewalk);
    b.gx((-70.0, 70.0), (-0.1, 0.06), (-8.85, -8.75), M::Stone);
    b.gx((-70.0, 70.0), (-0.1, 0.06), (-15.25, -15.15), M::Stone);
    b.gx((4.2, 5.8), (-0.1, -0.04), (-7.0, -2.3), M::Sidewalk);

    for i in -12..12 {
        let x = i as f32 * 3.0;
        b.gx((x, x + 1.5), (-0.03, -0.022), (-12.05, -11.95), M::RoadLine);
    }

    street_lamp(b, 11.0, -7.7, true, 180_000.0);
    street_lamp(b, -9.0, -7.7, false, 0.0);
    street_lamp(b, 2.0, -16.1, false, 0.0);

    maple(b, &mut rng, -3.5, -3.8);
    maple(b, &mut rng, 14.5, -2.6);
    dead_tree(b, &mut rng, 16.5, -5.2, 5.0);
    dead_tree(b, &mut rng, -7.5, -5.5, 4.2);
    dead_tree(b, &mut rng, -6.0, -19.5, 5.5);
    dead_tree(b, &mut rng, 12.0, -20.0, 6.0);

    car(b, -3.0, -10.2);

    neighbor(b, -2.0, -21.0, 8.0, 7.0, 6.0, Some(1.0));
    neighbor(b, 10.0, -22.0, 9.0, 7.0, 5.5, None);
    neighbor(b, -16.0, -23.0, 8.0, 7.0, 6.0, None);
    neighbor(b, 24.0, -21.5, 8.0, 7.0, 6.0, None);

    b.bx((8.45, 8.55), (-0.05, 1.1), (-6.55, -6.45), M::Metal);
    b.gx((8.2, 8.8), (1.1, 1.4), (-6.7, -6.3), M::Metal);

    scatter_leaves(b, &mut rng);
}

fn street_lamp(b: &mut Builder, x: f32, z: f32, lit: bool, intensity: f32) {
    b.post(x, z, -0.05, 5.0, 0.07, M::Metal, false);
    b.gx((x - 0.04, x + 0.04), (4.9, 5.0), (z - 1.3, z), M::Metal);
    let bulb = if lit { M::Bulb } else { M::Metal };
    b.gx((x - 0.18, x + 0.18), (4.72, 4.88), (z - 1.5, z - 1.1), bulb);
    if lit {
        let e = b.light(
            Vec3::new(x, 4.6, z - 1.3),
            Color::srgb(1.0, 0.72, 0.4),
            intensity,
            18.0,
            false,
        );
        b.flicker(e, intensity, 2.1);
    }
}

fn maple(b: &mut Builder, rng: &mut Rng, x: f32, z: f32) {
    b.post(x, z, -0.05, 3.4, 0.22, M::Bark, false);
    for _ in 0..7 {
        let dx = rng.range(-1.6, 1.6);
        let dz = rng.range(-1.6, 1.6);
        let y = rng.range(3.8, 5.4);
        let s = rng.range(1.5, 2.3);
        b.ellipsoid(
            Vec3::new(x + dx, y, z + dz),
            Vec3::new(s, s * 0.8, s),
            M::Foliage,
        );
    }
}

fn dead_tree(b: &mut Builder, rng: &mut Rng, x: f32, z: f32, h: f32) {
    b.post(x, z, -0.05, h, 0.18, M::Bark, false);
    for i in 0..7 {
        let t = i as f32 / 6.0;
        let y = h * (0.45 + 0.5 * t);
        let yaw = rng.range(0.0, TAU);
        let tilt = rng.range(0.5, 1.0);
        let len = rng.range(1.0, 2.0) * (1.2 - 0.5 * t);
        let rot = Quat::from_rotation_y(yaw) * Quat::from_rotation_z(tilt);
        let dir = rot * Vec3::X;
        let base = Vec3::new(x, y, z);
        b.tilted(
            base + dir * (len * 0.5),
            Vec3::new(len, 0.08, 0.08),
            rot,
            M::Bark,
        );
        let rot2 = Quat::from_rotation_y(yaw + 0.8) * Quat::from_rotation_z(tilt + 0.5);
        let dir2 = rot2 * Vec3::X;
        let tip = base + dir * len;
        b.tilted(
            tip + dir2 * (len * 0.25),
            Vec3::new(len * 0.5, 0.05, 0.05),
            rot2,
            M::Bark,
        );
    }
}

fn car(b: &mut Builder, x: f32, z: f32) {
    b.gx(
        (x - 2.2, x + 2.2),
        (0.3, 0.9),
        (z - 0.9, z + 0.9),
        M::CarBody,
    );
    b.gx(
        (x - 1.2, x + 1.0),
        (0.9, 1.4),
        (z - 0.8, z + 0.8),
        M::CarBody,
    );
    b.gx(
        (x - 1.1, x + 0.9),
        (0.95, 1.35),
        (z - 0.82, z + 0.82),
        M::Black,
    );
    for (wx, wz) in [(-1.4, -0.9), (1.4, -0.9), (-1.4, 0.9), (1.4, 0.9)] {
        b.gx(
            (x + wx - 0.3, x + wx + 0.3),
            (0.0, 0.6),
            (z + wz - 0.1, z + wz + 0.1),
            M::Black,
        );
    }
}

fn neighbor(b: &mut Builder, x: f32, z_front: f32, w: f32, d: f32, h: f32, lit: Option<f32>) {
    b.gx((x, x + w), (-0.05, h), (z_front - d, z_front), M::WallExt);
    b.gx(
        (x - 0.3, x + w + 0.3),
        (h, h + 0.3),
        (z_front - d - 0.3, z_front + 0.3),
        M::Roof,
    );
    let mut wx = x + 1.0;
    while wx + 1.2 < x + w - 0.5 {
        b.gx(
            (wx, wx + 1.2),
            (1.0, 2.0),
            (z_front, z_front + 0.03),
            M::Black,
        );
        b.gx(
            (wx, wx + 1.2),
            (3.6, 4.6),
            (z_front, z_front + 0.03),
            M::Black,
        );
        wx += 2.4;
    }
    if let Some(off) = lit {
        b.gx(
            (x + 1.0 + off, x + 2.2 + off),
            (3.6, 4.6),
            (z_front + 0.03, z_front + 0.05),
            M::LitWindow,
        );
    }
}

fn scatter_leaves(b: &mut Builder, rng: &mut Rng) {
    for _ in 0..180 {
        let x = rng.range(-12.0, 24.0);
        let z = rng.range(-16.0, -0.4);
        if (3.6..6.4).contains(&x) && z > -2.4 {
            continue;
        }
        if z > -2.3 && z < 0.0 && x > -0.3 && x < 12.3 {
            continue;
        }
        let surface = if z > -7.0 {
            -0.05
        } else if z > -8.8 {
            -0.01
        } else if z > -15.2 {
            -0.03
        } else {
            -0.01
        };
        let m = match (rng.unit() * 3.0) as u32 {
            0 => M::Leaf1,
            1 => M::Leaf2,
            _ => M::Leaf3,
        };
        let rot = Quat::from_rotation_y(rng.range(0.0, TAU))
            * Quat::from_rotation_x(rng.range(-0.15, 0.15));
        b.tilted(
            Vec3::new(x, surface + 0.008, z),
            Vec3::new(0.2, 0.012, 0.14),
            rot,
            m,
        );
    }
}
