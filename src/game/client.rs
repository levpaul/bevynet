use bevy::prelude::*;
use orbit::OrbitCamera;

use super::*;
use bevy::render::camera::Camera;

pub fn sys_user_input(
    mut player_commands: ResMut<Events<PlayerCommand>>,
    kbi: Res<Input<KeyCode>>,
) {
    if kbi.pressed(KeyCode::W) {
        player_commands.send(PlayerCommand::MoveForward);
    }
    if kbi.pressed(KeyCode::A) {
        player_commands.send(PlayerCommand::MoveLeft);
    }
    if kbi.pressed(KeyCode::S) {
        player_commands.send(PlayerCommand::MoveBackward);
    }
    if kbi.pressed(KeyCode::D) {
        player_commands.send(PlayerCommand::MoveRight);
    }
    if kbi.just_pressed(KeyCode::Space) {
        player_commands.send(PlayerCommand::AttackPrimary);
    }
}

pub fn sys_player_cmds(
    events: Res<Events<PlayerCommand>>,
    mut reader: Local<EventReader<PlayerCommand>>,
    mut player_query: Query<(&mut Transform, &mut PlayerOb)>,
    camera_query: Query<(&Transform, &Camera, &OrbitCamera)>,
) {
    let mut dir_x = 0.0;
    let mut dir_z = 0.0;
    for ev in reader.iter(&events) {
        match ev {
            PlayerCommand::MoveRight => {
                dir_x += 1.0;
            }
            PlayerCommand::MoveLeft => {
                dir_x -= 1.0;
            }
            PlayerCommand::MoveForward => {
                dir_z -= 1.0;
            }
            PlayerCommand::MoveBackward => {
                dir_z += 1.0;
            }
            PlayerCommand::AttackPrimary => println!("Attacj"),
        }
    }

    let mut cam_f = Vec3::default();
    for q in camera_query.iter() {
        cam_f = q.0.forward();
    }

    for mut q in player_query.iter_mut() {
        q.1.velocity *= 0.9; // decel
        let new_force = Vec3::from((0.01 * dir_x, 0.0, 0.01 * dir_z));

        let cam_angle = f32::atan2(cam_f.x, cam_f.z);
        let new_q = Quat::from_axis_angle(Vec3::new(0., 1., 0.), cam_angle);

        q.1.velocity += new_q.mul_vec3(new_force);
        q.0.translation += q.1.velocity;
    }
}
