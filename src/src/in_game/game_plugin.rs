use bevy::prelude::*;
use bevy_rapier2d::prelude::{CollisionGroups, Group};

use super::ScreenState;
use super::despawn_screen;

use super::data_classes::generic_components::GameScreenMarker;
use super::player::player_systems::PlayerSystems;
use super::rigid_body::rigid_body_systems::RigidBodySystems;
use super::arena::arena_systems::ArenaSystems;
use super::movement::movement_systems::MovementSystems;
// use super::movement::collision_systems::CollisionSystems;
use super::enemy::enemy_systems::EnemySystems;
use super::level::level_systems::LevelSystems;
use super::bullet::bullet_systems::BulletSystems;

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnExit(ScreenState::Game), despawn_screen::<GameScreenMarker>)
            .add_systems(
                OnEnter(ScreenState::Game), (
                    PlayerSystems::spawn,
                    ArenaSystems::spawn_boxy_arena
                )
            )
            .add_systems(
                FixedUpdate, (
                    PlayerSystems::set_velocity,
                    PlayerSystems::shoot,
                    BulletSystems::despawn_bullets,
                    RigidBodySystems::rotate,
                ).run_if(in_state(ScreenState::Game))
            )
        ;

        // On enter
        // LevelSystems::spawn_levels,

        // Fixed update
        // LevelSystems::handle_game_over,
        // LevelSystems::set_current_level.after(LevelSystems::handle_game_over),
        // LevelSystems::spawn_enemies_for_current_level,
        // EnemySystems::despawn_enemies,
        // EnemySystems::set_directions.after(EnemySystems::despawn_enemies),
        // CollisionSystems::handle_player_enemy_collision.after(PlayerSystems::set_direction),

    }
}
