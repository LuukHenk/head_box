



use bevy::prelude::{Entity, Event};

#[derive(Event)]
pub struct PlayerShootEvent(pub(crate) Entity);