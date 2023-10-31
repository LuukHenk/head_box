use bevy::prelude::*;


#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum ScreenState {
    #[default]
    Game,
    MainMenu,
}


pub fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}

#[derive(Component)]
pub struct InGameCamera;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), InGameCamera));
}