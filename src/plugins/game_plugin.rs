use bevy::prelude::*;



use crate::states::screen_state::ScreenState;

use crate::components::generic_components::GameScreenMarker;

use crate::events::enemy_spawn_events::SpawnZombieEvent;
use crate::events::atttack_events::{BulletSpawnEvent, AttackRequestEvent, WeaponSelectionEvent};

use crate::systems::arena_systems::ArenaSystems;
use crate::systems::bullet_systems::BulletSystems;
use crate::systems::camera_systems::CameraSystems;
use crate::systems::collision_systems::CollisionSystems;
use crate::systems::enemy_systems::EnemySystems;
use crate::systems::level_systems::LevelSystems;
use crate::systems::player_systems::PlayerSystems;
use crate::systems::generic_systems::despawn_screen;
use crate::systems::physics_systems::PhysicsSystems;
use crate::systems::weapon_systems::WeaponSystems;
use crate::systems::sound_systems::SoundSystems;
use crate::systems::sprite_systems::SpriteSystems;

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(ScreenState::Game),
            (
                ArenaSystems::spawn_arena,
                ArenaSystems::set_enemy_spawn_locations,
                LevelSystems::spawn_levels,
                CameraSystems::zoom_camera,
                WeaponSystems::spawn_default_player_weapons,
                SoundSystems::spawn_zombie_tense_sounds,
                SoundSystems::spawn_in_game_background_sounds,
            ),
        )
        .add_systems(
            OnExit(ScreenState::Game),
            (
                despawn_screen::<GameScreenMarker>,
                CameraSystems::reset_zoom,
                SoundSystems::toggle_in_game_background_sounds,

            ),
        )
        .add_event::<AttackRequestEvent>()
        .add_event::<WeaponSelectionEvent>()
        .add_event::<BulletSpawnEvent>()
        .add_event::<SpawnZombieEvent>()
        .add_systems(
            FixedUpdate,
            (
                ( // Levels
                    LevelSystems::handle_game_over,
                    LevelSystems::set_current_level.after(LevelSystems::handle_game_over),
                    LevelSystems::spawn_enemies_for_current_level.after(LevelSystems::set_current_level),
                ),
                ( // Physics
                    PlayerSystems::set_velocity,
                    PhysicsSystems::set_rotation_degrees.after(PlayerSystems::set_velocity),
                    EnemySystems::set_velocity.after(PlayerSystems::set_velocity),
                    CollisionSystems::handle_player_enemy_collision,
                    CollisionSystems::handle_bullet_collision,

                ),
                ( // Enemies
                    EnemySystems::spawn_zombies.after(LevelSystems::spawn_enemies_for_current_level),
                    EnemySystems::despawn_enemies,
                ),
                ( // Camera
                    CameraSystems::follow_player.after(PlayerSystems::set_velocity),
                ),
                ( // Weapons and attacking
                  WeaponSystems::update_transform.after(EnemySystems::set_velocity),
                  WeaponSystems::hide_weapon_when_owner_is_moving.after(EnemySystems::set_velocity),
                  PlayerSystems::weapon_selection,
                  WeaponSystems::set_active_weapon.after(PlayerSystems::weapon_selection),
                  PlayerSystems::attack.after(PlayerSystems::set_velocity),
                  WeaponSystems::attack.after(PlayerSystems::attack),
                  BulletSystems::spawn_player_bullet.after(WeaponSystems::attack),
                  SoundSystems::play_attack_sound.after(WeaponSystems::attack),
                  BulletSystems::despawn_bullets,
                ),
                ( // Sprites
                  SpriteSystems::change_character_sprite.after(PlayerSystems::set_velocity),
                ),
            )
                .run_if(in_state(ScreenState::Game)),
        );
    }
}