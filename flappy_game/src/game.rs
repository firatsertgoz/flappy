use std::process::Command;

use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::{
    game_over::DespawnOnReset, GameState, Ground, Scroll, BIRD_SIZE, BIRD_Z, GROUND_WIDTH,
};

use self::bird::Player;

const SCROLL_SPEED: f32 = 50.0;
const JUMP_AMOUNT: f32 = 1.5;
const FALL_SPEED: f32 = 5.0;
const FALL_VELOCITY_LIMIT: f32 = -2.0;
const MOVE_SPEED: f32 = 200.0;
const DEATH_HEIGHT: f32 = -125.0;
const PIPE_SPAWN_OFFSET: f32 = 180.0;
const PIPE_SPAWN_TIME: f32 = 4.0;
const PIPE_DESPAWN_OFFSET: f32 = 360.0;
const GAP_HEIGHT: f32 = 100.0;
const BIRD_ANIMATION_SPEED: f32 = 10.0;

pub mod bird;
pub mod pipes;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum PlayState {
    #[default]
    Normal,
    Fail,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<PlayState>()
            .init_resource::<Score>()
            .insert_resource(PipeSpawnTimer(Timer::from_seconds(
                PIPE_SPAWN_TIME,
                TimerMode::Repeating,
            )))
            .add_systems(OnEnter(GameState::Playing), game_setup)
            .add_systems(OnExit(GameState::Playing), (reset_score, reset_timer))
            .add_systems(
                Update,
                (
                    bird::jump,
                    bird::animate_bird,
                    pipes::spawn_pipe,
                    pipes::despawn_pipe,
                    pipes::move_pipes,
                    pipes::check_if_passed_pipes,
                    pipes::check_pipe_collision,
                    scroll,
                    update_score_text,
                    reuse_ground,
                )
                    .run_if(in_state(GameState::Playing).and_then(in_state(PlayState::Normal))),
            )
            .add_systems(
                Update,
                (bird::fall, bird::move_player, check_death).run_if(in_state(GameState::Playing)),
            );
    }
}

fn game_setup(
    mut commands: Commands,
    mut play_state: ResMut<NextState<PlayState>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
) {
    let bird = asset_server.load("sprites/bird.png");
    let texture_atlas =
        texture_atlases.add(TextureAtlas::from_grid(bird, BIRD_SIZE, 4, 1, None, None));

    // Spawn the bird
    commands.spawn((
        Player::default(),
        DespawnOnReset,
        SpriteSheetBundle {
            texture_atlas,
            transform: Transform::from_xyz(0.0, 100.0, BIRD_Z),
            ..Default::default()
        },
    ));
    play_state.set(PlayState::Normal);
}

#[derive(Resource, Default)]
pub struct Score(usize);

#[derive(Component)]
struct ScoreText;

#[derive(Resource)]
struct PipeSpawnTimer(Timer);

#[derive(Component)]
struct Pipe;

fn update_score_text(mut query: Query<&mut Text, With<ScoreText>>, score: Res<Score>) {
    if !score.is_changed() {
        return;
    }

    for mut text in &mut query {
        text.sections[0].value = score.0.to_string();
    }
}

// Scroll all entities with the Scroll component
fn scroll(mut query: Query<&mut Transform, With<Scroll>>, time: Res<Time>) {
    for mut transform in &mut query {
        transform.translation.x -= SCROLL_SPEED * time.delta_seconds();
    }
}

fn reuse_ground(mut query: Query<&mut Transform, With<Ground>>) {
    for mut transform in &mut query {
        if transform.translation.x < -GROUND_WIDTH {
            transform.translation.x += GROUND_WIDTH * 2.0;
        }
    }
}

// End the game if the bird is below the death height
fn check_death(bird: Query<&Transform, With<Player>>, mut state: ResMut<NextState<GameState>>) {
    for bird in &bird {
        if bird.translation.y < DEATH_HEIGHT {
            state.set(GameState::GameOver);
        }
    }
}

fn reset_score(mut score: ResMut<Score>) {
    score.0 = 0;
}

fn reset_timer(mut timer: ResMut<PipeSpawnTimer>) {
    timer.0.reset();
}
