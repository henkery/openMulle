use std::borrow::{Borrow, BorrowMut};

use crate::{
    render::scaler::{OuterCamera, PIXEL_PERFECT_LAYERS},
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

use super::{
    mulle_asset_helper::{
        MacromediaCastBitmapMetadata, MulleAssetHelp, MulleAssetHelper, MulleImage,
    },
    mulle_car::{Car, CarEntity, PartDB, PartLocation},
};

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
    click: Vec<ClickAction>,
}
#[derive(Component)]
pub struct MulleDraggable {
    pub snap_location: Vec2,
    pub rect: Rect,
    pub height: f32,
    pub width: f32,
    pub being_dragged: bool,
    pub image_junk: Option<MulleImage>,
    pub attached_image: MulleImage,
    pub morphs: Vec<PartDB>,
    pub is_morph_of: Option<PartDB>,
    pub part_id: i32,
    pub is_attached: bool,
}

const CLICKABLE_LAYER: f32 = 1.;

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
    click: Vec<ClickAction>,
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
        rect_default: Rect::new(
            f32::from(-meta_default.bitmap_metadata.image_reg_x),
            (i32::from(meta_default.bitmap_metadata.image_reg_y)
                - i32::from(meta_default.bitmap_metadata.image_height)) as f32,
            -(i32::from(meta_default.bitmap_metadata.image_reg_x)
                - i32::from(meta_default.bitmap_metadata.image_width)) as f32,
            f32::from(meta_default.bitmap_metadata.image_reg_y),
        ),
        rect_hover: Rect::new(
            f32::from(-meta_hover.bitmap_metadata.image_reg_x),
            (i32::from(meta_hover.bitmap_metadata.image_reg_y)
                - i32::from(meta_hover.bitmap_metadata.image_height)) as f32,
            -(i32::from(meta_hover.bitmap_metadata.image_reg_x)
                - i32::from(meta_hover.bitmap_metadata.image_width)) as f32,
            f32::from(meta_hover.bitmap_metadata.image_reg_y),
        ),
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
                sprite: clickable.sprite_default.sprite.clone(),
                transform: Transform::from_xyz(
                    (clickable.rect_default.max.x + clickable.rect_default.min.x) / 2.,
                    (clickable.rect_default.max.y + clickable.rect_default.min.y) / 2.,
                    CLICKABLE_LAYER,
                ),
                ..default()
            },
            clickable.to_owned(),
            NotHovered,
            PIXEL_PERFECT_LAYERS,
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

fn image_metadata_to_rect(image: &MacromediaCastBitmapMetadata, part: &PartDB) -> Vec2 {
    let rect = Rect::new(
        f32::from(-image.image_reg_x) + 40.,
        (i32::from(image.image_reg_y) - i32::from(image.image_height)) as f32,
        -(i32::from(image.image_reg_x) - i32::from(image.image_width)) as f32 + 40.,
        f32::from(image.image_reg_y),
    )
    .center();
    Vec2 {
        x: rect.x + part.offset.x as f32,
        y: rect.y - part.offset.y as f32,
    }
}

