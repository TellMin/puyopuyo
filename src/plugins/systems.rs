use crate::states::app::AppState;
use bevy::prelude::*;

pub struct SystemsPlugin;

impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, bevy::window::close_on_esc)
            .init_state::<AppState>();
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
