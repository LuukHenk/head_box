


use bevy::prelude::*;
use bevy_rapier2d::prelude::*;




#[derive(Bundle)]
struct RigidBodyBundle {
    rigid_body: RigidBody,
    sprite_bundle: SpriteBundle,
    velocity: Velocity,
    gravity: GravityScale,
    collider: Collider,
    continuous_collision_detection: Ccd,
    sleeping: Sleeping,
    // collision_groups: CollisionGroups,
}


pub struct PlayerSystems;
impl PlayerSystems {
    pub fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
        let player = RigidBodyBundle {
            rigid_body: RigidBody::Dynamic,
            velocity: Velocity {
                linvel: Vec2::new(0.0, 0.0),
                angvel: 0.0,
            },
            gravity: GravityScale(0.0),
            collider: Collider::cuboid(10., 10.),
            continuous_collision_detection: Ccd::enabled(),
            sprite_bundle: SpriteBundle {
                texture: asset_server.load("textures/image11.png"),
                transform: Transform {
                    translation: Vec3::new(-100., 0., -1.),
                    ..default()
                },
                ..default()
            },
            sleeping: Sleeping::disabled(),
            // collision_groups: CollisionGroups::new(0b0001.into(), 0b0001.into())
        };
        commands.spawn(player);
    }
}


pub struct WallSystems;

impl WallSystems {
    pub fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
        let player = RigidBodyBundle {
            rigid_body: RigidBody::Fixed,
            velocity: Velocity {
                linvel: Vec2::new(0.0, 0.0),
                angvel: 0.0,
            },
            gravity: GravityScale(0.0),
            collider: Collider::cuboid(10., 10.),
            continuous_collision_detection: Ccd::enabled(),
            sprite_bundle: SpriteBundle {
                texture: asset_server.load("textures/image11.png"),
                transform: Transform {
                    translation: Vec3::new(-100., -100., -1.),
                    ..default()
                },
                ..default()
            },
            sleeping: Sleeping::default(),
        };
        commands.spawn(player);
    }
}
pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

}
pub fn set_velocity(
    keyboard_input: Res<Input<KeyCode>>,
    mut velocity_query: Query<&mut Velocity, With<RigidBody>>
) {
    let velocity_speed = 200.;
    for mut velocity in velocity_query.iter_mut() {

        velocity.linvel = Vec2::new(0., 0.);
        if keyboard_input.pressed(KeyCode::Right) {
            velocity.linvel[0] += velocity_speed;
        }
        if keyboard_input.pressed(KeyCode::Left) {
            velocity.linvel[0] -= velocity_speed;
        }
        if keyboard_input.pressed(KeyCode::Up) {
            velocity.linvel[1] += velocity_speed;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            velocity.linvel[1] -= velocity_speed;
        }
    }
}


pub fn rotate(mut query: Query<(&Velocity, &mut Transform), With<RigidBody>>) {
    for (velocity, mut transform) in query.iter_mut() {
        if velocity.linvel[0] < 0. && velocity.linvel[1] == 0. {
            transform.rotation = Quat::from_rotation_z(90.0_f32.to_radians())

        } else if velocity.linvel[0] < 0. && velocity.linvel[1] > 0. {
            transform.rotation = Quat::from_rotation_z(45.0_f32.to_radians())

        } else if velocity.linvel[0] == 0. && velocity.linvel[1] > 0. {
            transform.rotation = Quat::from_rotation_z(0.0_f32.to_radians())

        } else if velocity.linvel[0] > 0. && velocity.linvel[1] > 0. {
            transform.rotation = Quat::from_rotation_z(315.0_f32.to_radians())

        } else if velocity.linvel[0] > 0. && velocity.linvel[1] == 0. {
            transform.rotation = Quat::from_rotation_z(270.0_f32.to_radians())

        } else if velocity.linvel[0] > 0. && velocity.linvel[1] < 0. {
            transform.rotation = Quat::from_rotation_z(225.0_f32.to_radians())

        } else if velocity.linvel[0] == 0. && velocity.linvel[1] < 0. {
            transform.rotation = Quat::from_rotation_z(180.0_f32.to_radians())

        } else if velocity.linvel[0] < 0. && velocity.linvel[1] < 0. {
            transform.rotation = Quat::from_rotation_z(135.0_f32.to_radians())
        };
    }
}