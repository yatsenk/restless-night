use bevy::{camera::Exposure, prelude::*, window::PresentMode};

use crate::game::player::PlayerCamera;
use crate::game::world::{to_vec3, HouseRes};

#[derive(Resource, Default)]
pub struct DebugState {
    pub slots: bool,
}

pub fn keys(
    keys: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<DebugState>,
    mut exposure: Single<&mut Exposure, With<PlayerCamera>>,
    mut window: Single<&mut Window>,
) {
    if keys.just_pressed(KeyCode::F3) {
        state.slots = !state.slots;
    }
    if keys.just_pressed(KeyCode::BracketRight) {
        exposure.ev100 = (exposure.ev100 - 0.25).max(1.0);
        info!("exposure ev100 = {:.2}", exposure.ev100);
    }
    if keys.just_pressed(KeyCode::BracketLeft) {
        exposure.ev100 = (exposure.ev100 + 0.25).min(12.0);
        info!("exposure ev100 = {:.2}", exposure.ev100);
    }
    if keys.just_pressed(KeyCode::KeyV) {
        window.present_mode = if window.present_mode == PresentMode::AutoNoVsync {
            PresentMode::AutoVsync
        } else {
            PresentMode::AutoNoVsync
        };
        info!("present mode = {:?}", window.present_mode);
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
