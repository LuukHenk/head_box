
use bevy_rapier2d::prelude::{CollisionGroups, Group};

pub const DEFAULT_COLLISION_GROUPS: CollisionGroups = CollisionGroups::new(
    Group::from_bits(0b0001).unwrap(),
    Group::from_bits(0b1110).unwrap()
);

pub const PLAYER_COLLISION_GROUPS: CollisionGroups = CollisionGroups::new(
    Group::from_bits(0b0010).unwrap(),
    Group::from_bits(0b1101).unwrap()
);