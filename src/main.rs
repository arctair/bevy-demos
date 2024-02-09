use bevy::DefaultPlugins;
use bevy::prelude::{App, Camera2dBundle, Commands, default, OrthographicProjection, Startup};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, startup_camera)
        .run();
}

fn startup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            scale: 1. / 1.,
            ..default()
        },
        ..default()
    });
}
