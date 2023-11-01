use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::time::Duration;

use crate::utils::generic_constants::{CENTER_COORDINATES, SCALING, Z_VALUE};
use crate::utils::physics_constants::{
    DEFAULT_ACTIVE_EVENTS, DEFAULT_GRAVITY, DEFAULT_VELOCITY, PLAYER_COLLISION_GROUPS,
};

use crate::events::bullet_events::PlayerShootEvent;

use crate::components::asset_components::PlayerTextures;
use crate::components::generic_components::GameScreenMarker;
use crate::components::generic_components::Health;
use crate::components::player_components::{
    PlayerMarker, RotationDegrees, ShootingCoolDownTimer,
};
use crate::components::physics_components::WalkingVelocity;


const INITIAL_PLAYER_HEALTH: f32 = 300.;
const PLAYER_SIZE: f32 = 7.5;

#[derive(Bundle)]
struct PlayerBundle {
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

pub struct PlayerSystems;

impl PlayerSystems {
    pub fn spawn_player(mut commands: Commands, player_texture_query: Query<&PlayerTextures>) {
        let player = PlayerBundle {
            player_marker: PlayerMarker,
            game_screen_marker: GameScreenMarker,
            health: Health(INITIAL_PLAYER_HEALTH),
            shooting_cooldown_timer: ShootingCoolDownTimer(Timer::new(
                Duration::from_secs_f32(1.),
                TimerMode::Once,
            )),
            rotation_degrees: RotationDegrees(180_f32),
            transform: Transform {
                translation: Vec3::new(-20_f32, 730_f32, Z_VALUE),
                scale: SCALING,
                ..default()
            },
            global_transform: GlobalTransform::default(),
            walking_velocity: WalkingVelocity(300.),
            rigid_body: RigidBody::Dynamic,
            collider: Collider::cuboid(PLAYER_SIZE, PLAYER_SIZE),
            gravity: DEFAULT_GRAVITY,
            velocity: DEFAULT_VELOCITY,
            texture: player_texture_query.single().front.clone(),
            sprite: Sprite::default(),
            visibility: Default::default(),
            computed_visibility: Default::default(),
            continuous_collision_detection: Ccd::enabled(),
            sleeping: Sleeping::disabled(),
            collision_groups: PLAYER_COLLISION_GROUPS,
            active_events: DEFAULT_ACTIVE_EVENTS,
        };
        commands.spawn(player);
    }

    pub fn set_velocity(
        keyboard_input: Res<Input<KeyCode>>,
        mut velocity_query: Query<(&mut Velocity, &WalkingVelocity), With<PlayerMarker>>,
        test_query: Query<&Transform, With<PlayerMarker>>
    ) {
        println!("{:#?}", test_query.single().translation);
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

    pub fn set_rotation_degrees(
        mut query: Query<(&mut RotationDegrees, &Velocity), With<PlayerMarker>>,
    ) {
        let (mut rotation_degrees, velocity) = query.single_mut();
        if velocity.linvel[0] < 0. && velocity.linvel[1] == 0. {
            rotation_degrees.0 = 90.0_f32
        } else if velocity.linvel[0] < 0. && velocity.linvel[1] > 0. {
            rotation_degrees.0 = 45.0_f32
        } else if velocity.linvel[0] == 0. && velocity.linvel[1] > 0. {
            rotation_degrees.0 = 0.0_f32
        } else if velocity.linvel[0] > 0. && velocity.linvel[1] > 0. {
            rotation_degrees.0 = 315.0_f32
        } else if velocity.linvel[0] > 0. && velocity.linvel[1] == 0. {
            rotation_degrees.0 = 270.0_f32
        } else if velocity.linvel[0] > 0. && velocity.linvel[1] < 0. {
            rotation_degrees.0 = 225.0_f32
        } else if velocity.linvel[0] == 0. && velocity.linvel[1] < 0. {
            rotation_degrees.0 = 180.0_f32
        } else if velocity.linvel[0] < 0. && velocity.linvel[1] < 0. {
            rotation_degrees.0 = 135.0_f32
        };
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
        mut player_query: Query<(&RotationDegrees, &mut Handle<Image>), With<PlayerMarker>>,
        player_sprites_query: Query<&PlayerTextures>,
    ) {
        let player_sprites = player_sprites_query.single();
        for (rotation_degrees, mut player_texture) in player_query.iter_mut() {
            if rotation_degrees.0 > 0. && rotation_degrees.0 < 180. {
                *player_texture = player_sprites.side_flipped.clone();
            } else if rotation_degrees.0 > 180. {
                *player_texture = player_sprites.side.clone();
            } else if rotation_degrees.0 == 180. {
                *player_texture = player_sprites.front.clone();
            } else if rotation_degrees.0 == 0. {
                *player_texture = player_sprites.back.clone();
            }
        }
    }
}
