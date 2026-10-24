use super::*;

pub(super) fn stairs(b: &mut Builder) {
    let (x0, x1) = (S_X0, S_X1);
    for i in 1..RISERS {
        let z0 = S_Z0 + (i as f32 - 1.0) * TREAD;
        let z1 = z0 + TREAD;
        let top = i as f32 * RISE;
        b.fy(Mat::Wood, top, x0, x1, z0, z1, 1.0);
        b.fz(Mat::Wood, z0, x0, x1, (i as f32 - 1.0) * RISE, top, -1.0);
        b.fx(Mat::Wood, x1, 0.0, top, z0, z1, 1.0);
    }
    let last = (RISERS as f32 - 1.0) * RISE;
    b.fz(Mat::Wood, S_END, x0, x1, last, UPPER, -1.0);
    b.fz(Mat::Plaster, S_END, x0, x1, CEIL0, last, -1.0);
    b.fz(Mat::Plaster, S_END, x0, x1, 0.0, CEIL0, 1.0);
    b.fx(Mat::Plaster, S_X1, CEIL0, UPPER, HOLE_Z0, S_END, -1.0);
    b.fz(Mat::Plaster, HOLE_Z0, x0, x1, CEIL0, UPPER, 1.0);

    let rc = S_X1 - 0.05;
    let surf = |z: f32| RISE * 0.5 + (z - S_Z0) * (RISE / TREAD);
    let k = UPPER + RAIL_H - surf(S_END);
    let za = S_Z0 + 0.12;
    let zb = S_END;
    b.beam(
        Mat::Wood,
        v3(rc, surf(za) + k, za),
        v3(rc, surf(zb) + k, zb),
        0.07,
        0.05,
    );
    for i in 1..RISERS {
        let zc = S_Z0 + (i as f32 - 0.5) * TREAD;
        let top = i as f32 * RISE;
        let rail_y = surf(zc) + k;
        b.cuboid(
            Mat::Wood,
            v3(rc - 0.0175, top, zc - 0.0175),
            v3(rc + 0.0175, rail_y - 0.025, zc + 0.0175),
            SKIP_NY | SKIP_PY,
        );
    }
    b.cuboid(
        Mat::Wood,
        v3(rc - 0.06, 0.0, S_Z0 - 0.02),
        v3(rc + 0.04, 1.2, S_Z0 + 0.08),
        SKIP_NY,
    );
}

pub(super) fn hole_rails(b: &mut Builder) {
    let y = UPPER + RAIL_H;
    let rc = S_X1 - 0.055;
    b.beam(
        Mat::Wood,
        v3(rc, y, HOLE_Z0),
        v3(rc, y, S_END),
        0.07,
        0.05,
    );
    let mut z = HOLE_Z0 + 0.1;
    while z < S_END - 0.05 {
        b.cuboid(
            Mat::Wood,
            v3(rc - 0.0175, UPPER, z - 0.0175),
            v3(rc + 0.0175, y - 0.025, z + 0.0175),
            SKIP_NY | SKIP_PY,
        );
        z += 0.13;
    }
    b.cuboid(
        Mat::Wood,
        v3(rc - 0.05, UPPER, HOLE_Z0 - 0.05),
        v3(rc + 0.05, y + 0.12, HOLE_Z0 + 0.05),
        SKIP_NY,
    );
    b.beam(
        Mat::Wood,
        v3(S_X0 + 0.02, y, HOLE_Z0),
        v3(rc, y, HOLE_Z0),
        0.07,
        0.05,
    );
    let mut x = S_X0 + 0.12;
    while x < rc - 0.05 {
        b.cuboid(
            Mat::Wood,
            v3(x - 0.0175, UPPER, HOLE_Z0 - 0.0175),
            v3(x + 0.0175, y - 0.025, HOLE_Z0 + 0.0175),
            SKIP_NY | SKIP_PY,
        );
        x += 0.13;
    }
}
