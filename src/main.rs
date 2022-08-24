mod assets;
mod audio;
mod crab;
mod plane;
mod player;
mod spawner;

use bevy::audio::*;
use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_turborand::*;

fn setup(mut commands: Commands, server: Res<AssetServer>) {
    commands.spawn_bundle(Camera2dBundle::default());
    let image_assets = assets::ImageAssets {
        crab: server.load("rustacean-flat-noshadow.png"),
        player: server.load("player.png"),
        plane: server.load("plane.png"),
        plane_shadow: server.load("plane-shadow.png"),
    };
    player::spawn_player(&mut commands, &image_assets);
    plane::spawn_plane(&mut commands, &image_assets);
    commands.insert_resource(image_assets);
    commands.insert_resource(assets::SoundAssets {
        crab: server.load("sound/crab.ogg"),
    });
    // KNARK: Add abstraction to create spawners
    commands
        .spawn()
        .insert(spawner::Spawner { energy: 0 })
        .insert(Transform::from_translation(Vec3::new(200.0, 400.0, 0.0)));
    commands
        .spawn()
        .insert(spawner::Spawner { energy: 0 })
        .insert(Transform::from_translation(Vec3::new(-200.0, 400.0, 0.0)));
    commands
        .spawn()
        .insert(spawner::Spawner { energy: 0 })
        .insert(Transform::from_translation(Vec3::new(400.0, 400.0, 0.0)));
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(RngPlugin::default())
        .add_plugin(AudioPlugin)
        .insert_resource(ClearColor(Color::rgb(0.8, 0.85, 0.85)))
        .add_startup_system(setup)
        .add_system(spawner::spawn_tick)
        .add_system(audio::play_audio)
        .add_system(crab::move_crabs)
        .add_system(player::move_player)
        .add_system(plane::move_plane)
        .run();
}
