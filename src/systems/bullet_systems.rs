
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::time::Duration;

use crate::events::atttack_events::BulletSpawnEvent;

use crate::components::bullet_components::{BulletMarker, Damage, LifeTime};
use crate::components::generic_components::GameScreenMarker;
use crate::components::player_components::PlayerMarker;
use crate::components::asset_components::BulletTextureHandle;
use crate::components::weapon_components::{ActiveWeapon, DamagePerHit};
use crate::components::physics_components::RotationDegrees;

use crate::utils::generic_constants::{SCALING, Z_VALUE};
use crate::utils::physics_constants::{DEFAULT_ACTIVE_EVENTS, DEFAULT_GRAVITY, DEFAULT_VELOCITY};

const SHOOTER_DISTANCE_BUFFER: f32 = 10.;
const BULLET_LENGTH: f32 = 100.;
const BULLET_WIDTH: f32 = 0.5;

#[derive(Bundle)]
struct BulletBundle {
    // Markers
    bullet_marker: BulletMarker,
    game_screen_marker: GameScreenMarker,

    // Physics
    rigid_body: RigidBody,
    collider: Collider,
    gravity: GravityScale,
    velocity: Velocity,
    continuous_collision_detection: Ccd,
    sleeping: Sleeping,
    collision_groups: CollisionGroups,
    active_events: ActiveEvents,
    transform: Transform,
    global_transform: GlobalTransform,

    // Visibility
    texture: Handle<Image>,
    sprite: Sprite,
    visibility: Visibility,
    inherited_visibility: InheritedVisibility,
    view_visibility: ViewVisibility,

    // Other
    damage: Damage,
    life_time: LifeTime,
}

pub struct BulletSystems;
impl BulletSystems {

    pub fn spawn_bullet(
        mut commands: Commands,
        mut bullet_spawn_event_reader: EventReader<BulletSpawnEvent>,
    ) {
        for bullet_spawn_event in bullet_spawn_event_reader.read() {
            let bullet = Self::new_bullet(
                bullet_spawn_event.transform,
                bullet_spawn_event.collision_groups,
                bullet_spawn_event.texture.clone(),
                bullet_spawn_event.damage.0
            );
            commands.spawn(bullet);
        }
    }
    pub fn despawn_bullets(
        mut commands: Commands,
        mut bullet_query: Query<(&mut LifeTime, Entity), With<BulletMarker>>,
        time: Res<Time>,
    ) {
        for (mut life_time, entity) in bullet_query.iter_mut() {
            life_time.0.tick(time.delta());
            if life_time.0.finished() {
                commands.entity(entity).despawn()
            }
        }
    }

    fn new_bullet(
        transform: Transform,
        collision_groups: CollisionGroups,
        texture: Handle<Image>,
        damage: f32,
    ) -> BulletBundle {
        let bullet_timer = Timer::new(Duration::from_secs_f32(0.1), TimerMode::Once);

        BulletBundle {
            damage: Damage(damage),
            life_time: LifeTime(bullet_timer),
            bullet_marker: BulletMarker,
            game_screen_marker: GameScreenMarker,
            rigid_body: RigidBody::Fixed,
            velocity: DEFAULT_VELOCITY,
            gravity: DEFAULT_GRAVITY,
            collider: Collider::cuboid(BULLET_WIDTH, BULLET_LENGTH),
            continuous_collision_detection: Ccd::disabled(),
            texture,
            transform,
            visibility: Default::default(),
            inherited_visibility: InheritedVisibility::default(),
            view_visibility: ViewVisibility::default(),
            sprite: Sprite::default(),
            global_transform: GlobalTransform::default(),
            sleeping: Sleeping::disabled(),
            collision_groups,
            active_events: DEFAULT_ACTIVE_EVENTS,
        }
    }
}
