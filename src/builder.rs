use bevy::prelude::*;

use crate::collision::Solid;

pub const WALL_T: f32 = 0.2;

#[derive(Clone, Copy)]
pub enum Mat {
    Plaster,
    Wood,
    DarkWood,
    Ceiling,
    Fabric,
    Ceramic,
    Metal,
    Glass,
    Rug,
    Roof,
    Asphalt,
    Pavement,
    Soil,
    Bark,
    Leaf(u8),
}

pub struct Palette {
    plaster: Handle<StandardMaterial>,
    wood: Handle<StandardMaterial>,
    dark_wood: Handle<StandardMaterial>,
    ceiling: Handle<StandardMaterial>,
    fabric: Handle<StandardMaterial>,
    ceramic: Handle<StandardMaterial>,
    metal: Handle<StandardMaterial>,
    glass: Handle<StandardMaterial>,
    rug: Handle<StandardMaterial>,
    roof: Handle<StandardMaterial>,
    asphalt: Handle<StandardMaterial>,
    pavement: Handle<StandardMaterial>,
    soil: Handle<StandardMaterial>,
    bark: Handle<StandardMaterial>,
    leaf: [Handle<StandardMaterial>; 3],
}

impl Palette {
    fn new(materials: &mut Assets<StandardMaterial>) -> Self {
        let glass = materials.add(StandardMaterial {
            base_color: Color::srgba(0.45, 0.55, 0.65, 0.12),
            alpha_mode: AlphaMode::Blend,
            perceptual_roughness: 0.05,
            ..default()
        });
        let mut mk = |r: f32, g: f32, b: f32, rough: f32, metal: f32| {
            materials.add(StandardMaterial {
                base_color: Color::srgb(r, g, b),
                perceptual_roughness: rough,
                metallic: metal,
                ..default()
            })
        };
        Self {
            plaster: mk(0.34, 0.33, 0.30, 0.95, 0.0),
            wood: mk(0.20, 0.14, 0.09, 0.6, 0.0),
            dark_wood: mk(0.12, 0.09, 0.07, 0.7, 0.0),
            ceiling: mk(0.38, 0.37, 0.35, 0.95, 0.0),
            fabric: mk(0.17, 0.20, 0.22, 1.0, 0.0),
            ceramic: mk(0.55, 0.56, 0.55, 0.3, 0.0),
            metal: mk(0.35, 0.36, 0.38, 0.4, 0.8),
            glass,
            rug: mk(0.25, 0.07, 0.06, 1.0, 0.0),
            roof: mk(0.10, 0.10, 0.11, 0.9, 0.0),
            asphalt: mk(0.07, 0.07, 0.08, 0.9, 0.0),
            pavement: mk(0.22, 0.22, 0.22, 0.95, 0.0),
            soil: mk(0.07, 0.065, 0.045, 1.0, 0.0),
            bark: mk(0.08, 0.06, 0.05, 1.0, 0.0),
            leaf: [
                mk(0.30, 0.12, 0.04, 1.0, 0.0),
                mk(0.28, 0.17, 0.05, 1.0, 0.0),
                mk(0.17, 0.08, 0.04, 1.0, 0.0),
            ],
        }
    }

    fn get(&self, m: Mat) -> Handle<StandardMaterial> {
        match m {
            Mat::Plaster => self.plaster.clone(),
            Mat::Wood => self.wood.clone(),
            Mat::DarkWood => self.dark_wood.clone(),
            Mat::Ceiling => self.ceiling.clone(),
            Mat::Fabric => self.fabric.clone(),
            Mat::Ceramic => self.ceramic.clone(),
            Mat::Metal => self.metal.clone(),
            Mat::Glass => self.glass.clone(),
            Mat::Rug => self.rug.clone(),
            Mat::Roof => self.roof.clone(),
            Mat::Asphalt => self.asphalt.clone(),
            Mat::Pavement => self.pavement.clone(),
            Mat::Soil => self.soil.clone(),
            Mat::Bark => self.bark.clone(),
            Mat::Leaf(i) => self.leaf[i as usize % 3].clone(),
        }
    }
}

#[derive(Clone, Copy)]
pub enum Axis {
    X,
    Z,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Kind {
    Window,
    Door,
    Sealed,
}

#[derive(Clone, Copy)]
pub struct Opening {
    pub from: f32,
    pub to: f32,
    pub sill: f32,
    pub top: f32,
    pub kind: Kind,
}

impl Opening {
    pub const fn window(from: f32, to: f32) -> Self {
        Self { from, to, sill: 0.9, top: 2.1, kind: Kind::Window }
    }

