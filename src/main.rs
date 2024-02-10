mod quadtree;

use bevy::DefaultPlugins;
use bevy::input::ButtonState;
use bevy::input::keyboard::KeyboardInput;
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::prelude::{App, BuildChildren, Camera2dBundle, Color, Commands, Component, EventReader, Gizmos, KeyCode, OrthographicProjection, Query, Startup, Transform, TransformBundle, Update, Vec2};
use bevy_rapier2d::prelude::{Collider, ExternalForce, LockedAxes, NoUserData, RapierDebugRenderPlugin, RapierPhysicsPlugin, RigidBody, Vect, Velocity};
use crate::quadtree::{QuadTree, QuadTreeBuilder};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1.))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, startup_player)
        .add_systems(Update, update_player)
        .add_systems(Startup, startup_terrain)
        .add_systems(Update, update_terrain)
        .run();
}

fn startup_player(mut commands: Commands) {
    commands.spawn(RigidBody::Dynamic)
        .insert(Collider::capsule(Vect::new(0., -2.), Vect::new(0., 2.), 2.))
        .insert(TransformBundle::from_transform(Transform::from_xyz(0., 256., 0.)))
        .insert(Velocity::default())
        .insert(ExternalForce::default())
        .insert(PlayerControls::default())
        .insert(LockedAxes::ROTATION_LOCKED)
        .with_children(|parent| { parent.spawn(Camera2dBundle::default()).insert(ZoomConfiguration { min: 0.5, max: 8., speed: 0.25 }); });
}

#[derive(Component, Default)]
struct PlayerControls {
    up: bool,
    left: bool,
    down: bool,
    right: bool,
    action: bool,
}

#[derive(Component)]
struct ZoomConfiguration {
    min: f32,
    max: f32,
    speed: f32,
}

fn update_player(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut camera_query: Query<(&mut OrthographicProjection, &ZoomConfiguration)>,
    mut keyboard_events: EventReader<KeyboardInput>,
    mut player_query: Query<(&Velocity, &mut ExternalForce, &mut PlayerControls)>,
) {
    let (mut projection, zoom) = camera_query.single_mut();
    for event in mouse_wheel_events.read() {
        if let MouseScrollUnit::Line = event.unit {
            let mut scale_inverse = 1. / projection.scale;
            scale_inverse = zoom.min.max(zoom.max.min(scale_inverse + zoom.speed * event.y));
            projection.scale = 1. / scale_inverse;
        }
    }

    let (velocity, mut external_force, mut player_controls) = player_query.single_mut();
    for keyboard_event in keyboard_events.read() {
        match (keyboard_event.key_code, keyboard_event.state) {
            (Some(KeyCode::W), ButtonState::Pressed) => { player_controls.up = true }
            (Some(KeyCode::W), ButtonState::Released) => { player_controls.up = false }
            (Some(KeyCode::A), ButtonState::Pressed) => { player_controls.left = true }
            (Some(KeyCode::A), ButtonState::Released) => { player_controls.left = false }
            (Some(KeyCode::S), ButtonState::Pressed) => { player_controls.down = true }
            (Some(KeyCode::S), ButtonState::Released) => { player_controls.down = false }
            (Some(KeyCode::D), ButtonState::Pressed) => { player_controls.right = true }
            (Some(KeyCode::D), ButtonState::Released) => { player_controls.right = false }
            (Some(KeyCode::Space), ButtonState::Pressed) => { player_controls.action = true }
            (Some(KeyCode::Space), ButtonState::Released) => { player_controls.action = false }
            _ => {}
        }
    }

    let max_speed_x = 32.;
    let left = 4096. * if player_controls.left && velocity.linvel.x > -max_speed_x { Vect::NEG_X } else { Vect::ZERO };
    let right = 4096. * if player_controls.right && velocity.linvel.x < max_speed_x { Vect::X } else { Vect::ZERO };

    let up = 8192. * if player_controls.up && velocity.linvel.y < 32. { Vect::Y } else { Vect::ZERO };
    let down = 1024. * if player_controls.down && velocity.linvel.y > -2048. { Vect::NEG_Y } else { Vect::ZERO };
    external_force.force = left + right + up + down;
}

fn startup_terrain(mut commands: Commands) {
    let chunk_count_square_root = 1;
    let chunk_size = Vec2::splat(512.);
    let chunk_offset_global = 0.5 * (1 - chunk_count_square_root) as f32 * chunk_size;

    for i in 0..chunk_count_square_root {
        for j in 0..chunk_count_square_root {
            let root = QuadTreeBuilder::new(
                chunk_size,
                chunk_offset_global + chunk_size * Vec2::new(i as f32, j as f32),
                8,
                4.,
            ).build_root();

            let collider = root.collider();
            commands.spawn(root).insert(collider);
        }
    }
}

fn update_terrain(
    query: Query<&QuadTree>,
    mut gizmos: Gizmos,
) {
    for root in query.iter() {
        gizmos.rect_2d(root.position(), 0., root.size(), Color::GREEN);
    }
}
