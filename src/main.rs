use bevy::prelude::*;
use puyopuyo::plugins::{gameplay, systems, title};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(systems::SystemsPlugin)
        .add_plugins(title::TitlePlugin)
        .add_plugins(gameplay::GameplayPlugin)
        .run();
}
