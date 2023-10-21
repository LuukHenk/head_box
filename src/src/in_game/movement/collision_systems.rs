
use bevy::prelude::{Query, Transform, With, Entity, Mut, Commands, Res};
use bevy::sprite::collide_aabb::{collide, Collision};
use bevy_rapier2d::prelude::RapierContext;
use crate::in_game::data_classes::bullet_components::Damage;
use crate::in_game::data_classes::direction_constants::{DOWN, LEFT, RIGHT, UP};
use super::movement_components::Movement;
use super::bullet_components::BulletOwner;
use super::player_components::PlayerMarker;
use super::enemy_components::EnemyMarker;
use super::generic_components::Health;
use super::wall_components::WallMarker;
use super::bullet_components::BulletMarker;


pub struct CollisionSystems;

impl CollisionSystems {

    pub fn handle_player_enemy_collision(
        rapier_context: Res<RapierContext>,
        mut player_query: Query<(Entity, &mut Health), With<PlayerMarker>>,
        enemy_query: Query<Entity, With<EnemyMarker>>
    ) {
        for (player_entity, mut player_health) in player_query.iter_mut() {
            for enemy_entity in enemy_query.iter() {
                if Self::detect_collision(player_entity, enemy_entity, &rapier_context) {
                    player_health.0 -= 1.;
                    println!("Auch! HP: {:#?}", player_health.0)
                }
            }
        }
    }
    pub fn handle_bullet_collision(
        rapier_context: Res<RapierContext>,
        mut target_query: Query<(Entity, &mut Health), With<Health>>,
        bullet_query: Query<Entity, With<BulletMarker>>
    ){
        for (target_entity, mut target_health) in target_query.iter_mut() {
            for enemy_entity in bullet_query.iter() {
                if Self::detect_collision(target_entity, enemy_entity, &rapier_context) {
                    target_health.0 -= 1.;
                    println!("Auch! HP: {:#?}", target_health.0)
                }
            }
        }
    }
    fn detect_collision(entity1: Entity, entity2: Entity, rapier_context: &Res<RapierContext>) -> bool {
        if let Some(contact_pair) = rapier_context.contact_pair(entity1, entity2) {
            return contact_pair.has_any_active_contacts()
        }
        false
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
        for (transform_a, mut movement_a) in moving_objects_query.iter_mut() {
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