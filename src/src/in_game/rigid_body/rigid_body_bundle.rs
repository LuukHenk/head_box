use bevy::prelude::{
    Bundle,
    SpriteBundle,
};
use bevy_rapier2d::prelude::{RigidBody, Velocity, GravityScale, Collider, Ccd, Sleeping, CollisionGroups};




#[derive(Bundle)]
pub struct RigidBodyBundle {
    pub rigid_body: RigidBody,
    pub sprite_bundle: SpriteBundle,
    pub velocity: Velocity,
    pub gravity: GravityScale,
    pub collider: Collider,
    pub continuous_collision_detection: Ccd,
    pub sleeping: Sleeping,
    pub collision_groups: CollisionGroups,
}
