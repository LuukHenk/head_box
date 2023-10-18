use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};
use super::game_components::{EnemyMarker, WallMarker};
use super::generic::generic_components::{
    Movement,
    Health,
};
use super::player::player_components::PlayerMarker;
const WEAK_COLLISION_PUSHBACK: f32 = 0.2;
const STRONG_COLLISION_PUSHBACK: f32 = 0.24;

use std::f32::consts::TAU;


pub fn move_objects(mut query: Query<(&mut Transform, &mut Movement), With<Movement>>) {
    for (mut transform, mut movement) in query.iter_mut() {

        rotate_object(&mut *transform, &mut *movement);
        transform.translation.x += movement.direction_x * movement.velocity;
        transform.translation.y += movement.direction_y * movement.velocity;

    }
}

fn rotate_object(transform: &mut Transform, movement: &mut Movement) {
    if movement.direction_y == 0. && movement.direction_x == -1. {
        transform.rotation = Quat::from_rotation_z((90.0_f32.to_radians()))
    } else if movement.direction_y == 1. && movement.direction_x == -1. {
        transform.rotation = Quat::from_rotation_z((45.0_f32.to_radians()))
    } else if movement.direction_y == 1. && movement.direction_x == 0. {
        transform.rotation = Quat::from_rotation_z((0.0_f32.to_radians()))
    } else if movement.direction_y == 1. && movement.direction_x == 1. {
        transform.rotation = Quat::from_rotation_z((315.0_f32.to_radians()))
    } else if movement.direction_y == 0. && movement.direction_x == 1. {
        transform.rotation = Quat::from_rotation_z((270.0_f32.to_radians()))
    } else if movement.direction_y == -1. && movement.direction_x == 1. {
        transform.rotation = Quat::from_rotation_z((225.0_f32.to_radians()))
    } else if movement.direction_y == -1. && movement.direction_x == 0. {
        transform.rotation = Quat::from_rotation_z((180.0_f32.to_radians()))
    } else if movement.direction_y == -1. && movement.direction_x == -1. {
        transform.rotation = Quat::from_rotation_z((135.0_f32.to_radians()))
    };
}
pub fn handle_player_enemy_collision(
    mut player_query: Query<(&Transform, &mut Movement, &mut Health), With<PlayerMarker>>,
    enemies_query: Query<&Transform, With<EnemyMarker>>,
) {
    for (player_transform, mut player_movement, mut player_health) in player_query.iter_mut() {
        for enemy_transform in enemies_query.iter() {
            let collision = check_for_collision(player_transform, enemy_transform);
            if let Some(collision) = collision {
                player_movement = apply_collision_pushback(
                    collision,
                    player_movement,
                    STRONG_COLLISION_PUSHBACK
                );
                player_health.0 -= 1.;
                println!("Auch! HP: {:#?}", player_health.0) // TODO: Remove when there is a health display
            }
        }
    }
}
pub fn prevent_enemy_enemy_collision(
    mut enemies_query_a: Query<(Entity, &Transform, &mut Movement), With<EnemyMarker>>,
    enemies_query_b: Query<(Entity, &Transform), With<EnemyMarker>>,
) {
    for (entity_a, transform_a, mut movement_a) in enemies_query_a.iter_mut() {
        for (entity_b, transform_b) in enemies_query_b.iter() {
            if entity_a == entity_b {continue}
            let collision = check_for_collision(transform_a, transform_b);
            if let Some(collision) = collision {
                movement_a = apply_collision_pushback(
                    collision,
                    movement_a,
                    WEAK_COLLISION_PUSHBACK
                );
            }
        }
    }
}
pub fn prevent_wall_collision(
    mut moving_objects_query: Query<(&Transform, &mut Movement), With<Movement>>,
    wall_query: Query<&Transform, With<WallMarker>>,
) {
    for (transform_a, mut movement_a) in moving_objects_query.iter_mut() {
        for transform_b in wall_query.iter() {
            let collision = check_for_collision(transform_a, transform_b);
            if let Some(collision) = collision {
                movement_a = apply_collision_pushback(
                    collision,
                    movement_a,
                    WEAK_COLLISION_PUSHBACK,
                );
            }
        }
    }
}

pub fn set_directions_to_target(
    mut movement: Mut<Movement>,
    position: Vec3,
    target_position: Vec3
) {
    movement.direction_x = set_direction_to_target(
        movement.velocity,
        position[0],
        target_position[0],
    );
    movement.direction_y = set_direction_to_target(
        movement.velocity,
        position[1],
        target_position[1],
    );
}
fn set_direction_to_target(minimum_distance_difference: f32, position: f32, target_position: f32) -> f32 {
    let target_distance =  target_position - position;
    if target_distance > minimum_distance_difference {1.} else if target_distance < -minimum_distance_difference {-1.} else {0.}
}

fn check_for_collision(transform_a: &Transform, transform_b: &Transform) -> Option<Collision> {
    collide(
        transform_a.translation,
        transform_a.scale.truncate(),
        transform_b.translation,
        transform_b.scale.truncate()
    )
}

fn apply_collision_pushback(
    collision: Collision,
    mut movement: Mut<Movement>,
    pushback_strength: f32
) -> Mut<Movement>{
    match collision {
        Collision::Left => movement.direction_x = -pushback_strength,
        Collision::Right => movement.direction_x = pushback_strength,
        Collision::Top => movement.direction_y = pushback_strength,
        Collision::Bottom => movement.direction_y = -pushback_strength,
        Collision::Inside => {
        }

    }
    movement
}