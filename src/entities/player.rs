use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use super::{player_input::add_input_manager, shared::Movable};

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum PlayerAction {
    MoveForward,
    MoveBackwards,
    TurnLeft,
    TurnRight,
}

#[derive(Component, Default)]
pub struct PlayerControlled;

#[derive(Bundle, Default)]
pub struct PlayerBundle {
    pub player_control: PlayerControlled,

    pub movable: Movable,

    #[bundle]
    pub player_input: InputManagerBundle<PlayerAction>,
    #[bundle]
    pub sprite: SpriteBundle,
}

impl PlayerBundle {
    pub fn new() -> Self {
        let player_sprite = Sprite {
            color: Color::rgb(1.0, 0.0, 0.0),
            custom_size: Some(Vec2 { x: 64.0, y: 64.0 }), // FIXME asset sizes should be in a bevy asset
            ..Default::default()
        };

        let mut player_bundle = PlayerBundle {
            movable: Movable {
                speed: 100,
                rotation_speed_rad: f32::to_radians(80.0),
                ..Default::default()
            },
            sprite: SpriteBundle {
                sprite: player_sprite,
                ..Default::default()
            },
            ..Default::default()
        };

        add_input_manager(&mut player_bundle);

        player_bundle
    }
}
