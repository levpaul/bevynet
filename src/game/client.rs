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
) {
    for ev in reader.iter(&events) {
        match ev {
            PlayerCommand::MoveRight => {
                println!("right")
            }
            PlayerCommand::MoveLeft => {
                println!("left")
            }
            PlayerCommand::MoveForward => {
                println!("forward")
            }
            PlayerCommand::MoveBackward => {
                println!("backawr")
            }
            PlayerCommand::AttackPrimary => println!("Attacj"),
        }
    }
}
