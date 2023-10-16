use bevy::{
    prelude::*,
    time::Stopwatch,
};
use super::ZombieBundle;
#[derive(Component)]
pub struct Level {
    id: u32,
    total_enemies: u32,
    spawned_enemies: u32,
    killed_enemies: u32,
    enemies_spawn_delay: f32,
    elapsed_time: Stopwatch,
}
impl Level {
    pub fn new(id: u32, total_enemies: u32, enemies_spawn_delay: f32) -> Level {
        Level {
            id,
            total_enemies,
            spawned_enemies: 0,
            killed_enemies: 0,
            enemies_spawn_delay,
            elapsed_time: Stopwatch::new(),
        }
    }

    pub fn spawn_enemy(&mut self, commands: Commands) {
        if !self.all_enemies_spawned() {
            ZombieBundle::spawn(commands);
            self.spawned_enemies += 1;
        }
    }

    pub fn spawn_timed_enemy(&mut self, time: Res<Time>, commands: Commands) {
        if self.enemy_ready_to_spawn() {
            self.spawn_enemy(commands)
        }
        self.elapsed_time.tick(time.delta());
    }
    pub fn level_over(&self) -> bool {
        self.killed_enemies < self.total_enemies
    }

    pub fn all_enemies_spawned(&self) -> bool {
        self.spawned_enemies >= self.total_enemies
    }

    pub fn enemy_ready_to_spawn(&self) -> bool {
        let expected_spawned_enemies = (self.elapsed_time.elapsed_secs() / self.enemies_spawn_delay + 1.).floor();
        self.spawned_enemies < expected_spawned_enemies as u32
    }
    pub fn get_id(&self) ->  u32 {
        self.id
    }
}
