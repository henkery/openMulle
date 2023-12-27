use std::path::{Path, PathBuf};

use bevy::{window::{PrimaryWindow, Window}, ecs::{system::{Resource, ResMut, Query}, query::{With, self}, component::Component}, math::Vec2, render::camera::Camera, transform::components::{GlobalTransform, Transform}, app::{Update, Plugin, App}, sprite::{Sprite, SpriteBundle}, prelude::*};
use crate::render::scaler::{OuterCamera, HIGH_RES_LAYERS};

pub struct MullePointandClickPlugin;

impl Plugin for MullePointandClickPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<MyWorldCoords>()
            .init_resource::<MulleClickables>()
            .add_systems(Update, my_cursor_system)
            .add_systems(Update, update_clickables);
    }
}

// Tag component used to tag entities added on the splash screen
#[derive(Component)]
struct MullePointAndClick;

#[derive(Component)]
pub struct MulleClickable {
    sprite_default: PathBuf,
    sprite_hover: PathBuf,
    click: (),
    x_min: f32,
    x_max: f32,
    y_min: f32,
    y_max: f32,
    created: bool
}

pub fn mulle_clickable(sprite_default: PathBuf, sprite_hover: PathBuf, click: (), x_min: f32, x_max: f32, y_min: f32, y_max: f32) -> MulleClickable {
    MulleClickable { sprite_default: sprite_default, sprite_hover: sprite_hover, click: click, x_min: x_min, x_max: x_max, y_min: y_min, y_max: y_max, created: false }
}

/// We will store the world position of the mouse cursor here.
#[derive(Resource, Default)]
pub struct MulleClickables {
    pub clickables: Vec<MulleClickable>
}

fn update_clickables(
    mut clickables: ResMut<MulleClickables>,
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    for clickable in clickables.clickables.iter_mut() {
        if !clickable.created {
            clickable.created = true;
            commands.spawn((
                SpriteBundle {
                    texture: asset_server.load(clickable.sprite_default.display().to_string()),
                    transform: Transform::from_xyz((clickable.x_max+clickable.x_min)/2., (clickable.y_max+clickable.y_min)/2., 2.),
                    ..default()
                },
                MullePointAndClick,
                HIGH_RES_LAYERS,
            ));

        }
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
    clickables: Res<MulleClickables>,
    mut query: Query<(&mut Handle<Image>, &MullePointAndClick)>,
    asset_server: Res<AssetServer>
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so Query::single() is OK
    let (camera, camera_transform) = q_camera.single();

    // There is only one primary window, so we can similarly get it from the query:
    let window = q_window.single();

    // check if the cursor is inside the window and get its position
    // then, ask bevy to convert into world coordinates, and truncate to discard Z
    if let Some(world_position) = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        mycoords.0 = world_position;
        eprintln!("World coords: {}/{}", world_position.x, world_position.y);
        for clickable in &clickables.clickables {
            if world_position.x > clickable.x_min && world_position.x < clickable.x_max && world_position.y > clickable.y_min && world_position.y < clickable.y_max {
                eprint!("Hover!!!");
                for (mut imageHandle, pointandclick) in query.iter_mut() {
                    *imageHandle = asset_server.load(clickable.sprite_hover.display().to_string());
                }
            }
            else {
                for (mut imageHandle, pointandclick) in query.iter_mut() {
                    *imageHandle = asset_server.load(clickable.sprite_default.display().to_string());
                }
            }
        }
    }
}