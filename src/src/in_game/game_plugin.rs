use bevy::prelude::*;
use super::ScreenState;
use super::despawn_screen;
use super::game_components::*;
use super::movement::{
    move_objects,
    handle_player_enemy_collision,
    prevent_enemy_enemy_collision,
    prevent_wall_collision,
};
use super::player_bundle::PlayerBundle;
use super::zombie_bundle::ZombieBundle;
use super::Boxy;

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                OnEnter(ScreenState::Game), (
                    Boxy::spawn,
                    PlayerBundle::spawn,
                    ZombieBundle::spawn,
                    ZombieBundle::spawn,
                )
            )
            .add_systems(OnExit(ScreenState::Game), despawn_screen::<GameScreenMarker>)
            .add_systems(FixedUpdate, (
                    PlayerBundle::set_direction,
                    ZombieBundle::set_directions,
                    handle_player_enemy_collision.after(PlayerBundle::set_direction),
                    prevent_enemy_enemy_collision.after(ZombieBundle::set_directions),
                    prevent_wall_collision
                        .after(PlayerBundle::set_direction)
                        .after(ZombieBundle::set_directions)
                    ,
                    move_objects.after(prevent_wall_collision),
                ).run_if(in_state(ScreenState::Game))
            )
        ;
    }
}
