use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use super::shared::Movable;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum PlayerAction {
    MoveForward,
    MoveBackwards,
    TurnLeft,
    TurnRight,
    FireCannon,
}

#[derive(Component, Default)]
pub struct PlayerControlled;

#[derive(Bundle, Default)]
pub struct TankBundle {
    pub movable: Movable,

    #[bundle]
    pub tank_body: TankBodyBundle,
}

#[derive(Component, Bundle)]
pub struct TankBodyBundle {
    #[bundle]
    pub sprite: SpriteBundle,
}

impl Default for TankBodyBundle {
    fn default() -> Self {
        let tank_body_sprite = Sprite {
            color: Color::rgb(1.0, 0.0, 0.0),
            custom_size: Some(Vec2 { x: 64.0, y: 64.0 }), // FIXME asset sizes should be in a bevy asset
            ..Default::default()
        };

        TankBodyBundle {
            sprite: SpriteBundle {
                sprite: tank_body_sprite,
                ..Default::default()
            },
        }
    }
}

impl TankBundle {
    pub fn new() -> Self {
        let tank_bundle = TankBundle {
            movable: Movable {
                speed: 100,
                rotation_speed_rad: f32::to_radians(80.0),
                ..Default::default()
            },
            ..Default::default()
        };

        tank_bundle
    }
}