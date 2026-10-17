use bevy::{camera::Exposure, light::VolumetricFog, prelude::*};

use crate::player::PlayerCamera;
use crate::world::{to_vec3, HouseRes};

#[derive(Resource, Default)]
pub struct DebugState {
    pub slots: bool,
    pub volumetric_off: bool,
}

pub fn keys(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<DebugState>,
    mut cam: Single<(Entity, &mut Exposure), With<PlayerCamera>>,
) {
    if keys.just_pressed(KeyCode::F3) {
        state.slots = !state.slots;
    }
    if keys.just_pressed(KeyCode::BracketRight) {
        cam.1.ev100 = (cam.1.ev100 - 0.25).max(1.0);
        info!("exposure ev100 = {:.2}", cam.1.ev100);
    }
    if keys.just_pressed(KeyCode::BracketLeft) {
        cam.1.ev100 = (cam.1.ev100 + 0.25).min(12.0);
        info!("exposure ev100 = {:.2}", cam.1.ev100);
    }
    if keys.just_pressed(KeyCode::F4) {
        state.volumetric_off = !state.volumetric_off;
        if state.volumetric_off {
            commands.entity(cam.0).remove::<VolumetricFog>();
        } else {
            commands.entity(cam.0).insert(VolumetricFog {
                ambient_intensity: 0.0,
                step_count: 32,
                ..default()
            });
        }
    }
}

pub fn draw_slots(state: Res<DebugState>, house: Res<HouseRes>, mut gizmos: Gizmos) {
    if !state.slots {
        return;
    }
    for s in &house.0.slots {
        let min = to_vec3(s.min);
        let max = to_vec3(s.max);
        let c = (min + max) * 0.5;
        let size = max - min;
        gizmos.cube(
            Transform::from_translation(c).with_scale(size),
            Color::srgb(0.9, 0.7, 0.2),
        );
    }
}
