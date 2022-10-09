use bevy::prelude::*;

use super::{
    enemy::{Enemy, EnemyBundle},
    player::PlayerBundle,
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
    commands.spawn().insert_bundle(PlayerBundle::new());
}
