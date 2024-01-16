use bevy::{prelude::*, utils::HashMap};

use crate::{parsers::database_language::Point, render::scaler::PIXEL_PERFECT_LAYERS, GameState};

use super::mulle_asset_helper::{MulleAssetHelp, MulleAssetHelper};

pub struct MulleCarPlugin;

impl Plugin for MulleCarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_car)
            .add_systems(OnEnter(GameState::Garage), spawn_car_parts);
    }
}

fn init_car(mut commands: Commands, mulle_asset_helper: Res<MulleAssetHelp>) {
    let car = Car {
        parts: vec![
            mulle_asset_helper.part_db.get(&1).unwrap().to_owned(),
            mulle_asset_helper.part_db.get(&5).unwrap().to_owned(),
            mulle_asset_helper.part_db.get(&18).unwrap().to_owned(),
            mulle_asset_helper.part_db.get(&20).unwrap().to_owned(),
        ],
    };
    commands.insert_resource(car);
}
#[derive(Component, Clone)]
struct CarEntity;

fn spawn_car_parts(car: Res<Car>, mut commands: Commands, mulle_asset_helper: Res<MulleAssetHelp>) {
    for part in &car.parts {
        let image = mulle_asset_helper
            .get_mulle_image_by_name("cddata.cxt".to_owned(), part.use_view.clone())
            .unwrap();
        commands.spawn((
            SpriteBundle {
                texture: image.image.clone(),
                transform: Transform::from_xyz(
                    part.offset.x as f32,
                    part.offset.y as f32 * -1.,
                    2.,
                ),
                ..default()
            },
            PIXEL_PERFECT_LAYERS,
            CarEntity,
        ));
    }
}

#[derive(Resource)]
struct Car {
    parts: Vec<PartDB>,
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
