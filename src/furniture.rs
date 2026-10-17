use bevy::prelude::*;

use crate::builder::{Axis, Builder};
use crate::house::UPPER;
use crate::palette::M;

const U: f32 = UPPER;

pub fn furnish(b: &mut Builder) {
    living(b);
    kitchen(b);
    hall(b);
    study(b);
    bathroom_ground(b);
    master_bedroom(b);
    kids_room(b);
    upper_hall(b);
    dressing_room(b);
    bathroom_upper(b);
}

fn living(b: &mut Builder) {
    b.gx((1.0, 4.2), (0.0, 0.015), (1.2, 4.8), M::RugRed);

    b.bx((1.8, 4.6), (0.0, 0.45), (2.45, 3.3), M::SofaGreen);
    b.bx((1.8, 4.6), (0.45, 0.95), (2.3, 2.55), M::SofaGreen);
    b.bx((1.8, 2.05), (0.45, 0.7), (2.55, 3.3), M::SofaGreen);
    b.bx((4.35, 4.6), (0.45, 0.7), (2.55, 3.3), M::SofaGreen);
    b.gx((2.2, 2.8), (0.45, 0.6), (2.6, 3.1), M::FabricRed);
    b.gx((3.6, 4.2), (0.45, 0.6), (2.6, 3.1), M::Blanket);

    b.table((2.7, 3.7), 0.0, (3.85, 4.55), 0.4, M::WoodDark);
    b.ball(Vec3::new(3.2, 0.5, 4.2), 0.1, M::Pumpkin);
    b.light(
        Vec3::new(3.2, 0.62, 4.2),
        Color::srgb(1.0, 0.45, 0.1),
        8_000.0,
        3.0,
        false,
    );

    b.bx((2.2, 4.4), (0.0, 0.5), (5.35, 5.9), M::WoodDark);
    b.bx((2.6, 3.9), (0.9, 1.55), (5.8, 5.9), M::Black);
    b.gx((2.9, 3.6), (0.5, 0.9), (5.7, 5.74), M::Metal);
    b.ball(Vec3::new(4.1, 0.62, 5.6), 0.12, M::Pumpkin);

    b.bx((0.5, 1.4), (0.0, 0.45), (3.9, 4.8), M::FabricRed);
    b.bx((0.5, 0.75), (0.45, 0.95), (3.9, 4.8), M::FabricRed);

    b.bx((0.15, 0.5), (0.0, 2.1), (4.95, 5.85), M::WoodDark);
    for (y0, y1) in [(0.3, 0.6), (0.85, 1.15), (1.4, 1.7)] {
        b.books(Axis::Z, (0.5, 0.54), 5.0, 5.8, y0, y1);
    }
    b.floor_lamp(1.0, 0.0, 5.5, 90_000.0, true);

    b.gx((6.36, 6.42), (1.3, 2.0), (0.9, 1.8), M::Trim);
    b.gx((6.35, 6.36), (1.35, 1.95), (0.95, 1.75), M::Blanket);
    b.gx((0.15, 0.2), (1.2, 1.9), (0.3, 1.1), M::Trim);
    b.gx((0.2, 0.21), (1.25, 1.85), (0.35, 1.05), M::Curtain);
}

