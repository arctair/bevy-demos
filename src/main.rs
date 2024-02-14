use bevy::DefaultPlugins;
use bevy::prelude::{App, Assets, AssetServer, Camera2dBundle, Color, Commands, default, Gizmos, ImagePlugin, Mesh, PluginGroup, Res, ResMut, Startup, Transform, Update, Vec2};
use bevy::prelude::shape::Quad;
use bevy::sprite::{ColorMaterial, MaterialMesh2dBundle};
use bevy_rapier2d::prelude::{NoUserData, RapierDebugRenderPlugin, RapierPhysicsPlugin};

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
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands.spawn(Camera2dBundle::default());

    let scale = Vec2::splat(512.);
    for leaf in QuadTreeNode::new().leafs() {
        commands.spawn(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(Quad::default())).into(),
            transform: Transform::default()
                .with_translation((scale * leaf.center() + Vec2::splat(-256.)).extend(0.))
                .with_scale((scale * leaf.size()).extend(0.)),
            material: materials.add(ColorMaterial::from(asset_server.load("air.png"))),
            ..default()
        });
    }
}

fn update(
    mut gizmos: Gizmos,
) {
    let scale = Vec2::splat(512.);
    let translation = Vec2::splat(-256.);
    for leaf in QuadTreeNode::new().leafs() {
        gizmos.rect_2d(
            scale * leaf.center() + translation,
            0.,
            scale * leaf.size(),
            Color::RED,
        );
    }
}

struct QuadTree {
    vec: Vec<QuadTreeNode>,
}

impl QuadTree {
    fn leafs(&self) -> impl Iterator<Item=&QuadTreeNode> {
        self.vec[1..].into_iter()
    }
}

struct QuadTreeNode {
    center: Vec2,
    size: Vec2,
}

impl QuadTreeNode {
    fn new() -> QuadTree {
        QuadTree {
            vec: vec![
                QuadTreeNode {
                    center: Vec2::new(0.5, 0.5),
                    size: Vec2::new(1., 1.),
                },
                QuadTreeNode {
                    center: Vec2::new(0.25, 0.25),
                    size: Vec2::new(0.5, 0.5),
                },
                QuadTreeNode {
                    center: Vec2::new(0.75, 0.25),
                    size: Vec2::new(0.5, 0.5),
                },
                QuadTreeNode {
                    center: Vec2::new(0.25, 0.75),
                    size: Vec2::new(0.5, 0.5),
                },
                QuadTreeNode {
                    center: Vec2::new(0.75, 0.75),
                    size: Vec2::new(0.5, 0.5),
                },
            ]
        }
    }

    fn center(&self) -> Vec2 { self.center }
    fn size(&self) -> Vec2 { self.size }
}