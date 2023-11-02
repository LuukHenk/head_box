use bevy::prelude::*;


use crate::events::bullet_events::PlayerShootEvent;
use crate::events::enemy_spawn_events::SpawnZombieEvent;

use crate::components::generic_components::GameScreenMarker;

use crate::systems::arena_systems::ArenaSystems;
use crate::systems::bullet_systems::BulletSystems;
use crate::systems::camera_systems::CameraSystems;
use crate::systems::collision_systems::CollisionSystems;
use crate::systems::enemy_systems::EnemySystems;
use crate::systems::level_systems::LevelSystems;
use crate::systems::player_systems::PlayerSystems;
use crate::systems::generic_systems::despawn_screen;

use crate::states::screen_state::ScreenState;
use crate::systems::sound_systems::SoundSystems;

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
                ArenaSystems::set_enemy_spawn_locations,
                LevelSystems::spawn_levels,
                SoundSystems::play_zombie_tense_sounds,
                CameraSystems::zoom_camera,
            ),
        )
        .add_event::<PlayerShootEvent>()
        .add_event::<SpawnZombieEvent>()
        .add_systems(
            FixedUpdate,
            (

                LevelSystems::handle_game_over,
                LevelSystems::set_current_level.after(LevelSystems::handle_game_over),
                LevelSystems::spawn_enemies_for_current_level
                    .after(LevelSystems::set_current_level),

                PlayerSystems::set_velocity,
                PlayerSystems::set_rotation_degrees.after(PlayerSystems::set_velocity),
                EnemySystems::set_velocity.after(PlayerSystems::set_velocity),

                EnemySystems::spawn_zombies.after(LevelSystems::spawn_enemies_for_current_level),
                EnemySystems::despawn_enemies,


                CameraSystems::follow_player.after(PlayerSystems::set_velocity),

                PlayerSystems::shoot.after(PlayerSystems::set_velocity),
                BulletSystems::spawn_player_bullet.after(PlayerSystems::shoot),
                SoundSystems::play_pistol_sound.after(PlayerSystems::shoot),
                BulletSystems::despawn_bullets,

                PlayerSystems::change_sprite.after(PlayerSystems::set_velocity),

                CollisionSystems::handle_player_enemy_collision,
                CollisionSystems::handle_bullet_collision,
            )
                .run_if(in_state(ScreenState::Game)),
        );
    }
}
