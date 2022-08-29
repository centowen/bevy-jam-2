use crate::{assets, collision, crab};
use bevy::prelude::*;

const PLAYER_SPEED: f32 = 150.0;

#[derive(Component)]
pub struct Player;

pub fn spawn_player(commands: &mut Commands, image_assets: &assets::ImageAssets) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: image_assets.player.clone(),
            sprite: Sprite {
                custom_size: Some(Vec2::new(40.0, 28.1)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 5.0),
                ..default()
            },
            ..default()
        })
        .insert(Player)
        .insert(Name::new("Player"))
        .insert(collision::Collisions::new());
}

pub fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut q_player: Query<
        (&mut Transform, &collision::Collisions),
        (With<Player>, Without<crab::Crab>),
    >,
    mut q_crabs: Query<&mut Transform, (With<crab::Crab>, Without<Player>)>,
    time: Res<Time>,
) {
    let (mut transform, collisions) = q_player.single_mut();
    let mut player_translation = Vec3::default();

    if keyboard_input.pressed(KeyCode::Right) {
        player_translation.x += PLAYER_SPEED * time.delta_seconds();
    }
    if keyboard_input.pressed(KeyCode::Left) {
        player_translation.x -= PLAYER_SPEED * time.delta_seconds();
    }
    if keyboard_input.pressed(KeyCode::Up) {
        player_translation.y += PLAYER_SPEED * time.delta_seconds();
    }
    if keyboard_input.pressed(KeyCode::Down) {
        player_translation.y -= PLAYER_SPEED * time.delta_seconds();
    };
    transform.translation += player_translation;

    for collision in collisions.collisions.iter() {
        if let Ok(mut crab_transform) = q_crabs.get_mut(collision.entity) {
            crab_transform.translation += player_translation;
        }
    }
}
