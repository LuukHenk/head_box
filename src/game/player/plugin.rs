use bevy::prelude::*;

use crate::GameState;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), setup);
        // app.add_systems(Update, ().run_if(in_state(GameState::Game)));
    }
}

#[derive(Component)]
struct Player;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Player,
        Mesh2d(meshes.add(Circle::new(25.))),
        MeshMaterial2d(materials.add(Color::srgb(1.0, 1.0, 1.0))),
        Transform::from_xyz(0., 0., 2.),
    ));
}
