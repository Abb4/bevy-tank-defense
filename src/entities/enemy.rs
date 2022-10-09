use std::time::Duration;

use bevy::prelude::*;

use super::{
    ai::enemy_ai::Idle,
    shared::{DisplayName, EntitySharedBundle, Movable},
};

#[derive(Component, Default)]
pub struct Enemy;

#[derive(Bundle, Default)]
pub struct EnemyBundle {
    pub enemy: Enemy,

    pub ai: Idle,

    pub movable: Movable,

    #[bundle]
    pub shared: EntitySharedBundle,
}

impl EnemyBundle {
    pub fn new(name: &str) -> Self {
        let mut shared = EntitySharedBundle::default();

        shared.name = DisplayName(name.to_string());

        EnemyBundle {
            shared: shared,
            ai: Idle {
                delay: Timer::new(Duration::from_secs(2), true),
                idle_move: Vec3::new(100.0, 100.0, 0.0),
                idle_walk_distance: 200,
                ..Default::default()
            },
            movable: Movable {
                speed: 50,
                ..Default::default()
            },
            ..Default::default()
        }
    }
}
