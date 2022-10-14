use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use super::{player_input::get_input_manager, shared::Movable};

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
    //pub sprite: SpriteBundle,
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
        let mut tank_bundle = TankBundle {
            movable: Movable {
                speed: 100,
                rotation_speed_rad: f32::to_radians(80.0),
                ..Default::default()
            },
            // sprite: SpriteBundle {
            //     sprite: player_sprite,
            //     ..Default::default()
            // },
            ..Default::default()
        };

        //add_input_manager(&mut tank_bundle);

        tank_bundle
    }
}

#[derive(Component, Default)]
pub struct MouseControlled;

#[derive(Bundle, Default)]
pub struct TankTurretBundle {
    #[bundle]
    pub sprite: SpriteBundle,
}

impl TankTurretBundle {
    pub fn new(tank_layer: i8) -> Self {
        let tank_turret_sprite = Sprite {
            color: Color::rgb(0.0, 1.0, 0.0),
            custom_size: Some(Vec2 {
                x: 2.0 * 32.0,
                y: 32.0,
            }), // FIXME asset sizes should be in a bevy asset
            // FIXME here manually set turret size, probably should set it with code to support different tank component sizes
            ..Default::default()
        };

        TankTurretBundle {
            sprite: SpriteBundle {
                sprite: tank_turret_sprite,
                transform: Transform {
                    translation: Vec3::new(0.0, 10.0, tank_layer as f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..default()
        }
    }
}
