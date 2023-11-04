use bevy::prelude::*;

#[derive(Component)]
pub struct PlayerTextureHandles {
    pub front: Handle<Image>,
    pub side: Handle<Image>,
    pub back: Handle<Image>,
    pub side_flipped: Handle<Image>,
}

#[derive(Component)]
pub struct ZombieTextureHandle(pub Handle<Image>);

#[derive(Component)]
pub struct BulletTextureHandle(pub Handle<Image>);

#[derive(Component)]
pub struct PistolSoundHandle(pub Handle<AudioSource>);

#[derive(Component)]
pub struct UziSoundHandle(pub Handle<AudioSource>);

#[derive(Component)]
pub struct ShotgunSoundHandle(pub Handle<AudioSource>);

#[derive(Component)]
pub struct ZombieTenseSoundHandle(pub Handle<AudioSource>);

#[derive(Component)]
pub struct BackgroundMusicHandle(pub Handle<AudioSource>);