fn update_clickables(
    mut query: Query<
        (&mut Sprite, &MulleClickable, &mut Transform),
        (With<Hovered>, Without<NotHovered>, Without<MulleDraggable>),
    >,
    mut query_unhover: Query<
        (&mut Sprite, &MulleClickable, &mut Transform),
        (With<NotHovered>, Without<Hovered>, Without<MulleDraggable>),
    >,
    mut query_draggables: Query<
        (&mut MulleDraggable, &mut Transform, &mut Sprite, Entity),
        With<MulleDraggable>,
    >,
    mycoords: Res<MyWorldCoords>,
    mut commands: Commands,
    mulle_asset_helper: Res<MulleAssetHelp>,
    car: Res<Car>,
) {
    for (mut image_handle, clickable, mut transform) in &mut query {
        update_and_transform_sprite(
            &mut image_handle,
            clickable.sprite_hover.sprite.clone(),
            &mut transform,
            &clickable.rect_hover,
        );
    }
    for (mut image_handle, clickable, mut transform) in &mut query_unhover {
        update_and_transform_sprite(
            &mut image_handle,
            clickable.sprite_default.sprite.clone(),
            &mut transform,
            &clickable.rect_default,
        );
    }
    let mut morph_master: Option<PartDB> = None;
    let mut entities_to_destoy = Vec::<Entity>::new();
    let mut destroy_entities = true;
    for (mut draggable, mut transform, mut image_handle, entity) in &mut query_draggables {
        if draggable.being_dragged {
            if !draggable.morphs.is_empty() {
                for morph in &draggable.morphs.clone() {
                    for use_view in [&morph.use_view, &morph.use_view_2] {
                        if use_view.is_empty() {
                            continue;
                        }
                        let image = mulle_asset_helper
                            .get_mulle_image_by_name("cddata.cxt".to_owned(), use_view.to_string())
                            .unwrap();
                        let snap_point = image_metadata_to_rect(&image.bitmap_metadata, morph);
                        if mycoords.0.distance(snap_point) < 25.
                            && car.can_or_is_attached_part(morph)
                        {
                            // destroy master
                            commands.entity(entity).despawn();
                            // create new children
                            create_morph_variant(
                                &mulle_asset_helper,
                                morph,
                                mycoords.0,
                                commands.borrow_mut(),
                                vec![&morph.use_view, &morph.use_view_2],
                                mulle_asset_helper.part_db.get(&draggable.part_id).cloned(),
                                true,
                            );
                        } else {
                            match &draggable.is_morph_of {
                                Some(draggable_morph_master) => {
                                    // destroy all morphs
                                    commands.entity(entity).despawn();
                                    morph_master = Some(draggable_morph_master.clone());
                                }
                                None => {
                                    draggable.is_attached = false;
                                    draggable.rect = update_draggable(
                                        &mut transform,
                                        mycoords.0,
                                        &mut image_handle,
                                        &draggable,
                                        &draggable.image_junk,
                                    );
                                }
                            }
                        }
                    }
                }
            } else if mycoords.0.distance(draggable.snap_location) < 25.
                && car.can_or_is_attached_part(
                    mulle_asset_helper.part_db.get(&draggable.part_id).unwrap(),
                )
            {
                draggable.is_attached = true;
                destroy_entities = false;
                draggable.rect = update_draggable(
                    &mut transform,
                    draggable.snap_location,
                    &mut image_handle,
                    &draggable,
                    &Some(draggable.attached_image.clone()),
                );
            } else {
                match &draggable.is_morph_of {
                    Some(draggable_morph_master) => {
                        // destroy all morphs
                        entities_to_destoy.push(entity);
                        morph_master = Some(draggable_morph_master.clone());
                    }
                    None => {
                        draggable.is_attached = false;
                        draggable.rect = update_draggable(
                            &mut transform,
                            mycoords.0,
                            &mut image_handle,
                            &draggable,
                            &draggable.image_junk,
                        );
                    }
                }
            }
        }
    }
    if destroy_entities {
        for entity in entities_to_destoy {
            commands.entity(entity).despawn();
        }
        // create new master for destroyed element
        // assume master doens't exist //TODO safely check if master doesn't exist yet
        if let Some(morph_master) = morph_master {
            create_morph_variant(
                &mulle_asset_helper,
                &morph_master,
                mycoords.0,
                commands.borrow_mut(),
                vec![&morph_master.junk_view],
                None,
                false,
            );
        }
    }
}

fn update_draggable(
    transform: &mut Transform,
    coords: Vec2,
    sprite: &mut Sprite,
    draggable: &MulleDraggable,
    replace_image: &Option<MulleImage>,
) -> Rect {
    *transform = Transform::from_xyz(coords.x, coords.y, 2.);
    if let Some(image) = replace_image {
        *sprite = image.sprite.clone();
    }
    Rect::new(
        coords.x - (draggable.width / 2.),
        coords.y - (draggable.height / 2.),
        coords.x + (draggable.width / 2.),
        coords.y + (draggable.height / 2.),
    )
}

fn update_and_transform_sprite(
    image_handle: &mut Sprite,
    image: Sprite,
    transform: &mut Transform,
    rect: &Rect,
) {
    *image_handle = image;
    *transform = Transform::from_xyz(
        (rect.max.x + rect.min.x) / 2.,
        (rect.max.y + rect.min.y) / 2.,
        CLICKABLE_LAYER,
    );
}

