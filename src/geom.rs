use crate::math::{v3, V3};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Mat {
    Floor,
    Paper,
    Plaster,
    Tile,
    Stucco,
    Concrete,
    Trim,
    Wood,
    Roof,
    Door,
    Glass,
    Frame,
}

pub const MAT_COUNT: usize = 12;

impl Mat {
    pub const ALL: [Mat; MAT_COUNT] = [
        Mat::Floor,
        Mat::Paper,
        Mat::Plaster,
        Mat::Tile,
        Mat::Stucco,
        Mat::Concrete,
        Mat::Trim,
        Mat::Wood,
        Mat::Roof,
        Mat::Door,
        Mat::Glass,
        Mat::Frame,
    ];

    pub fn index(self) -> usize {
        self as usize
    }

    pub fn uv_scale(self) -> f32 {
        match self {
            Mat::Floor => 0.62,
            Mat::Paper => 0.5,
            Mat::Plaster => 0.35,
            Mat::Tile => 1.4,
            Mat::Stucco => 0.35,
            Mat::Concrete => 0.5,
            Mat::Trim => 0.8,
            Mat::Wood => 0.9,
            Mat::Roof => 0.8,
            Mat::Door => 0.9,
            Mat::Glass => 1.0,
            Mat::Frame => 1.2,
        }
    }
}

pub const SKIP_NX: u8 = 1;
pub const SKIP_PX: u8 = 2;
pub const SKIP_NY: u8 = 4;
pub const SKIP_PY: u8 = 8;
pub const SKIP_NZ: u8 = 16;
pub const SKIP_PZ: u8 = 32;

#[derive(Clone, Default, Debug)]
pub struct MeshData {
    pub pos: Vec<[f32; 3]>,
    pub nor: Vec<[f32; 3]>,
    pub uv: Vec<[f32; 2]>,
    pub idx: Vec<u32>,
}

impl MeshData {
    pub fn triangles(&self) -> usize {
        self.idx.len() / 3
    }
}

#[derive(Clone, Debug)]
pub struct LoggedQuad {
    pub mat: Mat,
    pub p: [V3; 4],
    pub n: V3,
}

#[derive(Clone, Debug)]
pub struct Builder {
    pub meshes: Vec<MeshData>,
    log: Option<Vec<LoggedQuad>>,
}

impl Builder {
    pub fn new() -> Self {
        Builder {
            meshes: vec![MeshData::default(); MAT_COUNT],
            log: None,
        }
    }

    pub fn start_log(&mut self) {
        self.log = Some(Vec::new());
    }

    pub fn take_log(&mut self) -> Vec<LoggedQuad> {
        self.log.take().unwrap_or_default()
    }

    fn uv_for(m: Mat, p: V3, n: V3) -> [f32; 2] {
        let s = m.uv_scale();
        let ax = n.x.abs();
        let ay = n.y.abs();
        let az = n.z.abs();
        if ay >= ax && ay >= az {
            [p.x * s, p.z * s]
        } else if ax >= az {
            [p.z * s, -p.y * s]
        } else {
            [p.x * s, -p.y * s]
        }
    }

    pub fn quad(&mut self, m: Mat, a: V3, b: V3, c: V3, d: V3, n: V3) {
        if let Some(l) = self.log.as_mut() {
            l.push(LoggedQuad {
                mat: m,
                p: [a, b, c, d],
                n: n.norm(),
            });
        }
        let fnorm = (b - a).cross(c - a);
        let flip = fnorm.dot(n) < 0.0;
        let mesh = &mut self.meshes[m.index()];
        let base = mesh.pos.len() as u32;
        let nn = n.norm().arr();
        for p in [a, b, c, d] {
            mesh.pos.push(p.arr());
            mesh.nor.push(nn);
            mesh.uv.push(Self::uv_for(m, p, n));
        }
        if flip {
            mesh.idx
                .extend_from_slice(&[base, base + 2, base + 1, base, base + 3, base + 2]);
        } else {
            mesh.idx
                .extend_from_slice(&[base, base + 1, base + 2, base, base + 2, base + 3]);
        }
    }

    pub fn tri(&mut self, m: Mat, a: V3, b: V3, c: V3, n: V3) {
        let fnorm = (b - a).cross(c - a);
        let flip = fnorm.dot(n) < 0.0;
        let mesh = &mut self.meshes[m.index()];
        let base = mesh.pos.len() as u32;
        let nn = n.norm().arr();
        for p in [a, b, c] {
            mesh.pos.push(p.arr());
            mesh.nor.push(nn);
            mesh.uv.push(Self::uv_for(m, p, n));
        }
        if flip {
            mesh.idx.extend_from_slice(&[base, base + 2, base + 1]);
        } else {
            mesh.idx.extend_from_slice(&[base, base + 1, base + 2]);
        }
    }

