


use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::generic_constants::{SCALING, SCREEN_CENTER};
use crate::assets::asset_components::ZombieTexture;

use crate::events::enemy_spawn_events::SpawnZombieEvent;
use crate::in_game::data_classes::level_components::{ActiveLevelMarker, KilledEnemies};
use crate::in_game::data_classes::rigid_body_components::WalkingVelocity;
use crate::in_game::data_classes::player_components::PlayerMarker;
use crate::in_game::data_classes::enemy_components::EnemyMarker;
use crate::in_game::data_classes::player_constants::PLAYER_SIZE;



use crate::in_game::data_classes::bullet_components::Damage;
use crate::in_game::data_classes::generic_components::{GameScreenMarker, Health};
use crate::in_game::data_classes::rigid_body_constants::{DEFAULT_COLLISION_GROUPS, DEFAULT_GRAVITY, DEFAULT_VELOCITY};
use crate::generic_constants::Z_VALUE;

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
}

pub struct EnemySystems;

impl EnemySystems {


    pub fn spawn_zombies(
        mut spawn_zombie_event: EventReader<SpawnZombieEvent>,
        mut commands: Commands,
        zombie_texture_query: Query<&ZombieTexture>,
    ) {
        let texture = zombie_texture_query.single();
        for _ in spawn_zombie_event.iter() {
            let y = if rand::random::<bool>() { 1. } else { -1. };
            let x = if rand::random::<bool>() { 1. } else { -1. };
            let zombie = Self::new_enemy(SCREEN_CENTER * x, 350. * y, texture.0.clone());
            commands.spawn(zombie);
        }
    }


    pub fn despawn_enemies(
        mut commands: Commands,
        query: Query<(Entity, &Health), With<EnemyMarker>>,
        mut active_level_query: Query<&mut KilledEnemies, With<ActiveLevelMarker>>
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
        player_query: Query<&Transform, With<PlayerMarker>>
    ) {
        for player_transform in player_query.iter() {
            let player_position = player_transform.translation;
            for (mut enemy_velocity, enemy_transform, walking_velocity) in enemy_query.iter_mut() {
                let enemy_position = enemy_transform.translation;
                enemy_velocity.linvel[0] = Self::set_direction_to_target(
                    walking_velocity.0,
                    enemy_position[0],
                    player_position[0],
                );
                enemy_velocity.linvel[1] = Self::set_direction_to_target(
                    walking_velocity.0,
                    enemy_position[1],
                    player_position[1],
                );
            }
        }
    }

    fn set_direction_to_target(velocity: f32, position: f32, target_position: f32) -> f32 {
        let target_distance =  target_position - position;
        if target_distance > PLAYER_SIZE*2. {velocity} else if target_distance < - PLAYER_SIZE*2. {-velocity} else {0.}
    }

    fn new_enemy(x: f32, y: f32, texture: Handle<Image>) -> EnemyBundle {
        EnemyBundle{
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