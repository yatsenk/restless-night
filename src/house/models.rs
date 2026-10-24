use crate::sim::collision::Solid;
use crate::sim::math::{v3, V3};

#[derive(Clone, Debug)]
pub struct Placement {
    pub name: &'static str,
    pub file: &'static str,
    pub scale: f32,
    pub anchor: V3,
    pub pos: V3,
    pub quarters: i32,
    pub boxes: Vec<(V3, V3)>,
}

pub fn rotate_quarters(v: V3, quarters: i32) -> V3 {
    let (c, s) = match quarters.rem_euclid(4) {
        0 => (1.0, 0.0),
        1 => (0.0, 1.0),
        2 => (-1.0, 0.0),
        _ => (0.0, -1.0),
    };
    v3(v.x * c + v.z * s, v.y, -v.x * s + v.z * c)
}

impl Placement {
    pub fn translation(&self) -> V3 {
        self.pos - rotate_quarters(self.anchor * self.scale, self.quarters)
    }

    pub fn yaw(&self) -> f32 {
        self.quarters as f32 * std::f32::consts::FRAC_PI_2
    }

    pub fn world_point(&self, v: V3) -> V3 {
        self.pos + rotate_quarters((v - self.anchor) * self.scale, self.quarters)
    }

    pub fn colliders(&self) -> Vec<Solid> {
        self.boxes
            .iter()
            .map(|&(lo, hi)| {
                let mut mn = v3(f32::MAX, f32::MAX, f32::MAX);
                let mut mx = v3(f32::MIN, f32::MIN, f32::MIN);
                for &x in &[lo.x, hi.x] {
                    for &y in &[lo.y, hi.y] {
                        for &z in &[lo.z, hi.z] {
                            let p = self.world_point(v3(x, y, z));
                            mn = v3(mn.x.min(p.x), mn.y.min(p.y), mn.z.min(p.z));
                            mx = v3(mx.x.max(p.x), mx.y.max(p.y), mx.z.max(p.z));
                        }
                    }
                }
                Solid { min: mn, max: mx }
            })
            .collect()
    }
}

pub fn placements() -> Vec<Placement> {
    let sofa_box = (v3(-1.0, -0.301, -0.392), v3(1.0, 0.301, 0.392));
    let dresser_box = (v3(-41.714, 0.0, 0.0), v3(41.714, 85.128, 57.634));
    let bed_box = (v3(-5.44, 1.6, -17.68), v3(46.5, 13.2, 6.65));
    let bed_anchor = v3(20.53, 1.6, -5.5);
    let dresser = |name: &'static str, pos: V3, quarters: i32| Placement {
        name,
        file: "models/trim_sheet__example_dresser.glb",
        scale: 0.01,
        anchor: v3(0.0, 0.0, 0.0),
        pos,
        quarters,
        boxes: vec![dresser_box],
    };
    let bed = |name: &'static str, pos: V3| Placement {
        name,
        file: "models/bed.glb",
        scale: 0.0425,
        anchor: bed_anchor,
        pos,
        quarters: -1,
        boxes: vec![bed_box],
    };
    vec![
        Placement {
            name: "living_sofa",
            file: "models/modern__sofa.glb",
            scale: 1.2,
            anchor: v3(0.0, -0.301, 0.0),
            pos: v3(7.5, 0.0, 1.9),
            quarters: 1,
            boxes: vec![sofa_box],
        },
        dresser("hall_dresser", v3(6.79, 0.0, 2.75), 3),
        dresser("mb_dresser", v3(12.68, 2.8, 5.9), 3),
        bed("mb_bed", v3(10.4, 2.8, 7.64)),
        Placement {
            name: "kitchen",
            file: "models/wooden_kitchen.glb",
            scale: 1.0,
            anchor: v3(0.78, 0.0, -0.28),
            pos: v3(12.7, 0.0, 8.8),
            quarters: 2,
            boxes: vec![
                (v3(0.78, 0.0, -0.28), v3(4.22, 2.4, 0.47)),
                (v3(0.78, 0.0, 0.47), v3(1.55, 1.05, 1.85)),
            ],
        },
    ]
}
