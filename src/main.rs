mod components;
mod spawn_car;
mod resources;
mod events;
mod apply_velocity;
mod move_player;
mod collisions;
mod sprites;

use bevy::prelude::*;

use components::*;
use resources::*;
use events::*;
use spawn_car::spawn_car;
use apply_velocity::apply_velocity;
use move_player::move_player;
use collisions::check_for_collisions;
use sprites::animate_sprite;

const PLAYER_SPEED: f32 = 500.0;
const PLAYER_PADDING: f32 = 10.0;
const PLAYER_SIZE: Vec2 = Vec2::new(5.0, 8.0);
const CAR_SIZE: Vec2 = Vec2::new(20.0, 50.0);
const INITIAL_CAR_DIRECTION: Vec2 = Vec2::new(0.0, -0.5);
const CAR_SPEED: f32 = 400.0;
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

fn main() {
    App::new()
        .insert_resource(SpawnTimer(Timer::from_seconds(0.1, TimerMode::Repeating)))
        .insert_resource(Scoreboard { score: 0 })
        .add_event::<CollisionEvent>()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())
            .set(
                WindowPlugin {
                    primary_window: Some(Window {
                        title: "RoadStomp".into(),
                        resolution: (900., 600.).into(),
                        ..default()
                    }),
                    ..default()
                }
            ))
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, (
            spawn_car,
            check_for_collisions,
            apply_velocity.before(check_for_collisions),
            move_player
                .before(check_for_collisions)
                .after(apply_velocity),
        ))
        .add_systems(Update, (update_scoreboard, animate_sprite, bevy::window::close_on_esc))
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("chicken_sheet.png");
    let background_image = asset_server.load("road.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 2, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    // Use only the subset of sprites in the sheet that make up the run animation
    let animation_indices = AnimationIndices { first: 0, last: 1 };

    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(animation_indices.first),
            transform: Transform::from_xyz(0.0, -250., 2.0),
            ..default()
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        Player,
        Collider,
    ));
    commands.spawn(
        SpriteBundle {
            texture: background_image,
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        }
    );
    commands.spawn(
        TextBundle::from_sections([
            TextSection::new(
                "Score: ",
                TextStyle {
                    font_size: SCOREBOARD_FONT_SIZE,
                    color: TEXT_COLOR,
                    ..default()
                },
            ),
            TextSection::from_style(TextStyle {
                font_size: SCOREBOARD_FONT_SIZE,
                color: SCORE_COLOR,
                ..default()
            }),
        ])
            .with_style(Style {
                position_type: PositionType::Absolute,
                top: SCOREBOARD_TEXT_PADDING,
                left: SCOREBOARD_TEXT_PADDING,
                ..default()
            }),
    );
}

fn update_scoreboard(scoreboard: Res<Scoreboard>, mut query: Query<&mut Text>) {
    let mut text = query.single_mut();
    text.sections[1].value = scoreboard.score.to_string();
}
