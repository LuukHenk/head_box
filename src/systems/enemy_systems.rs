use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;
use crate::components::arena_components::EnemySpawnLocation;

use crate::events::enemy_spawn_events::SpawnZombieEvent;

use crate::components::enemy_components::{EnemyMarker, ZombieMarker};
use crate::components::level_components::{ActiveLevelMarker, KilledEnemies};
use crate::components::player_components::PlayerMarker;
use crate::components::physics_components::{RotationDegrees, WalkingVelocity};
use crate::components::asset_components::{CharacterTextureHandles, CurrentAnimationFrame, ZombieTextureMarker};
use crate::components::bullet_components::Damage;
use crate::components::generic_components::{GameScreenMarker, Health};

use crate::utils::generic_constants::{SCALING, Z_VALUE,};
use crate::utils::physics_constants::{DEFAULT_COLLISION_GROUPS, DEFAULT_GRAVITY, DEFAULT_VELOCITY,};


#[derive(Bundle)]
pub struct EnemyBundle {
    // Markers
    game_screen_marker: GameScreenMarker,
    enemy_marker: EnemyMarker,

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
    damage: Damage,
}

pub struct EnemySystems;

impl EnemySystems {
    pub fn spawn_zombies(
        mut spawn_zombie_event: EventReader<SpawnZombieEvent>,
        mut commands: Commands,
        enemy_spawn_location_query: Query<&EnemySpawnLocation>,
        zombie_texture_handles_query: Query<&CharacterTextureHandles, With<ZombieTextureMarker>>,
    ) {
        let zombie_texture_handles = zombie_texture_handles_query.single();

        let mut enemy_spawn_locations: Vec<Vec3> = Vec::new();
        for enemy_spawn_location in enemy_spawn_location_query.iter() {
            enemy_spawn_locations.push(enemy_spawn_location.0);
        }

        for _ in spawn_zombie_event.read() {
            let random_position = rand::thread_rng().gen_range(0..enemy_spawn_locations.len());
            let spawn_position = enemy_spawn_locations[random_position];
            let zombie = Self::new_enemy(spawn_position.x, spawn_position.y, zombie_texture_handles);
            commands.spawn((zombie, ZombieMarker));
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

    fn new_enemy(x: f32, y: f32, enemy_texture_handles: &CharacterTextureHandles) -> EnemyBundle {
        let current_texture = enemy_texture_handles.front[0].clone();

        EnemyBundle {
            // Markers
            game_screen_marker: GameScreenMarker,
            enemy_marker: EnemyMarker,

            // Physics
            rotation_degrees: RotationDegrees(180.),
            walking_velocity: WalkingVelocity(100.),
            rigid_body: RigidBody::Dynamic,
            collider: Collider::cuboid(4., 8.),
            gravity: DEFAULT_GRAVITY,
            velocity: DEFAULT_VELOCITY,
            continuous_collision_detection: Ccd::enabled(),
            sleeping: Sleeping::disabled(),
            collision_groups: DEFAULT_COLLISION_GROUPS,
            active_events: ActiveEvents::COLLISION_EVENTS,
            locked_axis: LockedAxes::ROTATION_LOCKED,
            transform: Transform {
                translation: Vec3::new(x, y, Z_VALUE),
                scale: SCALING,
                ..default()
            },
            global_transform: GlobalTransform::default(),


            // Visibility
            current_animation_frame: CurrentAnimationFrame(1),
            character_texture_handles: enemy_texture_handles.clone(),
            texture: current_texture,
            sprite: Sprite::default(),
            visibility: Default::default(),
            inherited_visibility: InheritedVisibility::default(),
            view_visibility: ViewVisibility::default(),

            // Others
            health: Health(10.),
            damage: Damage(1.),
        }
    }
}
