
use bevy::prelude::{Bundle, SpriteBundle, Transform, Vec3, default, Sprite, Color, Res, AssetServer, Vec2};
use bevy_rapier2d::prelude::{Ccd, Collider, GravityScale, RigidBody, Velocity, Sleeping, CollisionGroups};

use crate::in_game::data_classes::generic_components::{GameScreenMarker, Health};
use crate::in_game::data_classes::rigid_body_constants::DEFAULT_COLLISION_GROUPS;
use crate::in_game::rigid_body::rigid_body_bundle::RigidBodyBundle;
use super::generic_constants::Z_VALUE;
use super::data_classes::movement_components::{
    Movement,
};
use super::enemy_components::EnemyMarker;

#[derive(Bundle)]
pub struct ZombieBundle {
    enemy_marker: EnemyMarker,
    game_screen_marker: GameScreenMarker,
    rigid_body_bundle: RigidBodyBundle,
    health: Health,
}
impl ZombieBundle {
    pub fn new(x: f32, y: f32, asset_server: Res<AssetServer>) -> ZombieBundle {

        let zombie_rigid_body = RigidBodyBundle {
            rigid_body: RigidBody::Dynamic,
            velocity: Velocity {
                linvel: Vec2::new(0.0, 0.0),
                angvel: 0.0,
            },
            gravity: GravityScale(0.0),
            collider: Collider::cuboid(10., 10.),
            continuous_collision_detection: Ccd::enabled(),
            sprite_bundle: SpriteBundle {
                texture: asset_server.load("textures/player.png"),
                transform: Transform {
                    translation: Vec3::new(x, y, Z_VALUE),
                    ..default()
                },
                ..default()
            },
            sleeping: Sleeping::disabled(),
            collision_groups: DEFAULT_COLLISION_GROUPS,
        };

        ZombieBundle {
            game_screen_marker: GameScreenMarker,
            enemy_marker: EnemyMarker,
            health: Health(10.),
            rigid_body_bundle: zombie_rigid_body,
        }
    }
}
