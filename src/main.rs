mod game;
mod house;
mod sim;

use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin,
    image::{ImageAddressMode, ImageFilterMode, ImageSamplerDescriptor},
    light::{DirectionalLightShadowMap, PointLightShadowMap},
    prelude::*,
    window::PresentMode,
};

use game::{atmosphere, debug, frame_limit, hud, player, props, quality, world};

const TARGET_FPS: Option<f32> = Some(60.0);
use sim::doors;

fn main() {
    let house = house::build();
    let doors = doors::Doors::new(&house.doors);
    let sampler = ImageSamplerDescriptor {
        address_mode_u: ImageAddressMode::Repeat,
        address_mode_v: ImageAddressMode::Repeat,
        address_mode_w: ImageAddressMode::Repeat,
        mag_filter: ImageFilterMode::Linear,
        min_filter: ImageFilterMode::Linear,
        mipmap_filter: ImageFilterMode::Linear,
        anisotropy_clamp: 8,
        ..default()
    };
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Restless Night".into(),
                        present_mode: PresentMode::AutoNoVsync,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin {
                    default_sampler: sampler,
                    ..default()
                }),
        )
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .insert_resource(world::HouseRes(house))
        .insert_resource(world::DoorsRes(doors))
        .init_resource::<player::DoorHint>()
        .insert_resource(quality::Quality::Medium)
        .insert_resource(frame_limit::FrameLimit::new(TARGET_FPS))
        .insert_resource(PointLightShadowMap { size: 512 })
        .insert_resource(DirectionalLightShadowMap { size: 1024 })
        .init_resource::<debug::DebugState>()
        .add_systems(Update, frame_limit::keys)
        .add_systems(Last, frame_limit::wait)
        .add_systems(
            Startup,
            (
                world::spawn_house,
                props::spawn_props,
                atmosphere::setup,
                player::setup,
                hud::setup,
            ),
        )
        .add_systems(
            Update,
            (
                player::cursor_grab,
                player::look,
                player::door_interact,
                player::walk,
                player::apply_camera,
                player::flashlight,
                quality::keys,
                quality::apply,
                atmosphere::shadow_budget,
                hud::update_fps,
                hud::update_hint,
                hud::update_door_prompt,
                world::update_doors,
                props::strip_shadows,
                props::sanitize_materials,
                debug::keys,
                debug::draw_slots,
            )
                .chain(),
        )
        .run();
}
