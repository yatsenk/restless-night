use bevy::math::Vec3;

pub const RADIUS: f32 = 0.28;
pub const HEIGHT: f32 = 1.75;
pub const STEP: f32 = 0.31;
const GRAVITY: f32 = 18.0;

#[derive(Clone, Copy, Debug)]
pub struct Solid {
    pub min: Vec3,
    pub max: Vec3,
}

fn touches(p: Vec3, s: &Solid) -> bool {
    let cx = p.x.clamp(s.min.x, s.max.x);
    let cz = p.z.clamp(s.min.z, s.max.z);
    let dx = p.x - cx;
    let dz = p.z - cz;
    dx * dx + dz * dz < RADIUS * RADIUS
}

pub fn blocked(p: Vec3, solids: &[Solid]) -> bool {
    let head = p.y + HEIGHT;
    solids
        .iter()
        .any(|s| s.max.y > p.y + STEP && s.min.y < head && touches(p, s))
}

pub fn ground_height(p: Vec3, solids: &[Solid]) -> Option<f32> {
    solids
        .iter()
        .filter(|s| s.max.y <= p.y + STEP && touches(p, s))
        .map(|s| s.max.y)
        .reduce(f32::max)
}

pub fn step(
    mut pos: Vec3,
    mut vel_y: f32,
    delta: Vec3,
    dt: f32,
    solids: &[Solid],
) -> (Vec3, f32, f32) {
    let nx = Vec3::new(pos.x + delta.x, pos.y, pos.z);
    if !blocked(nx, solids) {
        pos = nx;
    }
    let nz = Vec3::new(pos.x, pos.y, pos.z + delta.z);
    if !blocked(nz, solids) {
        pos = nz;
    }

    let mut snapped = 0.0;
    if let Some(ground) = ground_height(pos, solids) {
        if pos.y > ground + 0.005 {
            vel_y -= GRAVITY * dt;
            pos.y += vel_y * dt;
            if pos.y <= ground {
                pos.y = ground;
                vel_y = 0.0;
            }
        } else {
            snapped = (ground - pos.y).max(0.0);
            pos.y = ground;
            vel_y = 0.0;
        }
    }
    (pos, vel_y, snapped)
}
