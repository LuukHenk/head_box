use bevy::prelude::*;
use super::ScreenState;
use super::despawn_screen;

use super::movement::{
    move_objects,
    handle_player_enemy_collision,
    prevent_enemy_enemy_collision,
    prevent_wall_collision,
};
use super::player::player_systems::PlayerSystems;
use super::generic::generic_components::GameScreenMarker;
use super::zombie_bundle::ZombieBundle;
use super::Boxy;
use super::level_selection_handler::{
    spawn_levels,
    spawn_enemies_for_current_level,
    handle_game_over,
    set_current_level,
};

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                OnEnter(ScreenState::Game), (
                    Boxy::spawn,
                    PlayerSystems::spawn,
                    spawn_levels,
                )
            )
            .add_systems(OnExit(ScreenState::Game), despawn_screen::<GameScreenMarker>)
            .add_systems(FixedUpdate, (
                    handle_game_over,
                    set_current_level.after(handle_game_over),
                    spawn_enemies_for_current_level,
                    PlayerSystems::set_direction,
                    ZombieBundle::set_directions,
                    handle_player_enemy_collision.after(PlayerSystems::set_direction),
                    PlayerSystems::shoot,
                    prevent_enemy_enemy_collision.after(ZombieBundle::set_directions),
                    prevent_wall_collision
                        .after(PlayerSystems::set_direction)
                        .after(ZombieBundle::set_directions)
                    ,
                    move_objects.after(prevent_wall_collision),
                ).run_if(in_state(ScreenState::Game))
            )
        ;
    }
}
