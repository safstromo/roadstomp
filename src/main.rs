use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

const PLAYER_SPEED: f32 = 500.0;
const PLAYER_PADDING: f32 = 10.0;
const PLAYER_SIZE: Vec2 = Vec2::new(6.0, 20.0);
const CAR_SIZE: Vec2 = Vec2::new(20.0, 50.0);
const WALL_THICKNESS: f32 = 10.0;

// x coordinates
const LEFT_WALL: f32 = -450.;
const RIGHT_WALL: f32 = 450.;
// y coordinates
const BOTTOM_WALL: f32 = -300.;
const TOP_WALL: f32 = 300.;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, (move_player,spawn_car))

        .run();
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Car;


fn setup(
    mut commands: Commands,
) {
    commands.spawn(Camera2dBundle::default());

    // Rectangle
    commands.spawn((SpriteBundle {
        sprite: Sprite {
            color: Color::BLACK,
            custom_size: Some(PLAYER_SIZE),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(-50., 0., 0.)),
        ..default()
    }, Player));
}

fn spawn_car(
    mut commands: Commands
) {
    commands.spawn(
        (SpriteBundle {
            sprite: Sprite {
                color: Color::BLACK,
                custom_size: Some(CAR_SIZE),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(-50., 50., 1.)),
            ..default()
        }, Car));
}


fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    time_step: Res<FixedTime>,
) {
    let mut player_transform = query.single_mut();
    let mut direction = 0.0;

    if keyboard_input.pressed(KeyCode::Left) {
        direction -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::Right) {
        direction += 1.0;
    }

    // Calculate the new horizontal player position based on player input
    let new_player_position =
        player_transform.translation.x + direction * PLAYER_SPEED * time_step.period.as_secs_f32();

    // Update the player position,
    // making sure it doesn't cause the player to leave the arena
    let left_bound = LEFT_WALL + WALL_THICKNESS / 2.0 + PLAYER_SIZE.x / 2.0 + PLAYER_PADDING;
    let right_bound = RIGHT_WALL - WALL_THICKNESS / 2.0 - PLAYER_SIZE.x / 2.0 - PLAYER_PADDING;

    player_transform.translation.x = new_player_position.clamp(left_bound, right_bound);
}
