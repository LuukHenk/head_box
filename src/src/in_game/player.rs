use std::time::Duration;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::generic_constants::CENTER_COORDINATES;

use crate::assets::asset_components::PlayerTextures;

use crate::in_game::data_classes::rigid_body_components::WalkingVelocity;
use crate::in_game::data_classes::rigid_body_constants::{DEFAULT_ACTIVE_EVENTS, DEFAULT_GRAVITY, DEFAULT_VELOCITY, PLAYER_COLLISION_GROUPS};
use crate::in_game::data_classes::generic_components::GameScreenMarker;
use crate::in_game::data_classes::generic_components::Health;

use crate::in_game::data_classes::player_components::{PlayerMarker, RotationDegrees, ShootingCoolDownTimer};
use crate::in_game::data_classes::bullet_events::PlayerShootEvent;

const INITIAL_PLAYER_HEALTH: f32 = 300.;
const PLAYER_SIZE: f32 = 7.5;

#[derive(Bundle)]
pub(crate) struct Player {
    player_marker: PlayerMarker,
    game_screen_marker: GameScreenMarker,
    health: Health,
    shooting_cooldown_timer: ShootingCoolDownTimer,
    transform: Transform,
    global_transform: GlobalTransform,
    rotation_degrees: RotationDegrees,
    walking_velocity: WalkingVelocity,
    rigid_body: RigidBody,
    collider: Collider,
    gravity: GravityScale,
    velocity: Velocity,
    texture: Handle<Image>,
    sprite: Sprite,
    visibility: Visibility,
    computed_visibility: ComputedVisibility,
    continuous_collision_detection: Ccd,
    sleeping: Sleeping,
    collision_groups: CollisionGroups,
    active_events: ActiveEvents,
}


impl Player {

    pub fn spawn(
        mut commands: Commands,
        player_texture_query: Query<&PlayerTextures>,
    ) {
        let texture = player_texture_query.single();
        let player = Self::new(texture.back.clone());
        commands.spawn(player);
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
    fn new(texture: Handle<Image>) -> Self {
        Player {
            player_marker: PlayerMarker,
            game_screen_marker: GameScreenMarker,
            health: Health(INITIAL_PLAYER_HEALTH),
            shooting_cooldown_timer: ShootingCoolDownTimer(Timer::new(Duration::from_secs_f32(1.), TimerMode::Once)),
            rotation_degrees: RotationDegrees(0_f32),
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
            continuous_collision_detection: Ccd::enabled(),
            sleeping: Sleeping::disabled(),
            collision_groups: PLAYER_COLLISION_GROUPS,
            active_events: DEFAULT_ACTIVE_EVENTS,
        }
    }
}
