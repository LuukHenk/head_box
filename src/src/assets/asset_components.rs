use bevy::prelude::*;


#[derive(Component)]
pub struct PlayerTexture(pub Handle<Image>);

#[derive(Component)]
pub struct ZombieTexture(pub Handle<Image>);

#[derive(Component)]
pub struct BulletTexture(pub Handle<Image>);
