use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub mod game;

const PIPE_Z: f32 = 1.0;
const GROUND_Z: f32 = 2.0;
const BIRD_Z: f32 = 3.0;
const UI_Z: f32 = 4.0;

const BIRD_SIZE: Vec2 = Vec2::new(34.0, 24.0);
const PIPE_SIZE: Vec2 = Vec2::new(52.0, 320.0);
const GROUND_WIDTH: f32 = 336.0;
const GROUND_HEIGHT: f32 = 112.0;
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum GameState {
    #[default]
    Playing,
    GameOver,
}
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, game::bird::spawn_player)
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, game::bird::confine_player_movement)
        .add_systems(Startup, game::pipes::spawn_pipes)
        .run();
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 100.0),
        ..default()
    });
}

#[derive(Component)]
struct Ground;
#[derive(Component)]
struct Scroll;
