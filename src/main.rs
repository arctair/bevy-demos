use bevy::DefaultPlugins;
use bevy::prelude::{App, Assets, AssetServer, BuildChildren, Camera2dBundle, Color, Commands, Component, default, Gizmos, ImagePlugin, Mesh, PluginGroup, Query, Res, ResMut, SpatialBundle, Startup, Transform, Update, Vec2, Vec3Swizzles};
use bevy::prelude::shape::Quad;
use bevy::sprite::{ColorMaterial, MaterialMesh2dBundle, Mesh2dHandle};
use bevy_rapier2d::prelude::{NoUserData, RapierDebugRenderPlugin, RapierPhysicsPlugin};
use quadtree_demo::quadtree::QuadTree;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1.))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, startup)
        .add_systems(Update, update)
        .run();
}


#[derive(Component)]
struct Container {
    quadtree: QuadTree<()>,
}

fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands.spawn(Camera2dBundle::default());

    let container = Container { quadtree: QuadTree::new(0, |_| ()) };
    let transform = Transform::default()
        .with_translation(Vec2::splat(-256.).extend(0.))
        .with_scale(Vec2::splat(512.).extend(0.));

    let material = materials.add(ColorMaterial::from(asset_server.load("air.png")));
    let mesh: Mesh2dHandle = meshes.add(Mesh::from(Quad::default())).into();
    commands.spawn_empty()
        .with_children(|parent| {
            for leaf in container.quadtree.nodes() {
                let transform = Transform::default()
                    .with_translation(leaf.id.center().extend(0.))
                    .with_scale(leaf.id.size().extend(0.));
                parent.spawn(MaterialMesh2dBundle {
                    material: material.clone(),
                    mesh: mesh.clone(),
                    transform,
                    ..default()
                });
            }
        })
        .insert(container)
        .insert(SpatialBundle::from_transform(transform));
}

fn update(
    query: Query<(&Container, &Transform)>,
    mut gizmos: Gizmos,
) {
    for (container, transform) in query.iter() {
        for leaf in container.quadtree.nodes() {
            gizmos.rect_2d(
                transform.transform_point(leaf.id.center().extend(0.)).xy(),
                0.,
                transform.scale.xy() * leaf.id.size(),
                Color::RED,
            );
        }
    }
}