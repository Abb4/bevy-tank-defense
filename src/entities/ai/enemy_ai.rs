use std::time::Duration;

use bevy::prelude::*;
use bevy_rand_utils::prelude::*;
use bevy_transform_utils::move_towards;

use crate::entities::{enemy::*, shared::Movable};

#[derive(Component)]
pub struct Idle {
    pub delay: Timer,
    pub idle_move: Vec3,
    pub idle_walk_distance: i32,
}

impl Default for Idle {
    fn default() -> Self {
        Self {
            delay: Timer::new(Duration::from_secs(4), true), // TODO havin same duration feels unnatural, implement propper random time intervals which are adjusted more easily
            idle_move: Default::default(),
            idle_walk_distance: 200,
        }
    }
}

pub fn idle_enemy_behaviour(
    mut query: Query<
        (&mut Transform, &mut Idle, &Movable),
        (With<Enemy>, With<Idle>, With<Movable>),
    >,
    time: Res<Time>,
) {
    for (mut transform, mut idle_state, movable) in query.iter_mut() {
        if !idle_state.delay.finished() {
            idle_state.delay.tick(time.delta());
        } else {
            if let Some(_rest_distance) = move_towards(
                &mut transform,
                idle_state.idle_move,
                movable.speed as f32,
                &time,
                40.0,
            ) {
            } else {
                let new_idle_coords = Vec2::new_random_signed(&50.0, &80.0);

                let new_idle_pos = Vec3::from((new_idle_coords, idle_state.idle_move.z)); // Do not randomize z

                // TODO find extension method for changin Vec3 only by x,y of the Vec2, leaving z unchanged

                idle_state.idle_move += new_idle_pos;

                idle_state.delay.reset();
            }
        }
    }
}
