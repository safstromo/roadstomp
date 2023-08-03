mod car;
mod collisions;
mod events;
mod player;
mod resources;
mod sprites;
mod ui;
use bevy::app::AppExit;
use bevy::prelude::*;

use events::*;
use resources::*;
use crate::AppState::Menu;

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

const SCOREBOARD_FONT_SIZE: f32 = 40.0;
const SCOREBOARD_TEXT_PADDING: Val = Val::Px(5.0);
const TEXT_COLOR: Color = Color::rgb(0.5, 0.5, 1.0);
const SCORE_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);

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
        .insert_resource(Scoreboard { score: 5 })
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
        .add_systems(Startup, setup)
        // .add_systems(FixedUpdate, (
        // check_for_collisions,
        // apply_velocity.before(check_for_collisions),
        // move_player
        //     .before(check_for_collisions)
        //     .after(apply_velocity),
        // ))
        .add_systems(Update, (/*update_scoreboard,*/ bevy::window::close_on_esc))
        .add_systems(Update, toggle_gamestate.run_if(in_state(AppState::InGame)))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // let background_image = asset_server.load("road.png");

    commands.spawn(Camera2dBundle::default());

    // commands.spawn(SpriteBundle {
    //     texture: background_image,
    //     transform: Transform::from_xyz(0.0, 0.0, 0.0),
    //     ..default()
    // });
//     commands.spawn((
//         TextBundle::from_sections([
//             TextSection::new(
//                 "Lives: ",
//                 TextStyle {
//                     font_size: SCOREBOARD_FONT_SIZE,
//                     color: TEXT_COLOR,
//                     ..default()
//                 },
//             ),
//             TextSection::from_style(TextStyle {
//                 font_size: SCOREBOARD_FONT_SIZE,
//                 color: SCORE_COLOR,
//                 ..default()
//             }),
//         ])
//         .with_style(Style {
//             position_type: PositionType::Absolute,
//             top: SCOREBOARD_TEXT_PADDING,
//             left: SCOREBOARD_TEXT_PADDING,
//             ..default()
//         }),
//         ScoreBoard,
//     ));
}

// fn update_scoreboard(
//     scoreboard: Res<Scoreboard>,
//     mut query: Query<&mut Text>,
//     mut exit: EventWriter<AppExit>,
// ) {
//     let mut text = query.single_mut();
//     if scoreboard.score == 0 {
//         exit.send(AppExit);
//     }
//     text.sections[1].value = scoreboard.score.to_string();
// }
//
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
#[derive(Component)]
struct ScoreBoard;

