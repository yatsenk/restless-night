use bevy::prelude::*;

use crate::builder::{Axis, Builder, Opening, op};
use crate::palette::M;

pub const UPPER: f32 = 3.0;
pub const H: f32 = 2.8;

const STAIR_X0: f32 = 5.4;
const STAIR_STEPS: usize = 16;
const STAIR_TREAD: f32 = 0.22;
const STAIR_RISE: f32 = 3.0 / STAIR_STEPS as f32;
const STAIR_Z: (f32, f32) = (8.5, 9.85);

const GROUND_FRONT: [Opening; 3] = [
    op(0.9, 2.5, 0.85, 2.1),
    op(4.5, 5.5, 0.0, 2.2),
    op(8.0, 10.4, 0.95, 2.1),
];
const GROUND_BACK: [Opening; 3] = [
    op(1.5, 3.3, 0.85, 2.1),
    op(6.0, 7.2, 1.2, 2.3),
    op(10.0, 11.0, 1.5, 2.1),
];
const GROUND_LEFT: [Opening; 2] = [op(1.5, 3.9, 0.85, 2.1), op(7.4, 9.0, 0.85, 2.1)];
const GROUND_RIGHT: [Opening; 2] = [op(1.2, 2.8, 1.1, 2.1), op(3.6, 5.0, 1.1, 2.1)];

const UPPER_FRONT: [Opening; 3] = [
    op(1.0, 2.6, 0.85, 2.1),
    op(3.4, 5.0, 0.85, 2.1),
    op(7.4, 9.0, 0.85, 2.1),
];
const UPPER_BACK: [Opening; 3] = [
    op(1.5, 3.3, 0.85, 2.1),
    op(6.3, 7.5, 1.0, 2.2),
    op(10.0, 11.0, 1.6, 2.2),
];
const UPPER_LEFT: [Opening; 1] = [op(4.0, 5.2, 0.85, 2.1)];
const UPPER_RIGHT: [Opening; 1] = [op(1.8, 3.6, 0.85, 2.1)];

pub fn build_shell(b: &mut Builder) {
    ground_slab(b);
    upper_slab(b);
    roof(b);
    band(b);
    exterior(
        b,
        0.0,
        &GROUND_FRONT,
        &GROUND_BACK,
        &GROUND_LEFT,
        &GROUND_RIGHT,
    );
    exterior(
        b,
        UPPER,
        &UPPER_FRONT,
        &UPPER_BACK,
        &UPPER_LEFT,
        &UPPER_RIGHT,
    );
    interior(b, 0.0, true);
    interior(b, UPPER, false);
    stairs(b);
    porch(b);
}

fn ground_slab(b: &mut Builder) {
    b.bx((-0.15, 12.15), (-0.35, -0.03), (-0.15, 10.15), M::Concrete);
    b.bx((-0.15, 6.5), (-0.03, 0.0), (-0.15, 6.0), M::Floor);
    b.bx((6.5, 12.15), (-0.03, 0.0), (-0.15, 6.0), M::Tile);
    b.bx((-0.15, 4.5), (-0.03, 0.0), (6.0, 10.15), M::Floor);
    b.bx((4.5, 9.0), (-0.03, 0.0), (6.0, 10.15), M::Floor);
    b.bx((9.0, 12.15), (-0.03, 0.0), (6.0, 10.15), M::Tile);
}

fn upper_slab(b: &mut Builder) {
    let lo = UPPER - 0.2;
    let mid = UPPER - 0.03;
    b.bx((-0.15, 12.15), (lo, mid), (-0.15, 8.55), M::Ceiling);
    b.bx((-0.15, STAIR_X0), (lo, mid), (8.55, 10.15), M::Ceiling);
    b.bx((8.7, 12.15), (lo, mid), (8.55, 10.15), M::Ceiling);
    b.bx((STAIR_X0, 8.7), (lo, mid), (9.85, 10.15), M::Ceiling);

    b.bx((-0.15, 6.5), (mid, UPPER), (-0.15, 6.0), M::Floor);
    b.bx((6.5, 12.15), (mid, UPPER), (-0.15, 6.0), M::Floor);
    b.bx((-0.15, 4.5), (mid, UPPER), (6.0, 10.15), M::Floor);
    b.bx((9.0, 12.15), (mid, UPPER), (6.0, 10.15), M::Tile);
    b.bx((4.5, 9.0), (mid, UPPER), (6.0, 8.55), M::Floor);
    b.bx((4.5, STAIR_X0), (mid, UPPER), (8.55, 10.15), M::Floor);
    b.bx((8.7, 9.0), (mid, UPPER), (8.55, 10.15), M::Floor);
    b.bx((STAIR_X0, 8.7), (mid, UPPER), (9.85, 10.15), M::Floor);
}

