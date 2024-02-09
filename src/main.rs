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
    value: Option<usize>,
    children: Vec<Octree>,
}

fn startup_octree(mut commands: Commands) {
    commands.spawn(build_octree());
}

fn build_octree() -> Octree {
    let size = Vec2::new(1., 1.);
    let offset = Vec2::new(0., 0.);

    let mut octrees = vec![];
    for x in -1..1 {
        for y in -1..1 {
            let sub_size = 0.5 * size;
            let sub_offset = offset + 0.5 * sub_size + Vec2::new(x as f32, y as f32) * sub_size;
            octrees.push(Octree {
                x: sub_offset.x,
                y: sub_offset.y,
                width: sub_size.x,
                height: sub_size.y,
                value: Some(match simplex_noise_2d(sub_offset) {
                    value if value > 0. => 1,
                    _ => 0
                }),
                children: vec![],
            });
        }
    }

    for octree in &octrees {
        if octree.value != octrees[0].value {
            return Octree {
                x: offset.x,
                y: offset.y,
                width: size.x,
                height: size.y,
                value: None,
                children: octrees,
            };
        }
    }


    return Octree {
        x: 0.,
        y: 0.,
        width: 1.,
        height: 1.,
        value: octrees[0].value,
        children: vec![],
    };
}

fn update_octree(
    query: Query<&Octree>,
    mut gizmos: Gizmos,
) {
    for octree in query.iter() {
        show_octree(octree, &mut gizmos);
    }
}

fn show_octree(octree: &Octree, gizmos: &mut Gizmos) {
    if octree.children.is_empty() {
        gizmos.rect_2d(
            Vec2::new(octree.x, octree.y),
            0.,
            Vec2::new(octree.width, octree.height),
            match octree.value {
                Some(0) => Color::Hsla {
                    hue: 0.,
                    saturation: 0.,
                    lightness: 1.,
                    alpha: 1.,
                },
                Some(1) => Color::Hsla {
                    hue: 0.,
                    saturation: 1.,
                    lightness: 0.25,
                    alpha: 1.,
                },
                _ => Color::PINK,
            },
        );
    } else {
        octree.children.iter().for_each(|child| show_octree(child, gizmos));
    }
}
