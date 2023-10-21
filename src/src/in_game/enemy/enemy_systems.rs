


use bevy::prelude::{Commands, Query, Transform, With, Mut, Vec3, Entity, AssetServer, Res};
use bevy_rapier2d::prelude::Velocity;
use crate::in_game::data_classes::generic_components::{GameScreenMarker, Health};
use crate::in_game::data_classes::level_components::{ActiveLevelMarker, KilledEnemies};

use super::generic_constants::{
    SCREEN_CENTER,
    OUTER_Y_COORDINATES
};
use super::data_classes::movement_components::{
    Movement
};
use super::player_components::PlayerMarker;
use super::enemy_components::EnemyMarker;

pub struct EnemySystems;

use super::zombie_bundle::ZombieBundle;

impl EnemySystems {


    pub fn spawn_zombie(mut commands: Commands, asset_server: Res<AssetServer>) {
        let y = if rand::random::<bool>() { 1. } else { -1. };
        let x = if rand::random::<bool>() { 1. } else { -1. };
        let zombie = ZombieBundle::new(SCREEN_CENTER * x, 350. * y, asset_server);
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
        mut enemy_query: Query<(&mut Velocity, &Transform), With<EnemyMarker>>,
        player_query: Query<&Transform, With<PlayerMarker>>
    ) {
        for player_transform in player_query.iter() {
            let player_position = player_transform.translation;
            for (enemy_velocity, enemy_transform) in enemy_query.iter_mut() {
                let enemy_position = enemy_transform.translation;
                Self::set_directions_to_target(enemy_velocity, enemy_position, player_position);
            }
        }
    }


    fn set_directions_to_target(
        mut velocity: Mut<Velocity>,
        position: Vec3,
        target_position: Vec3
    ) {
        velocity.linvel[0] = Self::set_direction_to_target(
            0.,
            position[0],
            target_position[0],
        );
        velocity.linvel[1] = Self::set_direction_to_target(
            0.,
            position[1],
            target_position[1],
        );
    }
    fn set_direction_to_target(minimum_distance_difference: f32, position: f32, target_position: f32) -> f32 {
        let speed = 150.;
        let target_distance =  target_position - position;
        if target_distance > minimum_distance_difference {speed} else if target_distance < -minimum_distance_difference {-speed} else {0.}
    }
}