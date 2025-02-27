use crate::game::PlayerPlugin;
use bevy::prelude::*;

use crate::{despawn_entities, GameState};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), setup);
        app.add_systems(OnExit(GameState::Game), despawn_entities::<OnGameScreen>);
        app.add_plugins(PlayerPlugin);
    }
}

#[derive(Component)]
struct OnGameScreen;

fn setup(mut commands: Commands) {
    commands.spawn(OnGameScreen).insert(create_camera());
}
fn create_camera() -> (Camera2d, Camera) {
    (Camera2d, Camera { ..default() })
}
