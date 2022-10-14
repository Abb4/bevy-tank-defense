use bevy::prelude::*;
use leafwing_input_manager::InputManagerBundle;

use crate::entities::{
    player::{MouseControlled, PlayerAction, TankTurretBundle},
    player_input::get_input_manager,
};

use super::{
    enemy::{Enemy, EnemyBundle},
    player::{PlayerControlled, TankBundle},
    shared::DisplayName,
};

pub fn spawn_enemies_on_demand(mut commands: Commands, query: Query<&Enemy>) {
    if query.iter().count() < 3 {
        spawn_enemy("Enemy1", &mut commands);
    }
}

pub fn spawn_enemy(name: &str, commands: &mut Commands) {
    let enemy = EnemyBundle::new(name);

    commands.spawn_bundle(enemy);
}

pub fn log_enemies_on_spawn(query: Query<&DisplayName, Added<Enemy>>) {
    for name in query.into_iter() {
        info!("Spawned enemy {}", name.0);
    }
}

pub fn spawn_player(mut commands: Commands) {
    let tank_tower = commands
        .spawn()
        .insert_bundle(TankTurretBundle::new(1))
        .insert(MouseControlled::default())
        .insert_bundle(get_input_manager())
        .id();

    commands
        .spawn()
        .insert_bundle(TankBundle::new())
        .insert(PlayerControlled::default())
        .insert_bundle(get_input_manager())
        .add_child(tank_tower);

    info!("Spawned player");
}
