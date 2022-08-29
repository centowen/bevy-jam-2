mod assets;
mod audio;
mod collision;
mod crab;
mod plane;
mod player;
mod spawner;

use bevy::{prelude::*, window::PresentMode};
use bevy_egui::{egui, EguiContext, EguiPlugin};
// use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_kira_audio::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_turborand::*;

const WORLD_SIZE: (f32, f32) = (1280.0, 768.0);

fn build_airfield(commands: &mut Commands) {
    const X_MIN: f32 = -WORLD_SIZE.0 / 2.0;
    const X_MAX: f32 = WORLD_SIZE.0 / 2.0;

    const STRIP_WIDTH: f32 = 68.0;
    const WATER_WIDTH: f32 = 70.0;
    const GROUND_WIDTH: f32 = WORLD_SIZE.1 / 2.0 - STRIP_WIDTH / 2.0 - WATER_WIDTH;

    const STRIP_POS: f32 = 0.0;
    const GROUND_POS: f32 = STRIP_WIDTH / 2.0 + GROUND_WIDTH / 2.0;
    const WATER_POS: f32 = STRIP_WIDTH / 2.0 + GROUND_WIDTH + WATER_WIDTH / 2.0;

    let mut strip_builder = PathBuilder::new();
    strip_builder.move_to(Vec2::new(X_MIN, STRIP_POS));
    strip_builder.line_to(Vec2::new(X_MAX, STRIP_POS));
    let strip = strip_builder.build();
    let mut up_builder = PathBuilder::new();
    up_builder.move_to(Vec2::new(X_MIN, GROUND_POS));
    up_builder.line_to(Vec2::new(X_MAX, GROUND_POS));
    let up = up_builder.build();
    let mut down_builder = PathBuilder::new();
    down_builder.move_to(Vec2::new(X_MIN, -GROUND_POS));
    down_builder.line_to(Vec2::new(X_MAX, -GROUND_POS));
    let down = down_builder.build();

    let mut up_water_builder = PathBuilder::new();
    up_water_builder.move_to(Vec2::new(X_MIN, WATER_POS));
    up_water_builder.line_to(Vec2::new(X_MAX, WATER_POS));
    let up_water = up_water_builder.build();
    let mut down_water_builder = PathBuilder::new();
    down_water_builder.move_to(Vec2::new(X_MIN, -WATER_POS));
    down_water_builder.line_to(Vec2::new(X_MAX, -WATER_POS));
    let down_water = down_water_builder.build();

    let strip_color = Color::rgb_u8(0x53, 0x55, 0x4c);
    let ground_color = Color::rgb_u8(0xEA, 0xB8, 0x75);
    let water_color = Color::rgb_u8(0x53, 0xCC, 0xEC);
    commands.spawn_bundle(GeometryBuilder::build_as(
        &strip,
        DrawMode::Stroke(StrokeMode::new(strip_color, STRIP_WIDTH)),
        Transform::default(),
    ));
    commands.spawn_bundle(GeometryBuilder::build_as(
        &up,
        DrawMode::Stroke(StrokeMode::new(ground_color, GROUND_WIDTH)),
        Transform::default(),
    ));
    commands.spawn_bundle(GeometryBuilder::build_as(
        &down,
        DrawMode::Stroke(StrokeMode::new(ground_color, GROUND_WIDTH)),
        Transform::default(),
    ));
    commands.spawn_bundle(GeometryBuilder::build_as(
        &up_water,
        DrawMode::Stroke(StrokeMode::new(water_color, WATER_WIDTH)),
        Transform::default(),
    ));
    commands.spawn_bundle(GeometryBuilder::build_as(
        &down_water,
        DrawMode::Stroke(StrokeMode::new(water_color, WATER_WIDTH)),
        Transform::default(),
    ));
}

fn setup(mut commands: Commands, server: Res<AssetServer>, mut global_rng: ResMut<GlobalRng>) {
    commands.spawn_bundle(Camera2dBundle::default());

    build_airfield(&mut commands);

    let image_assets = assets::ImageAssets {
        crab: server.load("rustacean-flat-noshadow.png"),
        dead_crab: server.load("dead_crab.png"),
        player: server.load("player.png"),
        plane: server.load("plane.png"),
        plane_shadow: server.load("plane-shadow.png"),
        smoke: server.load("smoke.png"),
        tree: server.load("tree.png"),
    };

    player::spawn_player(&mut commands, &image_assets);
    plane::spawn_plane(&mut commands, &image_assets, &mut global_rng);

    commands.spawn_bundle(SpriteBundle {
        texture: image_assets.tree.clone(),
        transform: Transform {
            translation: Vec3::new(200.0, 300.0, 10.0),
            ..default()
        },
        ..default()
    });
    commands.spawn_bundle(SpriteBundle {
        texture: image_assets.tree.clone(),
        transform: Transform {
            translation: Vec3::new(-300.0, 250.0, 10.0),
            ..default()
        },
        ..default()
    });
    commands.spawn_bundle(SpriteBundle {
        texture: image_assets.tree.clone(),
        transform: Transform {
            translation: Vec3::new(300.0, -250.0, 10.0),
            ..default()
        },
        ..default()
    });

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

fn counter(q_crabs: Query<Entity, With<crab::DeadCrab>>, mut egui_context: ResMut<EguiContext>) {
    egui::Area::new("my_area")
        .fixed_pos(egui::pos2(32.0, 32.0))
        .show(egui_context.ctx_mut(), |ui| {
            ui.label(
                egui::RichText::new(format!("Dead crabs: {}", q_crabs.iter().count()))
                    .color(egui::Color32::BLACK),
            );
        });
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Comb Ine".to_string(),
            width: WORLD_SIZE.0,
            height: WORLD_SIZE.1,
            present_mode: PresentMode::AutoVsync,
            resizable: false,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(ShapePlugin)
        // .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(RngPlugin::default())
        .add_plugin(AudioPlugin)
        .insert_resource(ClearColor(Color::rgb(0.8, 0.85, 0.85)))
        .add_startup_system(setup)
        .add_startup_system(audio::start_background_audio)
        .add_system(counter)
        .add_system(spawner::spawn_tick)
        .add_system(audio::play_audio)
        .add_system(crab::move_crabs)
        .add_system(crab::despawn_crabs)
        .add_system(player::move_player)
        .add_system(plane::move_plane)
        .add_system(plane::despawn_plane)
        .add_system(plane::move_plane_shadow)
        .add_system(plane::spawn_smoke)
        .add_system(plane::move_smoke)
        .add_system(plane::collide_with_world)
        .add_system(collision::collide_stuff)
        .run();
}
