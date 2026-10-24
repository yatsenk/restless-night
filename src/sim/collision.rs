use crate::sim::math::{v3, V3};

pub const RADIUS: f32 = 0.3;
pub const HEIGHT: f32 = 1.78;
pub const STEP: f32 = 0.31;
pub const SNAP_DOWN: f32 = 0.5;
pub const GRAVITY: f32 = 18.0;
const MAX_SUBSTEP: f32 = 0.08;

#[derive(Clone, Copy, Debug)]
pub struct Solid {
    pub min: V3,
    pub max: V3,
}

impl Solid {
    pub fn new(min: [f32; 3], max: [f32; 3]) -> Self {
        Solid {
            min: v3(min[0], min[1], min[2]),
            max: v3(max[0], max[1], max[2]),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Seg {
    pub ax: f32,
    pub az: f32,
    pub bx: f32,
    pub bz: f32,
    pub half: f32,
    pub y0: f32,
    pub y1: f32,
}

impl Seg {
    pub fn nearest(&self, x: f32, z: f32) -> (f32, f32) {
        let ex = self.bx - self.ax;
        let ez = self.bz - self.az;
        let l2 = ex * ex + ez * ez;
        let t = if l2 > 1e-9 {
            (((x - self.ax) * ex + (z - self.az) * ez) / l2).clamp(0.0, 1.0)
        } else {
            0.0
        };
        (self.ax + ex * t, self.az + ez * t)
    }

    pub fn overlaps_circle(&self, x: f32, z: f32, radius: f32, feet: f32) -> bool {
        if self.y1 <= feet + STEP || self.y0 >= feet + HEIGHT {
            return false;
        }
        let (nx, nz) = self.nearest(x, z);
        let dx = x - nx;
        let dz = z - nz;
        let r = radius + self.half;
        dx * dx + dz * dz < r * r
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Ramp {
    pub x0: f32,
    pub x1: f32,
    pub z0: f32,
    pub z1: f32,
    pub y0: f32,
    pub y1: f32,
}

impl Ramp {
    pub fn height(&self, z: f32) -> f32 {
        let t = ((z - self.z0) / (self.z1 - self.z0)).clamp(0.0, 1.0);
        self.y0 + (self.y1 - self.y0) * t
    }

    pub fn contains(&self, x: f32, z: f32) -> bool {
        x >= self.x0 && x <= self.x1 && z >= self.z0 && z <= self.z1
    }
}

#[derive(Clone, Debug, Default)]
pub struct World {
    pub solids: Vec<Solid>,
    pub ramps: Vec<Ramp>,
    pub floor: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct Body {
    pub pos: V3,
    pub vy: f32,
    pub grounded: bool,
}

impl Body {
    pub fn new(pos: V3) -> Self {
        Body {
            pos,
            vy: 0.0,
            grounded: false,
        }
    }
}

fn push_circle_from_rect(
    cx: &mut f32,
    cz: &mut f32,
    x0: f32,
    x1: f32,
    z0: f32,
    z1: f32,
    nearest: (f32, f32),
) {
    let dx = *cx - nearest.0;
    let dz = *cz - nearest.1;
    let d2 = dx * dx + dz * dz;
    if d2 > 1e-10 {
        let d = d2.sqrt();
        let k = (RADIUS - d) / d;
        *cx += dx * k;
        *cz += dz * k;
    } else {
        let left = *cx - x0;
        let right = x1 - *cx;
        let back = *cz - z0;
        let front = z1 - *cz;
        let m = left.min(right).min(back).min(front);
        if m == left {
            *cx = x0 - RADIUS;
        } else if m == right {
            *cx = x1 + RADIUS;
        } else if m == back {
            *cz = z0 - RADIUS;
        } else {
            *cz = z1 + RADIUS;
        }
    }
}

impl World {
    pub fn new(floor: f32) -> Self {
        World {
            solids: Vec::new(),
            ramps: Vec::new(),
            floor,
        }
    }

    pub fn ground_at(&self, x: f32, z: f32, feet: f32) -> f32 {
        let limit = feet + STEP + 1e-4;
        let mut best = self.floor;
        for s in &self.solids {
            if x >= s.min.x && x <= s.max.x && z >= s.min.z && z <= s.max.z && s.max.y <= limit {
                best = best.max(s.max.y);
            }
        }
        for r in &self.ramps {
            if r.contains(x, z) {
                let h = r.height(z);
                if h <= limit {
                    best = best.max(h);
                }
            }
        }
        best
    }

    fn push_out(&self, x: &mut f32, z: &mut f32, feet: f32, segs: &[Seg]) -> bool {
        let mut moved = false;
        for s in segs {
            if s.y1 <= feet + STEP || s.y0 >= feet + HEIGHT {
                continue;
            }
            let (nx, nz) = s.nearest(*x, *z);
            let dx = *x - nx;
            let dz = *z - nz;
            let d2 = dx * dx + dz * dz;
            let r = RADIUS + s.half;
            if d2 < r * r {
                if d2 > 1e-10 {
                    let d = d2.sqrt();
                    let k = (r - d) / d;
                    *x += dx * k;
                    *z += dz * k;
                } else {
                    let ex = s.bx - s.ax;
                    let ez = s.bz - s.az;
                    let l = (ex * ex + ez * ez).sqrt().max(1e-6);
                    *x += -ez / l * r;
                    *z += ex / l * r;
                }
                moved = true;
            }
        }
        let lo = feet + STEP;
        let hi = feet + HEIGHT;
        for s in &self.solids {
            if s.max.y <= lo || s.min.y >= hi {
                continue;
            }
            let nx = (*x).clamp(s.min.x, s.max.x);
            let nz = (*z).clamp(s.min.z, s.max.z);
            let dx = *x - nx;
            let dz = *z - nz;
            if dx * dx + dz * dz < RADIUS * RADIUS {
                push_circle_from_rect(x, z, s.min.x, s.max.x, s.min.z, s.max.z, (nx, nz));
                moved = true;
            }
        }
        for r in &self.ramps {
            let nx = (*x).clamp(r.x0, r.x1);
            let nz = (*z).clamp(r.z0, r.z1);
            let dx = *x - nx;
            let dz = *z - nz;
            if dx * dx + dz * dz < RADIUS * RADIUS && r.height(nz) > lo {
                push_circle_from_rect(x, z, r.x0, r.x1, r.z0, r.z1, (nx, nz));
                moved = true;
            }
        }
        moved
    }

    fn settle(&self, b: &mut Body, dt: f32) {
        let g = self.ground_at(b.pos.x, b.pos.z, b.pos.y);
        if b.grounded {
            if g >= b.pos.y - SNAP_DOWN {
                b.pos.y = g;
                b.vy = 0.0;
            } else {
                b.grounded = false;
            }
        }
        if !b.grounded {
            b.vy -= GRAVITY * dt;
            b.pos.y += b.vy * dt;
            if b.pos.y <= g {
                b.pos.y = g;
                b.vy = 0.0;
                b.grounded = true;
            }
        }
    }

    pub fn step(&self, b: &mut Body, dx: f32, dz: f32, dt: f32) {
        self.step_with(b, dx, dz, dt, &[]);
    }

    pub fn step_with(&self, b: &mut Body, dx: f32, dz: f32, dt: f32, segs: &[Seg]) {
        let dist = (dx * dx + dz * dz).sqrt();
        let n = ((dist / MAX_SUBSTEP).ceil() as usize).max(1);
        let inv = 1.0 / n as f32;
        for _ in 0..n {
            b.pos.x += dx * inv;
            b.pos.z += dz * inv;
            for _ in 0..4 {
                if !self.push_out(&mut b.pos.x, &mut b.pos.z, b.pos.y, segs) {
                    break;
                }
            }
            self.settle(b, dt * inv);
        }
    }
}
