use bevy::prelude::*;

use crate::collision::Solid;
use crate::palette::{M, Palette};

#[derive(Clone, Copy)]
pub enum Axis {
    X,
    Z,
}

#[derive(Clone, Copy)]
pub struct Opening {
    pub a: f32,
    pub b: f32,
    pub bottom: f32,
    pub top: f32,
}

pub const fn op(a: f32, b: f32, bottom: f32, top: f32) -> Opening {
    Opening { a, b, bottom, top }
}

pub type Range = (f32, f32);

#[derive(Component)]
pub struct Flicker {
    pub base: f32,
    pub phase: f32,
}

pub struct Builder<'a, 'w, 's> {
    pub commands: &'a mut Commands<'w, 's>,
    pub pal: Palette,
    pub cube: Handle<Mesh>,
    pub cylinder: Handle<Mesh>,
    pub sphere: Handle<Mesh>,
    pub solids: Vec<Solid>,
}

impl<'a, 'w, 's> Builder<'a, 'w, 's> {
    pub fn new(
        commands: &'a mut Commands<'w, 's>,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<StandardMaterial>,
    ) -> Self {
        Self {
            pal: Palette::new(materials),
            cube: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
            cylinder: meshes.add(Cylinder::new(0.5, 1.0)),
            sphere: meshes.add(Sphere::new(0.5)),
            solids: Vec::new(),
            commands,
        }
    }

    pub fn put(&mut self, min: Vec3, max: Vec3, m: M, solid: bool) {
        let lo = min.min(max);
        let hi = min.max(max);
        self.commands.spawn((
            Mesh3d(self.cube.clone()),
            MeshMaterial3d(self.pal.get(m)),
            Transform::from_translation((lo + hi) * 0.5).with_scale(hi - lo),
        ));
        if solid {
            self.solids.push(Solid { min: lo, max: hi });
        }
    }

    pub fn collider(&mut self, x: Range, y: Range, z: Range) {
        self.solids.push(Solid {
            min: Vec3::new(x.0.min(x.1), y.0.min(y.1), z.0.min(z.1)),
            max: Vec3::new(x.0.max(x.1), y.0.max(y.1), z.0.max(z.1)),
        });
    }

    pub fn bx(&mut self, x: Range, y: Range, z: Range, m: M) {
        self.put(Vec3::new(x.0, y.0, z.0), Vec3::new(x.1, y.1, z.1), m, true);
    }

    pub fn gx(&mut self, x: Range, y: Range, z: Range, m: M) {
        self.put(Vec3::new(x.0, y.0, z.0), Vec3::new(x.1, y.1, z.1), m, false);
    }

    pub fn tilted(&mut self, center: Vec3, size: Vec3, rot: Quat, m: M) {
        self.commands.spawn((
            Mesh3d(self.cube.clone()),
            MeshMaterial3d(self.pal.get(m)),
            Transform::from_translation(center)
                .with_rotation(rot)
                .with_scale(size),
        ));
    }

    pub fn post(&mut self, x: f32, z: f32, y0: f32, y1: f32, r: f32, m: M, solid: bool) {
        self.commands.spawn((
            Mesh3d(self.cylinder.clone()),
            MeshMaterial3d(self.pal.get(m)),
            Transform::from_xyz(x, (y0 + y1) * 0.5, z).with_scale(Vec3::new(
                r * 2.0,
                (y1 - y0).abs(),
                r * 2.0,
            )),
        ));
        if solid {
            self.collider((x - r, x + r), (y0, y1), (z - r, z + r));
        }
    }

    pub fn ball(&mut self, c: Vec3, r: f32, m: M) {
        self.commands.spawn((
            Mesh3d(self.sphere.clone()),
            MeshMaterial3d(self.pal.get(m)),
            Transform::from_translation(c).with_scale(Vec3::splat(r * 2.0)),
        ));
    }

    pub fn ellipsoid(&mut self, c: Vec3, size: Vec3, m: M) {
        self.commands.spawn((
            Mesh3d(self.sphere.clone()),
            MeshMaterial3d(self.pal.get(m)),
            Transform::from_translation(c).with_scale(size),
        ));
    }

    pub fn light(
        &mut self,
        p: Vec3,
        color: Color,
        intensity: f32,
        range: f32,
        shadows: bool,
    ) -> Entity {
        self.commands
            .spawn((
                PointLight {
                    color,
                    intensity,
                    range,
                    shadow_maps_enabled: shadows,
                    ..default()
                },
                Transform::from_translation(p),
            ))
            .id()
    }

