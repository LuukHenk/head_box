use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::assets::asset_components::PlayerTextures;
use crate::in_game::data_classes::bullet_events::PlayerShootEvent;

use crate::in_game::data_classes::rigid_body_components::WalkingVelocity;
use crate::in_game::data_classes::player_components::{ShootingCoolDownTimer, PlayerMarker, RotationDegrees};

use crate::in_game::data_layers::player_bundle::PlayerBundle;

pub struct PlayerSystems;


impl PlayerSystems {

    pub fn spawn(
        mut commands: Commands,
        player_texture_query: Query<&PlayerTextures>,
    ) {
        let texture = player_texture_query.single();
        commands.spawn(PlayerBundle::new(texture.back.clone()));
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

    pub fn set_rotation_degrees(mut query: Query<(&mut RotationDegrees, &Velocity), With<PlayerMarker>>) {
        let (mut rotation_degrees, velocity) = query.single_mut();
        if      velocity.linvel[0] < 0.  && velocity.linvel[1] == 0. {rotation_degrees.0 = 90.0_f32}
        else if velocity.linvel[0] < 0.  && velocity.linvel[1] > 0.  {rotation_degrees.0 = 45.0_f32}
        else if velocity.linvel[0] == 0. && velocity.linvel[1] > 0.  {rotation_degrees.0 = 0.0_f32}
        else if velocity.linvel[0] > 0.  && velocity.linvel[1] > 0.  {rotation_degrees.0 = 315.0_f32}
        else if velocity.linvel[0] > 0.  && velocity.linvel[1] == 0. {rotation_degrees.0 = 270.0_f32}
        else if velocity.linvel[0] > 0.  && velocity.linvel[1] < 0.  {rotation_degrees.0 = 225.0_f32}
        else if velocity.linvel[0] == 0. && velocity.linvel[1] < 0.  {rotation_degrees.0 = 180.0_f32}
        else if velocity.linvel[0] < 0.  && velocity.linvel[1] < 0.  {rotation_degrees.0 = 135.0_f32};
    }
    pub fn shoot(
        keyboard_input: Res<Input<KeyCode>>,
        mut player_query: Query<(Entity, &mut ShootingCoolDownTimer), With<PlayerMarker>>,
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

    pub fn change_sprite(
        mut player_query: Query<(&RotationDegrees, &mut Handle<Image>, ), With<PlayerMarker>>,
        player_sprites_query: Query<&PlayerTextures>,
    ) {
        let player_sprites = player_sprites_query.single();
        for (rotation_degrees, mut player_texture) in player_query.iter_mut() {
            if rotation_degrees.0 > 0. && rotation_degrees.0 < 180. {
                *player_texture = player_sprites.side_flipped.clone();
            } else if rotation_degrees.0 > 180.  {
                *player_texture = player_sprites.side.clone();
            } else if rotation_degrees.0 == 180. {
                *player_texture = player_sprites.front.clone();
            } else if rotation_degrees.0 == 0.  {
                *player_texture = player_sprites.back.clone();
            }

        }
    }
}