use bevy::DefaultPlugins;
use bevy::prelude::*;
use bevy::prelude::shape::Quad;
use bevy::sprite::{ColorMaterial, MaterialMesh2dBundle, Mesh2dHandle};
use bevy_rapier2d::prelude::*;
use bevy_rapier2d::prelude::ColliderMassProperties::Density;

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
    let material = materials.add(ColorMaterial::from(Color::PINK));
    let mesh: Mesh2dHandle = meshes.add(Mesh::from(Quad::default())).into();

    commands.spawn(RigidBody::Fixed)
        .insert(Collider::cuboid(0.5, 0.5))
        .insert(SpatialBundle::from_transform(Transform::default()
            .with_scale(Vec3::new(1024., 1., 0.))));

    let entity = commands.spawn(RigidBody::Dynamic)
        .insert(Collider::cuboid(0.5, 0.5))
        .insert(SpatialBundle::from_transform(Transform::default()
            .with_translation(2. * Vec3::Y)
            .with_scale(2. * Vec3::ONE)))
        .insert(ExternalImpulse::default())
        .insert(Velocity::default())
        .insert(Density(0.5))
        .with_children(|parent| {
            parent.spawn(MaterialMesh2dBundle {
                material: material.clone(),
                mesh: mesh.clone(),
                ..default()
            });
        }).id();

    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            scale: 1. / 16.,
            ..default()
        },
        ..default()
    })
        .insert(ImpulseJoint::new(entity, RevoluteJointBuilder::new().build()))
        .insert(RigidBody::Dynamic)
        .insert(AdditionalMassProperties::Mass(f32::EPSILON));

    commands.spawn(RigidBody::Fixed)
        .insert(SpatialBundle::from_transform(Transform::default()
            .with_translation(4. * Vec3::NEG_X)
            .with_scale(4. * Vec3::ONE)))
        .insert(Collider::convex_decomposition(
            &[Vec2::new(0., 0.), Vec2::new(1., 0.), Vec2::new(0., 1.), Vec2::new(-1., 0.), Vec2::new(0., -1.)],
            &[[0, 1], [1, 2], [2, 3], [3, 4], [4, 0]],
        ));
}

fn update(
    keys: Res<Input<KeyCode>>,
    mut query: Query<(&mut ExternalImpulse, &Velocity)>,
) {
    let (mut external_impulse, velocity) = query.single_mut();
    let max_angular_velocity = 64.;
    let forward = if keys.pressed(KeyCode::D) { -((velocity.angvel + max_angular_velocity) / max_angular_velocity).clamp(0., 1.) } else { 0. };
    let back = if keys.pressed(KeyCode::A) { ((max_angular_velocity - velocity.angvel) / max_angular_velocity).clamp(0., 1.) } else { 0. };
    let impulse = if keys.just_pressed(KeyCode::Space) { 64. * Vec2::Y } else { Vec2::ZERO };
    let torque_impulse = 8. * (forward + back);
    external_impulse.impulse = impulse;
    external_impulse.torque_impulse = torque_impulse;
}