fn kitchen(b: &mut Builder) {
    b.bx((11.25, 11.85), (0.0, 0.9), (0.4, 5.5), M::WoodDark);
    b.gx((11.2, 11.85), (0.9, 0.94), (0.35, 5.55), M::Stone);
    b.bx((8.3, 11.25), (0.0, 0.9), (5.3, 5.9), M::WoodDark);
    b.gx((8.3, 11.25), (0.9, 0.94), (5.25, 5.9), M::Stone);

    b.bx((6.65, 7.35), (0.0, 1.9), (4.5, 5.2), M::Ceramic);
    b.gx((7.35, 7.4), (0.9, 1.4), (4.6, 4.65), M::Metal);

    b.gx((10.0, 10.8), (0.94, 0.97), (5.4, 5.85), M::Black);
    b.gx((9.9, 10.9), (1.75, 2.3), (5.4, 5.9), M::Metal);
    b.gx((8.4, 9.7), (1.5, 2.2), (5.6, 5.9), M::WoodDark);
    b.gx((9.0, 9.7), (0.94, 0.97), (5.45, 5.8), M::Metal);
    b.post(9.35, 5.75, 0.94, 1.2, 0.015, M::Metal, false);

    b.table((8.0, 9.8), 0.0, (2.2, 3.4), 0.75, M::WoodLight);
    b.chair(8.1, 0.0, 1.7, 0, M::FabricGrey);
    b.chair(9.1, 0.0, 1.7, 0, M::FabricGrey);
    b.chair(8.1, 0.0, 3.45, 1, M::FabricGrey);
    b.chair(9.1, 0.0, 3.45, 1, M::FabricGrey);
    b.chair(9.9, 0.0, 2.6, 3, M::FabricGrey);

    b.post(8.9, 2.8, 1.95, 2.8, 0.008, M::Black, false);
    b.ball(Vec3::new(8.9, 1.9, 2.8), 0.12, M::Bulb);
    b.light(
        Vec3::new(8.9, 1.85, 2.8),
        Color::srgb(1.0, 0.66, 0.35),
        55_000.0,
        7.0,
        false,
    );
    b.gx((8.5, 9.3), (0.75, 0.77), (2.6, 3.0), M::RugBrown);
}

fn hall(b: &mut Builder) {
    b.gx((5.6, 8.2), (0.0, 0.015), (6.8, 8.0), M::RugBrown);
    b.bx((4.58, 4.95), (0.0, 0.85), (7.8, 8.4), M::WoodDark);
    b.gx((4.58, 4.62), (1.2, 2.0), (7.8, 8.4), M::Mirror);
    b.table_lamp(4.76, 0.85, 8.1, 14_000.0, false);

    b.bx((8.45, 8.9), (0.0, 2.1), (6.15, 6.65), M::WoodDark);
    b.gx((8.42, 8.45), (1.55, 1.8), (6.3, 6.5), M::Ceramic);
    b.gx((8.42, 8.45), (0.4, 1.3), (6.3, 6.5), M::Trim);

    b.post(6.8, 7.4, 2.5, 2.8, 0.008, M::Black, false);
    b.ball(Vec3::new(6.8, 2.42, 7.4), 0.1, M::Bulb);
    let e = b.light(
        Vec3::new(6.8, 2.35, 7.4),
        Color::srgb(1.0, 0.78, 0.45),
        55_000.0,
        8.0,
        false,
    );
    b.flicker(e, 55_000.0, 0.4);
}

fn study(b: &mut Builder) {
    b.gx((1.4, 3.6), (0.0, 0.015), (7.0, 8.8), M::RugBrown);
    b.table((0.6, 2.4), 0.0, (9.0, 9.8), 0.76, M::WoodDark);
    b.chair(1.2, 0.0, 8.4, 0, M::FabricRed);
    b.table_lamp(2.1, 0.76, 9.5, 35_000.0, false);
    b.gx((0.9, 1.5), (0.76, 0.78), (9.2, 9.6), M::WoodLight);
    b.gx((1.55, 1.85), (0.76, 0.86), (9.3, 9.6), M::Book1);

    b.bx((0.15, 0.5), (0.0, 2.2), (6.2, 7.2), M::WoodDark);
    for (y0, y1) in [(0.3, 0.6), (0.85, 1.15), (1.4, 1.7), (1.9, 2.1)] {
        b.books(Axis::Z, (0.5, 0.54), 6.25, 7.15, y0, y1);
    }
    b.bx((3.95, 4.42), (0.0, 2.2), (8.1, 9.8), M::WoodDark);
    for (y0, y1) in [(0.3, 0.6), (0.85, 1.15), (1.4, 1.7), (1.9, 2.1)] {
        b.books(Axis::Z, (3.91, 3.95), 8.15, 9.75, y0, y1);
    }

    b.bx((2.7, 3.6), (0.0, 0.45), (6.5, 7.4), M::FabricRed);
    b.bx((2.7, 3.6), (0.45, 0.95), (7.2, 7.4), M::FabricRed);
    b.floor_lamp(3.9, 0.0, 6.4, 40_000.0, false);
}

