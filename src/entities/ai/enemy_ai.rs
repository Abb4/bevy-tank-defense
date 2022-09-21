use std::{ops::Add, time::Duration};

use bevy::prelude::*;

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
        let distance = transform.translation.distance(idle_state.idle_move);

        if distance < 50.0 {
            if !idle_state.delay.finished() {
                idle_state.delay.tick(time.delta());
            } else {
                warn!("Adding distance");

                let mut x = fastrand::i32(70..idle_state.idle_walk_distance) as f32; // TODO implement a better function for generating offset vector with min distance from center

                if fastrand::bool() {
                    x *= -1.0; // TODO could not fiure out how rand crate works with ranges, I need negative-to-positive range, just do it myself, I guess
                }

                let mut y = fastrand::i32(70..idle_state.idle_walk_distance) as f32; // TODO implement a better function

                if fastrand::bool() {
                    y *= -1.0;
                }

                idle_state.idle_move = idle_state.idle_move.add(Vec3::new(x, y, 0.0));

                idle_state.delay.reset();
            }
        } else {
            // TODO move this into a utility function
            transform.translation = transform.translation.lerp(
                idle_state.idle_move,
                (movable.speed as f32 / distance) * time.delta_seconds(),
            );
        }
    }
}
