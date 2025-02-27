use bevy::prelude::*;

use crate::{despawn_screen, GameState};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), setup_game);
        app.add_systems(OnExit(GameState::Game), despawn_screen::<OnGameScreen>);
        // app.add_systems(Update, ().run_if(in_state(GameState::Game)));
    }
}

#[derive(Component)]
struct OnGameScreen;

fn setup_game(mut commands: Commands) {
    commands.spawn(Text("in game".to_string()));
}
