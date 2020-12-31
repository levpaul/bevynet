use bevy::prelude::*;

use super::*;

/// set up a simple 3D scene
pub fn setup(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let (rows, cols) = (10, 10);
    let size = 2.0;
    let max_shade = (rows * cols * 1) as f32;
    for i in 0..rows {
        for j in 0..cols {
            let cur_shade = (cols * j + i) as f32;
            commands.spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Plane { size })),
                transform: Transform::from_translation(Vec3::new(
                    (-rows + 1) as f32 + i as f32 * size,
                    0.0,
                    (-cols + 1) as f32 + j as f32 * size,
                )),
                material: materials.add(
                    Color::rgb(
                        cur_shade / max_shade,
                        cur_shade / max_shade,
                        cur_shade / max_shade,
                    )
                    .into(),
                ),
                ..Default::default()
            });
        }
    }

    // add entities to the world
    commands
        // light
        .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
            ..Default::default()
        })
        // cube (player)
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 2.0 })),
            material: materials.add(Color::rgb(0.1, 0.2, 0.6).into()),
            transform: Transform::from_translation(Vec3::new(0.0, 1.0, 0.0)),
            ..Default::default()
        })
        .with(PlayerOb {
            velocity: Vec3::default(),
        })
        .with_children(|p| {
            p.spawn(Camera3dBundle::default())
                .with(cam_ctrl::PrimaryCameraHook::default());
        });
}
