

use bevy::prelude::*;
use bevy::time::Time;
use bevy_rapier2d::prelude::*;

use crate::assets::asset_components::BulletTexture;

use crate::in_game::data_classes::bullet_components::{BulletMarker, LifeTime};
use crate::in_game::data_classes::bullet_constants::{BULLET_LENGTH, SHOOTER_DISTANCE_BUFFER};
use crate::in_game::data_classes::bullet_events::PlayerShootEvent;
use crate::generic_constants::Z_VALUE;
use crate::in_game::data_classes::player_components::PlayerMarker;
use crate::in_game::data_classes::player_constants::PLAYER_SIZE;

use crate::in_game::data_layers::bullet_bundle::BulletBundle;
use crate::in_game::systems::rigid_body_systems::RigidBodySystems;


pub struct BulletSystems;
impl BulletSystems {

    pub fn spawn_player_bullet(
        mut commands: Commands,
        mut player_shoot_event: EventReader<PlayerShootEvent>,
        mut player_query: Query<(Entity, &mut Transform, &CollisionGroups, &Velocity), With<PlayerMarker>>,
        bullet_texture_query: Query<&BulletTexture>,
    ) {
        for shoot_event in player_shoot_event.iter() {
            for (entity, player_transform, collision_groups, velocity) in player_query.iter_mut() {
                if shoot_event.0 != entity {continue}
                let bullet_bundle = BulletBundle::new(
                    Self::generate_bullet_transform(player_transform, velocity),
                    *collision_groups,
                    bullet_texture_query.single().0.clone(),
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

    fn generate_bullet_transform(shooter_transform: Mut<Transform>, shooter_velocity: &Velocity) -> Transform {
        let mut shooter_transform_clone = shooter_transform.clone();
        shooter_transform_clone = RigidBodySystems::rotate_single(shooter_transform_clone, shooter_velocity);
        let shooter_front = (shooter_transform_clone.rotation * Vec3::Y).truncate().normalize();
        Transform {
            translation: Vec3::new(
                Self::get_bullet_start_axis(shooter_transform_clone.translation.x, shooter_front[0]),
                Self::get_bullet_start_axis(shooter_transform_clone.translation.y, shooter_front[1]),
                Z_VALUE
            ),
            rotation: shooter_transform_clone.rotation,
            ..default()
        }
    }
    fn get_bullet_start_axis(shooter_axis: f32, shooter_direction: f32) -> f32 {
        shooter_axis + (PLAYER_SIZE/2. + BULLET_LENGTH + SHOOTER_DISTANCE_BUFFER)* shooter_direction
    }
}