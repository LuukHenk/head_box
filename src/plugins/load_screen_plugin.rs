use bevy::prelude::*;
use crate::states::screen_state::ScreenState;
use crate::systems::player_systems::PlayerSystems;


pub struct LoadScreenPlugin;

impl Plugin for LoadScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(ScreenState::LoadScreen),
            PlayerSystems::spawn_player

        )
            .add_systems(
                Update,
                done_loading.run_if(in_state(ScreenState::LoadScreen)),
            );
    }
}

fn done_loading(mut game_state: ResMut<NextState<ScreenState>>,) {
    game_state.set(ScreenState::Game);
}