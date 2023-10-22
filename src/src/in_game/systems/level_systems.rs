
use bevy::prelude::*;

use crate::display_handler::display_handler::ScreenState;

use crate::in_game::data_classes::level_components::{
    LevelMarker,
    TotalEnemies,
    SpawnedEnemies,
    KilledEnemies,
    EnemySpawnDelay,
    LevelTimer,
    LevelId,
    ActiveLevelMarker,
};
use crate::in_game::data_classes::player_components::PlayerMarker;
use crate::in_game::data_classes::generic_components::Health;

use crate::in_game::data_layers::level_bundle::Level;

use crate::in_game::systems::enemy_systems::EnemySystems;


pub struct LevelSystems;

impl LevelSystems {
    pub fn spawn_levels(mut commands: Commands) {
        commands.spawn((Level::level_1(), ActiveLevelMarker));
        commands.spawn(Level::level_2());
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
        asset_server: Res<AssetServer>,
        mut level_query: Query<(&mut LevelTimer, &EnemySpawnDelay, &mut SpawnedEnemies, &TotalEnemies), With<ActiveLevelMarker>>
    ) {
        let (mut level_timer, enemy_spawn_delay, mut spawned_enemies, total_enemies) = level_query.single_mut();

        let expected_spawned_enemies = (level_timer.0.elapsed_secs() / enemy_spawn_delay.0 + 1.).floor();


        if spawned_enemies.0 < expected_spawned_enemies as u32 {
            if spawned_enemies.0 < total_enemies.0 {
                EnemySystems::spawn_zombie(commands, asset_server);
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

