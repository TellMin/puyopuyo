use crate::states::app::AppState;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Gameplay), spawn_puyo)
            .add_systems(Update, fall_puyo.run_if(in_state(AppState::Gameplay)))
            .add_systems(OnExit(AppState::Gameplay), remove_entity);
    }
}

#[derive(Component)]
struct Puyo;

#[derive(Resource)]
pub struct GameData {
    gameplay_entity: Entity,
}

fn spawn_puyo(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let puyo_entity = commands
        .spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(Circle::default()).into(),
                material: materials.add(Color::rgb(1.0, 0.5, 0.5)),
                transform: Transform::from_translation(Vec3::new(0.0, -50.0, 1.0))
                    .with_scale(Vec2::splat(30.).extend(1.)),
                ..default()
            },
            Puyo,
        ))
        .id();

    commands.insert_resource(GameData {
        gameplay_entity: puyo_entity,
    });
}

// ぷよを落下させる
fn fall_puyo(mut query: Query<&mut Transform, With<Puyo>>) {
    for mut transform in query.iter_mut() {
        if transform.translation.y > -200.0 {
            transform.translation.y -= 1.0;
        }
    }
}

// Entityを削除する
fn remove_entity(mut commands: Commands, game_data: Res<GameData>) {
    commands.entity(game_data.gameplay_entity).despawn();
}
