use bevy::prelude::*;

use self::{ai::enemy_ai::idle_enemy_behaviour, enemy_spawn::*};

pub mod ai;
pub mod enemy;
pub mod enemy_spawn;
pub mod shared;

pub struct EntitiesPlugin;

impl Plugin for EntitiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_enemies);
        app.add_system(log_enemies_on_spawn);
        app.add_system(idle_enemy_behaviour);
    }
}
