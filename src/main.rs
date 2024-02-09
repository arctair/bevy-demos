mod quadtree;

use bevy::DefaultPlugins;
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::prelude::{App, Camera2dBundle, Color, Commands, Component, default, EventReader, Gizmos, OrthographicProjection, Query, Startup, Update, Vec2};
use crate::quadtree::{QuadTree, QuadTreeBuilder};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, startup_camera)
        .add_systems(Update, update_camera)
        .add_systems(Startup, startup_terrain)
        .add_systems(Update, update_terrain)
        .run();
}

fn startup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            scale: 1. / 1.,
            ..default()
        },
        ..default()
    }).insert(ZoomConfiguration { min: 0.25, max: 4., speed: 0.25 });
}

fn update_camera(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query: Query<(&mut OrthographicProjection, &ZoomConfiguration)>,
) {
    let (mut projection, zoom) = query.single_mut();
    for event in mouse_wheel_events.read() {
        if let MouseScrollUnit::Line = event.unit {
            let mut scale_inverse = 1. / projection.scale;
            scale_inverse = zoom.min.max(zoom.max.min(scale_inverse + zoom.speed * event.y));
            projection.scale = 1. / scale_inverse;
        }
    }
}

#[derive(Component)]
struct ZoomConfiguration {
    min: f32,
    max: f32,
    speed: f32,
}

fn startup_terrain(mut commands: Commands) {
    let chunk_count_square_root = 1;
    let chunk_size = Vec2::splat(512.);
    let chunk_offset_global = 0.5 * (1 - chunk_count_square_root) as f32 * chunk_size;

    for i in 0..chunk_count_square_root {
        for j in 0..chunk_count_square_root {
            commands.spawn(QuadTreeBuilder::new(
                chunk_size,
                chunk_offset_global + chunk_size * Vec2::new(i as f32, j as f32),
                7,
                4.,
            ).build_root());
        }
    }
}

fn update_terrain(
    query: Query<&QuadTree>,
    mut gizmos: Gizmos,
) {
    for root in query.iter() {
        show_quadtree_root(root, &mut gizmos);
    }
}

fn show_quadtree_root(root: &QuadTree, gizmos: &mut Gizmos) {
    show_quadtree(root, gizmos);
    gizmos.rect_2d(root.position(), 0., root.size(), Color::GREEN);
}

fn show_quadtree(quadtree: &QuadTree, gizmos: &mut Gizmos) {
    match quadtree.value() {
        Some(1) => gizmos.rect_2d(quadtree.position(), 0., quadtree.size(), Color::RED),
        None => quadtree.children().iter().for_each(|child| show_quadtree(child, gizmos)),
        _ => {}
    }
}