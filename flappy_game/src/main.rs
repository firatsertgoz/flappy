use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowResolution};

pub mod game;
mod game_over;

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
    Paused,
    Playing,
    GameOver,
}
fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Flappy Bird"),
                        resolution: WindowResolution::new(288.0, 512.0),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_state::<GameState>()
        .add_systems(Startup, scene_setup)
        .add_systems(Update, unpause)
        .add_plugins((game::GamePlugin, game_over::GameOverPlugin))
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

fn scene_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn a 2D camera
    commands.spawn(Camera2dBundle::default());

    // Spawn the background sprite
    commands.spawn(SpriteBundle {
        texture: asset_server.load("sprites/background.png"),
        ..Default::default()
    });

    // Spawn 2 ground sprites so that they can scroll infinitely
    let texture_handle = asset_server.load("sprites/ground.png");
    for i in 0..2 {
        commands.spawn((
            Ground,
            Scroll,
            SpriteBundle {
                texture: texture_handle.clone(),
                transform: Transform::from_xyz(i as f32 * GROUND_WIDTH, -200.0, GROUND_Z),
                ..Default::default()
            },
        ));
    }
}

pub fn cleanup<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn unpause(mut state: ResMut<NextState<GameState>>, keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        state.set(GameState::Playing);
    }
}
