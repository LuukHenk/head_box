
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
    pub fn spawn_player_bullet(
        mut commands: Commands,
        mut bullet_spawn_events: EventReader<BulletSpawnEvent>,
        player_query: Query<
            (
                &RotationDegrees,
                &CollisionGroups,
                &Transform,
                &Collider,
            ),
            With<PlayerMarker>,
        >,
        weapon_query: Query<&DamagePerHit, With<ActiveWeapon>>,
        bullet_texture_query: Query<&BulletTextureHandle>,
    ) {
        let damage_per_hit = weapon_query.single();
        for bullet_rotation_offset in bullet_spawn_events.read() {
            for (rotation_degrees, collision_groups, transform, collider) in player_query.iter() {
                let bullet_transform = Self::generate_bullet_transform(rotation_degrees, transform, collider, bullet_rotation_offset.0);
                let bullet_bundle = Self::new_bullet(
                    bullet_transform,
                    *collision_groups,
                    bullet_texture_query.single().0.clone(),
                    damage_per_hit.0,
                );
                commands.spawn(bullet_bundle);
            }
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

    fn generate_bullet_transform(
        shooter_rotation_degrees: &RotationDegrees,
        shooter_transform: &Transform,
        shooter_collider: &Collider,
        bullet_rotation_offset: f32,
    ) -> Transform {
        let shooter_size = shooter_collider.as_cuboid().unwrap().half_extents();
        let shooter_rotation_degrees = shooter_rotation_degrees.0 + bullet_rotation_offset;
        let shooter_rotation = Quat::from_rotation_z(shooter_rotation_degrees.to_radians());
        let shooter_front = (shooter_rotation * Vec3::Y).truncate().normalize();

        let shooter_transform_x = Self::get_bullet_start_axis(
            shooter_transform.translation.x,
            shooter_front[0],
            shooter_size.x,
            SCALING.x,
        );
        let shooter_transform_y = Self::get_bullet_start_axis(
            shooter_transform.translation.y,
            shooter_front[1],
            shooter_size.y,
            SCALING.y,
        );
        Transform {
            translation: Vec3::new(shooter_transform_x, shooter_transform_y, Z_VALUE),
            rotation: shooter_rotation,
            scale: SCALING,
        }
    }
    fn get_bullet_start_axis(
        shooter_axis: f32,
        shooter_direction: f32,
        shooter_radius: f32,
        scaling: f32,
    ) -> f32 {
        shooter_axis
            + (shooter_radius + BULLET_LENGTH * scaling + SHOOTER_DISTANCE_BUFFER) * shooter_direction
    }

    fn new_bullet(
        transform: Transform,
        collision_groups: CollisionGroups,
        texture: Handle<Image>,
        damage_per_hit: f32,
    ) -> BulletBundle {
        let bullet_timer = Timer::new(Duration::from_secs_f32(0.1), TimerMode::Once);

        BulletBundle {
            damage: Damage(damage_per_hit),
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
