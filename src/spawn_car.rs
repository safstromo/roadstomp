use bevy::prelude::*;
use rand::Rng;
use crate::{CAR_SPEED, INITIAL_CAR_DIRECTION, LEFT_WALL, RIGHT_WALL, SpawnTimer, TOP_WALL};
use crate::components::{Car, Collider, Velocity};


pub fn spawn_car(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<SpawnTimer>,
    asset_server: Res<AssetServer>,
) {
    let mut rng = rand::thread_rng();
    if timer.0.tick(time.delta()).just_finished() {
        commands.spawn(
            (SpriteBundle {
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
             Collider
            ));
    }
}
