use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::components::bullet_components::{BulletMarker, Damage};
use crate::components::enemy_components::EnemyMarker;
use crate::components::generic_components::Health;
use crate::components::player_components::PlayerMarker;

pub struct CollisionSystems;
impl CollisionSystems {

    fn detect_collision(
        entity1: Entity,
        entity2: Entity,
        rapier_context: &Res<RapierContext>,
    ) -> bool {
        if let Some(contact_pair) = rapier_context.contact_pair(entity1, entity2) {
            return contact_pair.has_any_active_contacts();
        }
        false
    }

    // TODO: could we combine these two functions into a single one?
    // TODO: or maybe add an attack for the enemies, instead of using collision
    pub fn handle_player_enemy_collision(
        rapier_context: Res<RapierContext>,
        mut player_query: Query<(Entity, &mut Health), With<PlayerMarker>>,
        enemy_query: Query<Entity, With<EnemyMarker>>,
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
        bullet_query: Query<(Entity, &Damage), With<BulletMarker>>,
    ) {
        for (target_entity, mut target_health) in target_query.iter_mut() {
            for (enemy_entity, damage) in bullet_query.iter() {
                if Self::detect_collision(target_entity, enemy_entity, &rapier_context) {
                    target_health.0 -= damage.0;
                    println!("Auch! HP: {:#?}", target_health.0)
                }
            }
        }
    }

}
