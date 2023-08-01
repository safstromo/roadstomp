use bevy::a11y::accesskit::Size;
use bevy::prelude::*;
use crate::{AppState, GameState};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Menu), spawn_menu)
            .add_systems(OnExit(AppState::Menu), despawn_menu)
            .add_systems(OnEnter(AppState::InGame), spawn_game_background)
            .add_systems(OnExit(AppState::InGame), despawn_Gamebackground)
            .add_systems(Update, toggle_appstate)
        ;
    }
}

fn spawn_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>, ) {
    let menu_entity = build_menu(&mut commands, &asset_server);
}

fn despawn_menu(
    mut commands: Commands,
    menu_query: Query<Entity, With<Menu>>, ) {
    if let Ok(menu_entity) = menu_query.get_single() {
        commands.entity(menu_entity).despawn_recursive();
    }
}

fn build_menu(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) -> Entity {
    let menu_entity = commands
        .spawn((NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            background_color: Color::RED.into(),
            ..default()
        },
                Menu,
        ))
        .id();
    menu_entity
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
fn despawn_Gamebackground(
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
struct Menu;

#[derive(Component)]
struct GameBackground;
