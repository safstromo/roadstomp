mod menu;
mod styles;
mod buttons;

use bevy::prelude::*;
use crate::AppState;
use crate::ui::buttons::*;
use crate::ui::menu::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Menu), spawn_menu)
            .add_systems(OnExit(AppState::Menu), despawn_menu)
            .add_systems(Startup, spawn_game_background)
            // .add_systems(OnExit(AppState::InGame), despawn_game_background)
            .add_systems(Update, toggle_appstate)
            .add_systems(Update,
                         (
                             interact_with_play_button,
                             interact_with_quit_button
                         ).run_if(in_state(AppState::Menu)),
            );
    }
}



fn spawn_game_background(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let background_image = asset_server.load("road.png");
    commands.spawn((SpriteBundle {
        texture: background_image,
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    },
                    GameBackground,
    ));
}

fn despawn_game_background(
    mut commands: Commands,
    menu_query: Query<Entity, With<GameBackground>>, ) {
    if let Ok(entity) = menu_query.get_single() {
        commands.entity(entity).despawn_recursive();
    }
}

fn toggle_appstate(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    game_state: Res<State<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::M) && game_state.as_ref() == &AppState::InGame {
        println!("Appstate set to: Menu");
        commands.insert_resource(NextState(Some(AppState::Menu)));
    }
    if keyboard_input.just_pressed(KeyCode::M) && game_state.as_ref() == &AppState::Menu {
        println!("Appstate set to: InGame");
        commands.insert_resource(NextState(Some(AppState::InGame)));
    }
}

#[derive(Component)]
struct GameBackground;


