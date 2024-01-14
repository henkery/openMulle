use bevy::{prelude::*, utils::HashMap};

use crate::parsers::database_language::Point;

use super::mulle_asset_helper::MulleAssetHelp;

pub struct MulleCarPlugin;

impl Plugin for MulleCarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_car);
    }
}

fn init_car(
    mut commands: Commands,
    mulle_asset_helper: Res<MulleAssetHelp>
) {
    let basepart = mulle_asset_helper.part_db.get(&1).unwrap();
    let car = Car{
        parts: vec![basepart.clone()]
    };
    commands.insert_resource(car);
}

#[derive(Resource)]
struct Car {
    parts: Vec<PartDB>
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
    pub new: Vec<PartNew>
}
#[derive(Debug, Clone)]
pub struct PartNew {
    pub tag: String,
    pub point1: Point,
    pub point2: Point
}