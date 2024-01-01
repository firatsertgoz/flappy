use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::{BIRD_ANIMATION_SPEED, FALL_SPEED, FALL_VELOCITY_LIMIT, JUMP_AMOUNT};
pub const PLAYER_SIZE: f32 = 64.0;
pub const PLAYER_SPEED: f32 = 500.0;

#[derive(Component, Default)]
pub struct Player {
    velocity: f32,
}

pub fn jump(
    keyboard_input: Res<Input<KeyCode>>,
    mut player: Query<&mut Player, With<Player>>,
    time: Res<Time>,
) {
    for mut player in &mut player {
        if keyboard_input.just_pressed(KeyCode::Space) {
            player.velocity = JUMP_AMOUNT;
        }
    }
}

pub fn fall(mut player: Query<&mut Player, With<Player>>, time: Res<Time>) {
    for mut player in &mut player {
        player.velocity -= FALL_SPEED * time.delta_seconds();
        player.velocity = player.velocity.max(FALL_VELOCITY_LIMIT);
    }
}

pub fn move_player(mut player: Query<(&mut Transform, &Player), With<Player>>, time: Res<Time>) {
    for (mut transform, player) in &mut player {
        transform.translation.y += player.velocity * time.delta_seconds();
    }
}

pub(super) fn animate_bird(
    mut player: Query<&mut TextureAtlasSprite, With<Player>>,
    time: Res<Time>,
) {
    for mut player in &mut player {
        player.index = (time.elapsed_seconds() * BIRD_ANIMATION_SPEED) as usize % 4;
    }
}

pub fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    if let Ok(mut transform) = player_query.get_single_mut() {
        let half_player_size = PLAYER_SIZE / 2.0;
        let x_min = half_player_size;
        let x_max = window.width() - half_player_size;
        let y_min = half_player_size;
        let y_max = window.height() - half_player_size;

        if transform.translation.x < x_min {
            transform.translation.x = x_min;
        }
        if transform.translation.x > x_max {
            transform.translation.x = x_max;
        }
        if transform.translation.y < y_min {
            transform.translation.y = y_min;
        }
        if transform.translation.y > y_max {
            transform.translation.y = y_max;
        }
    }
}
