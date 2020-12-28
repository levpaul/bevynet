mod client;
mod scene;

use bevy::prelude::*;

pub struct Plugin;
impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(scene::setup.system())
            .add_event::<PlayerCommand>()
            .add_system(bevy::input::system::exit_on_esc_system.system())
            .add_stage_after(
                stage::UPDATE,
                "client_stage",
                SystemStage::serial()
                    .with_system(client::sys_user_input.system())
                    .with_system(client::sys_player_cmds.system()),
            );
    }
}

pub enum PlayerCommand {
    MoveForward,
    MoveRight,
    MoveBackward,
    MoveLeft,
    AttackPrimary,
}

pub struct PlayerOb;
