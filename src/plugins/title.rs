use crate::states::app::AppState;
use bevy::prelude::*;

pub struct TitlePlugin;

impl Plugin for TitlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Title), setup)
            .add_systems(Update, play_game.run_if(in_state(AppState::Title)))
            .add_systems(OnExit(AppState::Title), cleanup);
    }
}

const BUTTON_COLOR: Color = Color::rgb(0.08, 0.9, 0.08);
const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

#[derive(Resource)]
pub struct TitleData {
    title_entity: Entity,
}

fn setup(mut commands: Commands) {
    let frame_style = Style {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        ..default()
    };
    let button_style = Style {
        width: Val::Px(250.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let button_text_style = TextStyle {
        font_size: 40.0,
        color: TEXT_COLOR,
        ..default()
    };
    let title_entity = commands
        .spawn(NodeBundle {
            style: frame_style.clone(),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: button_style.clone(),
                    background_color: BUTTON_COLOR.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section("Start", button_text_style.clone()));
                });
        })
        .id();
    commands.insert_resource(TitleData { title_entity });
}

fn cleanup(mut commands: Commands, title_data: Res<TitleData>) {
    commands.entity(title_data.title_entity).despawn_recursive();
}

fn play_game(
    query: Query<&Interaction, (Changed<Interaction>, With<Button>)>,
    mut state: ResMut<NextState<AppState>>,
) {
    for interaction in query.iter() {
        if let Interaction::Pressed = interaction {
            state.set(AppState::Gameplay);
        }
    }
}
