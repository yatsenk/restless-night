use super::*;

const PITCH_MAIN: f32 = 0.42;
const PITCH_BAY: f32 = 0.40;
const ROOF_T: f32 = 0.14;

fn slab(b: &mut Builder, m: Mat, pts: [V3; 4], t: f32, edges: [bool; 4]) {
    let e1 = pts[1] - pts[0];
    let e2 = pts[3] - pts[0];
    let mut n = e1.cross(e2).norm();
    let mut p = pts;
    let flipped = n.y < 0.0;
    if flipped {
        p = [pts[0], pts[3], pts[2], pts[1]];
        n = -n;
    }
    let down = v3(0.0, -t, 0.0);
    b.quad(m, p[0], p[1], p[2], p[3], n);
    b.quad(m, p[0] + down, p[3] + down, p[2] + down, p[1] + down, -n);
    let c = (p[0] + p[1] + p[2] + p[3]) * 0.25;
    for i in 0..4 {
        if !edges[if flipped { 3 - i } else { i }] {
            continue;
        }
        let a = p[i];
        let bb = p[(i + 1) % 4];
        let mid = (a + bb) * 0.5;
        let mut en = (bb - a).cross(v3(0.0, 1.0, 0.0)).norm();
        if en.dot(mid - c) < 0.0 {
            en = -en;
        }
        b.quad(m, a, bb, bb + down, a + down, en);
    }
}

fn main_roof(b: &mut Builder) {
    let s = PITCH_MAIN;
    let t = ROOF_T;
    let ov = 0.6;
    let rake = 0.45;
    let x0 = BAY_X1 - 0.35;
    let x1 = OUT_X1 + rake;
    let zw0 = -EXT;
    let zw1 = OUT_Z1;
    let zr = (zw0 + zw1) * 0.5;
    let ze_f = zw0 - ov;
    let ze_b = zw1 + ov;
    let yf = |z: f32| TOP + t + s * (z - zw0);
    let yb = |z: f32| TOP + t + s * (zw1 - z);
    slab(
        b,
        Mat::Roof,
        [
            v3(x0, yf(ze_f), ze_f),
            v3(x1, yf(ze_f), ze_f),
            v3(x1, yf(zr), zr),
            v3(x0, yf(zr), zr),
        ],
        t,
        [true, true, false, true],
    );
    slab(
        b,
        Mat::Roof,
        [
            v3(x0, yb(ze_b), ze_b),
            v3(x1, yb(ze_b), ze_b),
            v3(x1, yb(zr), zr),
            v3(x0, yb(zr), zr),
        ],
        t,
        [true, true, false, true],
    );
    let peak = yf(zr) - t;
    b.tri(
        Mat::Siding,
        v3(OUT_X1, TOP, zw0),
        v3(OUT_X1, TOP, zw1),
        v3(OUT_X1, peak, zr),
        v3(1.0, 0.0, 0.0),
    );
    b.tri(
        Mat::Siding,
        v3(BAY_X1, TOP, zw0),
        v3(BAY_X1, TOP, zw1),
        v3(BAY_X1, peak, zr),
        v3(-1.0, 0.0, 0.0),
    );
    let ridge = yf(zr);
    b.cuboid(
        Mat::Roof,
        v3(x0, ridge, zr - 0.12),
        v3(x1, ridge + 0.06, zr + 0.12),
        SKIP_NY,
    );

    gutter_x(b, x0 + 0.05, x1 - 0.05, ze_f, yf(ze_f), -1.0);
    gutter_x(b, x0 + 0.05, x1 - 0.05, ze_b, yb(ze_b), 1.0);
    downspout(b, 12.82, yf(ze_f) - 0.1, ze_f - 0.06, 12.82, zw0 - 0.05);
    downspout(b, 12.82, yb(ze_b) - 0.1, ze_b + 0.06, 12.82, zw1 + 0.05);
    downspout(b, 3.95, yb(ze_b) - 0.1, ze_b + 0.06, 3.95, zw1 + 0.05);
}

