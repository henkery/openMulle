use crate::{
    render::scaler::{OuterCamera, HIGH_RES_LAYERS},
    screens::trash_heap::TrashState,
    GameState,
};
use bevy::{
    app::{App, Plugin, Update},
    ecs::{
        component::Component,
        query::With,
        system::{Query, ResMut, Resource},
    },
    input::{mouse::MouseButtonInput, ButtonState},
    math::Vec2,
    prelude::*,
    render::camera::Camera,
    sprite::SpriteBundle,
    transform::components::{GlobalTransform, Transform},
    window::{PrimaryWindow, Window},
};

use super::mulle_asset_helper::{MulleAssetHelp, MulleAssetHelper, MulleImage};

pub struct MullePointandClickPlugin;

impl Plugin for MullePointandClickPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MyWorldCoords>()
            .add_systems(Update, my_cursor_system)
            .add_systems(Update, mouse_click_system)
            .add_systems(Update, update_clickables);
    }
}

#[derive(Component, Clone)]
pub struct MulleClickable {
    sprite_default: MulleImage,
    sprite_hover: MulleImage,
    rect_default: Rect,
    rect_hover: Rect,
    click: ClickAction,
}
#[derive(Component)]
pub struct MulleDraggable {
    rect: Rect,
}

const CLICKABLE_LAYER: f32 = 2.;

// pub fn mulle_clickable(sprite_default: PathBuf, sprite_hover: PathBuf, click: (), x_min: f32, x_max: f32, y_min: f32, y_max: f32) -> MulleClickable {
//     if (y_min > y_max || x_min > x_max) {
//         panic!("Invalid coordinates {}x{} to {}x{}", x_min,y_min,x_max,y_max)
//     }
//     MulleClickable { sprite_default: sprite_default, sprite_hover: sprite_hover, click: click, x_min: x_min, x_max: x_max, y_min: y_min, y_max: y_max}
// }

// pub fn mulle_clickable_from_meta(sprite_default: PathBuf, sprite_hover: PathBuf, click: (), meta_default: &Member, meta_hover: &Member) -> MulleClickable {
//     MulleClickable { sprite_default: sprite_default, sprite_hover: sprite_hover, click: click, x_min: (meta_default.imageRegX.unwrap()*-1) as f32, x_max: ((meta_default.imageRegX.unwrap()-meta_default.imageWidth.unwrap() as i32)*-1) as f32, y_min: (meta_hover.imageRegY.unwrap()-meta_hover.imageHeight.unwrap() as i32) as f32, y_max: ((meta_hover.imageRegY.unwrap())) as f32 }
// }

pub fn mulle_clickable_from_name(
    click: ClickAction,
    dir_default: &str,
    name_default: u32,
    _dir_hover: &str,
    name_hover: u32,
    mulle_asset_helper: &bevy::prelude::Res<'_, MulleAssetHelp>,
) -> MulleClickable {
    let meta_default = mulle_asset_helper
        .get_mulle_image_by_asset_number(dir_default.to_string(), name_default)
        .unwrap();
    let meta_hover = mulle_asset_helper
        .get_mulle_image_by_asset_number(dir_default.to_string(), name_hover)
        .unwrap();
    MulleClickable {
        sprite_default: meta_default.clone(),
        sprite_hover: meta_hover.clone(),
        click,
        rect_default: Rect {
            min: Vec2 {
                x: -meta_default.bitmap_metadata.image_reg_x as f32,
                y: (meta_default.bitmap_metadata.image_reg_y as i32
                    - meta_default.bitmap_metadata.image_height as i32) as f32,
            },
            max: Vec2 {
                x: -(meta_default.bitmap_metadata.image_reg_x as i32
                    - meta_default.bitmap_metadata.image_width as i32) as f32,
                y: (meta_default.bitmap_metadata.image_reg_y) as f32,
            },
        },
        rect_hover: Rect {
            min: Vec2 {
                x: -meta_hover.bitmap_metadata.image_reg_x as f32,
                y: (meta_hover.bitmap_metadata.image_reg_y as i32
                    - meta_hover.bitmap_metadata.image_height as i32) as f32,
            },
            max: Vec2 {
                x: -(meta_hover.bitmap_metadata.image_reg_x as i32
                    - meta_hover.bitmap_metadata.image_width as i32) as f32,
                y: (meta_hover.bitmap_metadata.image_reg_y) as f32,
            },
        },
    }
}

