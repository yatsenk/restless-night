use bevy::prelude::*;

use crate::models::{self, STREET_LIGHT_HEIGHT};
use crate::world::to_vec3;

pub fn spawn_props(mut commands: Commands, assets: Res<AssetServer>) {
    for p in models::placements() {
        commands.spawn((
            WorldAssetRoot(assets.load(GltfAssetLabel::Scene(0).from_asset(p.file))),
            Transform {
                translation: to_vec3(p.translation()),
                rotation: Quat::from_rotation_y(p.yaw()),
                scale: Vec3::splat(p.scale),
            },
        ));
        if p.name == "street_light" {
            commands.spawn((
                PointLight {
                    color: Color::srgb(1.0, 0.74, 0.42),
                    intensity: 9000.0,
                    range: 28.0,
                    shadow_maps_enabled: true,
                    ..default()
                },
                Transform::from_translation(to_vec3(p.pos) + Vec3::Y * STREET_LIGHT_HEIGHT),
            ));
        }
    }
}
