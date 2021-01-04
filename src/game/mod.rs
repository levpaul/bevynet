mod cam_ctrl;
mod player_ctrl;
mod fps_display;
mod scene;

use bevy::prelude::*;

pub struct Plugin;
impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(scene::setup.system())
            .add_plugin(cam_ctrl::Plugin::default())
            .add_plugin(fps_display::Plugin)
            .add_event::<PlayerCommand>()
            .add_system(bevy::input::system::exit_on_esc_system.system())
            .add_stage_after(
                stage::UPDATE,
                "client_stage",
                SystemStage::serial()
                    .with_system(player_ctrl::sys_user_input.system())
                    .with_system(player_ctrl::sys_player_cmds.system()),
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

pub struct PlayerOb {
    velocity: Vec3,
}

pub struct Tick {
    delta: u16,
}
