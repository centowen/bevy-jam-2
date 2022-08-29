use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

pub struct ImageAssets {
    pub crab: Handle<Image>,
    pub dead_crab: Handle<Image>,
    pub player: Handle<Image>,
    pub plane: Handle<Image>,
    pub plane_shadow: Handle<Image>,
    pub smoke: Handle<Image>,
}

pub struct SoundAssets {
    pub crab: Handle<AudioSource>,
}
