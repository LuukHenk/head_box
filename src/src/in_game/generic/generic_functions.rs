
use bevy::prelude::{Commands, Bundle, Entity};
use super::GameScreenMarker;

pub struct GenericFunctions;

impl GenericFunctions {
    pub fn spawn<T: Bundle>(mut commands: Commands, object_to_spawn: T) -> Entity {
        commands.spawn(object_to_spawn).insert(GameScreenMarker).id()
    }
}