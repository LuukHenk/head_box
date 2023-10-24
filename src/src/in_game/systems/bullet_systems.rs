

use bevy::prelude::*;
use bevy::time::Time;
use bevy_rapier2d::prelude::*;

use crate::assets::asset_components::BulletTexture;

use crate::in_game::data_classes::bullet_components::{BulletMarker, LifeTime};
use crate::in_game::data_classes::bullet_events::PlayerShootEvent;
use crate::in_game::data_classes::player_components::PlayerMarker;
use crate::in_game::data_classes::player_constants::PLAYER_SIZE;

use crate::in_game::data_layers::bullet_bundle::BulletBundle;


pub struct BulletSystems;
impl BulletSystems {

    pub fn spawn_player_bullet(
        mut commands: Commands,
        mut player_shoot_event: EventReader<PlayerShootEvent>,
        mut player_query: Query<(Entity, &mut Transform, &CollisionGroups), With<PlayerMarker>>,
        bullet_texture_query: Query<&BulletTexture>,
    ) {
        for shoot_event in player_shoot_event.iter() {
            for (entity, transform, collision_groups) in player_query.iter_mut() {
                let texture = bullet_texture_query.single();
                if shoot_event.0 != entity {continue}
                let bullet_bundle = BulletBundle::new(
                    transform,
                    PLAYER_SIZE,
                    *collision_groups,
                    texture.0.clone(),
                );
                commands.spawn(bullet_bundle);
            }
        }
    }
    pub fn despawn_bullets(
        mut commands: Commands,
        mut bullet_query: Query<(&mut LifeTime, Entity), With<BulletMarker>>,
        time: Res<Time>
    ) {
        for (mut life_time, entity) in bullet_query.iter_mut() {
            life_time.0.tick(time.delta());
            if life_time.0.finished() {
                commands.entity(entity).despawn()
            }
        }
    }
}