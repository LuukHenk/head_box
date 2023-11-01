use bevy::prelude::*;
use crate::components::arena_components::EnemySpawnLocation;

use crate::components::generic_components::GameScreenMarker;
use crate::utils::generic_constants::Z_VALUE;

#[derive(Bundle)]
struct ArenaBundle {
    game_screen_marker: GameScreenMarker,
    texture: Handle<Image>,
    sprite: Sprite,
    visibility: Visibility,
    computed_visibility: ComputedVisibility,
    transform: Transform,
    global_transform: GlobalTransform,
}

pub struct ArenaSystems;

impl ArenaSystems {
    pub fn spawn_arena(mut commands: Commands, asset_server: Res<AssetServer>) {
        let arena_bundle = ArenaBundle {
            texture: asset_server.load("textures/arena/arena.png"),
            transform: Transform { ..default() },
            sprite: Sprite::default(),
            visibility: Default::default(),
            computed_visibility: Default::default(),
            game_screen_marker: GameScreenMarker,
            global_transform: GlobalTransform::default(),
        };
        commands.spawn(arena_bundle);
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
}
