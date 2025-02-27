use crate::game::PlayerPlugin;
use bevy::prelude::*;

use crate::{despawn_entities, GameState};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), setup);
        app.add_systems(OnEnter(GameState::Game), setup_test_world); // Replace with plugin
        app.add_systems(OnExit(GameState::Game), despawn_entities::<OnGameScreen>);
        app.add_plugins(PlayerPlugin);
    }
}

#[derive(Component)]
struct OnGameScreen;

fn setup(mut commands: Commands) {
    commands.spawn(OnGameScreen).insert(create_camera());
}
fn create_camera() -> (Camera2d, Camera) {
    (Camera2d, Camera { ..default() })
}

fn setup_test_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        OnGameScreen,
        Mesh2d(meshes.add(Rectangle::new(1000., 700.))),
        MeshMaterial2d(materials.add(Color::srgb(0.2, 0.2, 0.3))),
    ));
}
