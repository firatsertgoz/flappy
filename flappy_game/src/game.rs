use bevy::prelude::*;
use bevy::window::PrimaryWindow;

const SCROLL_SPEED: f32 = 50.0;
const JUMP_AMOUNT: f32 = 1.5;
const FALL_SPEED: f32 = 5.0;
const FALL_VELOCITY_LIMIT: f32 = -2.0;
const MOVE_SPEED: f32 = 200.0;
const DEATH_HEIGHT: f32 = -125.0;
const PIPE_SPAWN_OFFSET: f32 = 180.0;
const PIPE_SPAWN_TIME: f32 = 4.0;
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
            .add_systems(OnExit(Playstate::Playing), reset_score, reset_timer)
            .add_systems(OnEnter(PlayState::Fail), game_over)
            .add_systems(Update,(
                bird::jump,
                bird::fall,
                bird::move_player,
                bird::animate_bird,
                pipes::spawn_pipe,
                pipes::despawn_pipe,
                pipes::move_pipes,
                pipes::check_if_passed_pipes,
                scroll,
                update_score_text,
            ))
            ))
    }
}

#[derive(Resource, Default)]
pub struct Score(usize);

#[derive(Component)]
struct ScoreText;

#[derive(Resource)]
struct PipeSpawnTimer(Timer);

#[derive(Component)]
struct Pipe;
