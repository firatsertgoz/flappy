use crate::{BIRD_SIZE, PIPE_SIZE};

use super::{bird::Player, Pipe, PlayState, Scroll, GAP_HEIGHT, PIPE_SPAWN_OFFSET, SCROLL_SPEED};
use super::{PipeSpawnTimer, Score, PIPE_DESPAWN_OFFSET};
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use bevy::window::PrimaryWindow;
use rand::Rng;

pub(super) fn spawn_pipe(
    mut commands: Commands,
    mut timer: ResMut<PipeSpawnTimer>,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
) {
    timer.0.tick(time.delta());

    if !timer.0.finished() {
        return;
    }
    let mut rng = rand::thread_rng();
    let y = rng.gen_range(-50.0..50.0);
    let sprite = asset_server.load("sprites/pipe.png");

    commands.spawn((
        Pipe,
        Scroll,
        SpriteBundle {
            texture: sprite,
            transform: Transform::from_xyz(PIPE_SPAWN_OFFSET, y - 160.0, 0.0),
            ..Default::default()
        },
    ));
}

pub(super) fn despawn_pipe(mut commands: Commands, query: Query<(Entity, &Transform), With<Pipe>>) {
    for (entity, transform) in &query {
        if transform.translation.x < -PIPE_DESPAWN_OFFSET {
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
    pipes: Query<(Entity, &Transform), With<Pipe>>,
    bird: Query<&Transform, With<Player>>,
) {
    let bird = bird.single();
    for (entity, pipe) in &pipes {
        if pipe.translation.x + PIPE_SIZE.x / 2.0 < bird.translation.x - BIRD_SIZE.x / 2.0 {
            commands.entity(entity).despawn();
            score.0 += 1;
            break;
        }
    }
}

pub(super) fn check_pipe_collision(
    mut play_state: ResMut<NextState<PlayState>>,
    bird: Query<&Transform, With<Player>>,
    pipes: Query<&Transform, With<Pipe>>,
) {
    let bird = bird.single();
    for pipe in &pipes {
        if collide(bird.translation, BIRD_SIZE, pipe.translation, PIPE_SIZE).is_some() {
            play_state.set(PlayState::Fail);
        }
    }
}
