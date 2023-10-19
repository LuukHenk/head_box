


use bevy::prelude::{
    Commands,
    Query,
    Transform,
    With,
    Mut,
    Vec3,
};

use super::generic_constants::{
    SCREEN_CENTER,
    OUTER_Y_COORDINATES
};
use super::generic_functions::GenericFunctions;
use super::data_classes::movement_components::{
    Movement
};
use super::player_components::PlayerMarker;
use super::enemy_components::EnemyMarker;

pub struct EnemySystems;

use super::zombie_bundle::ZombieBundle;

impl EnemySystems {


    pub fn spawn_zombie(commands: Commands) {
        let y = if rand::random::<bool>() { 1. } else { -1. };
        let x = if rand::random::<bool>() { 1. } else { -1. };
        let zombie = ZombieBundle::new(SCREEN_CENTER * x, OUTER_Y_COORDINATES * y);
        GenericFunctions::spawn(commands , zombie);
    }

    pub fn set_directions(
        mut enemy_query: Query<(&mut Movement, &Transform), With<EnemyMarker>>,
        player_query: Query<&Transform, With<PlayerMarker>>
    ) {
        for player_transform in player_query.iter() {
            let player_position = player_transform.translation;
            for (enemy_movement, enemy_transform) in enemy_query.iter_mut() {
                let enemy_position = enemy_transform.translation;
                Self::set_directions_to_target(enemy_movement, enemy_position, player_position);
            }
        }
    }

    fn set_directions_to_target(
        mut movement: Mut<Movement>,
        position: Vec3,
        target_position: Vec3
    ) {
        movement.direction_x = Self::set_direction_to_target(
            movement.velocity,
            position[0],
            target_position[0],
        );
        movement.direction_y = Self::set_direction_to_target(
            movement.velocity,
            position[1],
            target_position[1],
        );
    }
    fn set_direction_to_target(minimum_distance_difference: f32, position: f32, target_position: f32) -> f32 {
        let target_distance =  target_position - position;
        if target_distance > minimum_distance_difference {1.} else if target_distance < -minimum_distance_difference {-1.} else {0.}
    }
}