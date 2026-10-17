use bevy::{
    light::{CascadeShadowConfigBuilder, FogVolume, VolumetricLight},
    prelude::*,
};

use crate::layout::Flicker;
use crate::player::PlayerCamera;
use crate::world::{HouseRes, to_vec3};

pub const MOON_LUX: f32 = 22.0;
pub const AMBIENT_BRIGHTNESS: f32 = 1.3;
pub const FOG_DENSITY: f32 = 0.034;
pub const SHADOWED_ROOM_LIGHTS: usize = 4;

#[derive(Component)]
pub struct RoomLight {
    pub base: f32,
    pub flicker: Flicker,
    pub phase: f32,
}

pub fn setup(mut commands: Commands, house: Res<HouseRes>) {
    commands.insert_resource(ClearColor(Color::srgb(0.012, 0.016, 0.024)));
    commands.insert_resource(GlobalAmbientLight {
        color: Color::srgb(0.5, 0.6, 0.92),
        brightness: AMBIENT_BRIGHTNESS,
        ..default()
    });

    commands.spawn((
        DirectionalLight {
            color: Color::srgb(0.55, 0.68, 1.0),
            illuminance: MOON_LUX,
            shadow_maps_enabled: true,
            ..default()
        },
        VolumetricLight,
        Transform::default().looking_to(Vec3::new(0.45, -0.55, 0.7), Vec3::Y),
        CascadeShadowConfigBuilder {
            num_cascades: 3,
            minimum_distance: 0.1,
            maximum_distance: 40.0,
            first_cascade_far_bound: 7.0,
            ..default()
        }
        .build(),
    ));

    commands.spawn((
        FogVolume {
            fog_color: Color::srgb(0.55, 0.64, 0.8),
            density_factor: 0.03,
            ..default()
        },
        Transform::from_xyz(8.0, 3.3, 6.0).with_scale(Vec3::new(60.0, 16.0, 60.0)),
    ));

    for (i, l) in house.0.lights.iter().enumerate() {
        commands.spawn((
            PointLight {
                color: Color::srgb(l.color[0], l.color[1], l.color[2]),
                intensity: l.lumens,
                range: l.range,
                shadow_maps_enabled: false,
                radius: 0.08,
                ..default()
            },
            Transform::from_translation(to_vec3(l.pos)),
            RoomLight {
                base: l.lumens,
                flicker: l.flicker,
                phase: i as f32 * 7.31,
            },
        ));
    }
}

fn hash(x: f32) -> f32 {
    ((x * 12.9898).sin() * 43758.547).fract().abs()
}

fn pattern(f: Flicker, t: f32, phase: f32) -> f32 {
    match f {
        Flicker::Steady => 1.0,
        Flicker::Breathe => {
            1.0 + 0.07 * (t * 0.9 + phase).sin() + 0.035 * (t * 2.7 + phase * 1.7).sin()
        }
        Flicker::Sputter => {
            let slot = (t * 13.0 + phase).floor();
            let blip = if hash(slot) > 0.9 { 0.25 } else { 1.0 };
            let outage_slot = (t * 0.28 + phase).floor();
            let outage_phase = (t * 0.28 + phase).fract();
            let outage = if hash(outage_slot * 3.7) > 0.86 && outage_phase < 0.16 {
                0.04
            } else {
                1.0
            };
            (0.92 + 0.08 * (t * 3.1 + phase).sin()) * blip * outage
        }
        Flicker::Fluorescent => {
            let cyc = (t * 0.45 + phase).floor();
            let in_cycle = (t * 0.45 + phase).fract();
            if hash(cyc * 1.9) > 0.7 && in_cycle < 0.22 {
                if hash((t * 34.0).floor() + phase) > 0.45 {
                    1.0
                } else {
                    0.12
                }
            } else {
                0.94 + 0.04 * (t * 100.0).sin()
            }
        }
    }
}

pub fn flicker(time: Res<Time>, mut q: Query<(&RoomLight, &mut PointLight)>) {
    let t = time.elapsed_secs();
    for (r, mut l) in &mut q {
        l.intensity = r.base * pattern(r.flicker, t, r.phase);
    }
}

pub fn shadow_budget(
    cam: Single<&Transform, With<PlayerCamera>>,
    mut q: Query<(Entity, &GlobalTransform, &mut PointLight), With<RoomLight>>,
) {
    let p = cam.translation;
    let mut order: Vec<(f32, Entity)> = q
        .iter()
        .map(|(e, g, _)| (g.translation().distance_squared(p), e))
        .collect();
    order.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    for (e, _, mut l) in &mut q {
        let rank = order
            .iter()
            .position(|(_, o)| *o == e)
            .unwrap_or(usize::MAX);
        let want = rank < SHADOWED_ROOM_LIGHTS;
        if l.shadow_maps_enabled != want {
            l.shadow_maps_enabled = want;
        }
    }
}
