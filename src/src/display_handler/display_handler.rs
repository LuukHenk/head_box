use bevy::prelude::*;
use super::GamePlugin;
use super::MainMenuPlugin;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum ScreenState {
    #[default]
    Game,
    MainMenu,

}
pub struct HeadBoxDisplayPlugin;
impl Plugin for HeadBoxDisplayPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(FixedTime::new_from_secs(1.0 / 60.0))
            .add_state::<ScreenState>()
            .add_systems(Startup, setup)
            .add_plugins((GamePlugin, MainMenuPlugin));
    }
}

pub fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}