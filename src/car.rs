use crate::collisions::Collider;
use crate::{AppState, GameState, LEFT_WALL, RIGHT_WALL, TOP_WALL};
use bevy::prelude::*;
use rand::Rng;
use crate::resources::SpawnTimer;

const INITIAL_CAR_DIRECTION: Vec2 = Vec2::new(0.0, -0.5);
const CAR_SPEED: f32 = 400.0;
pub const CAR_SIZE: Vec2 = Vec2::new(20.0, 50.0);

pub struct CarPlugin;

impl Plugin for CarPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnTimer(Timer::from_seconds(0.1, TimerMode::Repeating)))
            .add_systems(
                FixedUpdate,
                (spawn_car, apply_velocity)
                    .run_if(in_state(AppState::InGame))
                    .run_if(in_state(GameState::Running)),
            )
            .add_systems(OnExit(AppState::InGame), despawn_cars);
    }
}

fn spawn_car(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<SpawnTimer>,
    asset_server: Res<AssetServer>,
) {
    let mut rng = rand::thread_rng();
    if timer.0.tick(time.delta()).just_finished() {
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("car.png"),
                transform: Transform {
                    translation: Vec3::new(rng.gen_range(LEFT_WALL..RIGHT_WALL), TOP_WALL, 1.),
                    scale: Vec3::splat(4.0),
                    ..default()
                },
                ..default()
            },
            Car,
            Velocity(INITIAL_CAR_DIRECTION.normalize() * CAR_SPEED),
            Collider,
        ));
    }
}

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time_step: Res<FixedTime>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * time_step.period.as_secs_f32();
        transform.translation.y += velocity.y * time_step.period.as_secs_f32();
    }
}

fn despawn_cars(
    mut commands: Commands,
    cars_query: Query<Entity, With<Car>>,
) {
    for car in cars_query.iter() {
        commands.entity(car).despawn();
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(pub Vec2);

#[derive(Component, Debug)]
pub struct Car;