fn bathroom_ground(b: &mut Builder) {
    b.bx((10.2, 11.85), (0.0, 0.55), (9.05, 9.85), M::Ceramic);
    b.gx((10.25, 10.35), (0.55, 0.75), (9.4, 9.5), M::Metal);
    b.bx((11.35, 11.85), (0.0, 0.4), (6.5, 7.0), M::Ceramic);
    b.bx((11.65, 11.85), (0.4, 0.85), (6.5, 7.0), M::Ceramic);
    b.bx((11.45, 11.85), (0.0, 0.85), (7.9, 8.7), M::WoodLight);
    b.gx((11.5, 11.8), (0.85, 0.88), (8.05, 8.55), M::Ceramic);
    b.gx((11.82, 11.85), (1.1, 1.9), (7.9, 8.7), M::Mirror);
    b.gx((9.6, 10.6), (0.0, 0.015), (7.0, 8.0), M::FabricGrey);
}

fn master_bedroom(b: &mut Builder) {
    b.gx((0.9, 3.6), (U, U + 0.015), (1.4, 4.0), M::RugRed);

    b.bx((0.15, 2.25), (U, U + 0.35), (1.8, 3.4), M::WoodDark);
    b.bx((0.3, 2.2), (U + 0.35, U + 0.6), (1.85, 3.35), M::Mattress);
    b.gx((1.0, 2.2), (U + 0.6, U + 0.65), (1.85, 3.35), M::Blanket);
    b.gx((0.35, 0.8), (U + 0.6, U + 0.72), (1.95, 2.55), M::Mattress);
    b.gx((0.35, 0.8), (U + 0.6, U + 0.72), (2.65, 3.25), M::Mattress);
    b.bx((0.15, 0.3), (U, U + 1.1), (1.75, 3.45), M::WoodDark);
    b.gx((0.15, 0.2), (U + 1.3, U + 2.0), (1.9, 3.3), M::Trim);
    b.gx((0.2, 0.21), (U + 1.35, U + 1.95), (1.95, 3.25), M::Curtain);

    b.bx((0.15, 0.6), (U, U + 0.55), (1.2, 1.7), M::WoodDark);
    b.bx((0.15, 0.6), (U, U + 0.55), (3.5, 4.0), M::WoodDark);
    b.table_lamp(0.38, U + 0.55, 3.75, 50_000.0, true);

    b.bx((2.8, 4.8), (U, U + 2.2), (5.3, 5.9), M::WoodDark);
    b.gx((3.78, 3.82), (U + 0.1, U + 2.1), (5.28, 5.3), M::Trim);
    b.gx((3.7, 3.74), (U + 1.0, U + 1.2), (5.26, 5.28), M::Metal);
    b.gx((3.86, 3.9), (U + 1.0, U + 1.2), (5.26, 5.28), M::Metal);

    b.bx((5.95, 6.42), (U, U + 0.9), (0.8, 2.0), M::WoodLight);
    b.gx((6.4, 6.42), (U + 1.1, U + 1.9), (0.9, 1.9), M::Mirror);
}

fn kids_room(b: &mut Builder) {
    b.bx((10.85, 11.85), (U, U + 0.3), (0.15, 2.35), M::WoodLight);
    b.bx((10.9, 11.8), (U + 0.3, U + 0.5), (0.2, 2.3), M::Mattress);
    b.gx((10.9, 11.8), (U + 0.5, U + 0.54), (0.9, 2.3), M::Blanket);
    b.gx((11.1, 11.6), (U + 0.5, U + 0.6), (0.25, 0.7), M::Mattress);

    b.table((7.4, 9.0), U, (0.3, 0.9), 0.75, M::WoodLight);
    b.chair(8.0, U, 0.95, 1, M::FabricGrey);
    b.gx((7.7, 8.2), (U + 0.75, U + 0.77), (0.4, 0.8), M::Book2);

    b.bx((6.58, 6.95), (U, U + 1.6), (2.2, 4.2), M::WoodDark);
    for (y0, y1) in [(0.3, 0.6), (0.7, 1.0), (1.1, 1.4)] {
        b.books(Axis::Z, (6.95, 6.99), 2.25, 4.15, U + y0, U + y1);
    }
    b.bx((8.6, 10.4), (U, U + 2.1), (5.35, 5.9), M::WoodDark);
    b.gx((9.48, 9.52), (U + 0.1, U + 2.0), (5.33, 5.35), M::Trim);
    b.bx((9.0, 9.8), (U, U + 0.4), (3.6, 4.2), M::WoodLight);
    b.ball(Vec3::new(8.2, U + 0.1, 3.5), 0.1, M::FabricRed);
    b.gx((7.2, 8.4), (U, U + 0.015), (2.6, 4.2), M::RugBrown);
    b.light(
        Vec3::new(11.6, U + 0.9, 2.7),
        Color::srgb(0.5, 0.6, 1.0),
        7_000.0,
        3.5,
        false,
    );
}

