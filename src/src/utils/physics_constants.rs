
use bevy_rapier2d::prelude::*;
use bevy::prelude::*;

pub const DEFAULT_COLLISION_GROUPS: CollisionGroups = CollisionGroups::new(
    Group::from_bits(0b0001).unwrap(),
    Group::from_bits(0b1111).unwrap()
);

pub const PLAYER_COLLISION_GROUPS: CollisionGroups = CollisionGroups::new(
    Group::from_bits(0b0010).unwrap(),
    Group::from_bits(0b1101).unwrap()
);

pub const DEFAULT_GRAVITY: GravityScale = GravityScale(0.0);
pub const DEFAULT_VELOCITY: Velocity = Velocity {
    linvel: Vec2::new(0.0, 0.0),
    angvel: 0.0,
};
pub const DEFAULT_ACTIVE_EVENTS: ActiveEvents = ActiveEvents::COLLISION_EVENTS;
