use std::f32::consts::{PI, TAU};

use bevy::{
    prelude::*,
    render::{mesh::Indices, render_asset::RenderAssetUsages, render_resource::PrimitiveTopology},
};

use crate::util::{Rgba, scale_col};

#[derive(Default)]
pub struct MeshBuilder {
    pub pos: Vec<[f32; 3]>,
    pub col: Vec<Rgba>,
    pub idx: Vec<u32>,
}

impl MeshBuilder {
    pub fn vert(&mut self, p: Vec3, c: Rgba) -> u32 {
        self.pos.push(p.to_array());
        self.col.push(c);
        (self.pos.len() - 1) as u32
    }

    pub fn tri(&mut self, a: u32, b: u32, c: u32) {
        self.idx.extend_from_slice(&[a, b, c]);
    }

    pub fn tri3(&mut self, a: Vec3, b: Vec3, c: Vec3, col: Rgba) {
        let ia = self.vert(a, col);
        let ib = self.vert(b, col);
        let ic = self.vert(c, col);
        self.tri(ia, ib, ic);
    }

    pub fn tri_out(&mut self, a: Vec3, b: Vec3, c: Vec3, out: Vec3, col: Rgba) {
        if (b - a).cross(c - a).dot(out) < 0.0 {
            self.tri3(a, c, b, col);
        } else {
            self.tri3(a, b, c, col);
        }
    }

    pub fn quad(&mut self, a: Vec3, b: Vec3, c: Vec3, d: Vec3, col: Rgba) {
        let ia = self.vert(a, col);
        let ib = self.vert(b, col);
        let ic = self.vert(c, col);
        let id = self.vert(d, col);
        self.tri(ia, ib, ic);
        self.tri(ia, ic, id);
    }

    pub fn tri3c(&mut self, a: Vec3, b: Vec3, c: Vec3, ca: Rgba, cb: Rgba, cc: Rgba) {
        let ia = self.vert(a, ca);
        let ib = self.vert(b, cb);
        let ic = self.vert(c, cc);
        self.tri(ia, ib, ic);
    }

    pub fn quad_c(&mut self, p: [Vec3; 4], c: [Rgba; 4], out: Vec3) {
        let flip = (p[1] - p[0]).cross(p[2] - p[0]).dot(out) < 0.0;
        let order: [usize; 4] = if flip { [0, 3, 2, 1] } else { [0, 1, 2, 3] };
        let i: Vec<u32> = order.iter().map(|&k| self.vert(p[k], c[k])).collect();
        self.tri(i[0], i[1], i[2]);
        self.tri(i[0], i[2], i[3]);
    }

    pub fn quad_out(&mut self, a: Vec3, b: Vec3, c: Vec3, d: Vec3, out: Vec3, col: Rgba) {
        if (b - a).cross(c - a).dot(out) < 0.0 {
            self.quad(a, d, c, b, col);
        } else {
            self.quad(a, b, c, d, col);
        }
    }

    pub fn cuboid(&mut self, min: Vec3, max: Vec3, col: Rgba) {
        let c = (min + max) * 0.5;
        let h = (max - min) * 0.5;
        self.obox(c, Vec3::X, Vec3::Z, h, col);
    }

    pub fn obox(&mut self, c: Vec3, t: Vec3, n: Vec3, half: Vec3, col: Rgba) {
        let up = n.cross(t).normalize_or_zero();
        let up = if up.dot(Vec3::Y) < 0.0 { -up } else { up };
        let axes = [t, up, n];
        let hs = [half.x, half.y, half.z];
        for ai in 0..3 {
            let a1 = (ai + 1) % 3;
            let a2 = (ai + 2) % 3;
            for s in [-1.0_f32, 1.0] {
                let fc = c + axes[ai] * (s * hs[ai]);
                let u = axes[a1] * hs[a1];
                let v = axes[a2] * hs[a2];
                self.quad_out(
                    fc - u - v,
                    fc + u - v,
                    fc + u + v,
                    fc - u + v,
                    axes[ai] * s,
                    col,
                );
            }
        }
    }

    pub fn pyramid(&mut self, base_c: Vec3, hx: f32, hz: f32, height: f32, col: Rgba) {
        let apex = base_c + Vec3::Y * height;
        let p = [
            base_c + Vec3::new(-hx, 0.0, -hz),
            base_c + Vec3::new(hx, 0.0, -hz),
            base_c + Vec3::new(hx, 0.0, hz),
            base_c + Vec3::new(-hx, 0.0, hz),
        ];
        for i in 0..4 {
            let j = (i + 1) % 4;
            let mid = (p[i] + p[j]) * 0.5 + Vec3::Y * height * 0.3 - base_c;
            self.tri_out(p[i], p[j], apex, mid, col);
        }
        self.quad_out(p[0], p[1], p[2], p[3], -Vec3::Y, col);
    }

    #[allow(clippy::too_many_arguments)]
    pub fn tube(&mut self, from: Vec3, to: Vec3, r0: f32, r1: f32, sides: u32, c0: Rgba, c1: Rgba) {
        let axis = to - from;
        let len = axis.length();
        if len < 1e-4 {
            return;
        }
        let rot = Quat::from_rotation_arc(Vec3::Y, axis / len);
        let base = self.pos.len() as u32;
        for i in 0..sides {
            let a = i as f32 / sides as f32 * TAU;
            let (s, c) = a.sin_cos();
            let local = Vec3::new(c, 0.0, s);
            self.vert(from + rot * (local * r0), c0);
            self.vert(to + rot * (local * r1), c1);
        }
        for i in 0..sides {
            let j = (i + 1) % sides;
            let (b_i, t_i) = (base + i * 2, base + i * 2 + 1);
            let (b_j, t_j) = (base + j * 2, base + j * 2 + 1);
            self.tri(b_i, t_i, b_j);
            self.tri(b_j, t_i, t_j);
        }
    }

