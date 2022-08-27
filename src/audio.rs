use crate::assets;

use bevy_kira_audio::prelude::*;
use bevy_turborand::*;
use bevy::prelude::*;

// KNARK: Cleanup after sound has been played
// TODO: How support other sounds?
#[derive(Component)]
pub struct AudioEvent {
    pub played: bool,
}

pub fn play_audio(
    mut q_audio: Query<&mut AudioEvent>,
    sounds: Res<assets::SoundAssets>,
    audio: Res<Audio>,
    mut rng: ResMut<GlobalRng>,
) {
    for mut a in q_audio.iter_mut() {
        if a.played {
            continue;
        }

        audio
            .play(sounds.crab.clone())
            .with_volume(0.01)
            .with_playback_rate(0.05 + rng.f64() * 1.0);

        a.played = true;
    }
}

pub fn start_background_audio(
    asset_server: Res<AssetServer>,
    audio: Res<Audio>
) {
    audio.play(asset_server.load("sound/calypso.ogg")).looped();
}
