use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;
use crate::components::arena_components::EnemySpawnLocation;

use crate::events::enemy_spawn_events::SpawnZombieEvent;

use crate::components::enemy_components::EnemyMarker;
use crate::components::level_components::{ActiveLevelMarker, KilledEnemies};
use crate::components::player_components::PlayerMarker;
use crate::components::physics_components::WalkingVelocity;
use crate::components::asset_components::ZombieTexture;
use crate::components::bullet_components::Damage;
use crate::components::generic_components::{GameScreenMarker, Health};

use crate::utils::generic_constants::{SCALING, Z_VALUE,};
use crate::utils::physics_constants::{DEFAULT_COLLISION_GROUPS, DEFAULT_GRAVITY, DEFAULT_VELOCITY,};

const ZOMBIE_SIZE: f32 = 4.;

#[derive(Bundle)]
pub struct EnemyBundle {
    enemy_marker: EnemyMarker,
    game_screen_marker: GameScreenMarker,
    health: Health,
    damage: Damage,
    rigid_body: RigidBody,
    velocity: Velocity,
    walking_velocity: WalkingVelocity,
    gravity: GravityScale,
    collider: Collider,
    continuous_collision_detection: Ccd,
    sleeping: Sleeping,
    collision_groups: CollisionGroups,
    active_events: ActiveEvents,
    transform: Transform,
    global_transform: GlobalTransform,
    texture: Handle<Image>,
    sprite: Sprite,
    visibility: Visibility,
    computed_visibility: ComputedVisibility,
    locked_axis: LockedAxes,
}

pub struct EnemySystems;

impl EnemySystems {
    pub fn spawn_zombies(
        mut spawn_zombie_event: EventReader<SpawnZombieEvent>,
        mut commands: Commands,
        enemy_spawn_location_query: Query<&EnemySpawnLocation>,
        zombie_texture_query: Query<&ZombieTexture>,
    ) {
        let texture = zombie_texture_query.single();

        let mut enemy_spawn_locations: Vec<Vec3> = Vec::new();
        for enemy_spawn_location in enemy_spawn_location_query.iter() {
            enemy_spawn_locations.push(enemy_spawn_location.0);
        }

        for _ in spawn_zombie_event.iter() {
            let random_position = rand::thread_rng().gen_range(0..enemy_spawn_locations.len());
            let spawn_position = enemy_spawn_locations[random_position];
            let zombie = Self::new_enemy(spawn_position.x, spawn_position.y, texture.0.clone());
            commands.spawn(zombie);
        }
    }

    pub fn despawn_enemies(
        mut commands: Commands,
        query: Query<(Entity, &Health), With<EnemyMarker>>,
        mut active_level_query: Query<&mut KilledEnemies, With<ActiveLevelMarker>>,
    ) {
        for (entity, health) in query.iter() {
            if health.0 <= 0.0 {
                commands.entity(entity).despawn();
                let mut killed_enemies = active_level_query.single_mut();
                killed_enemies.0 += 1;
            }
        }
    }

    pub fn set_velocity(
        mut enemy_query: Query<(&mut Velocity, &Transform, &WalkingVelocity), With<EnemyMarker>>,
        player_query: Query<(&Transform, &Collider), With<PlayerMarker>>,
    ) {
        for (player_transform, player_collider) in player_query.iter() {
            let player_position = player_transform.translation;
            let player_size = player_collider.as_cuboid().unwrap().half_extents();
            for (mut enemy_velocity, enemy_transform, walking_velocity) in enemy_query.iter_mut() {
                let enemy_position = enemy_transform.translation;
                enemy_velocity.linvel[0] = Self::set_direction_to_target(
                    walking_velocity.0,
                    enemy_position[0],
                    player_position[0],
                    player_size[0],
                );
                enemy_velocity.linvel[1] = Self::set_direction_to_target(
                    walking_velocity.0,
                    enemy_position[1],
                    player_position[1],
                    player_size[1],
                );
            }
        }
    }

    fn set_direction_to_target(
        velocity: f32,
        position: f32,
        target_position: f32,
        target_size: f32,
    ) -> f32 {
        let target_distance = target_position - position;
        if target_distance > target_size * 2. {
            velocity
        } else if target_distance < -target_size * 2. {
            -velocity
        } else {
            0.
        }
    }

    fn new_enemy(x: f32, y: f32, texture: Handle<Image>) -> EnemyBundle {
        EnemyBundle {
            game_screen_marker: GameScreenMarker,
            enemy_marker: EnemyMarker,
            health: Health(10.),
            damage: Damage(1.),
            rigid_body: RigidBody::Dynamic,
            velocity: DEFAULT_VELOCITY,
            walking_velocity: WalkingVelocity(100.),
            gravity: DEFAULT_GRAVITY,
            collider: Collider::cuboid(ZOMBIE_SIZE, ZOMBIE_SIZE),
            continuous_collision_detection: Ccd::enabled(),
            sleeping: Sleeping::disabled(),
            collision_groups: DEFAULT_COLLISION_GROUPS,
            active_events: ActiveEvents::COLLISION_EVENTS,
            locked_axis: LockedAxes::ROTATION_LOCKED,
            texture,
            transform: Transform {
                translation: Vec3::new(x, y, Z_VALUE),
                scale: SCALING,
                ..default()
            },
            global_transform: GlobalTransform::default(),
            sprite: Sprite::default(),
            visibility: Default::default(),
            computed_visibility: Default::default(),
        }
    }
}
