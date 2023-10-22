

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::in_game::data_classes::rigid_body_components::WalkingVelocity;
use crate::in_game::data_classes::rigid_body_constants::{DEFAULT_ACTIVE_EVENTS, DEFAULT_GRAVITY, DEFAULT_VELOCITY};
use crate::in_game::data_classes::generic_components::GameScreenMarker;
use crate::in_game::data_classes::player_constants::{PLAYER_SIZE, INITIAL_PLAYER_HEALTH};
use crate::in_game::data_classes::generic_components::Health;
use crate::in_game::data_classes::generic_constants::CENTER_COORDINATES;
use crate::in_game::data_classes::player_components::PlayerMarker;
use crate::in_game::data_classes::rigid_body_constants::PLAYER_COLLISION_GROUPS;

use super::rigid_body_bundle::RigidBodyBundle;

#[derive(Bundle)]
pub struct PlayerBundle {
    player_marker: PlayerMarker,
    game_screen_marker: GameScreenMarker,
    health: Health,
    rigid_body_bundle: RigidBodyBundle,
}


impl PlayerBundle {
    pub fn new(asset_server: Res<AssetServer>) -> PlayerBundle {
        let player_rigid_body = RigidBodyBundle {
            rigid_body: RigidBody::Dynamic,
            velocity: DEFAULT_VELOCITY,
            walking_velocity: WalkingVelocity(200.),
            gravity: DEFAULT_GRAVITY,
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
            active_events: DEFAULT_ACTIVE_EVENTS,
        };
        let player = PlayerBundle {
            player_marker: PlayerMarker,
            game_screen_marker: GameScreenMarker,
            health: Health(INITIAL_PLAYER_HEALTH),
            rigid_body_bundle: player_rigid_body,
        };

        player
    }
}
