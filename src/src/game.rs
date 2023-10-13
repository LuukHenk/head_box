
use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};
use super::GameState;

const Z_VALUE: f32 = 1.;
const COLLISION_PUSHBACK: f32 = 0.07;
const INITIAL_PLAYER_HEALTH: f32 = 300.;


pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), game_setup)
            .add_systems(FixedUpdate, (
                handle_game_over.run_if(in_state(GameState::Game)),
                set_player_direction.run_if(in_state(GameState::Game)),
                set_enemy_directions.run_if(in_state(GameState::Game)),
                handle_player_enemy_collision
                    .after(set_player_direction)
                    .run_if(in_state(GameState::Game))
                ,
                prevent_enemy_enemy_collision
                    .after(set_enemy_directions)
                    .run_if(in_state(GameState::Game))
                ,
                prevent_wall_collision
                    .after(set_player_direction)
                    .after(set_enemy_directions)
                    .run_if(in_state(GameState::Game))
                ,
                move_objects
                    .after(prevent_wall_collision)
                    .run_if(in_state(GameState::Game))
                ,
            ));
    }
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
struct Health(f32);

#[derive(Component, Debug)]
struct Movement {
    direction_x: f32,
    direction_y: f32,
    velocity: f32,
}

#[derive(Bundle)]
struct PlayerBundle {
    player: Player,
    health: Health,
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
            player: Player,
            collider: Collider,
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
            health: Health(INITIAL_PLAYER_HEALTH),
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

fn game_setup(
    mut commands: Commands,
) {
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

fn handle_game_over(
    query: Query<&Health, With<Player>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    let health = query.single();
    if health.0 <= 0. {
        game_state.set(GameState::MainMenu);
    }
}

fn move_objects(mut query: Query<(&mut Transform, &Movement), With<Movement>>) {
    for (mut transform, movement) in query.iter_mut() {
        transform.translation.x += movement.direction_x * movement.velocity;
        transform.translation.y += movement.direction_y * movement.velocity;
    }
}

fn set_player_direction(keyboard_input: Res<Input<KeyCode>>, mut player_query: Query<&mut Movement, With<Player>>) {
    let mut movement = player_query.single_mut();
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

fn set_enemy_directions(
    mut enemy_query: Query<(&mut Movement, &Transform), With<Enemy>>,
    player_query: Query<&Transform, With<Player>>
) {
    let player_transform = player_query.single();
    let player_position = player_transform.translation;
    for (mut enemy_movement, enemy_transform) in enemy_query.iter_mut() {
        let enemy_position = enemy_transform.translation;
        let distance_x_with_target =  player_position[0] - enemy_position[0];
        let distance_y_with_target =  player_position[1] - enemy_position[1];
        enemy_movement.direction_x = set_enemy_direction(
            distance_x_with_target,
            enemy_movement.velocity
        );
        enemy_movement.direction_y = set_enemy_direction(
            distance_y_with_target,
            enemy_movement.velocity
        );
    }
}

fn set_enemy_direction(target_distance: f32, enemy_velocity: f32) -> f32 {
    if target_distance > enemy_velocity {1.} else if target_distance < -enemy_velocity {-1.} else {0.}
}


fn handle_player_enemy_collision(
    mut player_query: Query<(&Transform, &mut Movement, &mut Health), With<Player>>,
    enemies_query: Query<&Transform, With<Enemy>>,
) {
    let (player_transform, mut player_movement, mut player_health) = player_query.single_mut();
    for enemy_transform in enemies_query.iter() {
        let collision = __check_for_collision(player_transform, enemy_transform);
        if let Some(collision) = collision {
            player_movement = __apply_collision_pushback(collision, player_movement);
            player_health.0 -= 1.;
            println!("Auch! HP: {:#?}/{:#?}", player_health.0, INITIAL_PLAYER_HEALTH)
        }
    }
}
fn prevent_enemy_enemy_collision(
    mut enemies_query_a: Query<(Entity, &Transform, &mut Movement), With<Enemy>>,
    enemies_query_b: Query<(Entity, &Transform), With<Enemy>>,
) {
    for (entity_a, transform_a, mut movement_a) in enemies_query_a.iter_mut() {
        for (entity_b, transform_b) in enemies_query_b.iter() {
            if entity_a == entity_b {continue}
            let collision = __check_for_collision(transform_a, transform_b);
            if let Some(collision) = collision {
                movement_a = __apply_collision_pushback(collision, movement_a);
            }
        }
    }
}
fn prevent_wall_collision(
    mut moving_objects_query: Query<(&Transform, &mut Movement), With<Movement>>,
    wall_query: Query<&Transform, With<Wall>>,
) {
    for (transform_a, mut movement_a) in moving_objects_query.iter_mut() {
        for transform_b in wall_query.iter() {
            let collision = __check_for_collision(transform_a, transform_b);
            if let Some(collision) = collision {
                movement_a = __apply_collision_pushback(collision, movement_a);
            }
        }
    }
}

fn __check_for_collision(transform_a: &Transform, transform_b: &Transform) -> Option<Collision> {
    collide(
        transform_a.translation,
        transform_a.scale.truncate(),
        transform_b.translation,
        transform_b.scale.truncate()
    )
}

fn __apply_collision_pushback(collision: Collision, mut movement: Mut<Movement>) -> Mut<Movement> {
    match collision {
        Collision::Left => movement.direction_x = -COLLISION_PUSHBACK,
        Collision::Right => movement.direction_x = COLLISION_PUSHBACK,
        Collision::Top => movement.direction_y = COLLISION_PUSHBACK,
        Collision::Bottom => movement.direction_y = -COLLISION_PUSHBACK,
        Collision::Inside => { println!("Stuck!"); }
    }
    movement
}
// println!("{:#?}", something);