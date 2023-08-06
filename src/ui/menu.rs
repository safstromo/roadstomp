use bevy::prelude::*;
use crate::ui::buttons::{PlayButton, QuitButton};
use crate::ui::styles::*;

#[derive(Component)]
pub struct Menu;

pub fn spawn_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>, ) {
    build_menu(&mut commands, &asset_server);
}

pub fn despawn_menu(
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
                Menu,
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
                spawn_title_box(asset_server, parent, "RoadStomp");
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
                spawn_button_text_box(asset_server, parent, "Play")
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
                    spawn_button_text_box(asset_server, parent, "Quit");
                });
        })
        .id();
    menu_entity
}


