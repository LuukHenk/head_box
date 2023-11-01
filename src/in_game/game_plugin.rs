use bevy::prelude::*;

use crate::display_handler::display_handler::{despawn_screen, ScreenState};
use crate::events::bullet_events::PlayerShootEvent;
use crate::events::enemy_spawn_events::SpawnZombieEvent;

use crate::in_game::data_classes::generic_components::GameScreenMarker;

use crate::in_game::systems::arena_systems::ArenaSystems;
use crate::in_game::systems::bullet_systems::BulletSystems;
use crate::in_game::systems::camera_systems::CameraSystems;
use crate::in_game::systems::collision_systems::CollisionSystems;
use crate::in_game::systems::enemy_systems::EnemySystems;
use crate::in_game::systems::level_systems::LevelSystems;
use crate::in_game::systems::player_systems::PlayerSystems;

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnExit(ScreenState::Game),
            (
                despawn_screen::<GameScreenMarker>,
                CameraSystems::reset_zoom,
            ),
        )
        .add_systems(
            OnEnter(ScreenState::Game),
            (
                PlayerSystems::spawn_player,
                ArenaSystems::spawn_arena,
                LevelSystems::spawn_levels,
                CameraSystems::zoom_camera,
            ),
        )
        .add_event::<PlayerShootEvent>()
        .add_event::<SpawnZombieEvent>()
        .add_systems(
            FixedUpdate,
            (
                CameraSystems::follow_player,
                LevelSystems::set_current_level.after(LevelSystems::handle_game_over),
                LevelSystems::spawn_enemies_for_current_level
                    .after(LevelSystems::set_current_level),
                LevelSystems::handle_game_over,
                PlayerSystems::set_velocity,
                PlayerSystems::shoot.after(PlayerSystems::set_velocity),
                PlayerSystems::change_sprite.after(PlayerSystems::set_velocity),
                PlayerSystems::set_rotation_degrees.after(PlayerSystems::set_velocity),
                EnemySystems::set_velocity.after(PlayerSystems::set_velocity),
                EnemySystems::spawn_zombies,
                EnemySystems::despawn_enemies,
                BulletSystems::spawn_player_bullet,
                BulletSystems::despawn_bullets,
                CollisionSystems::handle_player_enemy_collision,
                CollisionSystems::handle_bullet_collision,
            )
                .run_if(in_state(ScreenState::Game)),
        );
    }
}
