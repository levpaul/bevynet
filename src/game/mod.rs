mod cam_ctrl;
mod client;
mod fps;
mod orbit;
mod scene;

use bevy::prelude::*;
use cam_ctrl::PluginParams;

pub struct Plugin;
impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(scene::setup.system())
            .add_plugin(cam_ctrl::Plugin {
                params: PluginParams { x: 45 },
            })
            .add_plugin(fps::Plugin { font_size: 30.0 })
            .add_event::<PlayerCommand>()
            .add_event::<Tick>()
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

pub struct PlayerOb {
    velocity: Vec3,
}

pub struct Tick {
    delta: u16,
}
