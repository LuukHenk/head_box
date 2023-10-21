use std::time::Duration;
use bevy::prelude::{
    Bundle,
    Transform,
    Vec3,
    SpriteBundle,
    default,
    Timer,
    TimerMode,
    Mut,
    Entity,
    Vec2,
    Res,
    AssetServer
};
use bevy_rapier2d::prelude::{Ccd, Collider, GravityScale, RigidBody, Sleeping, Velocity};

use super::data_classes::bullet_constants::{BULLET_LENGTH, BULLET_WIDTH, SHOOTER_DISTANCE_BUFFER};
use super::data_classes::bullet_components::{BulletMarker, BulletOwner, Damage, LifeTime};
use super::data_classes::generic_components::GameScreenMarker;
use super::rigid_body::rigid_body_bundle::RigidBodyBundle;
use super::data_classes::generic_constants::{Z_VALUE};

#[derive(Bundle)]
pub struct BulletBundle {
    bullet_marker: BulletMarker,
    damage: Damage,
    life_time: LifeTime,
    bullet_owner: BulletOwner,
    rigid_body_bundle: RigidBodyBundle,
    game_screen_marker: GameScreenMarker,
}

impl BulletBundle {
    pub fn new(
        shooter_transform: Mut<Transform>,
        shooter_size: f32,
        bullet_owner: Entity,
        asset_server: &Res<AssetServer>
    ) -> BulletBundle {

        let shooter_front = (shooter_transform.rotation * Vec3::Y).truncate().normalize();

        let transform = Transform {
            translation: Vec3::new(
                shooter_transform.translation.x + (shooter_size/2. + BULLET_LENGTH + SHOOTER_DISTANCE_BUFFER)* shooter_front[0],
                shooter_transform.translation.y + (shooter_size/2. + BULLET_LENGTH + SHOOTER_DISTANCE_BUFFER)* shooter_front[1],
                Z_VALUE
            ),
            rotation: shooter_transform.rotation,
            ..default()
        };

        let bullet_rigid_body = RigidBodyBundle {
            rigid_body: RigidBody::Fixed,
            velocity: Velocity {
                linvel: Vec2::new(0.0, 0.0),
                angvel: 0.0,
            },
            gravity: GravityScale(0.0),
            collider: Collider::cuboid(BULLET_WIDTH, BULLET_LENGTH),
            continuous_collision_detection: Ccd::disabled(),
            sprite_bundle: SpriteBundle {
                texture: asset_server.load("textures/bullet.png"),
                transform,
                ..default()
            },
            sleeping: Sleeping::disabled(),
        };


        BulletBundle {
            damage: Damage(0.5),
            life_time: LifeTime(Timer::new(Duration::from_secs_f32(0.1), TimerMode::Once)),
            bullet_marker: BulletMarker,
            bullet_owner: BulletOwner(bullet_owner),
            rigid_body_bundle: bullet_rigid_body,
            game_screen_marker: GameScreenMarker,
        }
    }
}