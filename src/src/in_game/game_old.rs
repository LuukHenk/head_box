
use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
    time::Stopwatch,
};
use super::ScreenState;
use super::despawn_screen;


const COLLISION_PUSHBACK: f32 = 0.2;

const HIDDEN_WALL_COLOR: Color = Color::BLUE;


pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(ScreenState::Game), game_setup)
            .add_systems(OnExit(ScreenState::Game), despawn_screen::<OnGameScreen>)
            .add_systems(FixedUpdate, (
                    handle_game_over,
                    set_current_level.after(handle_game_over),
                    spawn_enemies_for_current_level,
                    set_player_direction,
                    set_enemy_directions.after(spawn_enemies_for_current_level),
                    handle_player_enemy_collision.after(set_player_direction),
                    prevent_enemy_enemy_collision.after(set_enemy_directions),
                    prevent_wall_collision
                        .after(set_player_direction)
                        .after(set_enemy_directions)
                    ,
                    move_objects.after(prevent_wall_collision),
                ).run_if(in_state(ScreenState::Game))
            );
    }
}


#[derive(Component)]
struct OnGameScreen;


#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct Wall;











#[derive(Bundle)]
struct WallBundle {
    wall: Wall,
    sprite_bundle: SpriteBundle,
    collider: Collider,
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

#[derive(Component)]
struct Level {
    id: u32,
    total_enemies: u32,
    spawned_enemies: u32,
    killed_enemies: u32,
    enemies_spawn_delay: f32,
    elapsed_time: Stopwatch,
}
impl Level {
    fn new(id: u32, total_enemies: u32, enemies_spawn_delay: f32) -> Level {
        Level {
            id,
            total_enemies,
            spawned_enemies: 0,
            killed_enemies: 0,
            enemies_spawn_delay,
            elapsed_time: Stopwatch::new(),
        }
    }
}

#[derive(Component)]
struct ActiveLevel;

fn spawn_outer_walls(commands: &mut Commands) {
    // Top
    commands.spawn((WallBundle::new(0., OUTER_Y_COORDINATES, 2000., 80., HIDDEN_WALL_COLOR), OnGameScreen));
    commands.spawn((WallBundle::new(-400., 340., 600., 40., Color::BLACK), OnGameScreen));
    commands.spawn((WallBundle::new(400., 340., 600., 40., Color::BLACK), OnGameScreen));
    // Bottom
    commands.spawn((WallBundle::new(0., -OUTER_Y_COORDINATES, 2000., 80., HIDDEN_WALL_COLOR), OnGameScreen));
    commands.spawn((WallBundle::new(-400., -340., 600., 40., Color::BLACK), OnGameScreen));
    commands.spawn((WallBundle::new(400., -340., 600., 40., Color::BLACK), OnGameScreen));
    // Sides
    commands.spawn((WallBundle::new(-620., 0., 40., 2000.,Color::BLACK), OnGameScreen));
    commands.spawn((WallBundle::new(620., 0., 40., 2000.,Color::BLACK), OnGameScreen));
}


fn game_setup(
    mut commands: Commands,
) {
    spawn_outer_walls(&mut commands);


    let first_level_entity = commands.spawn((Level::new(1, 100, 1.), OnGameScreen)).id();
    // commands.spawn((Level::new(2, 6, 5.), OnGameScreen));
    commands.entity(first_level_entity).insert(ActiveLevel);

}


fn set_current_level(
    mut commands: Commands,
    active_level_query: Query<(Entity, &Level), With<ActiveLevel>>,
    level_query: Query<(Entity, &Level)>,
    mut game_state: ResMut<NextState<ScreenState>>
) {
    let (active_level_entity, active_level) = active_level_query.single();
    if active_level.killed_enemies < active_level.total_enemies { return }

    commands.entity(active_level_entity).remove::<ActiveLevel>();

    let next_level_id = active_level.id + 1;
    for (level_entity, level) in level_query.iter() {
        if level.id != next_level_id { continue }
        commands.entity(level_entity).insert(ActiveLevel);
        return
    }
    game_state.set(ScreenState::MainMenu); // If all levels are done, go back to the main menu
}

fn spawn_enemies_for_current_level(
    time: Res<Time>,
    commands: Commands,
    mut level_query: Query<&mut Level, With<ActiveLevel>>
) {
    let mut level = level_query.single_mut();
    if level.total_enemies > level.spawned_enemies {
        let expected_spawned_enemies = (level.elapsed_time.elapsed_secs() / level.enemies_spawn_delay + 1.).floor();
        if level.spawned_enemies < expected_spawned_enemies as u32 {
            spawn_enemy(commands);
            level.spawned_enemies += 1;
        }
    }
    level.elapsed_time.tick(time.delta());
}

fn handle_game_over(
    query: Query<&Health, With<Player>>,
    mut game_state: ResMut<NextState<ScreenState>>,
) {
    let health = query.single();
    if health.0 <= 0. {
        game_state.set(ScreenState::MainMenu);
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
        Collision::Inside => {
            // println!("Stuck!");
        }
    }
    movement
}
// println!("{:#?}", something);