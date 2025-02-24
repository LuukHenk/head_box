use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::components::arena_components::EnemySpawnLocation;
use crate::components::generic_components::GameScreenMarker;
use crate::utils::generic_constants::{SCALING, Z_VALUE};
use crate::utils::physics_constants::{DEFAULT_ACTIVE_EVENTS, DEFAULT_GRAVITY, WALL_COLLISION_GROUPS};

#[derive(Bundle)]
struct ArenaBundle {
    game_screen_marker: GameScreenMarker,
    texture: Handle<Image>,
    sprite: Sprite,
    visibility: Visibility,
    inherited_visibility: InheritedVisibility,
    view_visibility: ViewVisibility,
    transform: Transform,
    global_transform: GlobalTransform,
}

#[derive(Bundle)]
struct OuterWallBundle {
    game_screen_marker: GameScreenMarker,
    transform: Transform,
    global_transform: GlobalTransform,
    collision_groups: CollisionGroups,
    active_events: ActiveEvents,
    rigid_body: RigidBody,
    gravity: GravityScale,
    collider: Collider,
    continuous_collision_detection: Ccd,
}
pub struct ArenaSystems;

impl ArenaSystems {
    pub fn spawn_arena(mut commands: Commands, asset_server: Res<AssetServer>) {
        let arena_bundle = ArenaBundle {
            texture: asset_server.load("textures/arena/arena.png"),
            transform: Transform {
                ..default() },
            sprite: Sprite::default(),
            visibility: Default::default(),
            inherited_visibility: InheritedVisibility::default(),
            view_visibility: ViewVisibility::default(),
            game_screen_marker: GameScreenMarker,
            global_transform: GlobalTransform::default(),
        };
        commands.spawn(arena_bundle);
        Self::spawn_outer_walls(commands);
    }

    pub fn set_enemy_spawn_locations(mut commands: Commands) {
        let spawn_locations: Vec<Vec3> = vec![
            Vec3::new(-480., -323., Z_VALUE),
            Vec3::new(-435., -348., Z_VALUE),
            Vec3::new(-385., -333., Z_VALUE),
            Vec3::new(127., 154., Z_VALUE),
            Vec3::new(152., 299., Z_VALUE),
            Vec3::new(157., 146., Z_VALUE),
        ];
        for spawn_location in spawn_locations.iter() {
            commands.spawn(EnemySpawnLocation(*spawn_location));
        }
    }

    fn spawn_outer_walls(mut commands: Commands) {
        commands.spawn(OuterWallBundle{
            game_screen_marker: GameScreenMarker,
            rigid_body: RigidBody::Fixed,
            collider: Collider::cuboid(10., 1000.),
            transform: Transform{
                translation: Vec3::new(210., 0., Z_VALUE),
                scale: SCALING,
                ..default()
            },
            global_transform: GlobalTransform::default(),
            collision_groups: WALL_COLLISION_GROUPS,
            active_events: DEFAULT_ACTIVE_EVENTS,
            gravity: DEFAULT_GRAVITY,
            continuous_collision_detection: Ccd::disabled(),
        });
        commands.spawn(OuterWallBundle{
            game_screen_marker: GameScreenMarker,
            rigid_body: RigidBody::Fixed,
            collider: Collider::cuboid(10., 1000.),
            transform: Transform{
                translation: Vec3::new(-1935., 0., Z_VALUE),
                scale: SCALING,
                ..default()
            },
            global_transform: GlobalTransform::default(),
            collision_groups: WALL_COLLISION_GROUPS,
            active_events: DEFAULT_ACTIVE_EVENTS,
            gravity: DEFAULT_GRAVITY,
            continuous_collision_detection: Ccd::disabled(),
        });
        commands.spawn(OuterWallBundle{
            game_screen_marker: GameScreenMarker,
            rigid_body: RigidBody::Fixed,
            collider: Collider::cuboid(1000., 10.),
            transform: Transform{
                translation: Vec3::new(0., 1095., Z_VALUE),
                scale: SCALING,
                ..default()
            },
            global_transform: GlobalTransform::default(),
            collision_groups: WALL_COLLISION_GROUPS,
            active_events: DEFAULT_ACTIVE_EVENTS,
            gravity: DEFAULT_GRAVITY,
            continuous_collision_detection: Ccd::disabled(),
        });
        commands.spawn(OuterWallBundle{
            game_screen_marker: GameScreenMarker,
            rigid_body: RigidBody::Fixed,
            collider: Collider::cuboid(1000., 10.),
            transform: Transform{
                translation: Vec3::new(0., -408., Z_VALUE),
                scale: SCALING,
                ..default()
            },
            global_transform: GlobalTransform::default(),
            collision_groups: WALL_COLLISION_GROUPS,
            active_events: DEFAULT_ACTIVE_EVENTS,
            gravity: DEFAULT_GRAVITY,
            continuous_collision_detection: Ccd::disabled(),
        });
    }
}
