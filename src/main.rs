use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rand::Rng;

const PLAYER_SPEED: f32 = 500.0;
const PLAYER_PADDING: f32 = 10.0;
const PLAYER_SIZE: Vec2 = Vec2::new(5.0, 8.0);
const CAR_SIZE: Vec2 = Vec2::new(20.0, 50.0);
const INITIAL_CAR_DIRECTION: Vec2 = Vec2::new(0.0, -0.5);
const CAR_SPEED: f32 = 400.0;
const WALL_THICKNESS: f32 = 10.0;

// x coordinates
const LEFT_WALL: f32 = -450.;
const RIGHT_WALL: f32 = 450.;
// y coordinates
const BOTTOM_WALL: f32 = -300.;
const TOP_WALL: f32 = 300.;


fn main() {
    App::new()
        .insert_resource(SpawnTimer(Timer::from_seconds(0.1, TimerMode::Repeating)))
        .add_plugins((DefaultPlugins, ))
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, (move_player, spawn_car, apply_velocity))

        .run();
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Car;

#[derive(Resource)]
struct SpawnTimer(Timer);

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

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
        transform: Transform::from_translation(Vec3::new(-50., BOTTOM_WALL + 20.0, 0.)),
        ..default()
    }, Player));
}

fn spawn_car(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<SpawnTimer>,
) {
    let mut rng = rand::thread_rng();
    if timer.0.tick(time.delta()).just_finished() {
        commands.spawn(
            (SpriteBundle {
                sprite: Sprite {
                    color: Color::BLACK,
                    custom_size: Some(CAR_SIZE),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(rng.gen_range(LEFT_WALL..RIGHT_WALL), TOP_WALL, 1.)),
                ..default()
            }, Car,
             Velocity(INITIAL_CAR_DIRECTION.normalize() * CAR_SPEED)
            ));
    }
}

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time_step: Res<FixedTime>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * time_step.period.as_secs_f32();
        transform.translation.y += velocity.y * time_step.period.as_secs_f32();
    }
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
