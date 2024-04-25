use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_puyo);
    }
}

fn spawn_puyo(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Circle::default()).into(),
        material: materials.add(Color::rgb(1.0, 0.5, 0.5)),
        transform: Transform::from_translation(Vec3::new(0.0, -50.0, 1.0))
            .with_scale(Vec2::splat(30.).extend(1.)),
        ..default()
    });
}
