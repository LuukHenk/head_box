
use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};

const Z_VALUE: f32 = 1.;
const COLLISION_PUSHBACK: f32 = 0.07;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(FixedTime::new_from_secs(1.0 / 60.0))
        .add_systems(Startup, setup)
        // .add_systems(FixedUpdate, (
        //
        // ))
        .run();
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct Wall;

#[derive(Component)]
struct Collider;

#[derive(Component, Debug)]
struct Movement {
    direction_x: f32,
    direction_y: f32,
    velocity: f32,
}

#[derive(Bundle)]
struct PlayerBundle {
    player: Player,
    sprite_bundle: SpriteBundle,
    movement: Movement,
    collider: Collider,
}

#[derive(Bundle)]
struct EnemyBundle {
    enemy: Enemy,
    sprite_bundle: SpriteBundle,
    movement: Movement,
    collider: Collider
}

#[derive(Bundle)]
struct WallBundle {
    wall: Wall,
    sprite_bundle: SpriteBundle,
    collider: Collider,
}

impl PlayerBundle {
    fn new() -> PlayerBundle {
        PlayerBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(0., 0., Z_VALUE),
                    scale: Vec3::new(20.0, 20.0, Z_VALUE),
                    ..default()
                },
                sprite: Sprite { color: Color::PURPLE, ..default() },
                ..default()
            },
            movement: Movement {
                direction_x: 0.,
                direction_y: 0.,
                velocity: 7.,
            },
            player: Player,
            collider: Collider,
        }
    }
}

impl EnemyBundle {
    fn new(x: f32, y: f32) -> EnemyBundle {
        EnemyBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(x, y, Z_VALUE),
                    scale: Vec3::new(20.0, 20.0, Z_VALUE),
                    ..default()
                },
                sprite: Sprite { color: Color::LIME_GREEN, ..default() },
                ..default()
            },
            movement: Movement {
                direction_x: 0.,
                direction_y: 0.,
                velocity: 2.,
            },
            enemy: Enemy,
            collider: Collider,
        }
    }
}

impl WallBundle {
    fn new(x: f32, y: f32, width: f32, height: f32, color: Color) -> WallBundle {
        WallBundle {
            wall: Wall,
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(x, y, Z_VALUE),
                    scale: Vec3::new(width, height, Z_VALUE),
                    ..default()
                },
                sprite: Sprite { color, ..default() },
                ..default()
            },
            collider: Collider
        }
    }
}

fn setup(
    mut commands: Commands,
) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(PlayerBundle::new());
    commands.spawn(EnemyBundle::new(-50., 320.));
    commands.spawn(EnemyBundle::new(50., 320.));
    commands.spawn(EnemyBundle::new(-50., -320.));
    commands.spawn(EnemyBundle::new(50., -320.));
    commands.spawn(WallBundle::new(0., 340., 2000., 40., Color::BLACK));
    commands.spawn(WallBundle::new(0., -340., 2000., 40.,Color::BLACK));
    commands.spawn(WallBundle::new(-620., 0., 40., 2000.,Color::BLACK));
    commands.spawn(WallBundle::new(620., 0., 40., 2000.,Color::BLACK));
    commands.spawn(WallBundle::new(0., 325., 200., 10., Color::DARK_GRAY));
    commands.spawn(WallBundle::new(0., -325., 200., 10., Color::DARK_GRAY));
}