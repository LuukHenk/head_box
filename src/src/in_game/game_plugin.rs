use bevy::prelude::*;

use crate::display_handler::display_handler::{ScreenState, despawn_screen};
use crate::in_game::data_classes::bullet_events::PlayerShootEvent;

use crate::in_game::data_classes::generic_components::GameScreenMarker;

use crate::in_game::systems::player_systems::PlayerSystems;
use crate::in_game::systems::rigid_body_systems::RigidBodySystems;
use crate::in_game::systems::arena_systems::ArenaSystems;
use crate::in_game::systems::collision_systems::CollisionSystems;
use crate::in_game::systems::enemy_systems::EnemySystems;
use crate::in_game::systems::level_systems::LevelSystems;
use crate::in_game::systems::bullet_systems::BulletSystems;

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
            .add_event::<PlayerShootEvent>()
            .add_systems(
                FixedUpdate, (
                    LevelSystems::set_current_level.after(LevelSystems::handle_game_over),
                    LevelSystems::spawn_enemies_for_current_level.after(LevelSystems::set_current_level),
                    LevelSystems::handle_game_over,
                    PlayerSystems::set_velocity,
                    PlayerSystems::shoot.after(PlayerSystems::set_velocity),
                    EnemySystems::set_velocity.after(PlayerSystems::set_velocity),
                    EnemySystems::despawn_enemies,
                    BulletSystems::spawn_player_bullet,
                    BulletSystems::despawn_bullets,
                    // RigidBodySystems::rotate,
                    CollisionSystems::handle_player_enemy_collision,
                    CollisionSystems::handle_bullet_collision,
                ).run_if(in_state(ScreenState::Game))
            )
        ;
    }
}



