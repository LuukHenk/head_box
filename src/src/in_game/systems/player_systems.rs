


use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::in_game::data_classes::rigid_body_components::WalkingVelocity;
use crate::in_game::data_classes::player_components::{CoolDownTimer, PlayerMarker};
use crate::in_game::data_classes::player_constants::PLAYER_SIZE;

use crate::in_game::data_layers::bullet_bundle::BulletBundle;
use crate::in_game::data_layers::player_bundle::PlayerBundle;

pub struct PlayerSystems;


impl PlayerSystems {

    pub fn spawn(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
    ) {
        commands.spawn(PlayerBundle::new(asset_server));
    }

    pub fn set_velocity(
        keyboard_input: Res<Input<KeyCode>>,
        mut velocity_query: Query<(&mut Velocity, &WalkingVelocity), With<PlayerMarker>>
    ) {
        for (mut velocity, walking_velocity) in velocity_query.iter_mut() {
            velocity.angvel = 0.;
            velocity.linvel = Vec2::new(0., 0.);
            if keyboard_input.pressed(KeyCode::Right) {
                velocity.linvel[0] += walking_velocity.0;
            }
            if keyboard_input.pressed(KeyCode::Left) {
                velocity.linvel[0] -= walking_velocity.0;
            }
            if keyboard_input.pressed(KeyCode::Up) {
                velocity.linvel[1] += walking_velocity.0;
            }
            if keyboard_input.pressed(KeyCode::Down) {
                velocity.linvel[1] -= walking_velocity.0;
            }
        }
    }

    pub fn shoot(
        keyboard_input: Res<Input<KeyCode>>,
        mut player_query: Query<(&mut Transform, &CollisionGroups, &mut CoolDownTimer), With<PlayerMarker>>,
        mut commands: Commands,
        time: Res<Time>,
        asset_server: Res<AssetServer>
    ) {
        for (transform, collision_groups, mut cooldown_timer) in player_query.iter_mut() {
            cooldown_timer.0.tick(time.delta());
            if !keyboard_input.pressed(KeyCode::Space) || !cooldown_timer.0.finished() {continue};
            let bullet_bundle = BulletBundle::new(transform, PLAYER_SIZE, *collision_groups, &asset_server);
            commands.spawn(bullet_bundle);
            cooldown_timer.0.reset();
        }
    }
}