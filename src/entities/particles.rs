use bevy::prelude::*;

#[derive(Component)]
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
            move_delta: Vec2::new(- rotation_angle.cos() * speed, - rotation_angle.sin() * speed),
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