fn create_morph_variant(
    mulle_asset_helper: &Res<MulleAssetHelp>,
    morph_master: &PartDB,
    current_coords: Vec2,
    commands: &mut Commands,
    views: Vec<&String>,
    morph_of: Option<PartDB>,
    is_attached: bool,
) {
    for use_view in views {
        if use_view.is_empty() {
            continue;
        }
        let image = mulle_asset_helper
            .get_mulle_image_by_name("cddata.cxt".to_owned(), use_view.to_string())
            .unwrap();
        let snap_point = {
            let rect = Rect::new(
                f32::from(-image.bitmap_metadata.image_reg_x) + 40.,
                (i32::from(image.bitmap_metadata.image_reg_y)
                    - i32::from(image.bitmap_metadata.image_height)) as f32,
                -(i32::from(image.bitmap_metadata.image_reg_x)
                    - i32::from(image.bitmap_metadata.image_width)) as f32
                    + 40.,
                f32::from(image.bitmap_metadata.image_reg_y),
            )
            .center();
            Vec2 {
                x: rect.x + morph_master.offset.x as f32,
                y: rect.y - morph_master.offset.y as f32,
            }
        };
        let current_coords = {
            if is_attached {
                // if it is a morph it has to be snapped
                snap_point
            } else {
                current_coords
            }
        };
        let current_rect = Rect::new(
            current_coords.x - (f32::from(image.bitmap_metadata.image_width) / 2.),
            current_coords.y - (f32::from(image.bitmap_metadata.image_height) / 2.),
            current_coords.x + (f32::from(image.bitmap_metadata.image_width) / 2.),
            current_coords.y + (f32::from(image.bitmap_metadata.image_height) / 2.),
        );

        commands.spawn((
            SpriteBundle {
                sprite: image.sprite.clone(),
                transform: Transform::from_xyz(current_coords.x, current_coords.y, 2.),
                ..default()
            },
            MulleDraggable {
                rect: current_rect,
                being_dragged: true,
                height: f32::from(image.bitmap_metadata.image_height),
                width: f32::from(image.bitmap_metadata.image_width),
                snap_location: Vec2 {
                    x: snap_point.x,
                    y: snap_point.y,
                },
                attached_image: image.to_owned(),
                image_junk: None,
                morphs: morph_master
                    .morphs_to
                    .iter()
                    .filter_map(|morph_id| mulle_asset_helper.part_db.get(morph_id))
                    .cloned()
                    .collect(),
                is_morph_of: morph_of.clone(),
                part_id: morph_master.part_id,
                is_attached,
            },
            PIXEL_PERFECT_LAYERS,
            CarEntity,
        ));
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
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor).ok())
        .map(|ray| ray.origin.truncate())
    {
        mycoords.0 = world_position;
        // eprintln!("World coords: {}/{}", world_position.x, world_position.y);
        for (entity, clickable) in &mut query {
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
    mut query2: Query<&mut MulleDraggable>,
    mut game_state: ResMut<NextState<GameState>>,
    current_game_state: Res<State<GameState>>,
    mut trash_state: ResMut<NextState<TrashState>>,
    current_trash_state: Res<State<TrashState>>,
    mut car: ResMut<Car>,
    mulle_asset_helper: Res<MulleAssetHelp>,
) {
    let world_position = mycoords.0;
    for event in mouse_button_input_events.read() {
        if event.button == MouseButton::Left && event.state == ButtonState::Released {
            for clickable in query.iter() {
                if clickable.rect_default.contains(world_position) {
                    for click in &clickable.click {
                        match click {
                            ClickAction::GamestateTransition { goal_state } => {
                                game_state.set(goal_state.to_owned());
                            }
                            ClickAction::PlayCutscene { cutscene_name: _ } => {}
                            ClickAction::TrashstateTransition { goal_state } => {
                                trash_state.set(goal_state.to_owned());
                            }
                        }
                    }
                }
            }
            if matches!(
                current_game_state.get(),
                GameState::GarageWithCar | GameState::YardWithCar
            ) {
                let attached_parts: Vec<&PartDB> = query2
                    .iter()
                    .filter_map(|draggable| {
                        if draggable.is_attached {
                            Some(mulle_asset_helper.part_db.get(&draggable.part_id).unwrap())
                        } else {
                            None
                        }
                    })
                    .collect();
                car.sync_parts(
                    attached_parts,
                    match current_game_state.get() {
                        GameState::GarageWithCar | GameState::GarageWithoutCar => {
                            &PartLocation::Garage
                        }
                        GameState::YardWithCar | GameState::YardWithoutCar => &PartLocation::Yard,
                        GameState::TrashHeap => match current_trash_state.get() {
                            TrashState::Blue => &PartLocation::HeapBlue,
                            TrashState::Green => &PartLocation::HeapGreen,
                            TrashState::Purple => &PartLocation::HeapPurple,
                            TrashState::Red => &PartLocation::HeapRed,
                            TrashState::Turquise => &PartLocation::HeapTurquise,
                            TrashState::Yellow => &PartLocation::HeapYellow,
                            _ => panic!("Illegal state!"),
                        },
                        _ => panic!("Illegal state!"),
                    },
                );
            }

            for mut draggable in &mut query2 {
                if draggable.being_dragged {
                    draggable.being_dragged = false;
                }
            }
        } else if event.button == MouseButton::Left
            && event.state == ButtonState::Pressed
            && current_game_state.get() == &GameState::GarageWithCar
        {
            //find the draggable entity we may be on
            if !query2.iter().any(|dragging| dragging.being_dragged) {
                let mut marked_part_id: Option<i32> = None;
                for draggable in query2.iter() {
                    if draggable.rect.contains(world_position) {
                        marked_part_id = Some(draggable.part_id);
                        break;
                    }
                }
                if let Some(marked_part_id) = marked_part_id {
                    for mut draggable in &mut query2 {
                        if draggable.part_id == marked_part_id {
                            draggable.being_dragged = true;
                        }
                    }
                }
            }
        }
    }
}