    pub fn disc(&mut self, center: Vec3, r: f32, sides: u32, up: bool, col: Rgba) {
        let c = self.vert(center, col);
        let start = self.pos.len() as u32;
        for i in 0..sides {
            let a = i as f32 / sides as f32 * TAU;
            self.vert(center + Vec3::new(a.cos() * r, 0.0, a.sin() * r), col);
        }
        for i in 0..sides {
            let j = (i + 1) % sides;
            if up {
                self.tri(c, start + j, start + i);
            } else {
                self.tri(c, start + i, start + j);
            }
        }
    }

    pub fn lathe(&mut self, profile: &[(f32, f32)], sides: u32, c0: Rgba, c1: Rgba) {
        let base = self.pos.len() as u32;
        let n = profile.len() as u32;
        for (k, (r, y)) in profile.iter().enumerate() {
            let t = k as f32 / (n - 1).max(1) as f32;
            let col = [
                c0[0] + (c1[0] - c0[0]) * t,
                c0[1] + (c1[1] - c0[1]) * t,
                c0[2] + (c1[2] - c0[2]) * t,
                1.0,
            ];
            for i in 0..sides {
                let a = i as f32 / sides as f32 * TAU;
                self.vert(Vec3::new(a.cos() * r, *y, a.sin() * r), col);
            }
        }
        for k in 0..n - 1 {
            for i in 0..sides {
                let j = (i + 1) % sides;
                let v00 = base + k * sides + i;
                let v01 = base + k * sides + j;
                let v10 = base + (k + 1) * sides + i;
                let v11 = base + (k + 1) * sides + j;
                self.tri(v00, v10, v01);
                self.tri(v01, v10, v11);
            }
        }
    }

    pub fn surface(&mut self, rings: u32, segs: u32, mut f: impl FnMut(f32, f32) -> (Vec3, Rgba)) {
        let base = self.pos.len() as u32;
        for r in 0..=rings {
            let theta = r as f32 / rings as f32 * PI;
            for s in 0..segs {
                let phi = s as f32 / segs as f32 * TAU;
                let (p, c) = f(theta, phi);
                self.vert(p, c);
            }
        }
        for r in 0..rings {
            for s in 0..segs {
                let s2 = (s + 1) % segs;
                let v00 = base + r * segs + s;
                let v01 = base + r * segs + s2;
                let v10 = base + (r + 1) * segs + s;
                let v11 = base + (r + 1) * segs + s2;
                self.tri(v00, v01, v10);
                self.tri(v01, v11, v10);
            }
        }
    }

    pub fn blob(
        &mut self,
        center: Vec3,
        radius: f32,
        squash: f32,
        c_dark: Rgba,
        c_light: Rgba,
        rings: u32,
        segs: u32,
        seed: f32,
    ) {
        self.surface(rings, segs, |theta, phi| {
            let d = Vec3::new(
                theta.sin() * phi.cos(),
                theta.cos(),
                theta.sin() * phi.sin(),
            );
            let n = 0.5 * (d.x * 3.2 + seed).sin() * (d.y * 2.7 + seed * 1.3).sin()
                + 0.35 * (d.z * 4.1 + seed * 0.7).sin() * (d.x * 2.3 + seed).cos();
            let rr = radius * (1.0 + 0.2 * n);
            let p = center + Vec3::new(d.x * rr, d.y * rr * squash, d.z * rr);
            let shade = (d.y * 0.5 + 0.5).clamp(0.0, 1.0);
            let col = [
                c_dark[0] + (c_light[0] - c_dark[0]) * shade,
                c_dark[1] + (c_light[1] - c_dark[1]) * shade,
                c_dark[2] + (c_light[2] - c_dark[2]) * shade,
                1.0,
            ];
            (p, scale_col(col, 0.9 + 0.2 * n))
        });
    }

    pub fn build(self) -> Mesh {
        let n = self.pos.len();
        let mut normals = vec![Vec3::ZERO; n];
        for t in self.idx.chunks_exact(3) {
            let (a, b, c) = (t[0] as usize, t[1] as usize, t[2] as usize);
            let pa = Vec3::from(self.pos[a]);
            let pb = Vec3::from(self.pos[b]);
            let pc = Vec3::from(self.pos[c]);
            let face = (pb - pa).cross(pc - pa);
            normals[a] += face;
            normals[b] += face;
            normals[c] += face;
        }
        let normals: Vec<[f32; 3]> = normals
            .into_iter()
            .map(|v| {
                if v.length_squared() > 1e-12 {
                    v.normalize().to_array()
                } else {
                    Vec3::Y.to_array()
                }
            })
            .collect();
        let uvs: Vec<[f32; 2]> = self
            .pos
            .iter()
            .zip(normals.iter())
            .map(|(p, nr)| {
                let (ax, ay, az) = (nr[0].abs(), nr[1].abs(), nr[2].abs());
                if ay >= ax && ay >= az {
                    [p[0] * 0.5, p[2] * 0.5]
                } else if ax >= az {
                    [p[2] * 0.5, p[1] * 0.5]
                } else {
                    [p[0] * 0.5, p[1] * 0.5]
                }
            })
            .collect();

        Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::default(),
        )
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, self.pos)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
        .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, self.col)
        .with_inserted_indices(Indices::U32(self.idx))
    }
}
