
use bevy::prelude::{Bundle};
use bevy::time::Stopwatch;
use crate::in_game::data_classes::generic_components::GameScreenMarker;
use super::level_components::{
    LevelMarker,
    TotalEnemies,
    SpawnedEnemies,
    KilledEnemies,
    EnemySpawnDelay,
    LevelTimer,
    LevelId,
};

#[derive(Bundle)]
pub struct Level {
    id: LevelId,
    total_enemies: TotalEnemies,
    spawned_enemies: SpawnedEnemies,
    killed_enemies: KilledEnemies,
    enemy_spawn_delay: EnemySpawnDelay,
    level_timer: LevelTimer,
    level_marker: LevelMarker,
    game_screen_marker: GameScreenMarker,
}

impl Level {
    pub fn new(id: u32, total_enemies: u32, enemy_spawn_delay: f32) -> Level {
        Level {
            id: LevelId(id),
            total_enemies: TotalEnemies(total_enemies),
            spawned_enemies: SpawnedEnemies(0),
            killed_enemies: KilledEnemies(0),
            enemy_spawn_delay: EnemySpawnDelay(enemy_spawn_delay),
            level_timer: LevelTimer(Stopwatch::new()),
            level_marker: LevelMarker,
            game_screen_marker: GameScreenMarker,
        }
    }
}