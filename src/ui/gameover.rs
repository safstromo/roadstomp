use bevy::prelude::*;
use crate::ui::buttons::{PlayButton, QuitButton};
use crate::ui::styles::{BUTTON_STYLE, get_button_text_style, get_chicken_image_bundle, get_title_text_style, NORMAL_BUTTON_COLOR};

#[derive(Component)]
pub struct GameOver;

pub fn spawn_gameover(
    mut commands: Commands,
    asset_server: Res<AssetServer>, ) {
    build_gameover(&mut commands, &asset_server);
}

pub fn despawn_gameover(
    mut commands: Commands,
    menu_query: Query<Entity, With<GameOver>>, ) {
    if let Ok(gameover_entity) = menu_query.get_single() {
        commands.entity(gameover_entity).despawn_recursive();
    }
}

fn build_gameover(commands: &mut Commands,
                  asset_server: &Res<AssetServer>,
) -> Entity {
    let gameover_entity = commands
        .spawn((NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                row_gap: Val::Px(20.0),
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            ..default()
        },
                GameOver,
        ))
        .with_children(|parent| {
            //title
            parent.spawn(
                NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        width: Val::Px(300.0),
                        height: Val::Px(120.0),
                        ..default()
                    },
                    ..default()
                }).with_children(|parent| {
                //image
                parent.spawn(
                    get_chicken_image_bundle(asset_server));
                //Title text
                parent.spawn(
                    TextBundle {
                        text: Text {
                            sections: vec![
                                TextSection::new(
                                    "GAME OVER",
                                    get_title_text_style(asset_server),
                                )
                            ],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                //image
                parent.spawn(
                    get_chicken_image_bundle(asset_server));
            }
            );
            //playbutton
            parent.spawn(
                (
                    ButtonBundle {
                        style: BUTTON_STYLE,
                        background_color: NORMAL_BUTTON_COLOR.into(),
                        ..default()
                    },
                    PlayButton,
                )
            ).with_children(|parent| {
                parent.spawn(
                    TextBundle {
                        text: Text {
                            sections: vec![
                                TextSection::new(
                                    "Restart",
                                    get_button_text_style(asset_server),
                                )
                            ],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    }
                );
            });
            //quitbutton
            parent.spawn(
                (
                    ButtonBundle {
                        style: BUTTON_STYLE,
                        background_color: NORMAL_BUTTON_COLOR.into(),
                        ..default()
                    },
                    QuitButton
                )
            )
                .with_children(|parent| {
                    parent.spawn(
                        TextBundle {
                            text: Text {
                                sections: vec![
                                    TextSection::new(
                                        "Quit",
                                        get_button_text_style(asset_server),
                                    )
                                ],
                                alignment: TextAlignment::Center,
                                ..default()
                            },
                            ..default()
                        }
                    );
                });
        })
        .id();
    gameover_entity
}


