use bevy::prelude::*;

use crate::AppState;

use self::{resources::*, systems::*};

use super::SimulationState;

pub mod components;
pub mod constants;
pub mod resources;
mod systems;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            // RESOURCES
            .init_resource::<EnemySpawnTimer>()
            // ENTER STATES SYSTEMS
            .add_system(spawn_enemies.in_schedule(OnEnter(AppState::Game)))
            // SYSTEMS
            .add_systems(
                (
                    enemy_movement,
                    update_enemy_dir,
                    confine_enemy_movement,
                    tick_enemy_spawn_timer,
                    spawn_enemy_over_time,
                )
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            )
            // EXIT STATE SYSTEMS
            .add_system(despawn_enemies.in_schedule(OnExit(AppState::Game)));
    }
}
