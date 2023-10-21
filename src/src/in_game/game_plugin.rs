use bevy::prelude::*;

use super::ScreenState;
use super::despawn_screen;

use super::data_classes::generic_components::GameScreenMarker;
use super::player::player_systems::PlayerSystems;
use super::rigid_body::rigid_body_systems::RigidBodySystems;
use super::arena::arena_systems::ArenaSystems;
use super::movement::collision_systems::CollisionSystems;
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
                    ArenaSystems::spawn_boxy_arena,
                    LevelSystems::spawn_levels,
                )
            )
            .add_systems(
                FixedUpdate, (
                    LevelSystems::set_current_level.after(LevelSystems::handle_game_over),
                    LevelSystems::spawn_enemies_for_current_level.after(LevelSystems::set_current_level),
                    LevelSystems::handle_game_over,
                    PlayerSystems::set_velocity,
                    PlayerSystems::shoot.after(PlayerSystems::set_velocity),
                    EnemySystems::set_velocity.after(PlayerSystems::set_velocity),
                    EnemySystems::despawn_enemies,
                    BulletSystems::despawn_bullets,
                    RigidBodySystems::rotate,
                    CollisionSystems::handle_player_enemy_collision,
                    CollisionSystems::handle_bullet_collision,
                ).run_if(in_state(ScreenState::Game))
            )
        ;
    }
}



