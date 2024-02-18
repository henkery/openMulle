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
            .add_systems(OnEnter(GameState::Garage), spawn_car_parts);
    }
}

fn init_car(mut commands: Commands, mulle_asset_helper: Res<MulleAssetHelp>) {
    let car = Car {
        parts: HashMap::from([
            (1, mulle_asset_helper.part_db.get(&1).unwrap().to_owned()),
            (
                100,
                mulle_asset_helper.part_db.get(&100).unwrap().to_owned(),
            ),
            (62, mulle_asset_helper.part_db.get(&62).unwrap().to_owned()),
            (91, mulle_asset_helper.part_db.get(&91).unwrap().to_owned()),
            (88, mulle_asset_helper.part_db.get(&88).unwrap().to_owned()),
            (85, mulle_asset_helper.part_db.get(&85).unwrap().to_owned()),
            (75, mulle_asset_helper.part_db.get(&75).unwrap().to_owned()),
        ]),
    };
    commands.insert_resource(car);
}

#[derive(Component, Clone)]
pub struct CarEntity;

struct UseView1;
struct UseView2;
enum UseViews {
    UseView1,
    UseView2,
}

fn spawn_car_parts(car: Res<Car>, mut commands: Commands, mulle_asset_helper: Res<MulleAssetHelp>) {
    for part in car.parts.values() {
        for (num, use_view) in [
            (UseViews::UseView1, &part.use_view),
            (UseViews::UseView2, &part.use_view_2),
        ] {
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
            let layer = {
                match num {
                    UseViews::UseView1 => car.get_render_layer_of_part(part) + 2.,
                    UseViews::UseView2 => 1.1,
                }
            };
            if part.part_id == 1 {
                commands.spawn((
                    SpriteBundle {
                        texture: image.image.clone(),
                        transform: Transform::from_xyz(
                            ((rect.max.x + rect.min.x) / 2.) + part.offset.x as f32, // It is a mystery why, but this entire scene seems offset by 40 to the back
                            ((rect.max.y + rect.min.y) / 2.) - part.offset.y as f32,
                            1.2, // how to layer stuff?
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
                            layer, // how to layer stuff?
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
                        is_morph_of: master.clone(),
                        part_id: part.part_id,
                        is_attached: true,
                    },
                    PIXEL_PERFECT_LAYERS,
                    CarEntity,
                ));
            }
        }
    }
}

#[derive(Resource)]
pub struct Car {
    parts: HashMap<i32, PartDB>,
}

pub trait CarFuncs {
    fn get_render_layer_of_part(&self, part: &PartDB) -> f32;
    fn can_or_is_attached_part(&self, part: &PartDB) -> bool;
    fn remove_part(&mut self, part_id: i32);
    fn attempt_add_part(&mut self, part: &PartDB);
    fn sync_parts(&mut self, parts: Vec<&PartDB>);
}

impl CarFuncs for Car {
    fn get_render_layer_of_part(&self, part: &PartDB) -> f32 {
        match self
            .parts
            .clone()
            .into_iter()
            .find(|(_, s)| s.part_id == part.part_id)
        {
            None => 0.,
            Some(part) => part.1.new.first().map_or(0., |new| new.point1.y as f32),
        }
    }
    fn can_or_is_attached_part(&self, part: &PartDB) -> bool {
        if self.parts.contains_key(&part.part_id) {
            return true;
        }
        let all_covers: Vec<String> = self
            .parts
            .clone()
            .into_iter()
            .flat_map(|(_, s)| s.covers)
            .collect();
        let all_available_news: Vec<String> = self
            .parts
            .values()
            .flat_map(|s| s.new.clone())
            .filter_map(|s| {
                if all_covers.contains(&s.tag) {
                    None
                } else {
                    Some(s.tag)
                }
            })
            .collect();
        part.requires.iter().all(|s| all_available_news.contains(s))
    }
    fn remove_part(&mut self, part_id: i32) {
        println!("removed part {part_id}");
        self.parts.remove(&part_id);
    }
    fn attempt_add_part(&mut self, part: &PartDB) {
        if !self.parts.contains_key(&part.part_id) {
            println!("added part {}", part.part_id);
            self.parts.insert(part.part_id, part.to_owned());
        }
    }

    fn sync_parts(&mut self, found_parts: Vec<&PartDB>) {
        if !found_parts.is_empty() {
            for (id, part) in self.parts.clone() {
                if !found_parts.iter().any(|p| p.part_id == part.part_id) && id != 1 {
                    self.remove_part(id);
                }
            }
            for part in found_parts {
                // if part is not in carparts
                if !self.parts.contains_key(&part.part_id) {
                    self.attempt_add_part(part);
                }
            }
        }
    }
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
