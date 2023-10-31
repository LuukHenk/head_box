use std::time::Duration;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::in_game::data_classes::rigid_body_components::WalkingVelocity;
use crate::in_game::data_classes::rigid_body_constants::{DEFAULT_GRAVITY, DEFAULT_VELOCITY};
use crate::in_game::data_classes::generic_components::GameScreenMarker;
use crate::in_game::data_classes::player_constants::{PLAYER_SIZE, INITIAL_PLAYER_HEALTH};
use crate::in_game::data_classes::generic_components::Health;
use crate::generic_constants::CENTER_COORDINATES;
use crate::in_game::data_classes::player_components::{PlayerMarker, ShootingCoolDownTimer};

#[derive(Bundle)]
pub struct PlayerBundle {
    player_marker: PlayerMarker,
    game_screen_marker: GameScreenMarker,
    health: Health,
    shooting_cooldown_timer: ShootingCoolDownTimer,
    transform: Transform,
    global_transform: GlobalTransform,
    walking_velocity: WalkingVelocity,
    rigid_body: RigidBody,
    collider: Collider,
    gravity: GravityScale,
    velocity: Velocity,
    texture: Handle<Image>,
    sprite: Sprite,
    visibility: Visibility,
    computed_visibility: ComputedVisibility,
}


impl PlayerBundle {
    pub fn new(texture: Handle<Image>) -> PlayerBundle {
        let player = PlayerBundle {
            player_marker: PlayerMarker,
            game_screen_marker: GameScreenMarker,
            health: Health(INITIAL_PLAYER_HEALTH),
            shooting_cooldown_timer: ShootingCoolDownTimer(Timer::new(Duration::from_secs_f32(1.), TimerMode::Once)),
            transform: Transform {
                translation: CENTER_COORDINATES,
                scale: Vec3::new(1.5, 1.5, 1.),
                ..default()
            },
            global_transform: GlobalTransform::default(),
            walking_velocity: WalkingVelocity(300.),
            rigid_body: RigidBody::Dynamic,
            collider: Collider::cuboid(PLAYER_SIZE, PLAYER_SIZE),
            gravity: DEFAULT_GRAVITY,
            velocity: DEFAULT_VELOCITY,
            texture,
            sprite: Sprite::default(),
            visibility: Default::default(),
            computed_visibility: Default::default(),
        };
        player
    }

}
