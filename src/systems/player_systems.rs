use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::time::Duration;

use crate::utils::generic_constants::{SCALING, Z_VALUE};
use crate::utils::physics_constants::{
    DEFAULT_ACTIVE_EVENTS, DEFAULT_GRAVITY, DEFAULT_VELOCITY, PLAYER_COLLISION_GROUPS,
};

use crate::events::shooting_events::{ShootRequestEvent, WeaponSelectionEvent};

use crate::components::asset_components::{CurrentAnimationFrame, PlayerTextureHandles};
use crate::components::generic_components::GameScreenMarker;
use crate::components::generic_components::Health;
use crate::components::player_components::PlayerMarker;
use crate::components::physics_components::RotationDegrees;
use crate::components::shooting_components::{GunType, ShootingCoolDownTimer};
use crate::components::physics_components::WalkingVelocity;


const INITIAL_PLAYER_HEALTH: f32 = 300.;

#[derive(Bundle)]
struct PlayerBundle {
    // Markers
    player_marker: PlayerMarker,
    game_screen_marker: GameScreenMarker,

    // Physics
    rotation_degrees: RotationDegrees,
    walking_velocity: WalkingVelocity,
    rigid_body: RigidBody,
    collider: Collider,
    gravity: GravityScale,
    velocity: Velocity,
    continuous_collision_detection: Ccd,
    sleeping: Sleeping,
    collision_groups: CollisionGroups,
    active_events: ActiveEvents,
    locked_axis: LockedAxes,
    transform: Transform,
    global_transform: GlobalTransform,

    // Visibility
    current_animation_frame: CurrentAnimationFrame,
    texture: Handle<Image>,
    sprite: Sprite,
    visibility: Visibility,
    computed_visibility: ComputedVisibility,

    // Other
    health: Health,
    shooting_cooldown_timer: ShootingCoolDownTimer,
}

pub struct PlayerSystems;

impl PlayerSystems {
    pub fn spawn_player(
        mut commands: Commands,
        player_texture_query: Query<&PlayerTextureHandles>,
    ) {
        let player_texture = player_texture_query.single().front[0].clone();


        let player = PlayerBundle {
            player_marker: PlayerMarker,
            game_screen_marker: GameScreenMarker,
            health: Health(INITIAL_PLAYER_HEALTH),
            shooting_cooldown_timer: ShootingCoolDownTimer(Timer::new(
                Duration::from_secs_f32(0.1),
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
            collider: Collider::cuboid(5., 6.),
            gravity: DEFAULT_GRAVITY,
            velocity: DEFAULT_VELOCITY,
            current_animation_frame: CurrentAnimationFrame(1),
            texture: player_texture,
            sprite: Sprite::default(),
            visibility: Default::default(),
            computed_visibility: Default::default(),
            continuous_collision_detection: Ccd::enabled(),
            sleeping: Sleeping::disabled(),
            collision_groups: PLAYER_COLLISION_GROUPS,
            active_events: DEFAULT_ACTIVE_EVENTS,
            locked_axis: LockedAxes::ROTATION_LOCKED,
        };
        commands.spawn(player);
    }

    pub fn set_velocity(
        keyboard_input: Res<Input<KeyCode>>,
        mut velocity_query: Query<(&mut Velocity, &WalkingVelocity), With<PlayerMarker>>,
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
        mut player_shoot_event: EventWriter<ShootRequestEvent>,
    ) {
        if keyboard_input.pressed(KeyCode::Space) {
            player_shoot_event.send(ShootRequestEvent);
        };
    }

    pub fn weapon_selection(
        keyboard_input: Res<Input<KeyCode>>,
        mut weapon_selection_event: EventWriter<WeaponSelectionEvent>,
    ) {
        if keyboard_input.pressed(KeyCode::Key1) {
            weapon_selection_event.send(WeaponSelectionEvent(GunType::Pistol));
        } else if keyboard_input.pressed(KeyCode::Key2){
            weapon_selection_event.send(WeaponSelectionEvent(GunType::Uzi));
        } else if keyboard_input.pressed(KeyCode::Key3){
            weapon_selection_event.send(WeaponSelectionEvent(GunType::Shotgun));
        }
    }

}
