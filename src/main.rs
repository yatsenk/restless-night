mod atmosphere;
mod collision;
mod debug;
mod geom;
mod hud;
mod layout;
mod math;
mod models;
mod player;
mod props;
mod world;

use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin,
    image::{ImageAddressMode, ImageFilterMode, ImageSamplerDescriptor},
    prelude::*,
};

fn main() {
    let house = layout::build();
    let sampler = ImageSamplerDescriptor {
        address_mode_u: ImageAddressMode::Repeat,
        address_mode_v: ImageAddressMode::Repeat,
        address_mode_w: ImageAddressMode::Repeat,
        mag_filter: ImageFilterMode::Linear,
        min_filter: ImageFilterMode::Linear,
        mipmap_filter: ImageFilterMode::Linear,
        anisotropy_clamp: 16,
        ..default()
    };
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Restless Night".into(),
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
        .init_resource::<debug::DebugState>()
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
                player::walk,
                player::apply_camera,
                player::flashlight,
                atmosphere::flicker,
                atmosphere::shadow_budget,
                hud::update_fps,
                hud::update_hint,
                debug::keys,
                debug::draw_slots,
            )
                .chain(),
        )
        .run();
}
