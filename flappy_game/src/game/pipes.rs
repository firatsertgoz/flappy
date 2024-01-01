use super::PipeSpawnTimer;
use super::{bird::Player, Pipe, PlayState, Scroll, GAP_HEIGHT, PIPE_SPAWN_OFFSET, SCROLL_SPEED};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub(super) fn spawn_pipe(
    mut commands: Commands,
    mut timer: ResMut<PipeSpawnTimer>,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    timer.0.tick(time.delta());

    if !timer.0.finished() {
        return;
    }

    let sprite = asset_server.load("sprites/pipe.png");

    commands.spawn((
        Pipe,
        Scroll,
        SpriteBundle {
            texture: sprite,
            transform: Transform::from_xyz(PIPE_SPAWN_OFFSET, 0.0, 0.0),
            ..Default::default()
        },
    ));
}

pub(super) fn despawn_pipe(mut commands: Commands, query: Query<(Entity, &Transform), With<Pipe>>) {
    for (entity, transform) in &query {
        if transform.translation.x < -PIPE_SPAWN_OFFSET {
            commands.entity(entity).despawn();
        }
    }
}

pub(super) fn move_pipes(mut pipe_query: Query<&mut Transform, With<Pipe>>, time: Res<Time>) {
    for mut transform in &mut pipe_query {
        transform.translation.x -= SCROLL_SPEED * time.delta_seconds();
    }
}

pub(super) fn check_if_passed_pipes(
    mut commands: Commands,
    mut score: ResMut<Score>,
    mut pipe_query: Query<&mut Transform, With<Pipe>>,
    player_query: Query<&Transform, With<Player>>,
) {
    for mut transform in &mut pipe_query {
        for player_transform in player_query.iter() {
            if transform.translation.x < player_transform.translation.x {
                score.0 += 1;
            }
        }
    }
}

pub(super) fn check_collision(
    mut commands: Commands,
    mut state: ResMut<NextState<PlayState>>,
    mut pipe_query: Query<&mut Transform, With<Pipe>>,
    player_query: Query<&Transform, With<Player>>,
) {
    for mut transform in &mut pipe_query {
        for player_transform in player_query.iter() {
            if transform.translation.x < player_transform.translation.x {
                let player_x = player_transform.translation.x;
                let player_y = player_transform.translation.y;
                let pipe_x = transform.translation.x;
                let pipe_y = transform.translation.y;

                if (player_x - pipe_x).abs() < 32.0 && (player_y - pipe_y).abs() > GAP_HEIGHT / 2.0
                {
                    state.set(PlayState::Fail);
                }
            }
        }
    }
}
