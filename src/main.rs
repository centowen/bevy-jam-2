mod assets;
mod audio;
mod collision;
mod crab;
mod plane;
mod player;
mod spawner;

use bevy::{prelude::*, window::PresentMode};
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_kira_audio::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_turborand::*;

fn setup(mut commands: Commands, server: Res<AssetServer>) {
    commands.spawn_bundle(Camera2dBundle::default());
    // Airfield test begins
    let mut strip_builder = PathBuilder::new();
    strip_builder.move_to(Vec2::new(-640.0, 0.0));
    strip_builder.line_to(Vec2::new(640.0, 0.0));
    let strip = strip_builder.build();
    let mut up_builder = PathBuilder::new();
    up_builder.move_to(Vec2::new(-640.0, 209.0));
    up_builder.line_to(Vec2::new(640.0, 209.0));
    let up = up_builder.build();
    let mut down_builder = PathBuilder::new();
    down_builder.move_to(Vec2::new(-640.0, -209.0));
    down_builder.line_to(Vec2::new(640.0, -209.0));
    let down = down_builder.build();

    commands.spawn_bundle(GeometryBuilder::build_as(
        &strip,
        DrawMode::Stroke(StrokeMode::new(Color::BLACK, 68.0)),
        Transform::default(),
    ));
    commands.spawn_bundle(GeometryBuilder::build_as(
        &up,
        DrawMode::Stroke(StrokeMode::new(Color::BLUE, 350.0)),
        Transform::default(),
    ));
    commands.spawn_bundle(GeometryBuilder::build_as(
        &down,
        DrawMode::Stroke(StrokeMode::new(Color::BLUE, 350.0)),
        Transform::default(),
    ));
    // Airfield test ends
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
        .insert(Transform::from_translation(Vec3::new(200.0, 400.0, 1.0)));
    commands
        .spawn()
        .insert(spawner::Spawner { energy: 0 })
        .insert(Transform::from_translation(Vec3::new(-200.0, 400.0, 1.0)));
    commands
        .spawn()
        .insert(spawner::Spawner { energy: 0 })
        .insert(Transform::from_translation(Vec3::new(400.0, 400.0, 1.0)));
}

fn main() {
    App::new()
        // TODO: Disable window resizing...
        .insert_resource(WindowDescriptor {
            title: "Comb Ine".to_string(),
            width: 1280.,
            height: 768.,
            present_mode: PresentMode::AutoVsync,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(RngPlugin::default())
        .add_plugin(AudioPlugin)
        .insert_resource(ClearColor(Color::rgb(0.8, 0.85, 0.85)))
        .add_startup_system(setup)
        .add_startup_system(audio::start_background_audio)
        .add_system(spawner::spawn_tick)
        .add_system(audio::play_audio)
        .add_system(crab::move_crabs)
        .add_system(crab::despawn_crabs)
        .add_system(player::move_player)
        .add_system(plane::move_plane)
        .add_system(plane::move_plane_shadow)
        .add_system(plane::collide_with_world)
        .add_system(collision::collide_stuff)
        .run();
}
