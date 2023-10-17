
use std::time::Duration;
use bevy::prelude::*;
use super::game_components::*;
use super::game_constants::*;
const INITIAL_PLAYER_HEALTH: f32 = 300.;
const PLAYER_SIZE: f32 = 20.;


#[derive(Bundle)]
pub struct PlayerBundle {
    player_marker: PlayerMarker,
    health: Health,
    sprite_bundle: SpriteBundle,
    movement: Movement,
    collision_marker: CollisionMarker,
}


impl PlayerBundle {
    fn new(asset_server: Res<AssetServer>) -> PlayerBundle {
        let ship_handle = asset_server.load("textures/image10.png");
        PlayerBundle {
            player_marker: PlayerMarker,
            collision_marker: CollisionMarker,
            sprite_bundle: SpriteBundle {
                texture: ship_handle,
                transform: Transform {
                    translation: Vec3::new(0., 0., Z_VALUE),
                    // scale: Vec3::new(20.0, 20.0, Z_VALUE),
                    ..default()
                },
                sprite: Sprite { color: Color::PURPLE, ..default() },
                ..default()
            },
            movement: Movement {
                direction_x: 0.,
                direction_y: 0.,
                velocity: 6.,
            },
            health: Health(INITIAL_PLAYER_HEALTH),
        }
    }

    pub fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
        commands.spawn((PlayerBundle::new(asset_server), GameScreenMarker));
    }

    pub fn set_direction(keyboard_input: Res<Input<KeyCode>>, mut player_query: Query<&mut Movement, With<PlayerMarker>>) {
        for mut movement in player_query.iter_mut() {
            movement.direction_x = 0.;
            movement.direction_y = 0.;
            if keyboard_input.pressed(KeyCode::Right) {
                movement.direction_x += 1.;
            }
            if keyboard_input.pressed(KeyCode::Left) {
                movement.direction_x -= 1.;
            }
            if keyboard_input.pressed(KeyCode::Up) {
                movement.direction_y += 1.;
            }
            if keyboard_input.pressed(KeyCode::Down) {
                movement.direction_y -= 1.;
            }
        }
    }

    pub fn shoot(
        keyboard_input: Res<Input<KeyCode>>,
        mut player_query: Query<(&Movement, &mut Transform), With<PlayerMarker>>,
        mut commands: Commands,
    ) {
        if keyboard_input.pressed(KeyCode::Space) {
            for (movement, mut transform) in player_query.iter_mut() {
                let bullet = Bullet::new( &mut transform);
                commands.spawn((bullet, GameScreenMarker));
            }
        }
    }
}





// Bullet

#[derive(Component)]
struct BulletMarker;

#[derive(Component)]
struct Damage(f32);

#[derive(Component)]
struct LifeTime(Timer);

#[derive(Bundle)]
pub struct Bullet {
    bullet_marker: BulletMarker,
    damage: Damage,
    life_time: LifeTime,
    sprite_bundle: SpriteBundle,
}

impl Bullet {
    fn new(mut player_transform: &mut Transform) -> Bullet {
        let bullet_length = 100.;
        let front = (player_transform.rotation * Vec3::Y).truncate().normalize();

        let mut transform = Transform {
            translation: Vec3::new(
                player_transform.translation.x + (PLAYER_SIZE/2. + bullet_length / 2.)*front[0],
                player_transform.translation.y + (PLAYER_SIZE/2. + bullet_length / 2.)*front[1],
                player_transform.translation.z
            ),
            scale: Vec3::new(1., bullet_length, Z_VALUE),
            rotation: player_transform.rotation,
        };

        Bullet {
            sprite_bundle: SpriteBundle {
                transform,
                sprite: Sprite { color: Color::BLUE, ..default() },
                ..default()
            },
            damage: Damage(0.5),
            life_time: LifeTime(Timer::new(Duration::from_secs(1), TimerMode::Once)),
            bullet_marker: BulletMarker,
        }
    }
}