    pub fn fx(&mut self, m: Mat, x: f32, y0: f32, y1: f32, z0: f32, z1: f32, dir: f32) {
        if y1 <= y0 || z1 <= z0 {
            return;
        }
        self.quad(
            m,
            v3(x, y0, z0),
            v3(x, y1, z0),
            v3(x, y1, z1),
            v3(x, y0, z1),
            v3(dir, 0.0, 0.0),
        );
    }

    pub fn fy(&mut self, m: Mat, y: f32, x0: f32, x1: f32, z0: f32, z1: f32, dir: f32) {
        if x1 <= x0 || z1 <= z0 {
            return;
        }
        self.quad(
            m,
            v3(x0, y, z0),
            v3(x1, y, z0),
            v3(x1, y, z1),
            v3(x0, y, z1),
            v3(0.0, dir, 0.0),
        );
    }

    pub fn fz(&mut self, m: Mat, z: f32, x0: f32, x1: f32, y0: f32, y1: f32, dir: f32) {
        if x1 <= x0 || y1 <= y0 {
            return;
        }
        self.quad(
            m,
            v3(x0, y0, z),
            v3(x1, y0, z),
            v3(x1, y1, z),
            v3(x0, y1, z),
            v3(0.0, 0.0, dir),
        );
    }

    pub fn cuboid(&mut self, m: Mat, min: V3, max: V3, skip: u8) {
        if max.x <= min.x || max.y <= min.y || max.z <= min.z {
            return;
        }
        if skip & SKIP_NX == 0 {
            self.fx(m, min.x, min.y, max.y, min.z, max.z, -1.0);
        }
        if skip & SKIP_PX == 0 {
            self.fx(m, max.x, min.y, max.y, min.z, max.z, 1.0);
        }
        if skip & SKIP_NY == 0 {
            self.fy(m, min.y, min.x, max.x, min.z, max.z, -1.0);
        }
        if skip & SKIP_PY == 0 {
            self.fy(m, max.y, min.x, max.x, min.z, max.z, 1.0);
        }
        if skip & SKIP_NZ == 0 {
            self.fz(m, min.z, min.x, max.x, min.y, max.y, -1.0);
        }
        if skip & SKIP_PZ == 0 {
            self.fz(m, max.z, min.x, max.x, min.y, max.y, 1.0);
        }
    }

    pub fn beam(&mut self, m: Mat, a: V3, b: V3, w: f32, h: f32) {
        let d = (b - a).norm();
        let up = if d.y.abs() > 0.98 {
            v3(1.0, 0.0, 0.0)
        } else {
            v3(0.0, 1.0, 0.0)
        };
        let s = d.cross(up).norm();
        let u = s.cross(d).norm();
        let hw = s * (w * 0.5);
        let hh = u * (h * 0.5);
        let c = |p: V3, sx: f32, sy: f32| p + hw * sx + hh * sy;
        let a0 = c(a, -1.0, -1.0);
        let a1 = c(a, 1.0, -1.0);
        let a2 = c(a, 1.0, 1.0);
        let a3 = c(a, -1.0, 1.0);
        let b0 = c(b, -1.0, -1.0);
        let b1 = c(b, 1.0, -1.0);
        let b2 = c(b, 1.0, 1.0);
        let b3 = c(b, -1.0, 1.0);
        self.quad(m, a0, a1, b1, b0, -u);
        self.quad(m, a3, a2, b2, b3, u);
        self.quad(m, a0, a3, b3, b0, -s);
        self.quad(m, a1, a2, b2, b1, s);
        self.quad(m, a0, a1, a2, a3, -d);
        self.quad(m, b0, b1, b2, b3, d);
    }

    pub fn grid_y(
        &mut self,
        m: Mat,
        y: f32,
        xs: &[f32],
        zs: &[f32],
        hole: Option<(usize, usize)>,
        dir: f32,
    ) {
        for i in 0..xs.len() - 1 {
            for j in 0..zs.len() - 1 {
                if hole == Some((i, j)) {
                    continue;
                }
                self.fy(m, y, xs[i], xs[i + 1], zs[j], zs[j + 1], dir);
            }
        }
    }

    pub fn total_triangles(&self) -> usize {
        self.meshes.iter().map(|m| m.triangles()).sum()
    }
}
