use bevy::prelude::*;
use crate::components::Player;
use crate::{BOTTOM_WALL, LEFT_WALL, PLAYER_PADDING, PLAYER_SIZE, PLAYER_SPEED, RIGHT_WALL, TOP_WALL, WALL_THICKNESS};

pub fn move_player(

    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    time_step: Res<FixedTime>,
) {
    let mut player_transform = query.single_mut();
    let mut horizontal = 0.0;
    let mut vertical = 0.0;

    if keyboard_input.pressed(KeyCode::Left) {
        horizontal -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::Right) {
        horizontal += 1.0;
    }
    if keyboard_input.pressed(KeyCode::Up) {
        vertical += 1.0;
    }
    if keyboard_input.pressed(KeyCode::Down) {
        vertical -= 1.0;
    }

    // Calculate the new horizontal player position based on player input
    let new_player_position_horizontal =
        player_transform.translation.x + horizontal * PLAYER_SPEED * time_step.period.as_secs_f32();

    let new_player_position_vertical =
        player_transform.translation.y + vertical * PLAYER_SPEED * time_step.period.as_secs_f32();
    // Update the player position,
    // making sure it doesn't cause the player to leave the arena
    let left_bound = LEFT_WALL + WALL_THICKNESS / 2.0 + PLAYER_SIZE.x / 2.0 + PLAYER_PADDING;
    let right_bound = RIGHT_WALL - WALL_THICKNESS / 2.0 - PLAYER_SIZE.x / 2.0 - PLAYER_PADDING;
    let top_bound = TOP_WALL - WALL_THICKNESS / 2.0 - PLAYER_SIZE.y / 2.0 - PLAYER_PADDING;
    let bottom_bound = BOTTOM_WALL + WALL_THICKNESS / 2.0 + PLAYER_SIZE.y / 2.0 + PLAYER_PADDING;

    player_transform.translation.x = new_player_position_horizontal.clamp(left_bound, right_bound);
    player_transform.translation.y = new_player_position_vertical.clamp(bottom_bound, top_bound);
}
