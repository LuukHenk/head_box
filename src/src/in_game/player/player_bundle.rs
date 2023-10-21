

use bevy::prelude::{Commands, Res, AssetServer, Bundle, Transform, Vec2, SpriteBundle, Vec3, default};
use bevy_rapier2d::prelude::{Ccd, Collider, GravityScale, RigidBody, Velocity, Sleeping, CollisionGroups};


use super::rigid_body_bundle::RigidBodyBundle;
use super::data_classes::generic_components::GameScreenMarker;
use super::data_classes::player_constants::{PLAYER_SIZE, INITIAL_PLAYER_HEALTH};
use super::generic_components::Health;
use super::data_classes::generic_constants::CENTER_COORDINATES;
use super::player_components::{
    PlayerMarker,
};
use super::data_classes::rigid_body_constants::PLAYER_COLLISION_GROUPS;
#[derive(Bundle)]
pub struct PlayerBundle {
    player_marker: PlayerMarker,
    game_screen_marker: GameScreenMarker,
    health: Health,
    rigid_body_bundle: RigidBodyBundle

}


impl PlayerBundle {
    pub fn new(asset_server: Res<AssetServer>) -> PlayerBundle {
        let player_rigid_body = RigidBodyBundle {
            rigid_body: RigidBody::Dynamic,
            velocity: Velocity {
                linvel: Vec2::new(0.0, 0.0),
                angvel: 0.0,
            },
            gravity: GravityScale(0.0),
            collider: Collider::cuboid(PLAYER_SIZE, PLAYER_SIZE),
            continuous_collision_detection: Ccd::enabled(),
            sprite_bundle: SpriteBundle {
                texture: asset_server.load("textures/player.png"),
                transform: Transform {
                    translation: CENTER_COORDINATES,
                    ..default()
                },
                ..default()
            },
            sleeping: Sleeping::disabled(),
            collision_groups: PLAYER_COLLISION_GROUPS,
        };
        let player = PlayerBundle {
            player_marker: PlayerMarker,
            game_screen_marker: GameScreenMarker,
            health: Health(INITIAL_PLAYER_HEALTH),
            rigid_body_bundle: player_rigid_body
        };

        player
    }
}
