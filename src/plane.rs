use crate::assets;
use bevy::prelude::*;

const PLANE_SPEED: f32 = 50.0;

#[derive(Component)]
pub struct Plane;

pub fn spawn_plane(commands: &mut Commands, image_assets: &assets::ImageAssets) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: image_assets.plane_shadow.clone(),
            transform: Transform {
                translation: Vec3::new(1000.0, 0.0, 50.0),
                ..default()
            },
            sprite: Sprite {
                flip_x: true,
                ..default()
            },
            ..default()
        })
        .insert(Plane)
        .insert(Name::new("Plane"))
        .with_children(|parent| {
            parent
                .spawn_bundle(SpriteBundle {
                    texture: image_assets.plane.clone(),
                    transform: Transform {
                        translation: Vec3::new(0.0, 20.0, 100.0),
                        ..default()
                    },
                    sprite: Sprite {
                        flip_x: true,
                        ..default()
                    },
                    ..default()
                })
                .insert(Name::new("Shadow"));
        });
}

pub fn move_plane(mut q_plane: Query<&mut Transform, With<Plane>>, time: Res<Time>) {
    let mut transform = q_plane.single_mut();
    transform.translation.x -= PLANE_SPEED * time.delta_seconds();
}
