use std::path::PathBuf;

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
    input::mouse::MouseButtonInput,
    math::Vec2,
    prelude::*,
    render::camera::Camera,
    sprite::SpriteBundle,
    transform::components::{GlobalTransform, Transform},
    window::{PrimaryWindow, Window},
};

use super::mulle_asset_helper::{MulleAssetHelp, MulleAssetHelper};

pub struct MullePointandClickPlugin;

impl Plugin for MullePointandClickPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MyWorldCoords>()
            .add_systems(Update, my_cursor_system)
            .add_systems(Update, mouse_click_system)
            .add_systems(Update, update_clickables);
    }
}

#[derive(Component)]
pub struct MulleClickable {
    sprite_default: Handle<Image>,
    sprite_hover: Handle<Image>,
    x_min: f32,
    x_max: f32,
    y_min: f32,
    y_max: f32,
    click: ClickAction,
}

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
    dir_hover: &str,
    name_hover: u32,
    mulle_asset_helper: &bevy::prelude::Res<'_, MulleAssetHelp>,
) -> MulleClickable {
    let meta_default = mulle_asset_helper
        .get_mulle_image_by_name(dir_default.to_string(), name_default)
        .unwrap();
    let meta_hover = mulle_asset_helper
        .get_mulle_image_by_name(dir_default.to_string(), name_default)
        .unwrap();
    MulleClickable {
        sprite_default: mulle_asset_helper
            .get_image_by_name(dir_default.to_string(), name_default)
            .unwrap()
            .clone(),
        sprite_hover: mulle_asset_helper
            .get_image_by_name(dir_default.to_string(), name_default)
            .unwrap()
            .clone(),
        click: click,
        x_min: (meta_default.bitmap_metadata.image_reg_x * -1) as f32,
        x_max: ((meta_default.bitmap_metadata.image_reg_x as i32
            - meta_default.bitmap_metadata.image_width as i32)
            * -1) as f32,
        y_min: (meta_hover.bitmap_metadata.image_reg_y as i32
            - meta_hover.bitmap_metadata.image_height as i32) as f32,
        y_max: (meta_hover.bitmap_metadata.image_reg_y) as f32,
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
                texture: clickable.sprite_default.to_owned(),
                transform: Transform::from_xyz(
                    (clickable.x_max + clickable.x_min) / 2.,
                    (clickable.y_max + clickable.y_min) / 2.,
                    2.,
                ),
                ..default()
            },
            MulleClickable {
                sprite_default: clickable.sprite_default.clone(),
                sprite_hover: clickable.sprite_hover.clone(),
                click: clickable.click.clone(),
                x_min: clickable.x_min,
                x_max: clickable.x_max,
                y_min: clickable.y_min,
                y_max: clickable.y_max,
            },
            NotHovered,
            HIGH_RES_LAYERS,
            component.clone(),
        ));
    }
}

#[derive(Clone, Debug)]
pub enum ClickAction {
    ActionGamestateTransition { goal_state: GameState },
    ActionTrashstateTransition { goal_state: TrashState },
    ActionPlayCutscene { cutscene_name: String },
}
#[derive(Component)]
struct Hovered;
#[derive(Component)]
struct NotHovered;

fn update_clickables(
    asset_server: Res<AssetServer>,
    mut query: Query<(&mut Handle<Image>, &MulleClickable), (With<Hovered>, Without<NotHovered>)>,
    mut query_unhover: Query<
        (&mut Handle<Image>, &MulleClickable),
        (With<NotHovered>, Without<Hovered>),
    >,
) {
    for (mut image_handle, clickable) in query.iter_mut() {
        *image_handle = clickable.sprite_hover.to_owned();
    }
    for (mut image_handle, clickable) in query_unhover.iter_mut() {
        *image_handle = clickable.sprite_default.to_owned();
    }
}

/// We will store the world position of the mouse cursor here.
#[derive(Resource, Default)]
struct MyWorldCoords(Vec2);

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
            let sprite_bounds = Rect::new(
                clickable.x_min,
                clickable.y_min,
                clickable.x_max,
                clickable.y_max,
            );
            if sprite_bounds.contains(world_position) {
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
    mut game_state: ResMut<NextState<GameState>>,
    mut trash_state: ResMut<NextState<TrashState>>,
) {
    let world_position = mycoords.0;
    for event in mouse_button_input_events.read() {
        if event.button == MouseButton::Left {
            for clickable in query.iter() {
                let sprite_bounds = Rect::new(
                    clickable.x_min,
                    clickable.y_min,
                    clickable.x_max,
                    clickable.y_max,
                );
                if sprite_bounds.contains(world_position) {
                    match &clickable.click {
                        ClickAction::ActionGamestateTransition { goal_state } => {
                            game_state.set(goal_state.to_owned())
                        }
                        ClickAction::ActionPlayCutscene { cutscene_name } => {}
                        ClickAction::ActionTrashstateTransition { goal_state } => {
                            trash_state.set(goal_state.to_owned())
                        }
                    }
                }
            }
        }
    }
}

pub fn destroy_clickables<T: Component>(
    mut commands: Commands,
    query: Query<(&MulleClickable, Entity), With<T>>,
) {
    for (clickable, entity) in query.iter() {
        commands.entity(entity).despawn();
    }
}
