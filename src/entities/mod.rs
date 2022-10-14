use bevy::prelude::*;

use self::{
    ai::enemy_ai::idle_enemy_behaviour,
    player_input::{handle_player_movement, rotate_tank_turet_to_cursor, handle_player_firing},
    spawner::*,
};

pub mod ai;
pub mod enemy;
pub mod player;
pub mod player_input;
pub mod shared;
pub mod spawner;
pub struct EntitiesPlugin;

#[derive(SystemLabel)]
enum GameSystems {
    PlayerInput,
}

#[derive(SystemLabel)]
enum EntitySystems {
    Prespawn,
    Spawn,
    PostSpawn,
}

impl Plugin for EntitiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::new()
                .label(GameSystems::PlayerInput)
                .with_system(handle_player_movement)
                .with_system(handle_player_firing)
                .with_system(rotate_tank_turet_to_cursor),
        );

        app.add_system_set(
            SystemSet::new()
                .after(GameSystems::PlayerInput)
                .label(EntitySystems::Prespawn)
                .with_system(log_enemies_on_spawn)
                .with_system(idle_enemy_behaviour),
        );

        app.add_system_set(
            SystemSet::new()
                .after(EntitySystems::Prespawn)
                .label(EntitySystems::Spawn)
                .with_system(spawn_enemies_on_demand),
        );

        app.add_startup_system(spawn_player);

        app.add_system_set(
            SystemSet::new()
                .after(EntitySystems::Spawn)
                .label(EntitySystems::PostSpawn),
        );
    }
}
