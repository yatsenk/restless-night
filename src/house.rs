use std::f32::consts::{PI, TAU};

use bevy::prelude::*;

use crate::builder::{Axis, Builder, Mat, Opening, WALL_T};
use crate::collision::Solid;

const STORY: f32 = 3.0;
const SLAB: f32 = 0.2;
const WALL_H: f32 = STORY - SLAB;
const WIDTH: f32 = 14.0;
const DEPTH: f32 = 10.0;

#[derive(Resource)]
pub struct Solids(pub Vec<Solid>);

pub struct HousePlugin;

impl Plugin for HousePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, build_world);
    }
}

fn build_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut b = Builder::new(&mut commands, &mut meshes, &mut materials);
    outdoors(&mut b);
    ground_slab(&mut b);
    upper_slab(&mut b);
    for level in 0..2 {
        storey(&mut b, level);
    }
    stairs(&mut b);
    roof(&mut b);
    furniture(&mut b);
    let solids = b.finish();
    commands.insert_resource(Solids(solids));
}

fn ground_slab(b: &mut Builder) {
    b.block(
        Vec3::new(-0.1, -SLAB, -0.1),
        Vec3::new(WIDTH + 0.1, 0.0, DEPTH + 0.1),
        Mat::Wood,
    );
}

fn upper_slab(b: &mut Builder) {
    let rects: [([f32; 2], [f32; 2]); 4] = [
        ([-0.1, WIDTH + 0.1], [1.4, DEPTH + 0.1]),
        ([-0.1, 1.0], [-0.1, 1.4]),
        ([5.2, WIDTH + 0.1], [-0.1, 1.4]),
        ([1.0, 5.2], [-0.1, 0.1]),
    ];
    for (x, z) in rects {
        b.visual(
            Vec3::new(x[0], STORY - SLAB, z[0]),
            Vec3::new(x[1], STORY - SLAB * 0.5, z[1]),
            Mat::Ceiling,
        );
        b.visual(
            Vec3::new(x[0], STORY - SLAB * 0.5, z[0]),
            Vec3::new(x[1], STORY, z[1]),
            Mat::Wood,
        );
        b.solid_only(
            Vec3::new(x[0], STORY - SLAB, z[0]),
            Vec3::new(x[1], STORY, z[1]),
        );
    }
}

fn storey(b: &mut Builder, level: usize) {
    let y0 = level as f32 * STORY;
    let y1 = y0 + WALL_H;
    let t = WALL_T * 0.5;

    let mut front = vec![
        Opening::window(1.0, 2.8),
        Opening::window(3.4, 5.2),
        Opening::window(9.0, 11.0),
        Opening::window(11.4, 13.2),
    ];
    front.push(if level == 0 {
        Opening::sealed(6.0, 7.2)
    } else {
        Opening::window(6.0, 7.2)
    });
    b.wall(Axis::X, DEPTH, -t, WIDTH + t, y0, y1, &front, Mat::Plaster);

    let mut back = vec![Opening::high(7.4, 8.6), Opening::window(11.0, 13.0)];
    if level == 1 {
        back.push(Opening::window(2.2, 3.8));
    }
    b.wall(Axis::X, 0.0, -t, WIDTH + t, y0, y1, &back, Mat::Plaster);

    let left = [Opening::window(6.0, 8.0), Opening::window(1.8, 3.0)];
    b.wall(Axis::Z, 0.0, -t, DEPTH + t, y0, y1, &left, Mat::Plaster);

    let right = [Opening::window(6.0, 8.0), Opening::window(1.3, 2.7)];
    b.wall(Axis::Z, WIDTH, -t, DEPTH + t, y0, y1, &right, Mat::Plaster);

    let middle = [
        Opening::door(3.0, 4.0),
        Opening::door(8.4, 9.4),
        Opening::door(11.4, 12.4),
    ];
    b.wall(
        Axis::X,
        4.0,
        0.1,
        WIDTH - 0.1,
        y0,
        y1,
        &middle,
        Mat::Plaster,
    );

    let between = [Opening::door(6.0, 7.4)];
    b.wall(
        Axis::Z,
        8.0,
        4.0,
        DEPTH - 0.1,
        y0,
        y1,
        &between,
        Mat::Plaster,
    );

    b.wall(Axis::Z, 6.0, 0.1, 4.0, y0, y1, &[], Mat::Plaster);
    b.wall(Axis::Z, 10.0, 0.1, 4.0, y0, y1, &[], Mat::Plaster);
}

