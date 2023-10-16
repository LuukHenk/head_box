use super::game;
use super::main_menu;
use bevy::prelude::*;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub(crate) enum ScreenState {
    #[default]
    Game,
    MainMenu,

}

pub fn main_game_script() {
    App::new()
        .add_plugins(DefaultPlugins)

}

pub(crate) fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}