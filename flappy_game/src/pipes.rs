use bevy::prelude::*;
use bevy::window::PrimaryWindow;

#[derive(Component)]
pub struct Pipe {}

pub fn spawn_pipes(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            texture: asset_server.load("sprites/pipe.png"),
            ..default()
        },
        Pipe {},
    ));
}