fn stairs(b: &mut Builder) {
    for i in 0..15 {
        let x0 = 1.0 + i as f32 * 0.28;
        let top = (i + 1) as f32 * 0.2;
        b.block(
            Vec3::new(x0, 0.0, 0.1),
            Vec3::new(x0 + 0.28, top, 1.4),
            Mat::DarkWood,
        );
    }
    b.block(
        Vec3::new(1.0, STORY, 1.33),
        Vec3::new(5.2, STORY + 0.95, 1.4),
        Mat::Metal,
    );
    b.block(
        Vec3::new(0.98, STORY, 0.1),
        Vec3::new(1.05, STORY + 0.95, 1.4),
        Mat::Metal,
    );
}

fn roof(b: &mut Builder) {
    let top = 2.0 * STORY;
    b.block(
        Vec3::new(-0.4, top - SLAB, -0.4),
        Vec3::new(WIDTH + 0.4, top, DEPTH + 0.4),
        Mat::Ceiling,
    );
    b.visual(
        Vec3::new(-0.4, top, -0.4),
        Vec3::new(WIDTH + 0.4, top + 0.06, DEPTH + 0.4),
        Mat::Roof,
    );
    let rim = [
        (
            Vec3::new(-0.4, top + 0.06, DEPTH + 0.2),
            Vec3::new(WIDTH + 0.4, top + 0.5, DEPTH + 0.4),
        ),
        (
            Vec3::new(-0.4, top + 0.06, -0.4),
            Vec3::new(WIDTH + 0.4, top + 0.5, -0.2),
        ),
        (
            Vec3::new(-0.4, top + 0.06, -0.2),
            Vec3::new(-0.2, top + 0.5, DEPTH + 0.2),
        ),
        (
            Vec3::new(WIDTH + 0.2, top + 0.06, -0.2),
            Vec3::new(WIDTH + 0.4, top + 0.5, DEPTH + 0.2),
        ),
    ];
    for (min, max) in rim {
        b.visual(min, max, Mat::Roof);
    }
}

fn furniture(b: &mut Builder) {
    let g = 0.0;
    b.item(g, [4.6, 7.0], [4.3, 5.2], 0.45, Mat::Fabric);
    b.item(g, [4.6, 7.0], [4.3, 4.5], 0.9, Mat::Fabric);
    b.item(g, [5.0, 6.6], [6.0, 6.9], 0.4, Mat::DarkWood);
    b.visual(
        Vec3::new(4.2, 0.0, 5.6),
        Vec3::new(7.4, 0.012, 7.3),
        Mat::Rug,
    );
    b.item(g, [0.1, 0.5], [4.2, 5.8], 2.0, Mat::DarkWood);
    b.item(g, [1.2, 2.1], [8.2, 9.1], 0.5, Mat::Fabric);

    b.item(g, [8.4, 13.8], [9.3, 9.9], 0.9, Mat::DarkWood);
    b.item(g, [10.0, 12.2], [6.3, 7.5], 0.75, Mat::DarkWood);
    b.item(g, [9.4, 9.8], [6.3, 6.7], 0.45, Mat::DarkWood);
    b.item(g, [9.4, 9.8], [7.1, 7.5], 0.45, Mat::DarkWood);
    b.item(g, [12.4, 12.8], [6.3, 6.7], 0.45, Mat::DarkWood);
    b.item(g, [12.4, 12.8], [7.1, 7.5], 0.45, Mat::DarkWood);
    b.item(g, [13.1, 13.9], [4.2, 5.0], 1.9, Mat::Metal);

    b.item(g, [0.1, 0.5], [2.4, 3.7], 0.8, Mat::DarkWood);

    b.item(g, [6.1, 8.0], [0.1, 0.85], 0.55, Mat::Ceramic);
    b.item(g, [9.0, 9.7], [0.1, 0.55], 0.85, Mat::Ceramic);
    b.item(g, [9.3, 9.8], [1.3, 1.8], 0.45, Mat::Ceramic);

    b.item(g, [13.5, 13.9], [2.8, 3.9], 1.8, Mat::DarkWood);
    b.item(g, [10.3, 11.1], [0.3, 1.1], 0.6, Mat::DarkWood);
    b.item(g, [10.3, 10.9], [1.2, 1.8], 0.4, Mat::DarkWood);

    let u = STORY;
    b.item(u, [0.1, 2.3], [6.7, 8.3], 0.5, Mat::Fabric);
    b.item(u, [0.1, 0.25], [6.7, 8.3], 1.1, Mat::DarkWood);
    b.item(u, [0.1, 0.6], [8.4, 8.9], 0.5, Mat::DarkWood);
    b.item(u, [6.8, 7.9], [4.2, 5.8], 2.1, Mat::DarkWood);
    b.visual(
        Vec3::new(2.6, u, 6.2),
        Vec3::new(5.6, u + 0.012, 8.8),
        Mat::Rug,
    );

    b.item(u, [12.4, 13.9], [8.6, 9.9], 0.75, Mat::DarkWood);
    b.item(u, [12.4, 12.9], [7.8, 8.3], 0.45, Mat::Fabric);
    b.item(u, [13.5, 13.9], [4.3, 6.0], 2.0, Mat::DarkWood);

    b.item(u, [0.1, 0.5], [2.4, 3.7], 0.8, Mat::DarkWood);

    b.item(u, [9.0, 9.7], [0.1, 0.55], 0.85, Mat::Ceramic);
    b.item(u, [9.3, 9.8], [1.3, 1.8], 0.45, Mat::Ceramic);
    b.item(u, [6.1, 7.1], [0.1, 1.1], 0.1, Mat::Ceramic);

    b.item(u, [10.3, 11.1], [0.3, 1.1], 0.6, Mat::DarkWood);
    b.item(u, [12.6, 13.4], [2.6, 3.4], 0.5, Mat::DarkWood);
}

