
use bevy::prelude::{
    Commands,
    Bundle,
};
use super::generic_components::GameScreenMarker;

pub struct GenericFunctions;

impl GenericFunctions {
    pub fn spawn<T: Bundle>(mut commands: Commands, object_to_spawn: T) {
        commands.spawn(object_to_spawn).insert(GameScreenMarker);
    }
}