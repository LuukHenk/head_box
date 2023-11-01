
use bevy::prelude::Component;
use bevy::time::Stopwatch;

#[derive(Component)]
pub struct LevelId(pub u32);

#[derive(Component)]
pub struct TotalEnemies(pub u32);

#[derive(Component)]
pub struct SpawnedEnemies(pub u32);

#[derive(Component)]
pub struct KilledEnemies(pub u32);

#[derive(Component)]
pub struct EnemySpawnDelay(pub f32);

#[derive(Component)]
pub struct LevelTimer(pub Stopwatch);

#[derive(Component)]
pub struct LevelMarker;

#[derive(Component)]
pub struct ActiveLevelMarker;


