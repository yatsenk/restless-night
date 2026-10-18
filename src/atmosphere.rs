use bevy::{light::CascadeShadowConfigBuilder, prelude::*};

use crate::layout::Flicker;
use crate::player::PlayerCamera;
use crate::quality::Quality;
use crate::world::{to_vec3, HouseRes};

pub const SUN_LUX: f32 = 45.0;
pub const AMBIENT_BRIGHTNESS: f32 = 3.2;
pub const FOG_DENSITY: f32 = 0.055;

pub fn haze_color() -> Color {
    Color::srgb(0.34, 0.37, 0.42)
}

#[derive(Component)]
pub struct RoomLight {
    pub base: f32,
    pub flicker: Flicker,
    pub phase: f32,
}

pub fn setup(mut commands: Commands, house: Res<HouseRes>) {
    commands.insert_resource(ClearColor(haze_color()));
    commands.insert_resource(GlobalAmbientLight {
        color: Color::srgb(0.58, 0.64, 0.76),
        brightness: AMBIENT_BRIGHTNESS,
        ..default()
    });

    commands.spawn((
        DirectionalLight {
            color: Color::srgb(0.78, 0.82, 0.94),
            illuminance: SUN_LUX,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::default().looking_to(Vec3::new(0.5, -0.45, 0.75), Vec3::Y),
        CascadeShadowConfigBuilder {
            num_cascades: 1,
            minimum_distance: 0.1,
            maximum_distance: 34.0,
            ..default()
        }
        .build(),
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
                if hash((t * 34.0).floor() + phase) > 0.45 { 1.0 } else { 0.12 }
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
    quality: Res<Quality>,
    cam: Single<&Transform, With<PlayerCamera>>,
    mut q: Query<(&GlobalTransform, &mut PointLight), With<RoomLight>>,
) {
    let p = cam.translation;
    let dist: Vec<f32> = q
        .iter()
        .map(|(g, _)| g.translation().distance_squared(p))
        .collect();
    if dist.is_empty() {
        return;
    }
    let mut sorted = dist.clone();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let cutoff = sorted[(quality.point_shadows() - 1).min(sorted.len() - 1)];
    for (i, (_, mut l)) in q.iter_mut().enumerate() {
        let want = dist[i] <= cutoff;
        if l.shadow_maps_enabled != want {
            l.shadow_maps_enabled = want;
        }
    }
}