fn upper_hall(b: &mut Builder) {
    b.gx((5.6, 8.4), (U, U + 0.015), (6.4, 8.0), M::RugBrown);
    b.bx((6.3, 6.9), (U, U + 0.8), (6.12, 6.45), M::WoodDark);
    b.gx((6.5, 6.7), (U + 0.8, U + 1.0), (6.2, 6.4), M::RedBulb);
    b.light(
        Vec3::new(6.6, U + 1.1, 6.3),
        Color::srgb(1.0, 0.25, 0.15),
        20_000.0,
        6.0,
        false,
    );
    b.gx((8.4, 8.9), (U + 1.2, U + 2.0), (6.12, 6.16), M::Trim);
    b.gx((8.45, 8.85), (U + 1.25, U + 1.95), (6.16, 6.17), M::Blanket);
}

fn dressing_room(b: &mut Builder) {
    b.bx((0.15, 0.7), (U, U + 2.3), (6.3, 9.7), M::WoodDark);
    for z in [7.1, 8.0, 8.9] {
        b.gx((0.7, 0.72), (U + 0.1, U + 2.2), (z, z + 0.03), M::Trim);
    }
    b.bx((3.4, 4.2), (U, U + 0.5), (8.8, 9.7), M::WoodLight);
    b.bx((3.5, 4.0), (U + 0.5, U + 0.9), (9.1, 9.6), M::WoodDark);

    b.post(2.5, 9.0, U, U + 1.1, 0.015, M::Metal, false);
    b.post(2.5, 9.0, U, U + 0.03, 0.2, M::Metal, false);
    b.ellipsoid(
        Vec3::new(2.5, U + 1.35, 9.0),
        Vec3::new(0.36, 0.6, 0.22),
        M::FabricGrey,
    );
    b.ball(Vec3::new(2.5, U + 1.78, 9.0), 0.11, M::Mattress);
    b.collider((2.3, 2.7), (U, U + 1.9), (8.8, 9.2));

    b.gx((4.38, 4.42), (U + 0.3, U + 2.0), (8.0, 8.8), M::Mirror);
}

fn bathroom_upper(b: &mut Builder) {
    b.bx((10.65, 11.85), (U, U + 0.1), (8.4, 9.85), M::Ceramic);
    b.gx((10.6, 10.65), (U + 0.1, U + 2.0), (8.4, 9.85), M::Glass);
    b.gx((10.6, 11.85), (U + 0.1, U + 2.0), (8.38, 8.42), M::Glass);
    b.collider((10.6, 10.65), (U, U + 2.0), (8.4, 9.85));
    b.collider((10.6, 11.85), (U, U + 2.0), (8.38, 8.42));
    b.gx((11.7, 11.8), (U + 1.6, U + 1.7), (9.7, 9.8), M::Metal);

    b.bx((11.35, 11.85), (U, U + 0.4), (6.5, 7.0), M::Ceramic);
    b.bx((11.65, 11.85), (U + 0.4, U + 0.85), (6.5, 7.0), M::Ceramic);
    b.bx((11.45, 11.85), (U, U + 0.85), (7.4, 8.2), M::WoodLight);
    b.gx((11.5, 11.8), (U + 0.85, U + 0.88), (7.55, 8.05), M::Ceramic);
    b.gx((11.82, 11.85), (U + 1.1, U + 1.9), (7.4, 8.2), M::Mirror);
}
