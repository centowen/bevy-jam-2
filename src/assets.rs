use bevy::audio::*;
use bevy::prelude::*;

pub struct ImageAssets {
    pub crab: Handle<Image>,
    pub player: Handle<Image>,
}

pub struct SoundAssets {
    pub crab: Handle<AudioSource>,
}
