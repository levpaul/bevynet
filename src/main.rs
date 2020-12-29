mod game;

use bevy::prelude::*;

struct MyEvent;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(game::Plugin)
        .run();
}
