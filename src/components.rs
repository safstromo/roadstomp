use bevy::prelude::*;

#[derive(Component)]
pub struct Collider;

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(pub Vec2);

#[derive(Component, Debug)]
pub struct Car;
