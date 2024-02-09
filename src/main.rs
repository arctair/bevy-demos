use bevy::DefaultPlugins;
use bevy::prelude::{App, Camera2dBundle, Color, Commands, Component, default, Gizmos, OrthographicProjection, Query, Startup, Update, Vec2};
use noisy_bevy::simplex_noise_2d;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, startup_camera)
        .add_systems(Startup, startup_octree)
        .add_systems(Update, update_octree)
        .run();
}

fn startup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            scale: 1. / 256.,
            ..default()
        },
        ..default()
    });
}

#[derive(Component)]
struct Octree {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    value: usize,
}

fn startup_octree(mut commands: Commands) {
    for x in -1..1 {
        for y in -1..1 {
            let size = Vec2::new(1., 1.);
            let position = Vec2::new(x as f32, y as f32) + 0.5 * size;
            commands.spawn(Octree {
                x: position.x,
                y: position.y,
                width: size.x,
                height: size.y,
                value: match simplex_noise_2d(position) {
                    value if value > 0. => 1,
                    _ => 0
                },
            });
        }
    }
}

fn update_octree(
    query: Query<&Octree>,
    mut gizmos: Gizmos,
) {
    for octree in query.iter() {
        gizmos.rect_2d(
            Vec2::new(octree.x, octree.y),
            0.,
            Vec2::new(octree.width, octree.height),
            match octree.value {
                0 => Color::Hsla {
                    hue: 0.,
                    saturation: 0.,
                    lightness: 1.,
                    alpha: 1.,
                },
                1 => Color::Hsla {
                    hue: 0.,
                    saturation: 1.,
                    lightness: 0.25,
                    alpha: 1.,
                },
                _ => Color::PINK,
            },
        );
    }
}
