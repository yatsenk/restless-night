use bevy::{
    anti_alias::{fxaa::Fxaa, smaa::Smaa},
    light::{DirectionalLightShadowMap, PointLightShadowMap},
    post_process::bloom::Bloom,
    prelude::*,
};

use crate::game::atmosphere::{AMBIENT_BRIGHTNESS, SUN_LUX};
use crate::game::player::{Flashlight, PlayerCamera};

#[derive(Resource, Clone, Copy, PartialEq, Eq)]
pub enum Quality {
    Low,
    Medium,
    High,
}

impl Quality {
    pub fn next(self) -> Self {
        match self {
            Quality::Low => Quality::Medium,
            Quality::Medium => Quality::High,
            Quality::High => Quality::Low,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Quality::Low => "low",
            Quality::Medium => "medium",
            Quality::High => "high",
        }
    }

    pub fn point_shadows(self) -> usize {
        match self {
            Quality::Low => 1,
            Quality::Medium => 3,
            Quality::High => 5,
        }
    }

    fn point_shadow_size(self) -> usize {
        match self {
            Quality::Low => 256,
            Quality::Medium => 512,
            Quality::High => 1024,
        }
    }

    fn sun_shadow_size(self) -> usize {
        match self {
            Quality::Low => 1024,
            Quality::Medium => 1024,
            Quality::High => 2048,
        }
    }

    fn sun_shadows(self) -> bool {
        self == Quality::High
    }

    fn flashlight_shadows(self) -> bool {
        self != Quality::Low
    }
}

pub fn keys(keys: Res<ButtonInput<KeyCode>>, mut quality: ResMut<Quality>) {
    if keys.just_pressed(KeyCode::F5) {
        *quality = quality.next();
        info!("quality = {}", quality.label());
    }
}

pub fn apply(
    quality: Res<Quality>,
    mut commands: Commands,
    cam: Single<Entity, With<PlayerCamera>>,
    mut sun: Single<&mut DirectionalLight>,
    mut flash: Single<&mut SpotLight, With<Flashlight>>,
    mut ambient: ResMut<GlobalAmbientLight>,
    mut point_map: ResMut<PointLightShadowMap>,
    mut sun_map: ResMut<DirectionalLightShadowMap>,
) {
    if !quality.is_changed() {
        return;
    }
    let q = *quality;
    let cam = cam.into_inner();

    sun.shadow_maps_enabled = q.sun_shadows();
    sun.illuminance = if q.sun_shadows() { SUN_LUX } else { 0.0 };
    flash.shadow_maps_enabled = q.flashlight_shadows();
    ambient.brightness = if q.sun_shadows() {
        AMBIENT_BRIGHTNESS
    } else {
        AMBIENT_BRIGHTNESS * 1.8
    };
    point_map.size = q.point_shadow_size();
    sun_map.size = q.sun_shadow_size();

    match q {
        Quality::Low => {
            commands
                .entity(cam)
                .remove::<Fxaa>()
                .remove::<Smaa>()
                .remove::<Bloom>();
        }
        Quality::Medium => {
            commands
                .entity(cam)
                .remove::<Smaa>()
                .remove::<Bloom>()
                .insert(Fxaa::default());
        }
        Quality::High => {
            commands.entity(cam).remove::<Fxaa>().insert((
                Smaa::default(),
                Bloom {
                    intensity: 0.1,
                    ..default()
                },
            ));
        }
    }
}
