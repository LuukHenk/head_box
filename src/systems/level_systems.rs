
use bevy::prelude::*;
use bevy::time::Stopwatch;

use crate::states::screen_state::ScreenState;
use crate::events::enemy_spawn_events::SpawnZombieEvent;

use crate::components::generic_components::{GameScreenMarker, Health};
use crate::components::level_components::{
    ActiveLevelMarker, EnemySpawnDelay, KilledEnemies, LevelId, LevelMarker, LevelTimer,
    SpawnedEnemies, TotalEnemies,
};
use crate::components::player_components::PlayerMarker;

#[derive(Bundle)]
pub struct LevelBundle {
    id: LevelId,
    total_enemies: TotalEnemies,
    spawned_enemies: SpawnedEnemies,
    killed_enemies: KilledEnemies,
    enemy_spawn_delay: EnemySpawnDelay,
    level_timer: LevelTimer,
    level_marker: LevelMarker,
    game_screen_marker: GameScreenMarker,
}

pub struct LevelSystems;

impl LevelSystems {
    pub fn spawn_levels(mut commands: Commands) {
        let level_1 = Self::new_level(1, 10, 0.1);
        let level_2 = Self::new_level(2, 15, 0.8);
        let level_3 = Self::new_level(3, 10000, 0.01);
        commands.spawn((level_1, ActiveLevelMarker));
        commands.spawn(level_2);
        commands.spawn(level_3);
    }

    pub fn set_current_level(
        mut commands: Commands,
        active_level_query: Query<
            (Entity, &KilledEnemies, &TotalEnemies, &LevelId),
            With<ActiveLevelMarker>,
        >,
        level_query: Query<(Entity, &LevelId), With<LevelMarker>>,
        mut game_state: ResMut<NextState<ScreenState>>,
    ) {
        let (active_level_entity, killed_enemies, total_enemies, current_level_id) =
            active_level_query.single();
        if killed_enemies.0 < total_enemies.0 {
            return;
        }
        commands
            .entity(active_level_entity)
            .remove::<ActiveLevelMarker>();
        let next_level_id = current_level_id.0 + 1;
        for (level_entity, level_id) in level_query.iter() {
            if level_id.0 != next_level_id {
                continue;
            }
            commands.entity(level_entity).insert(ActiveLevelMarker);
            return;
        }
        game_state.set(ScreenState::MainMenu); // If all level are done, go back to the main menu
    }

    pub fn spawn_enemies_for_current_level(
        time: Res<Time>,
        mut level_query: Query<
            (
                &mut LevelTimer,
                &EnemySpawnDelay,
                &mut SpawnedEnemies,
                &TotalEnemies,
            ),
            With<ActiveLevelMarker>,
        >,
        mut spawn_zombie_event: EventWriter<SpawnZombieEvent>,
    ) {
        let (mut level_timer, enemy_spawn_delay, mut spawned_enemies, total_enemies) =
            level_query.single_mut();

        let expected_spawned_enemies =
            (level_timer.0.elapsed_secs() / enemy_spawn_delay.0 + 1.).floor();

        if spawned_enemies.0 < expected_spawned_enemies as u32 {
            if spawned_enemies.0 < total_enemies.0 {
                spawn_zombie_event.send(SpawnZombieEvent);
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

    fn new_level(id: u32, total_enemies: u32, enemy_spawn_delay: f32) -> LevelBundle {
        LevelBundle {
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
