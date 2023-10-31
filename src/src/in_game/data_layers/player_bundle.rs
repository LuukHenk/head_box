use std::time::Duration;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::in_game::data_classes::rigid_body_components::WalkingVelocity;
use crate::in_game::data_classes::rigid_body_constants::{DEFAULT_ACTIVE_EVENTS, DEFAULT_GRAVITY, DEFAULT_VELOCITY};
use crate::in_game::data_classes::generic_components::GameScreenMarker;
use crate::in_game::data_classes::player_constants::{PLAYER_SIZE, INITIAL_PLAYER_HEALTH};
use crate::in_game::data_classes::generic_components::Health;
use crate::generic_constants::CENTER_COORDINATES;
use crate::in_game::data_classes::player_components::{PlayerMarker, CoolDownTimer};
use crate::in_game::data_classes::rigid_body_constants::PLAYER_COLLISION_GROUPS;

use super::rigid_body_bundle::RigidBodyBundle;

#[derive(Bundle)]
pub struct PlayerBundle {
    player_marker: PlayerMarker,
    game_screen_marker: GameScreenMarker,
    health: Health,
    rigid_body_bundle: RigidBodyBundle,
    shooting_cooldown_timer: CoolDownTimer
}


impl PlayerBundle {
    pub fn new(texture: Handle<Image>) -> PlayerBundle {
        let player_rigid_body = RigidBodyBundle {
            rigid_body: RigidBody::Dynamic,
            velocity: DEFAULT_VELOCITY,
            walking_velocity: WalkingVelocity(300.),
            gravity: DEFAULT_GRAVITY,
            collider: Collider::cuboid(PLAYER_SIZE, PLAYER_SIZE),
            continuous_collision_detection: Ccd::enabled(),
            sprite_bundle: SpriteBundle {
                texture,
                transform: Transform {
                    translation: CENTER_COORDINATES,
                    scale: Vec3::new(1.5, 1.5, 1.),
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
            shooting_cooldown_timer: CoolDownTimer(Timer::new(Duration::from_secs_f32(1.), TimerMode::Once))
        };

        player
    }
}
