use crate::sim::collision::{Seg, HEIGHT, RADIUS, STEP};

pub const LEAF_HALF: f32 = 0.025;
pub const OPEN_ANGLE: f32 = 1.55;
pub const LIMIT: f32 = 1.65;
const FRICTION: f32 = 3.5;
const MAX_OMEGA: f32 = 3.2;

#[derive(Clone, Copy, Debug)]
pub struct DoorSpec {
    pub hx: f32,
    pub hz: f32,
    pub closed: f32,
    pub len: f32,
    pub y0: f32,
    pub height: f32,
    pub lo: f32,
    pub hi: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct Door {
    pub spec: DoorSpec,
    pub ang: f32,
    pub omega: f32,
    pub target: Option<f32>,
}

impl Door {
    pub fn new(spec: DoorSpec) -> Self {
        Door {
            spec,
            ang: 0.0,
            omega: 0.0,
            target: None,
        }
    }

    pub fn plan_angle(&self) -> f32 {
        self.spec.closed + self.ang
    }

    pub fn yaw(&self) -> f32 {
        -self.plan_angle()
    }

    pub fn tip(&self) -> (f32, f32) {
        let a = self.plan_angle();
        (
            self.spec.hx + self.spec.len * a.cos(),
            self.spec.hz + self.spec.len * a.sin(),
        )
    }

    pub fn seg(&self) -> Seg {
        let (tx, tz) = self.tip();
        Seg {
            ax: self.spec.hx,
            az: self.spec.hz,
            bx: tx,
            bz: tz,
            half: LEAF_HALF,
            y0: self.spec.y0,
            y1: self.spec.y0 + self.spec.height,
        }
    }

    pub fn is_open(&self) -> bool {
        self.ang.abs() > 0.2
    }

    fn blocks(&self, ang: f32, c: (f32, f32), feet: f32) -> bool {
        let mut t = *self;
        t.ang = ang;
        t.seg().overlaps_circle(c.0, c.1, RADIUS, feet)
    }

    fn player_push(&mut self, end: (f32, f32), feet: f32, dt: f32) -> bool {
        let seg = self.seg();
        if seg.y1 <= feet + STEP || seg.y0 >= feet + HEIGHT {
            return false;
        }
        let (qx, qz) = seg.nearest(end.0, end.1);
        let dx = end.0 - qx;
        let dz = end.1 - qz;
        let dist = (dx * dx + dz * dz).sqrt();
        let reach = RADIUS + seg.half;
        if dist >= reach || dist < 1e-5 {
            return false;
        }
        let (nx, nz) = (dx / dist, dz / dist);
        let rx = qx - self.spec.hx;
        let rz = qz - self.spec.hz;
        let r = (rx * rx + rz * rz).sqrt();
        if r < 0.12 {
            return false;
        }
        let a = self.plan_angle();
        let along = -(nx * -a.sin() + nz * a.cos());
        if along.abs() < 0.25 {
            return false;
        }
        let pen = reach - dist;
        let dang = (pen / (r * along)).clamp(-0.4, 0.4);
        let na = (self.ang + dang).clamp(self.spec.lo, self.spec.hi);
        if (na - self.ang).abs() < 1e-6 {
            return false;
        }
        self.omega = ((na - self.ang) / dt).clamp(-MAX_OMEGA, MAX_OMEGA);
        self.ang = na;
        true
    }

    fn step(&mut self, start: (f32, f32), end: (f32, f32), feet: f32, dt: f32) {
        let dt = dt.clamp(1e-4, 0.05);
        if let Some(t) = self.target {
            let w = ((t - self.ang) * 5.0).clamp(-2.6, 2.6);
            let mut na = self.ang + w * dt;
            let reached = (t - na).abs() < 0.012 || (t - self.ang) * (t - na) <= 0.0;
            if reached {
                na = t;
            }
            if self.blocks(na, end, feet) || self.blocks(na, start, feet) {
                self.target = None;
                self.omega = 0.0;
            } else {
                self.ang = na;
                self.omega = if reached { 0.0 } else { w };
                if reached {
                    self.target = None;
                }
            }
            return;
        }
        let pushed = self.player_push(end, feet, dt);
        if pushed {
            return;
        }
        if self.omega.abs() > 1e-4 {
            let raw = self.ang + self.omega * dt;
            let na = raw.clamp(self.spec.lo, self.spec.hi);
            if na != raw {
                self.omega = -self.omega * 0.2;
            }
            if self.blocks(na, end, feet) || self.blocks(na, start, feet) {
                self.omega = 0.0;
            } else {
                self.ang = na;
            }
            self.omega *= (-FRICTION * dt).exp();
            if self.omega.abs() < 0.02 {
                self.omega = 0.0;
            }
        }
        if self.ang.abs() < 0.03 && self.omega.abs() < 0.35 && self.ang != 0.0 {
            self.ang = 0.0;
            self.omega = 0.0;
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Doors {
    pub list: Vec<Door>,
}

impl Doors {
    pub fn new(specs: &[DoorSpec]) -> Self {
        Doors {
            list: specs.iter().copied().map(Door::new).collect(),
        }
    }

    pub fn segs(&self) -> Vec<Seg> {
        self.list.iter().map(|d| d.seg()).collect()
    }

    pub fn update(&mut self, p: (f32, f32), feet: f32, d: (f32, f32), dt: f32) {
        let end = (p.0 + d.0, p.1 + d.1);
        for door in &mut self.list {
            door.step(p, end, feet, dt);
        }
    }

    pub fn interact(&mut self, i: usize, p: (f32, f32)) {
        let d = &mut self.list[i];
        if d.ang.abs() > 0.2 {
            d.target = Some(0.0);
            d.omega = 0.0;
            return;
        }
        let a = d.spec.closed;
        let rx = p.0 - d.spec.hx;
        let rz = p.1 - d.spec.hz;
        let cross = a.cos() * rz - a.sin() * rx;
        let away = if cross > 0.0 { -OPEN_ANGLE } else { OPEN_ANGLE };
        let pick = if away >= d.spec.lo && away <= d.spec.hi {
            away
        } else {
            -away
        };
        let t = pick.clamp(d.spec.lo, d.spec.hi);
        if t.abs() > 0.1 {
            d.target = Some(t);
            d.omega = 0.0;
        }
    }

    pub fn pick(&self, eye: (f32, f32, f32), look: (f32, f32, f32)) -> Option<usize> {
        let mut best: Option<(usize, f32)> = None;
        for (i, d) in self.list.iter().enumerate() {
            let s = d.seg();
            let mx = (s.ax + s.bx) * 0.5;
            let mz = (s.az + s.bz) * 0.5;
            let my = d.spec.y0 + 1.0;
            let v = (mx - eye.0, my - eye.1, mz - eye.2);
            let dist = (v.0 * v.0 + v.1 * v.1 + v.2 * v.2).sqrt();
            if !(0.05..=2.6).contains(&dist) {
                continue;
            }
            let c = (v.0 * look.0 + v.1 * look.1 + v.2 * look.2) / dist;
            if c < 0.88 {
                continue;
            }
            let score = c - dist * 0.04;
            if best.map_or(true, |(_, s0)| score > s0) {
                best = Some((i, score));
            }
        }
        best.map(|(i, _)| i)
    }
}
