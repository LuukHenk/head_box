use bevy::prelude::*;

use crate::{despawn_entities, GameState};

pub struct PlayerPlugin;

#[derive(Resource)]
struct TestGameOverTimer(Timer);

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TestGameOverTimer(Timer::from_seconds(
            5.0,
            TimerMode::Repeating,
        )));
        app.add_systems(OnEnter(GameState::Game), setup);
        app.add_systems(OnExit(GameState::Game), despawn_entities::<Player>);
        app.add_systems(Update, (test_game_over).run_if(in_state(GameState::Game)));
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

fn test_game_over(
    time: Res<Time>,
    mut timer: ResMut<TestGameOverTimer>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        game_state.set(GameState::Menu);
    }
}
