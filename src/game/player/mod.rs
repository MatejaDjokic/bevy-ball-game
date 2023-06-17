use bevy::prelude::*;

use crate::AppState;

use self::systems::*;

use super::SimulationState;

mod components;
mod constants;
mod systems;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum PlayerSystemSet {
    Movement,
    Confinement,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            // CONFIGURE SYSTEM SETS
            .configure_set(PlayerSystemSet::Movement.before(PlayerSystemSet::Confinement))
            // ON ENTER STATE
            .add_system(spawn_player.in_schedule(OnEnter(AppState::Game)))
            // SYSTEMS
            .add_system(
                player_movement
                    .in_set(PlayerSystemSet::Movement)
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            )
            .add_system(
                confine_player_movement
                    .in_set(PlayerSystemSet::Confinement)
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            )
            .add_systems(
                (confine_player_movement, player_hit_star, enemy_hit_player)
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            )
            // ON EXIT STATE
            .add_system(despawn_player.in_schedule(OnExit(AppState::Game)));
    }
}
