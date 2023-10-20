use bevy::prelude::*;
use super::ScreenState;
use super::despawn_screen;

use super::movement::movement_systems::MovementSystems;
use super::movement::collision_systems::CollisionSystems;
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
                    EnemySystems::set_directions,
                    PlayerSystems::set_direction,
                    PlayerSystems::shoot,
                    CollisionSystems::handle_player_enemy_collision.after(PlayerSystems::set_direction),
                    CollisionSystems::prevent_enemy_enemy_collision.after(EnemySystems::set_directions),
                    CollisionSystems::handle_bullet_collision
                        .after(PlayerSystems::shoot)
                        .after(PlayerSystems::set_direction)
                        .after(EnemySystems::set_directions),
                    CollisionSystems::prevent_wall_collision
                        .after(PlayerSystems::set_direction)
                        .after(EnemySystems::set_directions)
                    ,
                    MovementSystems::move_objects.after(CollisionSystems::prevent_wall_collision),
                ).run_if(in_state(ScreenState::Game))
            )
        ;
    }
}
