use bevy::prelude::*;

use super::*;

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
    mut query: Query<(&mut Transform, &PlayerOb)>,
) {
    let mut dir_x = 0.0;
    let mut dir_z = 0.0;
    for ev in reader.iter(&events) {
        match ev {
            PlayerCommand::MoveRight => {
                dir_z += 1.0;
            }
            PlayerCommand::MoveLeft => {
                dir_z -= 1.0;
            }
            PlayerCommand::MoveForward => {
                dir_x += 1.0;
            }
            PlayerCommand::MoveBackward => {
                dir_x -= 1.0;
            }
            PlayerCommand::AttackPrimary => println!("Attacj"),
        }
    }

    for mut q in query.iter_mut() {
        q.0.translation.x += 0.01 * dir_x;
        q.0.translation.z += 0.01 * dir_z;
    }
}
