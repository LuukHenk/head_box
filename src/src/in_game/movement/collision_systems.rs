
use bevy::prelude::{
    Query,
    Transform,
    With,
    Entity,
    Mut,
};
use bevy::sprite::collide_aabb::{collide, Collision};
use crate::in_game::data_classes::direction_constants::{DOWN, LEFT, RIGHT, UP};
use super::movement_components::Movement;
use super::bullet_components::BulletOwner;
use super::movement_constants::{WEAK_COLLISION_PUSHBACK, STRONG_COLLISION_PUSHBACK};
use super::player_components::PlayerMarker;
use super::enemy_components::EnemyMarker;
use super::generic_components::Health;
use super::wall_components::WallMarker;
use super::bullet_components::BulletMarker;


pub struct CollisionSystems;

impl CollisionSystems {

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
                    );
                    player_health.0 -= 1.;
                    println!("Auch! HP: {:#?}", player_health.0) // TODO: Remove when there is a health display
                }
            }
        }
    }

    pub fn handle_bullet_collision(
        bullet_query: Query<(&Transform, &BulletOwner), With<BulletMarker>>,
        mut collision_query: Query<(&Transform, &mut Movement, Entity), With<Movement>>
    ){
        for (target_transform, mut movement, target_entity) in collision_query.iter_mut() {
            for (bullet_transform, bullet_owner) in bullet_query.iter() {
                if target_entity == bullet_owner.0 {continue}
                let collision = Self::check_for_collision(bullet_transform, target_transform);
                if let Some(collision) = collision {

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
                let collision = Self::check_for_collision(transform_a, transform_b);
                if let Some(collision) = collision {
                    movement_a = Self::apply_collision_pushback(
                        collision,
                        movement_a,
                    );
                }
            }
        }
    }
    pub fn prevent_wall_collision(
        mut moving_objects_query: Query<(&Transform, &mut Movement), With<Movement>>,
        wall_query: Query<&Transform, With<WallMarker>>,
    ) {
        for (transform_a, mut movement_a) in moving_objects_query.iter_mut() { ;
            for transform_b in wall_query.iter() {
                let collision = Self::check_for_collision(transform_a, transform_b);
                if let Some(collision) = collision {
                    movement_a = Self::apply_collision_pushback(
                        collision,
                        movement_a,
                    );
                }
            }
        }
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
    ) -> Mut<Movement>{
        match collision {
            Collision::Left => movement.direction_x = LEFT,
            Collision::Right => movement.direction_x = RIGHT,
            Collision::Top => movement.direction_y = UP,
            Collision::Bottom => movement.direction_y = DOWN,
            Collision::Inside => {
                println!("Collision inside!")
            }
        }
        movement.current_velocity = 5.;
        movement
    }
}