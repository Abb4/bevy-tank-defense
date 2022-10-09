use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use super::{
    player::{PlayerAction, PlayerBundle, PlayerControlled},
    shared::Movable,
};

pub fn handle_player_movement(
    mut query: Query<
        (&ActionState<PlayerAction>, &mut Transform, &Movable),
        With<PlayerControlled>,
    >,
    time: Res<Time>,
) {
    let (action_state, mut transform, movable) = query.single_mut();

    if action_state.pressed(PlayerAction::MoveForward) {
        let movement_direction = transform.rotation * Vec3::Y;

        transform.translation += movement_direction * movable.speed as f32 * time.delta_seconds()
    }
    if action_state.pressed(PlayerAction::MoveBackwards) {
        let movement_direction = transform.rotation * Vec3::Y;

        transform.translation +=
            (-1.0) * movement_direction * movable.speed as f32 * time.delta_seconds()
    }
    if action_state.pressed(PlayerAction::TurnLeft) {
        transform.rotate_z(movable.rotation_speed_rad * time.delta_seconds());
    }
    if action_state.pressed(PlayerAction::TurnRight) {
        transform.rotate_z(movable.rotation_speed_rad * time.delta_seconds());
    }
}

pub fn add_input_manager(player_bundle: &mut PlayerBundle) {
    let player_input = InputManagerBundle::<PlayerAction> {
        action_state: ActionState::default(),
        input_map: InputMap::new([
            (KeyCode::W, PlayerAction::MoveForward),
            (KeyCode::S, PlayerAction::MoveBackwards),
            (KeyCode::A, PlayerAction::TurnLeft),
            (KeyCode::D, PlayerAction::TurnRight),
        ]),
    };

    player_bundle.player_input = player_input;
}
