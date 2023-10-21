use bevy::prelude::*;
use bevy_rapier2d::prelude::{CollisionEvent, CollisionGroups, ContactForceEvent, Group};

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
                    ArenaSystems::spawn_boxy_arena,
                    LevelSystems::spawn_levels,
                )
            )
            .add_systems(
                FixedUpdate, (
                    LevelSystems::set_current_level.after(LevelSystems::handle_game_over),
                    LevelSystems::spawn_enemies_for_current_level.after(LevelSystems::set_current_level),
                    PlayerSystems::set_velocity,
                    PlayerSystems::shoot.after(PlayerSystems::set_velocity),
                    EnemySystems::set_velocity.after(PlayerSystems::set_velocity),
                    BulletSystems::despawn_bullets,
                    RigidBodySystems::rotate,
                    display_events,
                ).run_if(in_state(ScreenState::Game))
            )
        ;

        // Fixed update
        // LevelSystems::handle_game_over,
        // EnemySystems::despawn_enemies,
        // CollisionSystems::handle_player_enemy_collision.after(PlayerSystems::set_direction),

    }
}


fn display_events(
    mut collision_events: EventReader<CollisionEvent>,
    mut contact_force_events: EventReader<ContactForceEvent>,
) {
    for collision_event in collision_events.iter() {
        println!("Received collision event: {:?}", collision_event);
    }

    for contact_force_event in contact_force_events.iter() {
        println!("Received contact force event: {:?}", contact_force_event);
    }
}