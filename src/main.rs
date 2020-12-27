mod game;

use bevy::prelude::*;
use bevy_orbit_controls::{OrbitCamera, OrbitCameraPlugin};

struct MyEvent;

fn main() {
    App::build()
        .add_plugin(bevy_orbit_controls::OrbitCameraPlugin)
        .add_plugins(DefaultPlugins)
        .add_plugin(game::Plugin)
        .run();
}
