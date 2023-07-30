use bevy::{prelude::*,
           sprite::collide_aabb::{collide, Collision},
           sprite::MaterialMesh2dBundle};
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

const SCOREBOARD_FONT_SIZE: f32 = 40.0;
const SCOREBOARD_TEXT_PADDING: Val = Val::Px(5.0);
const TEXT_COLOR: Color = Color::rgb(0.5, 0.5, 1.0);
const SCORE_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);

fn main() {
    App::new()
        .insert_resource(SpawnTimer(Timer::from_seconds(0.1, TimerMode::Repeating)))
        .insert_resource(Scoreboard { score: 0 })
        .add_event::<CollisionEvent>()
        .add_plugins((DefaultPlugins, ))
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, (
            spawn_car,
            check_for_collisions,
            apply_velocity.before(check_for_collisions),
            move_player
                .before(check_for_collisions)
                .after(apply_velocity),
        ))
        .add_systems(Update, (update_scoreboard, bevy::window::close_on_esc))
        .run();
}

#[derive(Component)]
struct Player;

#[derive(Component, Debug)]
struct Car;

#[derive(Resource)]
struct SpawnTimer(Timer);

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Resource)]
struct Scoreboard {
    score: usize,
}

#[derive(Component)]
struct Collider;

#[derive(Event, Default)]
struct CollisionEvent;


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
    },
                    Player,
                    Collider,
    ));
    commands.spawn(
        TextBundle::from_sections([
            TextSection::new(
                "Score: ",
                TextStyle {
                    font_size: SCOREBOARD_FONT_SIZE,
                    color: TEXT_COLOR,
                    ..default()
                },
            ),
            TextSection::from_style(TextStyle {
                font_size: SCOREBOARD_FONT_SIZE,
                color: SCORE_COLOR,
                ..default()
            }),
        ])
            .with_style(Style {
                position_type: PositionType::Absolute,
                top: SCOREBOARD_TEXT_PADDING,
                left: SCOREBOARD_TEXT_PADDING,
                ..default()
            }),
    );
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
             Velocity(INITIAL_CAR_DIRECTION.normalize() * CAR_SPEED),
             Collider
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

fn check_for_collisions(
    mut commands: Commands,
    mut scoreboard: ResMut<Scoreboard>,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    collider_query: Query<(Entity, &Transform), With<Car>>,
    mut collision_events: EventWriter<CollisionEvent>,
) {
    let (player_entity, player_transform) = player_query.single_mut();


    for (car_entity, car_transform) in collider_query.iter() {
        if collide(player_transform.translation,
                   PLAYER_SIZE,
                   car_transform.translation,
                   CAR_SIZE).is_some() {
            scoreboard.score += 1;
            collision_events.send_default();
            commands.entity(car_entity).despawn();
        }

        if car_transform.translation.y <= BOTTOM_WALL - CAR_SIZE.y {
            commands.entity(car_entity).despawn();
        }
    }
}

fn update_scoreboard(scoreboard: Res<Scoreboard>, mut query: Query<&mut Text>) {
    let mut text = query.single_mut();
    text.sections[1].value = scoreboard.score.to_string();
}
