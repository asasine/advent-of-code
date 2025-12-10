//! Visualize a 2D polyline read from points on stdin.
//!
//! Each line of input should contain two comma-separated floating point numbers.

use bevy::{
    camera::ScalingMode,
    math::bounding::{Bounded2d, BoundingVolume},
    prelude::*,
};
use std::io::stdin;

fn main() {
    let shape = polyline2d(stdin().lock());
    let setup = move |mut commands: Commands,
                      mut meshes: ResMut<Assets<Mesh>>,
                      mut materials: ResMut<Assets<ColorMaterial>>| {
        let aabb = shape.aabb_2d(Isometry2d::IDENTITY);
        commands.spawn((
            Camera2d,
            Transform::from_translation(aabb.center().extend(0.0)),
            Projection::Orthographic(OrthographicProjection {
                scaling_mode: ScalingMode::AutoMin {
                    min_width: (aabb.max.x - aabb.min.x).abs() * 1.1,
                    min_height: (aabb.max.y - aabb.min.y).abs() * 1.1,
                },
                ..OrthographicProjection::default_2d()
            }),
        ));
        info!(?aabb);
        let shape = meshes.add(shape.clone());
        commands.spawn((
            Mesh2d(shape),
            MeshMaterial2d(materials.add(Color::linear_rgb(1.0, 0.0, 0.0))),
        ));
    };

    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn polyline2d(input: impl std::io::BufRead) -> Polyline2d {
    let mut points: Vec<Vec2> = input
        .lines()
        .flatten()
        .map(|line| {
            let mut coords = line.split(',').map(str::trim).flat_map(str::parse);
            let x = coords.next().unwrap_or(0.0);
            let y = coords.next().unwrap_or(0.0);
            Vec2::new(x, y)
        })
        .collect();

    if let Some(first) = points.first()
        && let Some(last) = points.last()
        && first != last
    {
        points.push(*first); // close the loop
    }

    Polyline2d::new(points)
}
