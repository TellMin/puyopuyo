use bevy::prelude::*;
use puyopuyo::plugins::{gameplay, systems};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(systems::SystemsPlugin)
        .add_plugins(gameplay::GameplayPlugin)
        .run();
}
