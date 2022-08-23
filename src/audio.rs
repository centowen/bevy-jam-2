use crate::assets;
use bevy::prelude::*;
use bevy_turborand::*;

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
        audio.play_with_settings(
            sounds.crab.clone(),
            PlaybackSettings::LOOP
                .with_volume(0.01)
                .with_speed(0.05 + rng.f32() * 1.0),
        );
        a.played = true;
    }
}
