use bevy::prelude::*;

use super::{
    enemy::{Enemy, EnemyBundle},
    shared::DisplayName,
};

pub fn spawn_enemies(mut commands: Commands) {
    spawn_enemy("Enemy1", &mut commands);
    spawn_enemy("Enemy2", &mut commands);
}

pub fn log_enemies_on_spawn(query: Query<&DisplayName, Added<Enemy>>) {
    for name in query.into_iter() {
        info!("{}", name.0);
    }
}

pub fn spawn_enemy(name: &str, commands: &mut Commands) {
    let enemy = EnemyBundle::new(name);

    commands.spawn().insert_bundle(enemy);
}
