use bevy::DefaultPlugins;
use bevy::prelude::*;
use bevy::prelude::shape::Quad;
use bevy::sprite::{ColorMaterial, MaterialMesh2dBundle, Mesh2dHandle};
use bevy_rapier2d::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1.))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, startup)
        .add_systems(Update, update)
        .run();
}

fn startup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands.spawn(Camera2dBundle::default());

    let material = materials.add(ColorMaterial::from(Color::PINK));
    let mesh: Mesh2dHandle = meshes.add(Mesh::from(Quad::default())).into();
    let transform = Transform::default().with_scale(512. * Vec3::ONE);
    
    commands.spawn(MaterialMesh2dBundle {
        material: material.clone(),
        mesh: mesh.clone(),
        transform,
        ..default()
    });
}

fn update(mut gizmos: Gizmos) {
    gizmos.rect_2d(
        Vec2::ZERO,
        0.,
        512. * Vec2::ONE,
        Color::GREEN,
    );
}