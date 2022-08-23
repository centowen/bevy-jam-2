use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_turborand::*;
use bevy::audio::*;
use std::{f32::consts::PI, time::Duration};

struct CrabSpawnTimer {
    timer: Timer,
}

struct ImageAssets {
    crab: Handle<Image>,
}

struct SoundAssets {
    crab: Handle<AudioSource>,
}

#[derive(Component)]
struct Crab;

#[derive(Component, Default)]
struct Velocity(Vec2);

fn setup(mut commands: Commands, server: Res<AssetServer>) {
    commands.spawn_bundle(Camera2dBundle::default());
    commands.insert_resource(CrabSpawnTimer {
        timer: Timer::new(Duration::from_millis(500), true),
    });
    commands.insert_resource(ImageAssets {
        crab: server.load("rustacean-flat-noshadow.png"),
    });
    commands.insert_resource(SoundAssets {
        crab: server.load("sound/crab.ogg"),
    });
}

fn spawn_crab(
    mut commands: Commands,
    images: Res<ImageAssets>,
    sounds: Res<SoundAssets>,
    audio: Res<Audio>,
    time: Res<Time>,
    mut rng: ResMut<GlobalRng>,
    mut crab_timer: ResMut<CrabSpawnTimer>,
) {
    crab_timer.timer.tick(time.delta());

    if crab_timer.timer.just_finished() {
        commands
            .spawn_bundle(SpriteBundle {
                texture: images.crab.clone(),
                transform: Transform {
                    scale: Vec3::splat(0.02),
                    translation: Vec3::new(200.0, 400.0, 0.0),
                    ..default()
                },
                ..default()
            })
            .insert(Velocity::default())
            .insert(Crab)
            .insert(Name::new("Crab"));

        audio.play_with_settings(sounds.crab.clone(), PlaybackSettings::LOOP.with_volume(0.01).with_speed(0.05 + rng.f32() * 1.0));
    }
}

fn move_crabs(
    mut q_crabs: Query<(&mut Transform, &mut Velocity, &mut Sprite), With<Crab>>,
    mut rng: ResMut<GlobalRng>,
) {
    for (mut transform, mut velocity, mut sprite) in q_crabs.iter_mut() {
        let mut v_norm = (velocity.0.x.powf(2.0) + velocity.0.y.powf(2.0)).powf(0.5);
        if v_norm < 0.0001 {
            velocity.0 = Vec2::new(-transform.translation.x + 0.5, -transform.translation.y + 0.5); // Vec2::new(rng.f32() * 2.0 - 1.0, rng.f32() * 2.0 - 1.0);
            v_norm = (velocity.0.x.powf(2.0) + velocity.0.y.powf(2.0)).powf(0.5);
            velocity.0 = velocity.0 / v_norm;
        }
        let nv = velocity.0 / v_norm;
        let perp_nv = Vec2::new(nv.y, -nv.x);
        velocity.0 += (rng.f32() * 0.2 - 0.1) * perp_nv;
        velocity.0 += (rng.f32() * 0.1 - 0.05) * nv;
        transform.translation += velocity.0.extend(0.0);
        let angle = (PI / 2.0) - f32::atan2(velocity.0.x, velocity.0.y);
        transform.rotation = Quat::from_rotation_z(angle);
        sprite.flip_y = angle > PI / 2.0;
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(RngPlugin::default())
        .add_plugin(AudioPlugin)
        .insert_resource(ClearColor(Color::rgb(0.8, 0.85, 0.85)))
        .add_startup_system(setup)
        .add_system(spawn_crab)
        .add_system(move_crabs)
        .run();
}
