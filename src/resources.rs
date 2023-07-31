use bevy::prelude::*;

#[derive(Resource)]
pub struct SpawnTimer(pub Timer);

#[derive(Resource)]
pub struct Scoreboard {
    pub score: usize,
}

