use bevy::prelude::*;
use bevy_rand_utils::RandomFromRange;

use crate::entities::{
    player::{MouseControlled, TankTurretBundle},
    player_input::get_input_manager,
};

use super::{
    enemy::{Enemy, EnemyBundle},
    particles::{Collider, CollisionMask},
    player::{PlayerControlled, TankBundle},
    shared::{DisplayName, Health},
};

pub fn spawn_enemies_on_demand(mut commands: Commands, query: Query<&Enemy>) {
    if query.iter().count() < 1 {
        spawn_enemy("Enemy1", &mut commands);
    }
}

pub fn spawn_enemy(name: &str, commands: &mut Commands) {
    let enemy = EnemyBundle::new(name);

    commands
        .spawn_bundle(enemy)
        .insert(Collider::new(vec![CollisionMask::ENEMY]))
        .insert(Health::new(100))
        .insert_bundle(TransformBundle::from_transform(Transform {
            translation: Vec2::new_random_signed(&200.0, &300.0).extend(0.0),
            ..Default::default()
        })); // FIXME z layering needs to be read fromsome reasource
}

pub fn log_enemies_on_spawn(query: Query<&DisplayName, Added<Enemy>>) {
    for name in query.into_iter() {
        info!("Spawned enemy {}", name.0);
    }
}

pub fn spawn_player(mut commands: Commands) {
    // FIXME implement spawner functions for tank and tank tower instead of relying on TankTurretBundle and TankBundle
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
