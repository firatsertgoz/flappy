pub mod bird;
pub mod pipes;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, bird::spawn_player)
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, bird::player_movement)
        .add_systems(Update, bird::confine_player_movement)
        .add_systems(Startup, pipes::spawn_pipes)
        .run();
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 100.0),
        ..default()
    });
}
