use bevy::{light::CascadeShadowConfigBuilder, prelude::*};

use crate::game::player::PlayerCamera;
use crate::game::quality::Quality;
use crate::house::LightKind;
use crate::game::world::{to_vec3, HouseRes};

pub const SUN_LUX: f32 = 26.0;
pub const AMBIENT_BRIGHTNESS: f32 = 3.0;
pub const FOG_DENSITY: f32 = 0.02;

pub fn haze_color() -> Color {
    Color::srgb(0.38, 0.40, 0.43)
}

#[derive(Component)]
pub struct RoomLight;

pub fn setup(mut commands: Commands, house: Res<HouseRes>) {
    commands.insert_resource(ClearColor(haze_color()));
    commands.insert_resource(GlobalAmbientLight {
        color: Color::srgb(0.66, 0.72, 0.80),
        brightness: AMBIENT_BRIGHTNESS,
        ..default()
    });

    commands.spawn((
        DirectionalLight {
            color: Color::srgb(0.72, 0.80, 0.94),
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

    for l in house.0.lights.iter() {
        let light = PointLight {
            color: Color::srgb(l.color[0], l.color[1], l.color[2]),
            intensity: l.lumens,
            range: l.range,
            shadow_maps_enabled: false,
            radius: 0.08,
            ..default()
        };
        let t = Transform::from_translation(to_vec3(l.pos));
        match l.kind {
            LightKind::Practical => {
                commands.spawn((light, t, RoomLight));
            }
            LightKind::Window => {
                commands.spawn((light, t));
            }
        }
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