fn bay_roof(b: &mut Builder) {
    let s = PITCH_BAY;
    let t = ROOF_T;
    let ov = 0.5;
    let xw0 = -EXT;
    let xw1 = MX0;
    let xc = (xw0 + xw1) * 0.5;
    let zf = G_Z0 - EXT - ov;
    let zb = OUT_Z1 + 0.4;
    let base = CEIL0;
    let yw = |x: f32| base + t + s * (x - xw0);
    let ye = |x: f32| base + t + s * (xw1 - x);
    let ridge = yw(xc);
    let x_w = xw0 - ov;
    let x_e1 = xw1 + ov;
    let z_main = -EXT;
    slab(
        b,
        Mat::Roof,
        [
            v3(x_w, yw(x_w), zf),
            v3(xc, ridge, zf),
            v3(xc, ridge, zb),
            v3(x_w, yw(x_w), zb),
        ],
        t,
        [true, false, true, true],
    );
    slab(
        b,
        Mat::Roof,
        [
            v3(xc, ridge, zf),
            v3(x_e1, ye(x_e1), zf),
            v3(x_e1, ye(x_e1), z_main),
            v3(xc, ridge, z_main),
        ],
        t,
        [true, true, true, false],
    );
    slab(
        b,
        Mat::Roof,
        [
            v3(xc, ridge, z_main),
            v3(BAY_X1, ye(BAY_X1), z_main),
            v3(BAY_X1, ye(BAY_X1), zb),
            v3(xc, ridge, zb),
        ],
        t,
        [false, false, true, false],
    );
    let peak = ridge - t;
    b.tri(
        Mat::Siding,
        v3(xw0, base, G_Z0 - EXT),
        v3(xw1, base, G_Z0 - EXT),
        v3(xc, peak, G_Z0 - EXT),
        v3(0.0, 0.0, -1.0),
    );
    b.tri(
        Mat::Siding,
        v3(xw0, base, OUT_Z1),
        v3(BAY_X1, base, OUT_Z1),
        v3(xc, peak, OUT_Z1),
        v3(0.0, 0.0, 1.0),
    );
    b.tri(
        Mat::Siding,
        v3(BAY_X1, base, OUT_Z1),
        v3(BAY_X1, ye(BAY_X1) - t, OUT_Z1),
        v3(xc, peak, OUT_Z1),
        v3(0.0, 0.0, 1.0),
    );
    b.cuboid(
        Mat::Roof,
        v3(xc - 0.12, ridge, zf),
        v3(xc + 0.12, ridge + 0.06, zb),
        SKIP_NY,
    );
    gutter_z(b, x_w - 0.02, zf + 0.05, zb - 0.05, yw(x_w), -1.0);
    downspout(b, x_w - 0.08, yw(x_w) - 0.1, 7.9, xw0 - 0.05, 7.9);
    downspout(b, x_w - 0.08, yw(x_w) - 0.1, -0.2, xw0 - 0.05, -0.2);
}

fn gutter_x(b: &mut Builder, xa: f32, xb: f32, z: f32, y_top: f32, dir: f32) {
    let (z0, z1) = if dir < 0.0 {
        (z - 0.11, z + 0.01)
    } else {
        (z - 0.01, z + 0.11)
    };
    b.cuboid(
        Mat::Frame,
        v3(xa, y_top - 0.2, z0),
        v3(xb, y_top - 0.06, z1),
        0,
    );
}

fn gutter_z(b: &mut Builder, x: f32, za: f32, zb: f32, y_top: f32, dir: f32) {
    let (x0, x1) = if dir < 0.0 {
        (x - 0.11, x + 0.01)
    } else {
        (x - 0.01, x + 0.11)
    };
    b.cuboid(
        Mat::Frame,
        v3(x0, y_top - 0.2, za),
        v3(x1, y_top - 0.06, zb),
        0,
    );
}

fn downspout(b: &mut Builder, gx: f32, gy: f32, gz: f32, wx: f32, wz: f32) {
    let low = gy - 0.4;
    b.beam(Mat::Frame, v3(gx, gy, gz), v3(wx, low, wz), 0.08, 0.06);
    b.cuboid(
        Mat::Frame,
        v3(wx - 0.045, GROUND, wz - 0.045),
        v3(wx + 0.045, low, wz + 0.045),
        SKIP_NY,
    );
    b.cuboid(
        Mat::Frame,
        v3(wx - 0.07, GROUND, wz - 0.07),
        v3(wx + 0.07, GROUND + 0.1, wz + 0.07),
        SKIP_NY,
    );
}

pub(super) fn roofs(b: &mut Builder) {
    main_roof(b);
    bay_roof(b);
}

pub(super) fn porch(b: &mut Builder) {
    b.fz(Mat::Concrete, P_Z0, MX0, P_X1, GROUND, 0.0, -1.0);
    for x in [3.95f32, 6.5] {
        b.cuboid(
            Mat::Frame,
            v3(x - 0.09, 0.0, P_Z0),
            v3(x + 0.09, CEIL0, P_Z0 + 0.18),
            SKIP_NY,
        );
    }
    b.cuboid(
        Mat::Concrete,
        v3(STEP_X0, GROUND, P_Z0 - 0.6),
        v3(STEP_X1, GROUND + 0.18, P_Z0),
        SKIP_NY | SKIP_PZ,
    );
}

fn plane(b: &mut Builder, m: Mat, x0: f32, x1: f32, z0: f32, z1: f32, y: f32) {
    b.fy(m, y, x0, x1, z0, z1, 1.0);
}

pub(super) fn lawn_rects() -> [(f32, f32, f32, f32); 5] {
    [
        (-LAWN_R, OUT_X0, -LAWN_R, LAWN_R),
        (OUT_X1, LAWN_R, -LAWN_R, LAWN_R),
        (OUT_X0, OUT_X1, -LAWN_R, OUT_Z0),
        (OUT_X0, OUT_X1, OUT_Z1, LAWN_R),
        (MX0, OUT_X1, OUT_Z0, P_Z0),
    ]
}

