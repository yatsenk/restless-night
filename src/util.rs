use bevy::prelude::*;

pub type Rgba = [f32; 4];

pub fn lin(r: f32, g: f32, b: f32) -> Rgba {
    let c = Color::srgb(r, g, b).to_linear();
    [c.red, c.green, c.blue, 1.0]
}

pub fn lerp_col(a: Rgba, b: Rgba, t: f32) -> Rgba {
    [
        a[0] + (b[0] - a[0]) * t,
        a[1] + (b[1] - a[1]) * t,
        a[2] + (b[2] - a[2]) * t,
        1.0,
    ]
}

pub fn scale_col(c: Rgba, k: f32) -> Rgba {
    [c[0] * k, c[1] * k, c[2] * k, 1.0]
}

#[derive(Resource)]
pub struct GameRng(pub u64);

impl GameRng {
    pub fn next_u64(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        x.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }

    pub fn f32(&mut self) -> f32 {
        (self.next_u64() >> 40) as f32 / (1u64 << 24) as f32
    }

    pub fn range(&mut self, a: f32, b: f32) -> f32 {
        a + (b - a) * self.f32()
    }

    pub fn pick(&mut self, n: usize) -> usize {
        (self.next_u64() % n as u64) as usize
    }

    pub fn chance(&mut self, p: f32) -> bool {
        self.f32() < p
    }

    pub fn seeded(seed: u64) -> Self {
        let mut r = GameRng(seed | 1);
        for _ in 0..4 {
            r.next_u64();
        }
        r
    }
}

pub fn chunk_seed(c: IVec2) -> u64 {
    let mut h = (c.x as i64 as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15)
        ^ (c.y as i64 as u64).wrapping_mul(0xC2B2_AE3D_27D4_EB4F);
    h ^= h >> 29;
    h = h.wrapping_mul(0xBF58_476D_1CE4_E5B9);
    h ^= h >> 32;
    h | 1
}

pub fn hash2(x: i32, z: i32, s: u32) -> f32 {
    let mut h = (x as u32).wrapping_mul(0x27d4_eb2d)
        ^ (z as u32).wrapping_mul(0x1656_67b1)
        ^ s.wrapping_mul(0x9e37_79b1);
    h ^= h >> 15;
    h = h.wrapping_mul(0x85eb_ca6b);
    h ^= h >> 13;
    h = h.wrapping_mul(0xc2b2_ae35);
    h ^= h >> 16;
    (h & 0xffff) as f32 / 65535.0
}

pub fn vnoise(x: f32, z: f32, s: u32) -> f32 {
    let xi = x.floor();
    let zi = z.floor();
    let fx = x - xi;
    let fz = z - zi;
    let sx = fx * fx * (3.0 - 2.0 * fx);
    let sz = fz * fz * (3.0 - 2.0 * fz);
    let (xi, zi) = (xi as i32, zi as i32);
    let a = hash2(xi, zi, s);
    let b = hash2(xi + 1, zi, s);
    let c = hash2(xi, zi + 1, s);
    let d = hash2(xi + 1, zi + 1, s);
    let top = a + (b - a) * sx;
    let bot = c + (d - c) * sx;
    top + (bot - top) * sz
}

pub fn fbm(x: f32, z: f32, s: u32) -> f32 {
    let mut sum = 0.0;
    let mut amp = 0.5;
    let mut f = 1.0;
    let mut norm = 0.0;
    for o in 0..4 {
        sum += vnoise(x * f, z * f, s.wrapping_add(o * 101)) * amp;
        norm += amp;
        amp *= 0.5;
        f *= 2.0;
    }
    sum / norm
}
