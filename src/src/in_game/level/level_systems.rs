
use bevy::prelude::{
    Commands,
    Query,
    Entity,
    With,
    ResMut,
    NextState,
    Res,
    Time,
};
use super::PlayerMarker;
use super::Health;
use super::level_components::{
    LevelMarker,
    TotalEnemies,
    SpawnedEnemies,
    KilledEnemies,
    EnemySpawnDelay,
    LevelTimer,
    LevelId,
    ActiveLevelMarker,
};
use super::level_bundle::Level;
use super::data_classes::generic_components::GameScreenMarker;
use super::ScreenState;
use super::EnemySystems;
pub struct LevelSystems;

impl LevelSystems {
    pub fn spawn_levels(mut commands: Commands) {
        let level_1 = Level::new(1, 1, 1.);
        let level_2 = Level::new(2, 2, 2.);
        commands.spawn((level_1, GameScreenMarker, ActiveLevelMarker));
        commands.spawn((level_2, GameScreenMarker));
    }

    pub fn set_current_level(
        mut commands: Commands,
        active_level_query: Query<(Entity, &KilledEnemies, &TotalEnemies, &LevelId), With<ActiveLevelMarker>>,
        level_query: Query<(Entity, &LevelId), With<LevelMarker>>,
        mut game_state: ResMut<NextState<ScreenState>>
    ) {

        let (active_level_entity, killed_enemies, total_enemies, current_level_id) = active_level_query.single();
        if killed_enemies.0 < total_enemies.0 { return }
        commands.entity(active_level_entity).remove::<ActiveLevelMarker>();
        let next_level_id = current_level_id.0 + 1;
        for (level_entity, level_id) in level_query.iter() {
            if level_id.0 != next_level_id { continue }
            commands.entity(level_entity).insert(ActiveLevelMarker);
            return
        }
        game_state.set(ScreenState::MainMenu); // If all level are done, go back to the main menu
    }

    pub fn spawn_enemies_for_current_level(
        time: Res<Time>,
        commands: Commands,
        mut level_query: Query<(&mut LevelTimer, &EnemySpawnDelay, &mut SpawnedEnemies, &TotalEnemies), With<ActiveLevelMarker>>
    ) {
        let (mut level_timer, enemy_spawn_delay, mut spawned_enemies, total_enemies) = level_query.single_mut();

        let expected_spawned_enemies = (level_timer.0.elapsed_secs() / enemy_spawn_delay.0 + 1.).floor();


        if spawned_enemies.0 < expected_spawned_enemies as u32 {
            if spawned_enemies.0 < total_enemies.0 {
                EnemySystems::spawn_dummy(commands);
                spawned_enemies.0 += 1;
            }
        }
        level_timer.0.tick(time.delta());
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
}

