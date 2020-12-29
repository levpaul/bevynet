use bevy::prelude::*;

use super::*;

/// set up a simple 3D scene
pub fn setup(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // add entities to the world
    commands
        // plane
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..Default::default()
        })
        // light
        .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
            ..Default::default()
        })
        // cube (player)
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_translation(Vec3::new(0.0, 0.5, 0.0)),
            ..Default::default()
        })
        .with(PlayerOb {
            velocity: Vec3::default(),
            camera_angle: Vec3::default(),
        })
        .with_children(|p| {
            p.spawn(Camera3dBundle {
                transform: Transform::from_translation(Vec3::new(-2.0, 2.5, 5.0))
                    // global_transform: GlobalTransform::from_translation(Vec3::new(-2.0, 2.5, 5.0))
                    .looking_at(Vec3::default(), Vec3::unit_y()),
                ..Default::default()
            })
            .with(orbit::OrbitCamera::default());
        });
    // .spawn(
    //     Camera3dBundle {
    //         transform: Transform::from_translation(Vec3::new(-2.0, 2.5, 5.0))
    //             .looking_at(Vec3::default(), Vec3::unit_y()),
    //         ..Default::default()
    //     }, // orbit::OrbitCamera::default(),
    // )
    // .with(orbit::OrbitCamera::default());
}