    pub fn flicker(&mut self, e: Entity, base: f32, phase: f32) {
        self.commands.entity(e).insert(Flicker { base, phase });
    }

    fn span(&mut self, axis: Axis, f: Range, a: f32, b: f32, y0: f32, y1: f32, m: M, solid: bool) {
        let (min, max) = match axis {
            Axis::X => (Vec3::new(a, y0, f.0), Vec3::new(b, y1, f.1)),
            Axis::Z => (Vec3::new(f.0, y0, a), Vec3::new(f.1, y1, b)),
        };
        self.put(min, max, m, solid);
    }

    pub fn wall(
        &mut self,
        axis: Axis,
        f: Range,
        t: Range,
        y0: f32,
        h: f32,
        m: M,
        solid: bool,
        openings: &[Opening],
    ) {
        let mut cur = t.0;
        for o in openings {
            if o.a > cur {
                self.span(axis, f, cur, o.a, y0, y0 + h, m, solid);
            }
            if o.bottom > 0.0 {
                self.span(axis, f, o.a, o.b, y0, y0 + o.bottom, m, solid);
            }
            if o.top < h {
                self.span(axis, f, o.a, o.b, y0 + o.top, y0 + h, m, solid);
            }
            cur = o.b;
        }
        if t.1 > cur {
            self.span(axis, f, cur, t.1, y0, y0 + h, m, solid);
        }
    }

    pub fn window(&mut self, axis: Axis, c: f32, y0: f32, o: Opening) {
        let lo = y0 + o.bottom;
        let hi = y0 + o.top;
        let mid_a = (o.a + o.b) * 0.5;
        let mid_y = (lo + hi) * 0.5;
        self.span(
            axis,
            (c - 0.01, c + 0.01),
            o.a,
            o.b,
            lo,
            hi,
            M::Glass,
            false,
        );
        let f = (c - 0.07, c + 0.07);
        self.span(axis, f, o.a, o.a + 0.05, lo, hi, M::Trim, false);
        self.span(axis, f, o.b - 0.05, o.b, lo, hi, M::Trim, false);
        self.span(axis, f, o.a, o.b, lo, lo + 0.05, M::Trim, false);
        self.span(axis, f, o.a, o.b, hi - 0.05, hi, M::Trim, false);
        let g = (c - 0.04, c + 0.04);
        self.span(axis, g, mid_a - 0.02, mid_a + 0.02, lo, hi, M::Trim, false);
        self.span(
            axis,
            g,
            o.a,
            o.b,
            mid_y - 0.02,
            mid_y + 0.02,
            M::Trim,
            false,
        );
    }

    pub fn sill(&mut self, axis: Axis, face: f32, dir: f32, y0: f32, o: Opening) {
        let y = y0 + o.bottom;
        self.span(
            axis,
            (face, face + dir * 0.12),
            o.a - 0.05,
            o.b + 0.05,
            y,
            y + 0.04,
            M::Trim,
            false,
        );
    }

    pub fn curtains(&mut self, axis: Axis, face: f32, dir: f32, y0: f32, o: Opening) {
        let f = (face + dir * 0.02, face + dir * 0.12);
        let top = y0 + (o.top + 0.25).min(2.6);
        let bottom = y0 + 0.1;
        self.span(
            axis,
            f,
            o.a - 0.45,
            o.a + 0.08,
            bottom,
            top,
            M::Curtain,
            false,
        );
        self.span(
            axis,
            f,
            o.b - 0.08,
            o.b + 0.45,
            bottom,
            top,
            M::Curtain,
            false,
        );
        self.span(
            axis,
            (face + dir * 0.01, face + dir * 0.14),
            o.a - 0.5,
            o.b + 0.5,
            top,
            top + 0.04,
            M::Trim,
            false,
        );
    }

    pub fn door_frame(&mut self, axis: Axis, c: f32, thickness: f32, y0: f32, o: Opening) {
        let f = (c - thickness * 0.5 - 0.02, c + thickness * 0.5 + 0.02);
        self.span(axis, f, o.a - 0.06, o.a, y0, y0 + o.top, M::Trim, false);
        self.span(axis, f, o.b, o.b + 0.06, y0, y0 + o.top, M::Trim, false);
        self.span(
            axis,
            f,
            o.a - 0.06,
            o.b + 0.06,
            y0 + o.top,
            y0 + o.top + 0.08,
            M::Trim,
            false,
        );
    }

