use bevy::prelude::*;

#[derive(Clone, Copy, Debug)]
pub enum M {
    WallPaper,
    Wainscot,
    WallExt,
    Plinth,
    Concrete,
    Plank1,
    Plank2,
    Plank3,
    TileA,
    TileB,
    Ceiling,
    Trim,
    Casing,
    Roof,
    Glass,
    DoorWood,
    Stone,
    Asphalt,
    Sidewalk,
    Grass,
    StairWood,
    StairDark,
}

const ALL: [M; 22] = [
    M::WallPaper,
    M::Wainscot,
    M::WallExt,
    M::Plinth,
    M::Concrete,
    M::Plank1,
    M::Plank2,
    M::Plank3,
    M::TileA,
    M::TileB,
    M::Ceiling,
    M::Trim,
    M::Casing,
    M::Roof,
    M::Glass,
    M::DoorWood,
    M::Stone,
    M::Asphalt,
    M::Sidewalk,
    M::Grass,
    M::StairWood,
    M::StairDark,
];

fn solid(r: f32, g: f32, b: f32, rough: f32) -> StandardMaterial {
    StandardMaterial {
        base_color: Color::srgb(r, g, b),
        perceptual_roughness: rough,
        ..default()
    }
}

fn definition(m: M) -> StandardMaterial {
    match m {
        M::WallPaper => solid(0.34, 0.31, 0.26, 1.0),
        M::Wainscot => solid(0.15, 0.095, 0.065, 0.7),
        M::WallExt => solid(0.12, 0.13, 0.12, 0.9),
        M::Plinth => solid(0.20, 0.20, 0.19, 0.95),
        M::Concrete => solid(0.18, 0.18, 0.18, 0.95),
        M::Plank1 => solid(0.19, 0.11, 0.065, 0.5),
        M::Plank2 => solid(0.15, 0.09, 0.052, 0.5),
        M::Plank3 => solid(0.22, 0.13, 0.078, 0.5),
        M::TileA => solid(0.30, 0.31, 0.30, 0.25),
        M::TileB => solid(0.15, 0.16, 0.16, 0.25),
        M::Ceiling => solid(0.46, 0.44, 0.40, 1.0),
        M::Trim => solid(0.36, 0.32, 0.26, 0.6),
        M::Casing => solid(0.26, 0.26, 0.24, 0.7),
        M::Roof => solid(0.07, 0.07, 0.08, 0.9),
        M::Glass => StandardMaterial {
            base_color: Color::srgba(0.35, 0.45, 0.55, 0.10),
            perceptual_roughness: 0.05,
            alpha_mode: AlphaMode::Blend,
            ..default()
        },
        M::DoorWood => solid(0.15, 0.08, 0.05, 0.6),
        M::Stone => solid(0.25, 0.25, 0.24, 0.7),
        M::Asphalt => solid(0.055, 0.055, 0.065, 0.35),
        M::Sidewalk => solid(0.20, 0.20, 0.21, 0.9),
        M::Grass => solid(0.07, 0.09, 0.05, 1.0),
        M::StairWood => solid(0.17, 0.10, 0.06, 0.55),
        M::StairDark => solid(0.10, 0.06, 0.04, 0.6),
    }
}

pub struct Palette {
    handles: Vec<Handle<StandardMaterial>>,
}

impl Palette {
    pub fn new(materials: &mut Assets<StandardMaterial>) -> Self {
        let handles = ALL
            .iter()
            .enumerate()
            .map(|(i, m)| {
                debug_assert_eq!(*m as usize, i);
                materials.add(definition(*m))
            })
            .collect();
        Self { handles }
    }

    pub fn get(&self, m: M) -> Handle<StandardMaterial> {
        self.handles[m as usize].clone()
    }
}
