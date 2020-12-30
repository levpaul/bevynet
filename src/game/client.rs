use bevy::prelude::*;

use super::*;
use bevy::render::camera::Camera;

// pub fn sys_input_processing(kbi: Res<Input<KeyCode>>) {
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
    camera_query: Query<(&Transform, &Camera)>,
) {
    let mut dir_x = 0.0;
    let mut dir_z = 0.0;
    for ev in reader.iter(&events) {
        match ev {
            PlayerCommand::MoveRight => {
                dir_x -= 1.0;
            }
            PlayerCommand::MoveLeft => {
                dir_x += 1.0;
            }
            PlayerCommand::MoveForward => {
                dir_z += 1.0;
            }
            PlayerCommand::MoveBackward => {
                dir_z -= 1.0;
            }
            PlayerCommand::AttackPrimary => println!("Attacj"),
        }
    }

    // this.oC.update();
    // let timeDelta: number = clock.getDelta();
    // let cameraDirection: Vector3 = this.oC.object.getWorldDirection(new Vector3());
    // let camAngle: number = Math.atan2(cameraDirection.x, cameraDirection.z);
    //     this.velocity.x -= this.velocity.x * config.moveDecel * timeDelta;
    //     this.direction.z = Number(this.moveForward) - Number(this.moveBackward);
    //     this.direction.normalize(); // this ensures consistent movements in all directions - does it??
    //     let newForce:Vector3 = new Vector3();
    //     if (this.moveForward || this.moveBackward) newForce.z += this.direction.z * config.moveSpeed * timeDelta;
    //
    //     newForce.applyAxisAngle(new Vector3(0,1,0), camAngle);
    //	applyAxisAngle( axis, angle ) {
    // return this.applyQuaternion( _quaternion.setFromAxisAngle( axis, angle ) );
    // Vec3.applyQuaternion https://github.com/mrdoob/three.js/blob/4b1a71aae70a2c730c56e928a6a08a5569796b41/src/math/Vector3.js#L279
    // Quat.setFromAxisAngle: https://github.com/mrdoob/three.js/blob/4b1a71aae70a2c730c56e928a6a08a5569796b41/src/math/Quaternion.js#L267
    // }
    //     this.velocity.add(newForce);
    //     this.game.char.position.add(this.velocity);

    let mut cam_q = Quat::default();
    for q in camera_query.iter() {
        cam_q = q.0.rotation;
    }

    for mut q in player_query.iter_mut() {
        q.1.velocity *= 0.9;

        let cam_angle = f32::atan2(cam_q.x, cam_q.z);
        let new_q = Quat::from_axis_angle(Vec3::new(0., 1., 0.), cam_angle);

        let new_force = Vec3::from((0.01 * dir_x, 0.0, 0.01 * dir_z));
        let new_force = new_q.mul_vec3(new_force);
        q.1.velocity += new_force;
        q.0.translation += q.1.velocity;
    }
}
