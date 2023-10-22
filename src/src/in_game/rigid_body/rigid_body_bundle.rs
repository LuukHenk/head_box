use bevy::prelude::{
    Bundle,
    SpriteBundle,
};
use bevy_rapier2d::prelude::{RigidBody, Velocity, GravityScale, Collider, Ccd, Sleeping, CollisionGroups, ActiveEvents};
use crate::in_game::data_classes::rigid_body_components::WalkingVelocity;


#[derive(Bundle)]
pub struct RigidBodyBundle {
    pub rigid_body: RigidBody,
    pub sprite_bundle: SpriteBundle,
    pub velocity: Velocity,
    pub walking_velocity: WalkingVelocity,
    pub gravity: GravityScale,
    pub collider: Collider,
    pub continuous_collision_detection: Ccd,
    pub sleeping: Sleeping,
    pub collision_groups: CollisionGroups,
    pub active_events: ActiveEvents,
}