    pub fn table_lamp(&mut self, x: f32, y: f32, z: f32, intensity: f32, shadows: bool) -> Entity {
        self.post(x, z, y, y + 0.22, 0.04, M::Metal, false);
        self.gx(
            (x - 0.12, x + 0.12),
            (y + 0.22, y + 0.46),
            (z - 0.12, z + 0.12),
            M::Bulb,
        );
        self.light(
            Vec3::new(x, y + 0.5, z),
            Color::srgb(1.0, 0.62, 0.32),
            intensity,
            8.0,
            shadows,
        )
    }

    pub fn floor_lamp(&mut self, x: f32, y: f32, z: f32, intensity: f32, shadows: bool) -> Entity {
        self.post(x, z, y, y + 1.45, 0.025, M::Metal, false);
        self.post(x, z, y, y + 0.03, 0.18, M::Metal, false);
        self.gx(
            (x - 0.16, x + 0.16),
            (y + 1.45, y + 1.78),
            (z - 0.16, z + 0.16),
            M::Bulb,
        );
        self.light(
            Vec3::new(x, y + 1.6, z),
            Color::srgb(1.0, 0.6, 0.3),
            intensity,
            9.0,
            shadows,
        )
    }

    pub fn chair(&mut self, x: f32, y: f32, z: f32, facing: u8, m: M) {
        let s = 0.42;
        self.bx((x, x + s), (y + 0.42, y + 0.47), (z, z + s), m);
        for (dx, dz) in [
            (0.02, 0.02),
            (s - 0.06, 0.02),
            (0.02, s - 0.06),
            (s - 0.06, s - 0.06),
        ] {
            self.gx(
                (x + dx, x + dx + 0.04),
                (y, y + 0.42),
                (z + dz, z + dz + 0.04),
                M::WoodDark,
            );
        }
        let (bx, bz) = match facing {
            0 => ((x, x + s), (z, z + 0.04)),
            1 => ((x, x + s), (z + s - 0.04, z + s)),
            2 => ((x, x + 0.04), (z, z + s)),
            _ => ((x + s - 0.04, x + s), (z, z + s)),
        };
        self.bx(bx, (y + 0.47, y + 0.92), bz, m);
    }

    pub fn table(&mut self, x: Range, y: f32, z: Range, h: f32, m: M) {
        self.bx(x, (y + h - 0.05, y + h), z, m);
        for (px, pz) in [
            (x.0 + 0.04, z.0 + 0.04),
            (x.1 - 0.1, z.0 + 0.04),
            (x.0 + 0.04, z.1 - 0.1),
            (x.1 - 0.1, z.1 - 0.1),
        ] {
            self.bx(
                (px, px + 0.06),
                (y, y + h - 0.05),
                (pz, pz + 0.06),
                M::WoodDark,
            );
        }
    }

    pub fn books(&mut self, axis: Axis, f: Range, a: f32, b: f32, y0: f32, y1: f32) {
        let step = 0.07;
        let mut t = a;
        let mut i = 0;
        while t + step < b {
            let m = match i % 3 {
                0 => M::Book1,
                1 => M::Book2,
                _ => M::Book3,
            };
            let hh = y0 + (y1 - y0) * (0.65 + 0.35 * (((i * 7) % 5) as f32 / 4.0));
            self.span(axis, f, t, t + step * 0.9, y0, hh, m, false);
            t += step;
            i += 1;
        }
    }

    pub fn railing(&mut self, x: Range, z: Range, y0: f32) {
        self.collider(x, (y0, y0 + 1.0), z);
        let along_x = (x.1 - x.0).abs() >= (z.1 - z.0).abs();
        let (a, b) = if along_x { x } else { z };
        let (c0, c1) = if along_x { z } else { x };
        let count = (((b - a) / 0.45).ceil() as i32).max(1);
        let axis = if along_x { Axis::X } else { Axis::Z };
        for i in 0..=count {
            let t = a + (b - a) * (i as f32 / count as f32);
            self.span(
                axis,
                (c0, c1),
                t - 0.025,
                t + 0.025,
                y0,
                y0 + 0.95,
                M::Trim,
                false,
            );
        }
        self.span(
            axis,
            (c0 - 0.02, c1 + 0.02),
            a,
            b,
            y0 + 0.95,
            y0 + 1.02,
            M::WoodDark,
            false,
        );
        self.span(axis, (c0, c1), a, b, y0 + 0.45, y0 + 0.5, M::Trim, false);
    }
}
