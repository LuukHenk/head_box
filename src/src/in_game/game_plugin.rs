use bevy::prelude::*;
use super::ScreenState;
use super::despawn_screen;

use super::movement::movement_systems::MovementSystems;
use super::player::player_systems::PlayerSystems;
use super::data_classes::generic_components::GameScreenMarker;
use super::enemy::enemy_systems::EnemySystems;
use super::arena::boxy::Boxy;
use super::level::level_systems::LevelSystems;

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                OnEnter(ScreenState::Game), (
                    Boxy::spawn,
                    PlayerSystems::spawn,
                    LevelSystems::spawn_levels,
                )
            )
            .add_systems(OnExit(ScreenState::Game), despawn_screen::<GameScreenMarker>)
            .add_systems(FixedUpdate, (
                    LevelSystems::handle_game_over,
                    LevelSystems::set_current_level.after(LevelSystems::handle_game_over),
                    LevelSystems::spawn_enemies_for_current_level,
                    PlayerSystems::set_direction,
                    EnemySystems::set_directions,
                    MovementSystems::handle_player_enemy_collision.after(PlayerSystems::set_direction),
                    PlayerSystems::shoot,
                    MovementSystems::prevent_enemy_enemy_collision.after(EnemySystems::set_directions),
                    MovementSystems::prevent_wall_collision
                        .after(PlayerSystems::set_direction)
                        .after(EnemySystems::set_directions)
                    ,
                    MovementSystems::move_objects.after(MovementSystems::prevent_wall_collision),
                ).run_if(in_state(ScreenState::Game))
            )
        ;
    }
}
