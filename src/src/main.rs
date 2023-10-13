mod game;
mod main_menu;
use bevy::prelude::*;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum GameState {
    #[default]
    MainMenu,
    Game,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(FixedTime::new_from_secs(1.0 / 60.0))
        .add_state::<GameState>()
        .add_systems(Startup, setup)
        .add_plugins((game::GamePlugin, main_menu::MainMenuPlugin))
        .run();
}

fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}