use std::time::Duration;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::assets::asset_components::BulletTexture;
use crate::generic_constants::{SCALING, Z_VALUE};

use crate::utils::physics_constants::{DEFAULT_ACTIVE_EVENTS, DEFAULT_GRAVITY, DEFAULT_VELOCITY};
use crate::in_game::data_classes::bullet_components::{BulletMarker, Damage, LifeTime};
use crate::events::bullet_events::PlayerShootEvent;
use crate::in_game::data_classes::generic_components::GameScreenMarker;
use crate::in_game::data_classes::player_components::{PlayerMarker, RotationDegrees};

const SHOOTER_DISTANCE_BUFFER: f32 = 10.;
const BULLET_LENGTH: f32 = 100.;
const BULLET_WIDTH: f32 = 0.5;

#[derive(Bundle)]
struct BulletBundle {
    bullet_marker: BulletMarker,
    damage: Damage,
    life_time: LifeTime,
    game_screen_marker: GameScreenMarker,
    rigid_body: RigidBody,
    velocity: Velocity,
    gravity: GravityScale,
    collider: Collider,
    continuous_collision_detection: Ccd,
    sleeping: Sleeping,
    collision_groups: CollisionGroups,
    active_events: ActiveEvents,
    texture: Handle<Image>,
    transform: Transform,
    global_transform: GlobalTransform,
    sprite: Sprite,
    visibility: Visibility,
    computed_visibility: ComputedVisibility,
}

pub struct BulletSystems;
impl BulletSystems {
    pub fn spawn_player_bullet(
        mut commands: Commands,
        mut player_shoot_event: EventReader<PlayerShootEvent>,
        player_query: Query<(
            Entity,
            &RotationDegrees,
            &CollisionGroups,
            &Transform,
            &Collider,
        ), With<PlayerMarker>>,
        bullet_texture_query: Query<&BulletTexture>,
    ) {
        for shoot_event in player_shoot_event.iter() {
            for (entity, rotation_degrees, collision_groups, transform, collider) in player_query.iter() {
                if shoot_event.0 != entity {continue}
                let bullet_transform = Self::generate_bullet_transform(
                    rotation_degrees,
                    transform,
                    collider
                );
                let bullet_bundle = Self::new_bullet(
                    bullet_transform,
                    *collision_groups,
                    bullet_texture_query.single().0.clone(),
                );
                commands.spawn(bullet_bundle);
            }
        }
    }

    pub fn despawn_bullets(
        mut commands: Commands,
        mut bullet_query: Query<(&mut LifeTime, Entity), With<BulletMarker>>,
        time: Res<Time>
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
    ) -> Transform {
        let shooter_size = shooter_collider.as_cuboid().unwrap().half_extents();
        let shooter_rotation = Quat::from_rotation_z(shooter_rotation_degrees.0.to_radians());
        let shooter_front = (shooter_rotation * Vec3::Y).truncate().normalize();

        let shooter_transform_x = Self::get_bullet_start_axis(
            shooter_transform.translation.x,
            shooter_front[0],
            shooter_size.x
        );
        let shooter_transform_y = Self::get_bullet_start_axis(
            shooter_transform.translation.y,
            shooter_front[1],
            shooter_size.y
        );
        Transform {
            translation: Vec3::new(shooter_transform_x, shooter_transform_y, Z_VALUE),
            rotation: shooter_rotation,
            scale: SCALING,
        }
    }
    fn get_bullet_start_axis(shooter_axis: f32, shooter_direction: f32, shooter_radius: f32) -> f32 {
        shooter_axis + (shooter_radius + BULLET_LENGTH + SHOOTER_DISTANCE_BUFFER)* shooter_direction
    }

    fn new_bullet(
        transform: Transform,
        collision_groups: CollisionGroups,
        texture: Handle<Image>,
    ) -> BulletBundle {

        let bullet_timer = Timer::new(Duration::from_secs_f32(0.1), TimerMode::Once);

        BulletBundle {
            damage: Damage(0.5),
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
            computed_visibility: Default::default(),
            sprite: Sprite::default(),
            global_transform: GlobalTransform::default(),
            sleeping: Sleeping::disabled(),
            collision_groups,
            active_events: DEFAULT_ACTIVE_EVENTS,
        }
    }
}