


use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::assets::asset_components::PlayerTexture;
use crate::in_game::data_classes::bullet_events::PlayerShootEvent;

use crate::in_game::data_classes::rigid_body_components::WalkingVelocity;
use crate::in_game::data_classes::player_components::{CoolDownTimer, PlayerMarker};

use crate::in_game::data_layers::player_bundle::PlayerBundle;

pub struct PlayerSystems;


impl PlayerSystems {

    pub fn spawn(
        mut commands: Commands,
        player_texture_query: Query<&PlayerTexture>,
    ) {
        let texture = player_texture_query.single();
        commands.spawn(PlayerBundle::new(texture.0.clone()));
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
        mut player_query: Query<(Entity, &mut CoolDownTimer), With<PlayerMarker>>,
        mut player_shoot_event: EventWriter<PlayerShootEvent>,
        time: Res<Time>,

    ) {
        for (entity, mut cooldown_timer) in player_query.iter_mut() {
            cooldown_timer.0.tick(time.delta());
            if keyboard_input.pressed(KeyCode::Space) && cooldown_timer.0.finished() {
                player_shoot_event.send(PlayerShootEvent(entity));
                cooldown_timer.0.reset();
            };
        }
    }
}