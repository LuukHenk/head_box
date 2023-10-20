


use bevy::prelude::{
    Query,
    Transform,
    With,
    Entity,
    Quat,
    Mut,
};
use bevy::sprite::collide_aabb::{collide, Collision};
use crate::in_game::data_classes::movement_components::CollisionMarker;
use super::movement_components::Movement;
use super::movement_constants::{WEAK_COLLISION_PUSHBACK, STRONG_COLLISION_PUSHBACK};
use super::player_components::PlayerMarker;
use super::enemy_components::EnemyMarker;
use super::generic_components::Health;
use super::wall_components::WallMarker;
use super::bullet_components::BulletMarker;


pub struct MovementSystems;

impl MovementSystems{
    pub fn move_objects(mut query: Query<(&mut Transform, &mut Movement), With<Movement>>) {
        for (mut transform, mut movement) in query.iter_mut() {
            // println!("{:#?}", movement);
            Self::rotate_object(&mut *transform, &mut *movement);
            transform.translation.x += movement.direction_x * movement.velocity;
            transform.translation.y += movement.direction_y * movement.velocity;

        }
    }
    pub fn handle_player_enemy_collision(
        mut player_query: Query<(&Transform, &mut Movement, &mut Health), With<PlayerMarker>>,
        enemies_query: Query<&Transform, With<EnemyMarker>>,
    ) {
        for (player_transform, mut player_movement, mut player_health) in player_query.iter_mut() {
            for enemy_transform in enemies_query.iter() {
                let collision = Self::check_for_collision(player_transform, enemy_transform);
                if let Some(collision) = collision {
                    player_movement = Self::apply_collision_pushback(
                        collision,
                        player_movement,
                        0.3
                    );
                    player_health.0 -= 1.;
                    println!("Auch! HP: {:#?}", player_health.0) // TODO: Remove when there is a health display
                }
            }
        }
    }

    pub fn handle_bullet_collision(
        bullet_query: Query<&Transform, With<BulletMarker>>,
        collision_query: Query<&Transform, With<CollisionMarker>>
    ){

    }
    pub fn prevent_enemy_enemy_collision(
        mut enemies_query_a: Query<(Entity, &Transform, &mut Movement), With<EnemyMarker>>,
        enemies_query_b: Query<(Entity, &Transform), With<EnemyMarker>>,
    ) {
        for (entity_a, transform_a, mut movement_a) in enemies_query_a.iter_mut() {
            for (entity_b, transform_b) in enemies_query_b.iter() {
                if entity_a == entity_b {continue}
                let collision = Self::check_for_collision(transform_a, transform_b);
                if let Some(collision) = collision {
                    movement_a = Self::apply_collision_pushback(
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
            let pushback_strength = movement_a.velocity / 25.;
            for transform_b in wall_query.iter() {
                let collision = Self::check_for_collision(transform_a, transform_b);
                if let Some(collision) = collision {
                    movement_a = Self::apply_collision_pushback(
                        collision,
                        movement_a,
                        pushback_strength,
                    );
                }
            }
        }
    }
    fn rotate_object(transform: &mut Transform, movement: &mut Movement) {
        if movement.direction_y == 0. && movement.direction_x == -1. {
            transform.rotation = Quat::from_rotation_z(90.0_f32.to_radians())
        } else if movement.direction_y == 1. && movement.direction_x == -1. {
            transform.rotation = Quat::from_rotation_z(45.0_f32.to_radians())
        } else if movement.direction_y == 1. && movement.direction_x == 0. {
            transform.rotation = Quat::from_rotation_z(0.0_f32.to_radians())
        } else if movement.direction_y == 1. && movement.direction_x == 1. {
            transform.rotation = Quat::from_rotation_z(315.0_f32.to_radians())
        } else if movement.direction_y == 0. && movement.direction_x == 1. {
            transform.rotation = Quat::from_rotation_z(270.0_f32.to_radians())
        } else if movement.direction_y == -1. && movement.direction_x == 1. {
            transform.rotation = Quat::from_rotation_z(225.0_f32.to_radians())
        } else if movement.direction_y == -1. && movement.direction_x == 0. {
            transform.rotation = Quat::from_rotation_z(180.0_f32.to_radians())
        } else if movement.direction_y == -1. && movement.direction_x == -1. {
            transform.rotation = Quat::from_rotation_z(135.0_f32.to_radians())
        };
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
}