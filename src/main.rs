use bevy::{log::LogSettings, prelude::*};
use entities::{player::PlayerAction, EntitiesPlugin};
use leafwing_input_manager::prelude::*;

pub mod entities;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_system(bevy::window::close_on_esc)
        .insert_resource(LogSettings {
            filter: "debug,wgpu_core=warn,wgpu_hal=warn".into(),
            level: bevy::log::Level::DEBUG,
        })
        .add_startup_system(setup)
        .add_plugin(InputManagerPlugin::<PlayerAction>::default())
        .add_plugin(EntitiesPlugin)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}