pub fn deploy_clickables<T: Component + Clone>(
    mut commands: Commands,
    clickables: &[MulleClickable],
    component: T,
) {
    for clickable in clickables {
        commands.spawn((
            SpriteBundle {
                texture: clickable.sprite_default.image.clone(),
                transform: Transform::from_xyz(
                    (clickable.rect_default.max.x + clickable.rect_default.min.x) / 2.,
                    (clickable.rect_default.max.y + clickable.rect_default.min.y) / 2.,
                    CLICKABLE_LAYER,
                ),
                ..default()
            },
            clickable.to_owned(),
            NotHovered,
            HIGH_RES_LAYERS,
            component.clone(),
        ));
    }
}

#[derive(Clone, Debug)]
pub enum ClickAction {
    GamestateTransition { goal_state: GameState },
    TrashstateTransition { goal_state: TrashState },
    PlayCutscene { cutscene_name: String },
}
#[derive(Component)]
struct Hovered;
#[derive(Component)]
struct NotHovered;

fn update_clickables(
    mut query: Query<(&mut Handle<Image>, &MulleClickable, &mut Transform), (With<Hovered>, Without<NotHovered>)>,
    mut query_unhover: Query<
        (&mut Handle<Image>, &MulleClickable, &mut Transform),
        (With<NotHovered>, Without<Hovered>),
    >,
) {
    for (mut image_handle, clickable, mut transform) in query.iter_mut() {
        *image_handle = clickable.sprite_hover.image.clone();
        *transform = Transform::from_xyz(
            (clickable.rect_hover.max.x + clickable.rect_hover.min.x) / 2.,
            (clickable.rect_hover.max.y + clickable.rect_hover.min.y) / 2.,
            CLICKABLE_LAYER,
        );
    }
    for (mut image_handle, clickable, mut transform) in query_unhover.iter_mut() {
        *image_handle = clickable.sprite_default.image.clone();
        *transform = Transform::from_xyz(
            (clickable.rect_default.max.x + clickable.rect_default.min.x) / 2.,
            (clickable.rect_default.max.y + clickable.rect_default.min.y) / 2.,
            CLICKABLE_LAYER,
        );
    }
}

/// We will store the world position of the mouse cursor here.
#[derive(Resource, Default)]
pub struct MyWorldCoords(pub Vec2);

fn my_cursor_system(
    mut mycoords: ResMut<MyWorldCoords>,
    // query to get the window (so we can read the current cursor position)
    q_window: Query<&Window, With<PrimaryWindow>>,
    // query to get camera transform
    q_camera: Query<(&Camera, &GlobalTransform), With<OuterCamera>>,
    mut query: Query<(Entity, &MulleClickable)>,
    mut commands: Commands,
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so Query::single() is OK
    let (camera, camera_transform) = q_camera.single();

    // There is only one primary window, so we can similarly get it from the query:
    let window = q_window.single();

    // check if the cursor is inside the window and get its position
    // then, ask bevy to convert into world coordinates, and truncate to discard Z
    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        mycoords.0 = world_position;
        // eprintln!("World coords: {}/{}", world_position.x, world_position.y);
        for (entity, clickable) in query.iter_mut() {
            if clickable.rect_default.contains(world_position) {
                commands.entity(entity).remove::<NotHovered>();
                commands.entity(entity).insert(Hovered);
            } else {
                commands.entity(entity).insert(NotHovered);
                commands.entity(entity).remove::<Hovered>();
            }
        }
    }
}

fn mouse_click_system(
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    mycoords: ResMut<MyWorldCoords>,
    query: Query<&MulleClickable>,
    query2: Query<&MulleDraggable>,
    mut game_state: ResMut<NextState<GameState>>,
    mut trash_state: ResMut<NextState<TrashState>>,
) {
    let world_position = mycoords.0;
    for event in mouse_button_input_events.read() {
        if event.button == MouseButton::Left && event.state == ButtonState::Released {
            for clickable in query.iter() {
                if clickable.rect_default.contains(world_position) {
                    match &clickable.click {
                        ClickAction::GamestateTransition { goal_state } => {
                            game_state.set(goal_state.to_owned())
                        }
                        ClickAction::PlayCutscene { cutscene_name: _ } => {}
                        ClickAction::TrashstateTransition { goal_state } => {
                            trash_state.set(goal_state.to_owned())
                        }
                    }
                }
            }
        } else if event.button == MouseButton::Left && event.state == ButtonState::Pressed {
            //find the draggable entity we may be on
            for draggable in query2.iter() {}
        }
    }
}
