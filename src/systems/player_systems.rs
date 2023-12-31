use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::utils::generic_constants::{SCALING, Z_VALUE};
use crate::utils::physics_constants::{
    DEFAULT_ACTIVE_EVENTS, DEFAULT_GRAVITY, DEFAULT_VELOCITY, PLAYER_COLLISION_GROUPS,
};

use crate::events::atttack_events::{AttackRequestEvent, WeaponSelectionEvent};

use crate::components::asset_components::{CurrentAnimationFrame, CharacterTextureHandles, PlayerTextureMarker};
use crate::components::generic_components::GameScreenMarker;
use crate::components::generic_components::Health;
use crate::components::player_components::PlayerMarker;
use crate::components::physics_components::RotationDegrees;
use crate::components::weapon_components::{WeaponOwnerMarker, WeaponType};
use crate::components::physics_components::WalkingVelocity;


const INITIAL_PLAYER_HEALTH: f32 = 300.;

#[derive(Bundle)]
struct PlayerBundle {
    // Markers
    player_marker: PlayerMarker,
    game_screen_marker: GameScreenMarker,
    weapon_owner_marker: WeaponOwnerMarker,

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
    character_texture_handles: CharacterTextureHandles,
    texture: Handle<Image>,
    sprite: Sprite,
    visibility: Visibility,
    inherited_visibility: InheritedVisibility,
    view_visibility: ViewVisibility,

    // Other
    health: Health,
}

pub struct PlayerSystems;

impl PlayerSystems {
    pub fn spawn_player(
        mut commands: Commands,
        player_texture_handles_query: Query<&CharacterTextureHandles, With<PlayerTextureMarker>>,
    ) {
        let player_texture_handles = player_texture_handles_query.single();
        let current_texture = player_texture_handles.front[0].clone();


        let player = PlayerBundle {
            // Markers
            player_marker: PlayerMarker,
            game_screen_marker: GameScreenMarker,
            weapon_owner_marker: WeaponOwnerMarker,

            // Physics
            rotation_degrees: RotationDegrees(180_f32),
            walking_velocity: WalkingVelocity(300.),
            rigid_body: RigidBody::Dynamic,
            collider: Collider::cuboid(5., 6.),
            gravity: DEFAULT_GRAVITY,
            velocity: DEFAULT_VELOCITY,
            continuous_collision_detection: Ccd::enabled(),
            sleeping: Sleeping::disabled(),
            collision_groups: PLAYER_COLLISION_GROUPS,
            active_events: DEFAULT_ACTIVE_EVENTS,
            locked_axis: LockedAxes::ROTATION_LOCKED,
            transform: Transform {
                translation: Vec3::new(-20_f32, 730_f32, Z_VALUE),
                scale: SCALING,
                ..default()
            },
            global_transform: GlobalTransform::default(),

            // Visibility
            current_animation_frame: CurrentAnimationFrame(1),
            character_texture_handles: player_texture_handles.clone(),
            texture: current_texture,
            sprite: Sprite::default(),
            visibility: Default::default(),
            inherited_visibility: InheritedVisibility::default(),
            view_visibility: ViewVisibility::default(),


            // Others
            health: Health(INITIAL_PLAYER_HEALTH),

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

    pub fn attack(
        keyboard_input: Res<Input<KeyCode>>,
        mut attack_request_event_writer: EventWriter<AttackRequestEvent>,
        player_query: Query<Entity, With<PlayerMarker>>
    ) {
        if keyboard_input.pressed(KeyCode::Space) {
            let attack_event = AttackRequestEvent(player_query.single());
            attack_request_event_writer.send(attack_event);
        };
    }

    pub fn weapon_selection(
        keyboard_input: Res<Input<KeyCode>>,
        mut weapon_selection_event: EventWriter<WeaponSelectionEvent>,
    ) {
        if keyboard_input.pressed(KeyCode::Key1) {
            weapon_selection_event.send(WeaponSelectionEvent(WeaponType::Pistol));
        } else if keyboard_input.pressed(KeyCode::Key2){
            weapon_selection_event.send(WeaponSelectionEvent(WeaponType::Uzi));
        } else if keyboard_input.pressed(KeyCode::Key3){
            weapon_selection_event.send(WeaponSelectionEvent(WeaponType::Shotgun));
        }
    }

}
