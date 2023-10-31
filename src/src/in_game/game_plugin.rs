use bevy::prelude::*;

use crate::display_handler::display_handler::{ScreenState, despawn_screen};
use crate::in_game::data_classes::bullet_events::PlayerShootEvent;

use crate::in_game::data_classes::generic_components::GameScreenMarker;

use crate::in_game::player::Player;
use crate::in_game::systems::arena_systems::ArenaSystems;
use crate::in_game::systems::collision_systems::CollisionSystems;
use crate::in_game::systems::enemy_systems::EnemySystems;
use crate::in_game::systems::level_systems::LevelSystems;
use crate::in_game::systems::bullet_systems::BulletSystems;
use crate::in_game::systems::camera_systems::CameraSystems;

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                OnExit(ScreenState::Game), (
                    despawn_screen::<GameScreenMarker>,
                    CameraSystems::reset_zoom,
                )
            )
            .add_systems(
                OnEnter(ScreenState::Game), (

                    Player::spawn,
                    ArenaSystems::spawn_boxy_arena,
                    LevelSystems::spawn_levels,
                    CameraSystems::zoom_camera,
                )
            )
            .add_event::<PlayerShootEvent>()
            .add_systems(
                FixedUpdate, (
                    CameraSystems::follow_player,
                    LevelSystems::set_current_level.after(LevelSystems::handle_game_over),
                    LevelSystems::spawn_enemies_for_current_level.after(LevelSystems::set_current_level),
                    LevelSystems::handle_game_over,
                    Player::set_velocity,
                    Player::shoot.after(Player::set_velocity),
                    Player::change_sprite.after(Player::set_velocity),
                    Player::set_rotation_degrees.after(Player::set_velocity),
                    EnemySystems::set_velocity.after(Player::set_velocity),
                    EnemySystems::despawn_enemies,
                    BulletSystems::spawn_player_bullet,
                    BulletSystems::despawn_bullets,
                    CollisionSystems::handle_player_enemy_collision,
                    CollisionSystems::handle_bullet_collision,
                ).run_if(in_state(ScreenState::Game))
            )
        ;
    }
}




