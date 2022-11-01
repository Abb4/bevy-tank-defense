use bevy::{prelude::*, sprite::Anchor};
use bevy_rand_utils::RandomFromRange;

use crate::entities::player_input::get_input_manager;

use super::{
    enemy::{Enemy, EnemyBundle},
    player::{PlayerControlled, TankBundle},
    projectiles::{DirectedLinearMove, Projectile},
    shared::{Collider, CollisionMask, DisplayName, Health, Lifetime, MouseControlled},
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
    let tank_turret = spawn_tank_turret(&mut commands);

    commands
        .spawn()
        .insert_bundle(TankBundle::new())
        .insert(PlayerControlled::default())
        .insert_bundle(get_input_manager())
        .add_child(tank_turret);

    info!("Spawned player");
}

pub fn spawn_tank_turret(commands: &mut Commands) -> Entity {
    let tank_turret_sprite = Sprite {
        color: Color::rgb(0.0, 1.0, 0.0),
        custom_size: Some(Vec2 {
            x: 2.0 * 32.0,
            y: 32.0,
        }), // FIXME asset sizes should be in a bevy asset
        // FIXME here manually set turret size, probably should set it with code to support different tank component sizes
        anchor: Anchor::CenterRight,
        ..Default::default()
    };

    let tank_turret_transform = Transform {
        translation: Vec3::new(0., 10., 1.),
        ..Default::default()
    };

    let tank_tower = commands
        .spawn()
        .insert_bundle(SpriteBundle {
            sprite: tank_turret_sprite,
            transform: tank_turret_transform,
            ..Default::default()
        })
        .insert(MouseControlled::default())
        .insert_bundle(get_input_manager())
        .id();

    return tank_tower;
}

pub fn create_projectile(
    commands: &mut Commands,
    sprite: Sprite,
    translation: Vec3,
    rotation: Quat,
    duration_sec: f32,
    projectile_speed: f32,
) -> Entity {
    let projectile = commands
        .spawn()
        .insert(Projectile::default())
        .insert(Collider::new(vec![CollisionMask::ENEMY]))
        .insert(DirectedLinearMove::move_forwards_with_speed(
            rotation,
            projectile_speed,
        ))
        .insert(Lifetime::new(duration_sec))
        .insert_bundle(SpriteBundle {
            sprite,
            transform: Transform {
                translation,
                rotation,
                ..Default::default()
            },
            ..Default::default()
        })
        .id();

    return projectile;
}

#[derive(Component, Default)]
pub struct HomeTowardsEnemies {}

impl HomeTowardsEnemies {
    pub fn home_towards_nearest_enemy() -> Self {
        HomeTowardsEnemies { ..default() }
    }
}
