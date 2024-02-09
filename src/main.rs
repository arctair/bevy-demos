use bevy::DefaultPlugins;
use bevy::prelude::{App, Camera2dBundle, Color, Commands, Component, default, Gizmos, OrthographicProjection, Quat, Query, Startup, Update, Vec2, Vec3};

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
}

fn startup_octree(mut commands: Commands) {
    commands.spawn(Octree {
        x: 0.,
        y: 0.,
        width: 1.,
        height: 1.,
    });
}

fn update_octree(
    query: Query<&Octree>,
    mut gizmos: Gizmos,
) {
    for octree in query.iter() {
        gizmos.rect(
            Vec3::new(octree.x, octree.y, 0.),
            Quat::default(),
            Vec2::new(octree.width, octree.height),
            Color::GREEN,
        );
    }
}
