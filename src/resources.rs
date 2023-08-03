use bevy::prelude::*;

#[derive(Resource)]
pub struct Score {
    pub score: usize,
}
#[derive(Resource)]
pub struct Lives {
    pub lives: usize,
}
