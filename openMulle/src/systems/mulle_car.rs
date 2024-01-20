use bevy::{prelude::*, utils::HashMap};

use crate::{parsers::database_language::Point, render::scaler::PIXEL_PERFECT_LAYERS, GameState};

use super::{
    mulle_asset_helper::{MulleAssetHelp, MulleAssetHelper},
    mulle_point_and_click::{MulleDraggable, MyWorldCoords},
};

pub struct MulleCarPlugin;

impl Plugin for MulleCarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_car)
            .add_systems(Update, move_car_part)
            .add_systems(OnEnter(GameState::Garage), spawn_car_parts);
    }
}

fn init_car(mut commands: Commands, mulle_asset_helper: Res<MulleAssetHelp>) {
    let car = Car {
        parts: HashMap::from([
            (
                "0".to_owned(),
                mulle_asset_helper.part_db.get(&1).unwrap().to_owned(),
            ),
            (
                "5".to_owned(),
                mulle_asset_helper.part_db.get(&100).unwrap().to_owned(),
            ),
            (
                "6".to_owned(),
                mulle_asset_helper.part_db.get(&62).unwrap().to_owned(),
            ),
            (
                "8".to_owned(),
                mulle_asset_helper.part_db.get(&91).unwrap().to_owned(),
            ),
        ]),
    };
    commands.insert_resource(car);
}

fn move_car_part(_mycoords: ResMut<MyWorldCoords>, _query: Query<(&mut Transform, &CarEntity)>) {
    // for (mut transform, car) in query.iter_mut() {
    //     transform.translation.x = mycoords.0.x;
    //     transform.translation.y = mycoords.0.y;
    // }
    // println!("moved {} {}", mycoords.0.x, mycoords.0.y);
}

#[derive(Component, Clone)]
pub struct CarEntity;

fn spawn_car_parts(car: Res<Car>, mut commands: Commands, mulle_asset_helper: Res<MulleAssetHelp>) {
    for part in car.parts.values() {
        for use_view in [&part.use_view, &part.use_view_2] {
            if use_view.is_empty() {
                continue;
            }
            let image = mulle_asset_helper
                .get_mulle_image_by_name("cddata.cxt".to_owned(), use_view.to_string())
                .unwrap();
            let image_junk = mulle_asset_helper
                .get_mulle_image_by_name("cddata.cxt".to_owned(), part.junk_view.to_string())
                .cloned();
            let rect = Rect::new(
                f32::from(-image.bitmap_metadata.image_reg_x) + 40.,
                (i32::from(image.bitmap_metadata.image_reg_y)
                    - i32::from(image.bitmap_metadata.image_height)) as f32,
                -(i32::from(image.bitmap_metadata.image_reg_x)
                    - i32::from(image.bitmap_metadata.image_width)) as f32
                    + 40.,
                f32::from(image.bitmap_metadata.image_reg_y),
            );
            let master = {
                if part.master != 0 {
                    mulle_asset_helper.part_db.get(&part.master).cloned()
                } else {
                    None
                }
            };
            if part.part_id == 1 {
                commands.spawn((
                    SpriteBundle {
                        texture: image.image.clone(),
                        transform: Transform::from_xyz(
                            ((rect.max.x + rect.min.x) / 2.) + part.offset.x as f32, // It is a mystery why, but this entire scene seems offset by 40 to the back
                            ((rect.max.y + rect.min.y) / 2.) - part.offset.y as f32,
                            3., // how to layer stuff?
                        ),
                        ..default()
                    },
                    PIXEL_PERFECT_LAYERS,
                    CarEntity,
                ));
            } else {
                commands.spawn((
                    SpriteBundle {
                        texture: image.image.clone(),
                        transform: Transform::from_xyz(
                            ((rect.max.x + rect.min.x) / 2.) + part.offset.x as f32, // It is a mystery why, but this entire scene seems offset by 40 to the back
                            ((rect.max.y + rect.min.y) / 2.) - part.offset.y as f32,
                            3., // how to layer stuff?
                        ),
                        ..default()
                    },
                    MulleDraggable {
                        rect,
                        being_dragged: false,
                        height: f32::from(image.bitmap_metadata.image_height),
                        width: f32::from(image.bitmap_metadata.image_width),
                        snap_location: Vec2 {
                            x: ((rect.max.x + rect.min.x) / 2.) + part.offset.x as f32,
                            y: ((rect.max.y + rect.min.y) / 2.) - part.offset.y as f32,
                        },
                        attached_image: image.to_owned(),
                        image_junk,
                        morphs: Vec::default(),
                        is_morph_of: master,
                        part_id: part.part_id,
                    },
                    PIXEL_PERFECT_LAYERS,
                    CarEntity,
                ));
            }
        }
    }
}

#[derive(Resource)]
struct Car {
    parts: HashMap<String, PartDB>,
}

#[derive(Debug, Clone)]
pub struct PartDB {
    pub part_id: i32,
    pub master: i32,
    pub morphs_to: Vec<i32>,
    pub description: String,
    pub junk_view: String,
    pub use_view: String,
    pub use_view_2: String,
    pub offset: Point,
    pub properties: HashMap<String, i32>,
    pub requires: Vec<String>,
    pub covers: Vec<String>,
    pub new: Vec<PartNew>,
}
#[derive(Debug, Clone)]
pub struct PartNew {
    pub tag: String,
    pub point1: Point,
    pub point2: Point,
}