fn band(b: &mut Builder) {
    let lo = UPPER - 0.2;
    b.gx((-0.17, 12.17), (lo, UPPER), (-0.17, -0.15), M::WallExt);
    b.gx((-0.17, 12.17), (lo, UPPER), (10.15, 10.17), M::WallExt);
    b.gx((-0.17, -0.15), (lo, UPPER), (-0.15, 10.15), M::WallExt);
    b.gx((12.15, 12.17), (lo, UPPER), (-0.15, 10.15), M::WallExt);
}

fn roof(b: &mut Builder) {
    let top = UPPER + H;
    b.gx(
        (-0.15, 12.15),
        (top, top + 0.08),
        (-0.15, 10.15),
        M::Ceiling,
    );
    b.gx(
        (-0.3, 12.3),
        (top + 0.08, top + 0.28),
        (-0.3, 10.3),
        M::Roof,
    );
    b.gx(
        (-0.3, 12.3),
        (top + 0.28, top + 0.75),
        (-0.3, -0.1),
        M::Roof,
    );
    b.gx(
        (-0.3, 12.3),
        (top + 0.28, top + 0.75),
        (10.1, 10.3),
        M::Roof,
    );
    b.gx(
        (-0.3, -0.1),
        (top + 0.28, top + 0.75),
        (-0.1, 10.1),
        M::Roof,
    );
    b.gx(
        (12.1, 12.3),
        (top + 0.28, top + 0.75),
        (-0.1, 10.1),
        M::Roof,
    );
}

fn exterior(
    b: &mut Builder,
    y0: f32,
    front: &[Opening],
    back: &[Opening],
    left: &[Opening],
    right: &[Opening],
) {
    side(
        b,
        Axis::X,
        (-0.15, -0.05),
        (-0.05, 0.15),
        (-0.15, 12.15),
        (-0.05, 12.05),
        0.0,
        0.15,
        1.0,
        y0,
        front,
    );
    side(
        b,
        Axis::X,
        (10.05, 10.15),
        (9.85, 10.05),
        (-0.15, 12.15),
        (-0.05, 12.05),
        10.0,
        9.85,
        -1.0,
        y0,
        back,
    );
    side(
        b,
        Axis::Z,
        (-0.15, -0.05),
        (-0.05, 0.15),
        (-0.05, 10.05),
        (0.15, 9.85),
        0.0,
        0.15,
        1.0,
        y0,
        left,
    );
    side(
        b,
        Axis::Z,
        (12.05, 12.15),
        (11.85, 12.05),
        (-0.05, 10.05),
        (0.15, 9.85),
        12.0,
        11.85,
        -1.0,
        y0,
        right,
    );
}

fn side(
    b: &mut Builder,
    axis: Axis,
    outer_f: (f32, f32),
    inner_f: (f32, f32),
    outer_t: (f32, f32),
    inner_t: (f32, f32),
    center: f32,
    face: f32,
    dir: f32,
    y0: f32,
    ops: &[Opening],
) {
    b.wall(axis, outer_f, outer_t, y0, H, M::WallExt, false, ops);
    b.wall(axis, inner_f, inner_t, y0, H, M::WallInt, true, ops);
    for o in ops {
        if o.bottom <= 0.0 {
            front_door(b, axis, center, *o, y0);
        } else {
            b.window(axis, center, y0, *o);
            b.sill(axis, face, dir, y0, *o);
            if o.bottom < 1.2 {
                b.curtains(axis, face, dir, y0, *o);
            }
        }
    }
}

fn front_door(b: &mut Builder, axis: Axis, c: f32, o: Opening, y0: f32) {
    if let Axis::X = axis {
        b.bx((o.a, o.b), (y0, y0 + o.top), (c - 0.02, c + 0.1), M::Door);
        b.gx(
            (o.a + 0.1, o.b - 0.1),
            (y0 + 0.15, y0 + 1.0),
            (c - 0.04, c - 0.02),
            M::Trim,
        );
        b.gx(
            (o.a + 0.1, o.b - 0.1),
            (y0 + 1.15, y0 + 2.05),
            (c - 0.04, c - 0.02),
            M::Trim,
        );
        b.gx(
            (o.b - 0.2, o.b - 0.14),
            (y0 + 1.0, y0 + 1.1),
            (c + 0.1, c + 0.18),
            M::Metal,
        );
    }
    b.door_frame(axis, c, 0.3, y0, o);
}

