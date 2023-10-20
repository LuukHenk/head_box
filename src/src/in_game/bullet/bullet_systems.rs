

use bevy::prelude::{Commands, Entity, Query, Res, With};
use bevy::time::Time;
use crate::in_game::data_classes::bullet_components::{BulletMarker, LifeTime};

pub struct BulletSystems;

impl BulletSystems {

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