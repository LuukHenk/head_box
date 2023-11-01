use crate::components::generic_components::GameScreenMarker;
use bevy::prelude::*;

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
}
