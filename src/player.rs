use bevy::prelude::*;
use crate::{ BOTTOM_WALL,  LEFT_WALL,  RIGHT_WALL, TOP_WALL, WALL_THICKNESS};
use crate::collisions::Collider;
use crate::sprites::{AnimationIndices, AnimationTimer};
const PLAYER_SPEED: f32 = 500.0;
const PLAYER_PADDING: f32 = 10.0;
const PLAYER_SIZE: Vec2 = Vec2::new(5.0, 8.0);

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_player)
            .add_systems(FixedUpdate,
                         move_player,
            )

        ;
    }
}

#[derive(Component)]
pub struct Player;


fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("chicken_sheet.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 2, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    // Use only the subset of sprites in the sheet that make up the run animation
    let animation_indices = AnimationIndices { first: 0, last: 1 };

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(animation_indices.first),
            transform: Transform::from_xyz(0.0, -250., 2.0),
            ..default()
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        Player,
        Collider,
    ));
}

fn move_player(
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
