


use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::in_game::data_classes::generic_components::Health;
use crate::in_game::data_classes::level_components::{ActiveLevelMarker, KilledEnemies};
use crate::in_game::data_classes::rigid_body_components::WalkingVelocity;
use crate::in_game::data_classes::generic_constants::SCREEN_CENTER;
use crate::in_game::data_classes::player_components::PlayerMarker;
use crate::in_game::data_classes::enemy_components::EnemyMarker;

use crate::in_game::data_layers::enemy_bundle::EnemyBundle;


pub struct EnemySystems;

impl EnemySystems {


    pub fn spawn_zombie(mut commands: Commands, asset_server: Res<AssetServer>) {
        let y = if rand::random::<bool>() { 1. } else { -1. };
        let x = if rand::random::<bool>() { 1. } else { -1. };
        let zombie = EnemyBundle::new(SCREEN_CENTER * x, 350. * y, asset_server);
        commands.spawn(zombie);
    }


    pub fn despawn_enemies(
        mut commands: Commands,
        query: Query<(Entity, &Health), With<EnemyMarker>>,
        mut active_level_query: Query<&mut KilledEnemies, With<ActiveLevelMarker>>
    ) {

        for (entity, health) in query.iter() {
            if health.0 <= 0.0 {
                commands.entity(entity).despawn();
                let mut killed_enemies = active_level_query.single_mut();
                killed_enemies.0 += 1;
            }
        }
    }

    pub fn set_velocity(
        mut enemy_query: Query<(&mut Velocity, &Transform, &WalkingVelocity), With<EnemyMarker>>,
        player_query: Query<&Transform, With<PlayerMarker>>
    ) {
        for player_transform in player_query.iter() {
            let player_position = player_transform.translation;
            for (mut enemy_velocity, enemy_transform, walking_velocity) in enemy_query.iter_mut() {
                let enemy_position = enemy_transform.translation;
                enemy_velocity.linvel[0] = Self::set_direction_to_target(
                    walking_velocity.0,
                    enemy_position[0],
                    player_position[0],
                );
                enemy_velocity.linvel[1] = Self::set_direction_to_target(
                    walking_velocity.0,
                    enemy_position[1],
                    player_position[1],
                );
            }
        }
    }

    fn set_direction_to_target(velocity: f32, position: f32, target_position: f32) -> f32 {
        let target_distance =  target_position - position;
        if target_distance > 0. {velocity} else if target_distance < - 0. {-velocity} else {0.}
    }
}