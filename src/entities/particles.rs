use bevy::{prelude::*, sprite::collide_aabb::collide};

use super::shared::Health;

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
    // encode the particle move direction and speed on a vector once to avoid expensive computations
    move_delta: Vec2,
}

impl ParticleLinearMove {
    pub fn move_forwards_with_speed(rotation: Quat, speed: f32) -> Self {
        let (rotation_axis, mut rotation_angle) = rotation.to_axis_angle();

        rotation_angle *= rotation_axis.z;

        ParticleLinearMove {
            move_delta: Vec2::new(-rotation_angle.cos() * speed, -rotation_angle.sin() * speed),
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
        let particle_position_update = particle_move.move_delta.extend(0.0) * time.delta_seconds();

        transform.translation += particle_position_update;
    }
}

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
