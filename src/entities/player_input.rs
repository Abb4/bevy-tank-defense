use bevy::prelude::*;
use bevy_mouse_position_component::MousePosition2d;
use bevy_transform_utils::get_angle_from_transform;
use leafwing_input_manager::prelude::*;

use super::{
    player::{ PlayerAction, PlayerControlled},
    shared::{Movable, MouseControlled}, spawner::create_projectile,
};

pub fn get_input_manager() -> InputManagerBundle<PlayerAction> {
    let mut input_map = InputMap::default();

    input_map
        .insert(KeyCode::W, PlayerAction::MoveForward)
        .insert(KeyCode::S, PlayerAction::MoveBackwards)
        .insert(KeyCode::A, PlayerAction::TurnLeft)
        .insert(KeyCode::D, PlayerAction::TurnRight)
        .insert(MouseButton::Left, PlayerAction::FireCannon);

    InputManagerBundle::<PlayerAction> {
        action_state: ActionState::default(),
        input_map: input_map,
    }
}

pub fn handle_player_movement(
    mut query: Query<
        (&ActionState<PlayerAction>, &mut Transform, &Movable),
        With<PlayerControlled>,
    >,
    time: Res<Time>,
) {
    for (action_state, mut transform, movable) in query.iter_mut() {
        if action_state.pressed(PlayerAction::MoveForward) {
            let movement_direction = transform.rotation * Vec3::Y;

            transform.translation +=
                movement_direction * movable.speed as f32 * time.delta_seconds()
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
            transform.rotate_z((-1.0) * movable.rotation_speed_rad * time.delta_seconds());
        }
    }
}

pub fn handle_player_firing(
    mut query: Query<(&ActionState<PlayerAction>, &GlobalTransform), With<MouseControlled>>,
    mut commands: Commands,
) {
    for (action_state, global_transform) in query.iter_mut() {
        if action_state.just_pressed(PlayerAction::FireCannon) {
            let projectile_sprite = Sprite {
                custom_size: Some(Vec2::new(20.0, 20.0)),
                color: Color::rgb(0.0, 0.0, 1.0),
                ..Default::default()
            };

            let transform = global_transform.compute_transform();

            let projectile_pos = transform.translation; // FIXME here we inherit towers z position, should be instead some constant in some struct

            let projectile_rotation = transform.rotation;

            let duration_sec = 20.0;

            let particle_speed = 100.0;

            create_projectile(&mut commands, projectile_sprite, projectile_pos, projectile_rotation, duration_sec, particle_speed);
        }
    }
}

pub fn rotate_tank_tower_to_cursor(
    mut query: Query<(&mut Transform, &GlobalTransform), With<MouseControlled>>,
    mouse_position_q: Query<&MousePosition2d>,
) {
    let mouse_position = mouse_position_q.single().world_pos;
    let (mut transform, global_translation) = query.single_mut();

    let angle = get_angle_from_transform(&global_translation.compute_transform(), &mouse_position);

    transform.rotate_z(angle);
}