pub(super) fn ground(b: &mut Builder) {
    for (x0, x1, z0, z1) in lawn_rects() {
        plane(b, Mat::Grass, x0, x1, z0, z1, GROUND);
    }
    let o = 0.01;
    plane(
        b,
        Mat::Concrete,
        -40.0,
        40.0,
        SIDEWALK_Z0,
        SIDEWALK_Z1,
        GROUND + o,
    );
    b.cuboid(
        Mat::Concrete,
        v3(-40.0, GROUND, SIDEWALK_Z0 - 0.2),
        v3(40.0, GROUND + 0.12, SIDEWALK_Z0),
        SKIP_NY,
    );
    plane(
        b,
        Mat::Asphalt,
        -LAWN_R,
        LAWN_R,
        -15.5,
        SIDEWALK_Z0 - 0.2,
        GROUND + 0.005,
    );
    plane(
        b,
        Mat::Concrete,
        PATH_X0,
        PATH_X1,
        -0.9,
        SIDEWALK_Z1,
        GROUND + o,
    );
    let zd0 = SIDEWALK_Z1;
    let zd1 = G_Z0 - EXT;
    let ya = GROUND + o;
    let yb = -0.02;
    b.quad(
        Mat::Concrete,
        v3(DRIVE_X0, ya, zd0),
        v3(DRIVE_X1, ya, zd0),
        v3(DRIVE_X1, yb, zd1),
        v3(DRIVE_X0, yb, zd1),
        v3(0.0, 1.0, 0.0),
    );
    b.tri(
        Mat::Concrete,
        v3(DRIVE_X0, GROUND, zd0),
        v3(DRIVE_X0, GROUND, zd1),
        v3(DRIVE_X0, yb, zd1),
        v3(-1.0, 0.0, 0.0),
    );
    b.tri(
        Mat::Concrete,
        v3(DRIVE_X1, GROUND, zd0),
        v3(DRIVE_X1, GROUND, zd1),
        v3(DRIVE_X1, yb, zd1),
        v3(1.0, 0.0, 0.0),
    );
    b.cuboid(
        Mat::Concrete,
        v3(2.0, GROUND, OUT_Z1),
        v3(3.7, 0.0, OUT_Z1 + 0.5),
        SKIP_NY | SKIP_NZ,
    );
    b.cuboid(
        Mat::Concrete,
        v3(2.0, GROUND, OUT_Z1 + 0.5),
        v3(3.7, GROUND + 0.17, OUT_Z1 + 1.0),
        SKIP_NY | SKIP_NZ,
    );
}

pub(super) fn basement(b: &mut Builder) {
    let (x1, z1) = (BAY_X1, G_Z1);
    let m = Mat::Concrete;
    b.fy(m, B_FLOOR, 0.0, x1, B_Z0, z1, 1.0);
    b.grid_y(
        m,
        B_CEIL,
        &[0.0, B_X1, x1],
        &[B_Z0, B_HZ0, B_ZT, z1],
        Some((0, 1)),
        -1.0,
    );
    b.fx(m, 0.0, B_FLOOR, B_CEIL, B_Z0, z1, 1.0);
    b.fx(m, x1, B_FLOOR, B_CEIL, B_Z0, z1, -1.0);
    b.fz(m, B_Z0, 0.0, x1, B_FLOOR, B_CEIL, 1.0);
    b.fz(m, z1, 0.0, x1, B_FLOOR, B_CEIL, -1.0);
    b.fx(m, 0.0, B_CEIL, 0.0, B_HZ0, B_ZT, 1.0);
    b.fx(m, B_X1, B_FLOOR, 0.0, B_ZE, B_ZT, -1.0);
    b.fx(m, B_X1, B_CEIL, 0.0, B_HZ0, B_ZE, -1.0);
    b.fx(m, B_X1 + HT_B, B_FLOOR, B_CEIL, B_ZE, B_ZT, 1.0);
    b.fz(m, B_HZ0, 0.0, B_X1, B_CEIL, 0.0, 1.0);
    b.fz(m, B_ZE, B_X1, B_X1 + HT_B, B_FLOOR, B_CEIL, -1.0);
    b.fz(m, B_ZT, 0.0, B_X1 + HT_B, B_FLOOR, B_CEIL, 1.0);
    b.fz(m, B_ZT, 0.0, B_X1, B_CEIL, -B_RISE, -1.0);
    for i in 1..B_STEPS {
        let za = B_ZT - i as f32 * B_TREAD;
        let zb = za + B_TREAD;
        let top = -(i as f32) * B_RISE;
        b.fy(Mat::Wood, top, 0.0, B_X1, za, zb, 1.0);
        b.fz(Mat::Wood, zb, 0.0, B_X1, top, top + B_RISE, -1.0);
    }
    let last = -((B_STEPS - 1) as f32) * B_RISE;
    b.fz(Mat::Wood, B_ZE, 0.0, B_X1, B_FLOOR, last, -1.0);
}

const HT_B: f32 = 0.16;