    pub const fn high(from: f32, to: f32) -> Self {
        Self { from, to, sill: 1.4, top: 2.1, kind: Kind::Window }
    }

    pub const fn door(from: f32, to: f32) -> Self {
        Self { from, to, sill: 0.0, top: 2.2, kind: Kind::Door }
    }

    pub const fn sealed(from: f32, to: f32) -> Self {
        Self { from, to, sill: 0.0, top: 2.2, kind: Kind::Sealed }
    }
}

pub struct Builder<'a, 'w, 's> {
    commands: &'a mut Commands<'w, 's>,
    cube: Handle<Mesh>,
    pal: Palette,
    solids: Vec<Solid>,
}

impl<'a, 'w, 's> Builder<'a, 'w, 's> {
    pub fn new(
        commands: &'a mut Commands<'w, 's>,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<StandardMaterial>,
    ) -> Self {
        Self {
            commands,
            cube: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
            pal: Palette::new(materials),
            solids: Vec::new(),
        }
    }

    pub fn finish(self) -> Vec<Solid> {
        self.solids
    }

    pub fn oriented(&mut self, center: Vec3, size: Vec3, rotation: Quat, mat: Mat) {
        self.commands.spawn((
            Mesh3d(self.cube.clone()),
            MeshMaterial3d(self.pal.get(mat)),
            Transform {
                translation: center,
                rotation,
                scale: size,
            },
        ));
    }

    pub fn visual(&mut self, min: Vec3, max: Vec3, mat: Mat) {
        self.oriented((min + max) * 0.5, max - min, Quat::IDENTITY, mat);
    }

    pub fn solid_only(&mut self, min: Vec3, max: Vec3) {
        self.solids.push(Solid { min, max });
    }

    pub fn block(&mut self, min: Vec3, max: Vec3, mat: Mat) {
        self.visual(min, max, mat);
        self.solid_only(min, max);
    }

    pub fn item(&mut self, base: f32, x: [f32; 2], z: [f32; 2], height: f32, mat: Mat) {
        self.block(
            Vec3::new(x[0], base, z[0]),
            Vec3::new(x[1], base + height, z[1]),
            mat,
        );
    }

    fn span(axis: Axis, fixed: f32, half: f32, u0: f32, u1: f32, ya: f32, yb: f32) -> (Vec3, Vec3) {
        match axis {
            Axis::X => (
                Vec3::new(u0, ya, fixed - half),
                Vec3::new(u1, yb, fixed + half),
            ),
            Axis::Z => (
                Vec3::new(fixed - half, ya, u0),
                Vec3::new(fixed + half, yb, u1),
            ),
        }
    }

    fn wall_box(&mut self, axis: Axis, fixed: f32, u0: f32, u1: f32, ya: f32, yb: f32, mat: Mat) {
        if u1 - u0 < 1e-3 || yb - ya < 1e-3 {
            return;
        }
        let (min, max) = Self::span(axis, fixed, WALL_T * 0.5, u0, u1, ya, yb);
        self.block(min, max, mat);
    }

    #[allow(clippy::too_many_arguments)]
    pub fn wall(
        &mut self,
        axis: Axis,
        fixed: f32,
        a: f32,
        b: f32,
        y0: f32,
        y1: f32,
        openings: &[Opening],
        mat: Mat,
    ) {
        let mut sorted = openings.to_vec();
        sorted.sort_by(|p, q| p.from.total_cmp(&q.from));
        let mut cursor = a;
        for o in &sorted {
            self.wall_box(axis, fixed, cursor, o.from, y0, y1, mat);
            self.wall_box(axis, fixed, o.from, o.to, y0, y0 + o.sill, mat);
            self.wall_box(axis, fixed, o.from, o.to, y0 + o.top, y1, mat);
            match o.kind {
                Kind::Window => {
                    let (min, max) = Self::span(
                        axis,
                        fixed,
                        0.015,
                        o.from,
                        o.to,
                        y0 + o.sill,
                        y0 + o.top,
                    );
                    self.visual(min, max, Mat::Glass);
                    let (min, max) = Self::span(
                        axis,
                        fixed,
                        WALL_T * 0.5,
                        o.from,
                        o.to,
                        y0 + o.sill,
                        y0 + o.top,
                    );
                    self.solid_only(min, max);
                }
                Kind::Sealed => {
                    let (min, max) = Self::span(
                        axis,
                        fixed,
                        0.05,
                        o.from,
                        o.to,
                        y0,
                        y0 + o.top,
                    );
                    self.block(min, max, Mat::DarkWood);
                }
                Kind::Door => {}
            }
            cursor = o.to;
        }
        self.wall_box(axis, fixed, cursor, b, y0, y1, mat);
    }
}
