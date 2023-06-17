pub mod events;
mod game;
mod main_menu;
mod systems;

use game::GamePlugin;
use main_menu::MainMenuPlugin;
use systems::*;

use bevy::prelude::*;

fn main() {
    App::new()
        // BEVY PLUGINS
        .add_plugins(DefaultPlugins)
        // STATE
        .add_state::<AppState>()
        // MY PLUGINS
        .add_plugin(MainMenuPlugin)
        .add_plugin(GamePlugin)
        // STARTUP SYSTEMS
        .add_startup_system(spawn_camera)
        // SYSTEMS
        .add_system(transition_to_game_state)
        .add_system(transition_to_main_menu_state)
        .add_system(exit_game)
        .add_system(handle_game_over)
        .run();
}

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}
