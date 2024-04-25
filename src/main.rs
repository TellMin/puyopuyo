use bevy::prelude::*;
use puyopuyo::plugins::gameplay;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(gameplay::GameplayPlugin)
        .run();
}
