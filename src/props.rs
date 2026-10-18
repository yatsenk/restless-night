use bevy::{light::NotShadowCaster, prelude::*};

use crate::models;
use crate::world::to_vec3;

#[derive(Component)]
pub struct PropRoot;

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
                PropRoot,
            ))
            .id();
        if p.name == "kitchen" {
            commands.entity(entity).insert(NoShadowProp);
        }
    }
}

fn has_ancestor(start: Entity, parents: &Query<&ChildOf>, marked: impl Fn(Entity) -> bool) -> bool {
    let mut cur = start;
    loop {
        if marked(cur) {
            return true;
        }
        match parents.get(cur) {
            Ok(p) => cur = p.parent(),
            Err(_) => return false,
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
        if has_ancestor(e, &parents, |x| marks.contains(x)) {
            commands.entity(e).insert(NotShadowCaster);
        }
    }
}

pub fn sanitize_materials(
    new: Query<
        (Entity, &MeshMaterial3d<StandardMaterial>),
        Added<MeshMaterial3d<StandardMaterial>>,
    >,
    parents: Query<&ChildOf>,
    roots: Query<(), With<PropRoot>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (e, handle) in &new {
        if !has_ancestor(e, &parents, |x| roots.contains(x)) {
            continue;
        }
        if let Some(mut m) = materials.get_mut(&handle.0) {
            let glass = m.specular_transmission > 0.0 || m.diffuse_transmission > 0.0;
            m.metallic = 0.0;
            m.specular_transmission = 0.0;
            m.diffuse_transmission = 0.0;
            m.clearcoat = 0.0;
            if glass {
                let c = m.base_color.to_srgba();
                m.base_color = Color::srgba(c.red, c.green, c.blue, 0.22);
                m.alpha_mode = AlphaMode::Blend;
            }
        }
    }
}
