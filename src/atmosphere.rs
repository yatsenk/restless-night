use bevy::{light::CascadeShadowConfigBuilder, pbr::{DistanceFog, FogFalloff}, prelude::*};

pub struct AtmospherePlugin;

impl Plugin for AtmospherePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(fog_color()))
            .insert_resource(GlobalAmbientLight {
                color: Color::srgb(0.45, 0.55, 0.85),
                brightness: 14.0,
                ..default()
            })
            .add_systems(Startup, spawn_lights)
            .add_systems(Update, flicker);
    }
}

pub fn fog_color() -> Color {
    Color::srgb(0.055, 0.065, 0.085)
}

pub fn distance_fog() -> DistanceFog {
    DistanceFog {
        color: fog_color(),
        falloff: FogFalloff::ExponentialSquared { density: 0.045 },
        ..default()
    }
}

#[derive(Component)]
struct Flicker {
    base: f32,
    seed: f32,
}

struct Lamp {
    pos: [f32; 3],
    shadows: bool,
    flicker: bool,
}

const LAMPS: [Lamp; 10] = [
    Lamp { pos: [4.0, 2.6, 7.0], shadows: true, flicker: false },
    Lamp { pos: [11.0, 2.6, 7.0], shadows: true, flicker: false },
    Lamp { pos: [3.0, 2.6, 2.8], shadows: false, flicker: false },
    Lamp { pos: [8.0, 2.6, 2.0], shadows: false, flicker: false },
    Lamp { pos: [12.0, 2.6, 2.0], shadows: false, flicker: false },
    Lamp { pos: [4.0, 5.6, 7.0], shadows: true, flicker: false },
    Lamp { pos: [11.0, 5.6, 7.0], shadows: true, flicker: false },
    Lamp { pos: [3.0, 5.6, 2.8], shadows: true, flicker: true },
    Lamp { pos: [8.0, 5.6, 2.0], shadows: false, flicker: false },
    Lamp { pos: [12.0, 5.6, 2.0], shadows: false, flicker: false },
];

fn spawn_lights(mut commands: Commands) {
    commands.spawn((
        DirectionalLight {
            color: Color::srgb(0.55, 0.66, 1.0),
            illuminance: 650.0,
            shadow_maps_enabled: true,
            ..default()
        },
        CascadeShadowConfigBuilder {
            num_cascades: 3,
            first_cascade_far_bound: 5.0,
            maximum_distance: 60.0,
            ..default()
        }
        .build(),
        Transform::from_xyz(-10.0, 24.0, 40.0).looking_at(Vec3::new(7.0, 0.0, 5.0), Vec3::Y),
    ));

    let base = 70_000.0;
    for (i, lamp) in LAMPS.iter().enumerate() {
        let mut e = commands.spawn((
            PointLight {
                color: Color::srgb(1.0, 0.78, 0.52),
                intensity: base,
                range: 10.0,
                radius: 0.08,
                shadow_maps_enabled: lamp.shadows,
                ..default()
            },
            Transform::from_xyz(lamp.pos[0], lamp.pos[1], lamp.pos[2]),
        ));
        if lamp.flicker {
            e.insert(Flicker { base, seed: i as f32 * 1.7 });
        }
    }

    for x in [12.0, -22.0, 46.0] {
        commands.spawn((
            PointLight {
                color: Color::srgb(1.0, 0.62, 0.28),
                intensity: 90_000.0,
                range: 16.0,
                radius: 0.1,
                shadow_maps_enabled: false,
                ..default()
            },
            Transform::from_xyz(x, 4.5, 18.2),
        ));
    }
}

fn flicker(time: Res<Time>, mut lights: Query<(&Flicker, &mut PointLight)>) {
    let t = time.elapsed_secs();
    for (f, mut light) in &mut lights {
        let slow = (t * 1.3 + f.seed).sin();
        let fast = (t * 37.0 + f.seed * 3.0).sin() * (t * 11.0 + f.seed).sin();
        let dip = if slow > 0.82 && fast > 0.2 { 0.25 } else { 1.0 };
        light.intensity = f.base * dip * (0.96 + 0.04 * fast);
    }
}