struct Rng(u32);

impl Rng {
    fn next(&mut self) -> f32 {
        self.0 = self.0.wrapping_mul(1664525).wrapping_add(1013904223);
        (self.0 >> 8) as f32 / 16_777_216.0
    }

    fn range(&mut self, a: f32, b: f32) -> f32 {
        a + (b - a) * self.next()
    }
}

fn outdoors(b: &mut Builder) {
    b.block(
        Vec3::new(-120.0, -1.0, -120.0),
        Vec3::new(120.0, -0.02, 120.0),
        Mat::Soil,
    );
    b.block(
        Vec3::new(-120.0, -0.5, 20.0),
        Vec3::new(120.0, 0.0, 28.0),
        Mat::Asphalt,
    );
    b.block(
        Vec3::new(-120.0, -0.5, 17.2),
        Vec3::new(120.0, 0.05, 20.0),
        Mat::Pavement,
    );
    b.block(
        Vec3::new(-120.0, -0.5, 28.0),
        Vec3::new(120.0, 0.05, 30.8),
        Mat::Pavement,
    );
    b.block(
        Vec3::new(5.8, -0.5, DEPTH + 0.1),
        Vec3::new(7.4, 0.01, 17.2),
        Mat::Pavement,
    );

    for x in [12.0, -22.0, 46.0] {
        b.block(
            Vec3::new(x - 0.06, 0.0, 18.14),
            Vec3::new(x + 0.06, 4.8, 18.26),
            Mat::Metal,
        );
        b.visual(
            Vec3::new(x - 0.4, 4.75, 17.9),
            Vec3::new(x + 0.4, 4.85, 18.5),
            Mat::Metal,
        );
    }

    let mut rng = Rng(0x5eed_1234);

    let trees = [
        (-3.0, 14.0),
        (18.0, 13.0),
        (-13.0, 3.0),
        (25.0, -4.0),
        (5.0, -11.0),
        (-7.0, 33.0),
        (30.0, 34.0),
        (16.0, -16.0),
        (-20.0, 14.0),
        (36.0, 12.0),
    ];
    for (x, z) in trees {
        let yaw = rng.range(0.0, TAU);
        b.oriented(
            Vec3::new(x, 3.0, z),
            Vec3::new(0.45, 6.0, 0.45),
            Quat::from_rotation_y(yaw),
            Mat::Bark,
        );
        for i in 0..5 {
            let len = 2.8 - i as f32 * 0.35;
            let tilt = rng.range(0.7, 1.1);
            let rot = Quat::from_rotation_y(yaw + i as f32 * 1.256 + rng.range(-0.3, 0.3))
                * Quat::from_rotation_z(-tilt);
            let origin = Vec3::new(x, 3.2 + i as f32 * 0.65, z);
            b.oriented(
                origin + rot * Vec3::new(0.0, len * 0.5, 0.0),
                Vec3::new(0.14, len, 0.14),
                rot,
                Mat::Bark,
            );
        }
    }

    for _ in 0..260 {
        let x = rng.range(-38.0, 52.0);
        let z = rng.range(-30.0, 48.0);
        let in_house = (-1.0..15.0).contains(&x) && (-1.0..11.0).contains(&z);
        let in_street = (17.0..31.0).contains(&z);
        if in_house || in_street {
            continue;
        }
        let w = rng.range(0.3, 0.7);
        let d = rng.range(0.2, 0.5);
        let kind = (rng.next() * 3.0) as u8;
        b.oriented(
            Vec3::new(x, -0.012, z),
            Vec3::new(w, 0.012, d),
            Quat::from_rotation_y(rng.range(0.0, PI)),
            Mat::Leaf(kind),
        );
    }
}
