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
        let collision_group = CollisionGroups::new(Group::from_bits(0x0010).unwrap(), Group::from_bits(0b1101).unwrap());
        app
            .add_systems(OnExit(ScreenState::Game), despawn_screen::<GameScreenMarker>)
            .add_systems(OnEnter(ScreenState::Game), (PlayerSystems::spawn, ArenaSystems::spawn_boxy_arena))
            .add_systems(FixedUpdate, (
                PlayerSystems::set_velocity,
                PlayerSystems::shoot,
                BulletSystems::despawn_bullets,
                RigidBodySystems::rotate,
            ))
        ;

        // app
            //
            //
            //         Boxy::spawn,
            //         LevelSystems::spawn_levels,
            //     )
            // )

            //
                    // LevelSystems::handle_game_over,
                    // LevelSystems::set_current_level.after(LevelSystems::handle_game_over),
                    // LevelSystems::spawn_enemies_for_current_level,
                    // EnemySystems::despawn_enemies,
                    // EnemySystems::set_directions.after(EnemySystems::despawn_enemies),
                    // BulletSystems::despawn_bullets,
                    // PlayerSystems::set_direction,
                    // PlayerSystems::shoot.after(BulletSystems::despawn_bullets),
                    // CollisionSystems::handle_player_enemy_collision.after(PlayerSystems::set_direction),
                    // CollisionSystems::prevent_enemy_enemy_collision.after(EnemySystems::set_directions),
                    // CollisionSystems::handle_bullet_collision
                    //     .after(PlayerSystems::shoot)
                    //     .after(PlayerSystems::set_direction)
                    //     .after(EnemySystems::set_directions),
                    // CollisionSystems::prevent_wall_collision
                    //     .after(PlayerSystems::set_direction)
                    //     .after(EnemySystems::set_directions)
                    // ,
                    // MovementSystems::move_objects
                    //     .after(PlayerSystems::set_direction) // FIXME: TMP until the collision works again
                    //     .after(EnemySystems::set_directions)    // FIXME: TMP until the collision works again
                        // .after(CollisionSystems::prevent_wall_collision)
                        // .after(CollisionSystems::handle_bullet_collision)
                        // .after(CollisionSystems::prevent_enemy_enemy_collision)
                        // .after(CollisionSystems::handle_player_enemy_collision)
                    // ,
                // ).run_if(in_state(ScreenState::Game))
            // )
        // ;
    }
}
