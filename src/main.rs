mod car;
mod collisions;
mod events;
mod player;
mod resources;
mod sprites;
mod ui;

use bevy::prelude::*;

use events::*;
use resources::*;

use crate::car::CarPlugin;
use crate::collisions::CollisionPlugin;
use crate::player::PlayerPlugin;
use crate::sprites::SpritePlugin;
use crate::ui::UiPlugin;

const WALL_THICKNESS: f32 = 10.0;

// x coordinates
const LEFT_WALL: f32 = -450.;
const RIGHT_WALL: f32 = 450.;
// y coordinates
const BOTTOM_WALL: f32 = -300.;
const TOP_WALL: f32 = 300.;

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    Menu,
    InGame,
    GameOver,
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Paused,
    Running,
}

fn main() {
    App::new()
        .add_state::<GameState>()
        .add_state::<AppState>()
        .insert_resource(Lives { lives: 5 })
        .insert_resource(Score { score: 0 })
        .add_event::<CollisionEvent>()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "RoadStomp".into(),
                        resolution: (900., 600.).into(),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(UiPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(SpritePlugin)
        .add_plugins(CarPlugin)
        .add_plugins(CollisionPlugin)
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(Update, toggle_gamestate.run_if(in_state(AppState::InGame)))
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn toggle_gamestate(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    game_state: Res<State<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) && game_state.as_ref() == &GameState::Running {
        println!("Gamestate set to: Paused");
        commands.insert_resource(NextState(Some(GameState::Paused)));
    }
    if keyboard_input.just_pressed(KeyCode::Space) && game_state.as_ref() == &GameState::Paused {
        println!("Gamestate set to: Running");
        commands.insert_resource(NextState(Some(GameState::Running)));
    }
}
