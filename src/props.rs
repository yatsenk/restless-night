use bevy::{light::NotShadowCaster, prelude::*};

use crate::models;
use crate::world::to_vec3;

#[derive(Component)]
pub struct NoShadowProp;

pub fn spawn_props(mut commands: Commands, assets: Res<AssetServer>) {
    for p in models::placements() {
        let entity = commands
            .spawn((
                WorldAssetRoot(assets.load(GltfAssetLabel::Scene(0).from_asset(p.file))),
                Transform {
                    translation: to_vec3(p.translation()),
                    rotation: Quat::from_rotation_y(p.yaw()),
                    scale: Vec3::splat(p.scale),
                },
            ))
            .id();
        if p.name == "kitchen" {
            commands.entity(entity).insert(NoShadowProp);
        }
    }
}

pub fn strip_shadows(
    mut commands: Commands,
    new: Query<Entity, (Added<Mesh3d>, Without<NotShadowCaster>)>,
    parents: Query<&ChildOf>,
    marks: Query<(), With<NoShadowProp>>,
) {
    for e in &new {
        let mut cur = e;
        loop {
            if marks.contains(cur) {
                commands.entity(e).insert(NotShadowCaster);
                break;
            }
            match parents.get(cur) {
                Ok(p) => cur = p.parent(),
                Err(_) => break,
            }
        }
    }
}