fn interior(b: &mut Builder, y0: f32, ground: bool) {
    let mid_ops = [op(2.4, 3.6, 0.0, 2.1)];
    let mid: &[Opening] = if ground { &mid_ops } else { &[] };
    b.wall(
        Axis::Z,
        (6.42, 6.58),
        (0.15, 5.92),
        y0,
        H,
        M::WallInt,
        true,
        mid,
    );
    for o in mid {
        b.door_frame(Axis::Z, 6.5, 0.16, y0, *o);
    }

    let across = [op(5.0, 6.2, 0.0, 2.1), op(7.0, 8.0, 0.0, 2.1)];
    b.wall(
        Axis::X,
        (5.92, 6.08),
        (0.15, 11.85),
        y0,
        H,
        M::WallInt,
        true,
        &across,
    );
    for o in across {
        b.door_frame(Axis::X, 6.0, 0.16, y0, o);
    }

    let back = [op(6.6, 7.6, 0.0, 2.1)];
    b.wall(
        Axis::Z,
        (4.42, 4.58),
        (6.08, 9.85),
        y0,
        H,
        M::WallInt,
        true,
        &back,
    );
    b.wall(
        Axis::Z,
        (8.92, 9.08),
        (6.08, 9.85),
        y0,
        H,
        M::WallInt,
        true,
        &back,
    );
    for o in back {
        b.door_frame(Axis::Z, 4.5, 0.16, y0, o);
        b.door_frame(Axis::Z, 9.0, 0.16, y0, o);
    }
}

fn stairs(b: &mut Builder) {
    for i in 0..STAIR_STEPS {
        let x0 = STAIR_X0 + i as f32 * STAIR_TREAD;
        let top = (i + 1) as f32 * STAIR_RISE;
        b.bx((x0, x0 + STAIR_TREAD), (0.0, top), STAIR_Z, M::WoodLight);
        b.gx(
            (x0 + STAIR_TREAD - 0.03, x0 + STAIR_TREAD),
            (top - STAIR_RISE, top),
            STAIR_Z,
            M::WoodDark,
        );
    }
    let x_end = STAIR_X0 + STAIR_STEPS as f32 * STAIR_TREAD;
    let run = x_end - STAIR_X0;
    let angle = (3.0f32).atan2(run);
    let len = (run * run + 9.0).sqrt();
    b.tilted(
        Vec3::new((STAIR_X0 + x_end) * 0.5, 0.95 + 1.5, 8.47),
        Vec3::new(len, 0.06, 0.06),
        Quat::from_rotation_z(angle),
        M::WoodDark,
    );
    for i in (0..STAIR_STEPS).step_by(2) {
        let x = STAIR_X0 + i as f32 * STAIR_TREAD + 0.1;
        let top = (i + 1) as f32 * STAIR_RISE;
        b.gx((x, x + 0.04), (top, top + 0.95), (8.45, 8.49), M::Trim);
    }
    b.railing((STAIR_X0, 8.0), (8.5, 8.55), UPPER);
    b.railing((STAIR_X0 - 0.05, STAIR_X0), (8.55, 9.85), UPPER);
}

fn porch(b: &mut Builder) {
    b.gx((3.6, 6.4), (-0.3, 0.0), (-1.8, -0.15), M::Concrete);
    b.gx((3.8, 6.2), (-0.3, -0.12), (-2.4, -1.8), M::Concrete);
    b.gx((3.6, 6.4), (2.62, 2.72), (-1.8, -0.15), M::Trim);
    b.post(3.75, -1.65, 0.0, 2.62, 0.07, M::Trim, true);
    b.post(6.25, -1.65, 0.0, 2.62, 0.07, M::Trim, true);
    let e = b.light(
        Vec3::new(5.0, 2.4, -0.45),
        Color::srgb(1.0, 0.7, 0.35),
        40_000.0,
        7.0,
        false,
    );
    b.flicker(e, 40_000.0, 1.3);
    b.gx((4.88, 5.12), (2.38, 2.55), (-0.57, -0.33), M::Bulb);
    b.ball(Vec3::new(3.95, 0.2, -1.2), 0.2, M::Pumpkin);
    b.gx((3.92, 3.98), (0.38, 0.48), (-1.23, -1.17), M::Bark);
    b.ball(Vec3::new(6.0, 0.15, -1.0), 0.15, M::Pumpkin);
    b.gx((5.97, 6.03), (0.28, 0.35), (-1.03, -0.97), M::Bark);
}
