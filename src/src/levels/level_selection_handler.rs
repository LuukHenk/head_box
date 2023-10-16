
use bevy::prelude::*;
use super::game_components::*;
use super::level::Level;
use super::ScreenState;




#[derive(Component)]
pub struct ActiveLevel;


pub fn spawn_levels(mut commands: Commands) {
    let first_level_entity = commands.spawn((Level::new(1, 10, 1.), GameScreenMarker)).id();
    commands.spawn((Level::new(2, 6, 5.), GameScreenMarker));
    commands.entity(first_level_entity).insert(ActiveLevel);
}
pub fn set_current_level(
    mut commands: Commands,
    active_level_query: Query<(Entity, &Level), With<ActiveLevel>>,
    level_query: Query<(Entity, &Level)>,
    mut game_state: ResMut<NextState<ScreenState>>
) {
    let (active_level_entity, active_level) = active_level_query.single();
    if active_level.level_over() { return }

    commands.entity(active_level_entity).remove::<ActiveLevel>();

    let next_level_id = active_level.get_id() + 1;
    for (level_entity, level) in level_query.iter() {
        if level.get_id() != next_level_id { continue }
        commands.entity(level_entity).insert(ActiveLevel);
        return
    }
    game_state.set(ScreenState::MainMenu); // If all levels are done, go back to the main menu
}

pub fn spawn_enemies_for_current_level(
    time: Res<Time>,
    commands: Commands,
    mut level_query: Query<&mut Level, With<ActiveLevel>>
) {
    let mut level = level_query.single_mut();
    level.spawn_timed_enemy(time, commands)
}

pub fn handle_game_over(
    query: Query<&Health, With<PlayerMarker>>,
    mut game_state: ResMut<NextState<ScreenState>>,
) {
    for health in query.iter() {
        if health.0 <= 0. {
            game_state.set(ScreenState::MainMenu);
        }
    }

}