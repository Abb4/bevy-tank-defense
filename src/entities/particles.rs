use std::cmp::Ordering;

use bevy::{prelude::*, sprite::collide_aabb::collide};
use bevy_transform_utils::get_angle_from_transform;

use super::{enemy::Enemy, player_input::HomeTowardsEnemies, shared::Health};

#[derive(Component, Default)]
pub struct Particle {}

#[derive(Component, Default)]
pub struct ParticleLifetime {
    duration_sec: Timer,
}

impl ParticleLifetime {
    pub fn new(particle_lifetime_duration_sec: f32) -> Self {
        ParticleLifetime {
            duration_sec: Timer::from_seconds(particle_lifetime_duration_sec, true),
            ..Default::default()
        }
    }
}

#[derive(Component)]
pub struct ParticleLinearMove {
    move_direction: Vec2,
    speed: f32
}

impl ParticleLinearMove {
    pub fn move_forwards_with_speed(rotation: Quat, speed: f32) -> Self {
        let (rotation_axis, mut rotation_angle) = rotation.to_axis_angle();

        rotation_angle *= rotation_axis.z;

        ParticleLinearMove {
            move_direction: Vec2::new(-rotation_angle.cos(), -rotation_angle.sin()),
            speed
        }
    }
}

#[derive(Bundle, Default)]
pub struct ParticleBundle {
    particle_lifetime: ParticleLifetime,

    #[bundle]
    sprite: SpriteBundle,
}

impl ParticleBundle {
    pub fn new(
        sprite: Sprite,
        position: Vec3,
        rotation: Quat,
        particle_lifetime_duration_sec: f32,
    ) -> Self {
        let particle_lifetime = ParticleLifetime::new(particle_lifetime_duration_sec);

        ParticleBundle {
            particle_lifetime,
            sprite: SpriteBundle {
                sprite,
                transform: Transform {
                    translation: position,
                    rotation,
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        }
    }
}

pub fn move_linear_particles(
    mut query: Query<(&mut Transform, &ParticleLinearMove)>,
    time: Res<Time>,
) {
    for (mut transform, particle_move) in query.iter_mut() {
        let particle_position_update = particle_move.move_direction.extend(0.0) * particle_move.speed * time.delta_seconds();

        transform.translation += particle_position_update;
    }
}

pub fn rotate_homing_particles_towards_nearest_enemies(
    mut particles: Query<(&mut Transform, &mut ParticleLinearMove), With<HomeTowardsEnemies>>,
    enemies: Query<&Transform, (With<Enemy>, Without<HomeTowardsEnemies>)>,
    time: Res<Time>,
) {
    for (mut particle_tr, mut particle_move) in particles.iter_mut() {
        let nearest_enemy_tr_opt = enemies.iter().min_by(|a, b| {
            if a.translation.distance(particle_tr.translation)
                < b.translation.distance(particle_tr.translation)
            {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });

        if let Some(nearest_enemy_tr) = nearest_enemy_tr_opt {
            let angle = get_angle_from_transform(
                &particle_tr,
                &nearest_enemy_tr.translation.truncate(),
            );
            particle_tr.rotate_z(angle);

            let (rotation_axis, mut rotation_angle) = particle_tr.rotation.to_axis_angle();

            rotation_angle *= rotation_axis.z;

            particle_move.move_direction.x = -rotation_angle.cos();
            particle_move.move_direction.y = -rotation_angle.sin();
        }
    }
}

// pub fn rotate_particles_towards_nearest_enemies(
//     mut set: ParamSet<(
//      Query<&mut Transform, With<RotateTowardsEnemies>>,
//      Query<&Transform, With<Enemy>>)>
// ) {
//     let mut particles = set.p0();
//     let enemies = set.p1();

//     for mut particle_tr in particles.iter_mut() {
//         let nearest_enemy_tr = enemies.iter().min_by(|a, b| {
//             if a.translation.distance(particle_tr.translation)
//                 < b.translation.distance(particle_tr.translation)
//             {
//                 Ordering::Less
//             } else {
//                 Ordering::Greater
//             }
//         });

//         if let Some(nearest_enemy_position) = nearest_enemy_tr {
//             let angle = get_angle_from_transform(&particle_tr, &nearest_enemy_position.translation.truncate());
//             particle_tr.rotate_z(angle);
//         }
//     }
// }

pub fn despawn_particles_after_duration(
    mut query: Query<(Entity, &mut ParticleLifetime)>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (e, mut lifetime) in query.iter_mut() {
        if lifetime.duration_sec.finished() {
            commands.entity(e).despawn_recursive();
        } else {
            lifetime.duration_sec.tick(time.delta());
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum CollisionMask {
    PLAYER,
    ENEMY,
}

#[derive(Component)]
pub struct Collider {
    collision_mask: Vec<CollisionMask>,
}

impl Collider {
    pub fn new(collision_mask: Vec<CollisionMask>) -> Self {
        Collider { collision_mask }
    }
}

pub fn damage_entities_on_collision(
    query_particles: Query<(Entity, &Collider, &GlobalTransform, &Sprite), With<Particle>>,
    mut query_targets: Query<
        (Entity, &mut Health, &Collider, &GlobalTransform, &Sprite),
        Without<Particle>,
    >,
    mut commands: Commands,
) {
    for (particle, particle_collider, particle_global_transform, particle_sprite) in
        query_particles.iter()
    {
        let particle_translation = particle_global_transform.translation();

        let particle_size = particle_sprite.custom_size.unwrap();

        for (target, mut health, target_collider, target_global_transform, target_sprite) in
            query_targets.iter_mut()
        {
            if particle_collider
                .collision_mask
                .iter()
                .any(|e| target_collider.collision_mask.contains(e))
            {
                let target_translation = target_global_transform.translation();

                let target_size = target_sprite.custom_size.unwrap();

                if collide(
                    particle_translation,
                    particle_size,
                    target_translation,
                    target_size,
                )
                .is_some()
                {
                    if health.try_apply_damage(25).is_none() {
                        commands.entity(target).despawn_recursive();
                    }

                    commands.entity(particle).despawn_recursive();
                }
            }
        }
    }
}
