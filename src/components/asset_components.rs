use bevy::prelude::*;


#[derive(Component)]
pub struct PlayerTextureHandles {
    pub front: Vec<Handle<Image>>,
    pub right: Vec<Handle<Image>>,
    pub back: Vec<Handle<Image>>,
}

#[derive(Component)]
pub enum CurrentAnimationAction {
    WalkingLeft,
    WalkingRight,
    WalkingDown,
    WalkingUp,
    StandingLeft,
    StandingRight,
    StandingDown,
    StandingUp,
    // Hurting,
}

#[derive(Component)]
pub struct CurrentAnimationFrame(pub usize);


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
pub struct InGameMusicHandle(pub Handle<AudioSource>);

#[derive(Component)]
pub struct MenuMusicHandle(pub Handle<AudioSource>);