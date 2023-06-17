use bevy::prelude::*;

pub mod components;
pub mod constants;
mod resources;
mod systems;

use crate::AppState;

use self::{resources::*, systems::*};

use super::SimulationState;

pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut App) {
        app
            // RESOURCE
            .init_resource::<StarSpawnTimer>()
            // ON ENTER STATE
            .add_system(spawn_stars.in_schedule(OnEnter(AppState::Game)))
            // SYSTEMS
            .add_systems(
                (tick_star_spawn_timer, spawn_star_over_time)
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            )
            // ON ENTER STATE
            .add_system(despawn_stars.in_schedule(OnExit(AppState::Game)));
    }
}
