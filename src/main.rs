use bevy::{log::LogSettings, prelude::*};
use bevy_mouse_position_component::{MousePosition2d, MousePositionPlugin};
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
        .add_plugin(InputManagerPlugin::<PlayerAction>::default())
        .add_plugin(MousePositionPlugin)
        .add_plugin(EntitiesPlugin)
        .add_startup_system(add_camera_with_tracking)
        .run();
}

fn add_camera_with_tracking(mut commands: Commands) {
    commands
        .spawn()
        .insert_bundle(Camera2dBundle::default())
        .insert(MousePosition2d::default());
}
