use bevy::{input::mouse::MouseButtonInput, prelude::*, utils::HashMap};

use crate::{parsers::database_language::Point, render::scaler::PIXEL_PERFECT_LAYERS, GameState};

use super::{
    mulle_asset_helper::{MulleAssetHelp, MulleAssetHelper},
    mulle_point_and_click::MyWorldCoords,
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
                "7".to_owned(),
                mulle_asset_helper.part_db.get(&63).unwrap().to_owned(),
            ),
            (
                "8".to_owned(),
                mulle_asset_helper.part_db.get(&91).unwrap().to_owned(),
            ),
        ]),
    };
    commands.insert_resource(car);
}

fn move_car_part(mycoords: ResMut<MyWorldCoords>, mut query: Query<(&mut Transform, &CarEntity)>) {
    // for (mut transform, car) in query.iter_mut() {
    //     transform.translation.x = mycoords.0.x;
    //     transform.translation.y = mycoords.0.y;
    // }
    // println!("moved {} {}", mycoords.0.x, mycoords.0.y);
}

#[derive(Component, Clone)]
struct CarEntity;

fn spawn_car_parts(car: Res<Car>, mut commands: Commands, mulle_asset_helper: Res<MulleAssetHelp>) {
    for part in car.parts.values() {
        for use_view in [&part.use_view, &part.use_view_2] {
            if use_view.is_empty() {
                continue;
            }
            let image = mulle_asset_helper
                .get_mulle_image_by_name("cddata.cxt".to_owned(), use_view.to_string())
                .unwrap();

            let x_min = -image.bitmap_metadata.image_reg_x as f32;
            let x_max = -(image.bitmap_metadata.image_reg_x as i32
                - image.bitmap_metadata.image_width as i32) as f32;
            let y_min = (image.bitmap_metadata.image_reg_y as i32
                - image.bitmap_metadata.image_height as i32) as f32;
            let y_max = (image.bitmap_metadata.image_reg_y) as f32;
            commands.spawn((
                SpriteBundle {
                    texture: image.image.clone(),
                    transform: Transform::from_xyz(
                        ((x_max + x_min) / 2.) + part.offset.x as f32 + 40., // It is a mystery why, but this entire scene seems offset by 40 to the back
                        ((y_max + y_min) / 2.) - part.offset.y as f32,
                        3., // how to layer stuff?
                    ),
                    ..default()
                },
                PIXEL_PERFECT_LAYERS,
                CarEntity,
            ));
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
