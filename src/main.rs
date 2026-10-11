mod atmosphere;
mod builder;
mod collision;
mod house;
mod player;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Restless Night".into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins((
            atmosphere::AtmospherePlugin,
            house::HousePlugin,
            player::PlayerPlugin,
        ))
        .run();
}
