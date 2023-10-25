use bevy::prelude::*;


#[derive(Component)]
pub struct PlayerTextures{
    pub front: Handle<Image>,
    pub side: Handle<Image>,
    pub back: Handle<Image>,
    pub side_flipped: Handle<Image>,
}

#[derive(Component)]
pub struct ZombieTexture(pub Handle<Image>);

#[derive(Component)]
pub struct BulletTexture(pub Handle<Image>);
