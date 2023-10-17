
use std::time::Duration;
use bevy::prelude::*;
use crate::in_game::game_components::GameScreenMarker;
use crate::in_game::game_constants::Z_VALUE;


#[derive(Component)]
struct BulletMarker;

#[derive(Component)]
struct Damage(f32);

#[derive(Component)]
struct MaxLifeTime(Timer);

#[derive(Bundle)]
pub struct Bullet {
    damage: Damage,
    bullet_marker: BulletMarker,
    max_lifetime: MaxLifeTime,
    sprite_bundle: SpriteBundle,
}

impl Bullet {
    pub fn spawn(&mut self, x: f32, y: f32, mut commands: &Commands) {
        self.sprite_bundle.transform.translation = Vec3::new(x, y, Z_VALUE);
        commands.spawn((self, GameScreenMarker));
    }
}


pub trait PistolBullet{
    fn new() -> Bullet;
}

impl PistolBullet for Bullet {
    fn new() -> Bullet {
        Bullet {
            max_lifetime: MaxLifeTime(Timer::new(Duration::from_secs(1), TimerMode::Once)),
            damage: Damage(0.5),
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(0., 0., Z_VALUE),
                    scale: Vec3::new(20.0, 20.0, Z_VALUE),
                    ..default()
                },
                sprite: Sprite { color: Color::BLUE, ..default() },
                ..default()
            },
            bullet_marker: BulletMarker,
        }
    }
}