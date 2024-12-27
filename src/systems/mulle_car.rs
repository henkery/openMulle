use bevy::{prelude::*, utils::hashbrown::HashMap};

use crate::{
    despawn_screen, parsers::database_language::Point, render::scaler::PIXEL_PERFECT_LAYERS,
    GameState,
};

use super::{
    mulle_asset_helper::{MulleAssetHelp, MulleAssetHelper},
    mulle_point_and_click::MulleDraggable,
};

pub struct MulleCarPlugin;

impl Plugin for MulleCarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_car)
            .add_systems(OnEnter(GameState::GarageWithCar), spawn_car_parts)
            .add_systems(OnEnter(GameState::YardWithCar), spawn_car_parts)
            .add_systems(
                OnExit(GameState::GarageWithCar),
                despawn_screen::<CarEntity>,
            )
            .add_systems(OnExit(GameState::YardWithCar), despawn_screen::<CarEntity>);
    }
}
fn init_car(mut commands: Commands, mulle_asset_helper: Res<MulleAssetHelp>) {
    let car = Car {
        parts_locations: HashMap::from([
            {
                (
                    PartLocation::Car,
                    HashMap::from([
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
                )
            },
            { (PartLocation::Garage, HashMap::new()) },
            { (PartLocation::HeapBlue, HashMap::new()) },
            { (PartLocation::HeapGreen, HashMap::new()) },
            { (PartLocation::HeapPurple, HashMap::new()) },
            { (PartLocation::HeapRed, HashMap::new()) },
            { (PartLocation::HeapTurquise, HashMap::new()) },
            { (PartLocation::HeapYellow, HashMap::new()) },
            { (PartLocation::Yard, HashMap::new()) },
        ]),
    };
    commands.insert_resource(car);
}

#[derive(Component, Clone, Default)]
pub struct CarEntity;

enum UseViews {
    UseView1,
    UseView2,
}

fn spawn_car_parts(car: Res<Car>, mut commands: Commands, mulle_asset_helper: Res<MulleAssetHelp>) {
    for part in car
        .parts_locations
        .get(&PartLocation::Car)
        .expect("Failed to get carparts")
        .values()
    {
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
                    image.sprite.clone(),
                    Transform::from_xyz(
                        ((rect.max.x + rect.min.x) / 2.) + part.offset.x as f32, // It is a mystery why, but this entire scene seems offset by 40 to the back
                        ((rect.max.y + rect.min.y) / 2.) - part.offset.y as f32,
                        1.2, // how to layer stuff?
                    ),
                    PIXEL_PERFECT_LAYERS,
                    CarEntity,
                ));
            } else {
                commands.spawn((
                    image.sprite.clone(),
                    Transform::from_xyz(
                        ((rect.max.x + rect.min.x) / 2.) + part.offset.x as f32, // It is a mystery why, but this entire scene seems offset by 40 to the back
                        ((rect.max.y + rect.min.y) / 2.) - part.offset.y as f32,
                        layer, // how to layer stuff?
                    ),
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

#[derive(Eq, PartialEq, Hash, Debug)]
pub enum PartLocation {
    Car,
    Garage,
    Yard,
    HeapBlue,
    HeapGreen,
    HeapPurple,
    HeapRed,
    HeapTurquise,
    HeapYellow,
}

#[derive(Resource, Debug)]
pub struct Car {
    parts_locations: HashMap<PartLocation, HashMap<i32, PartDB>>,
}

pub enum MulleCarError<'a> {
    FailedToFindLocation(&'a str),
    FailedToGetPart(&'a str),
}

impl std::fmt::Display for MulleCarError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FailedToFindLocation(e) => {
                write!(f, "Failed to get parts database for location {}", e)
            }
            Self::FailedToGetPart(e) => write!(f, "Failed to get part data for part {}", e),
        }
    }
}

impl std::fmt::Debug for MulleCarError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

impl std::error::Error for MulleCarError<'_> {}

impl Car {
    pub fn get_render_layer_of_part(&self, part: &PartDB) -> f32 {
        self.parts_locations
            .get(&PartLocation::Car)
            .expect("Failed to get parts")
            .iter()
            .find(|(s, _)| **s == part.part_id)
            .map_or(0., |part| {
                part.1.new.first().map_or(0., |new| new.point1.y as f32)
            })
    }
    pub fn can_or_is_attached_part(&self, part: &PartDB) -> bool {
        let carparts = self
            .parts_locations
            .get(&PartLocation::Car)
            .expect("Failed to find carparts");
        if carparts.contains_key(&part.part_id) {
            return true;
        }
        let all_covers: Vec<String> = carparts
            .iter()
            .flat_map(|(_, s)| s.covers.clone())
            .collect();
        let all_available_news: Vec<String> = carparts
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
    pub fn move_part(
        &mut self,
        part_id: i32,
        from_location: &PartLocation,
        to_location: &PartLocation,
        _location: Option<Point>,
    ) -> Result<(), MulleCarError> {
        println!(
            "Moving part {part_id} from {:?} to {:?}",
            from_location, to_location
        );
        let part = self
            .parts_locations
            .get_mut(from_location)
            .ok_or(MulleCarError::FailedToFindLocation(
                "Failed to get collection",
            ))?
            .remove(&part_id)
            .ok_or(MulleCarError::FailedToGetPart("Failed to get item back"))?;
        self.parts_locations
            .get_mut(to_location)
            .ok_or(MulleCarError::FailedToFindLocation(
                "Failed to get collection",
            ))?
            .insert(part_id, part);
        Ok(())
    }
    // pub fn add_part(&mut self, part: &PartDB, location: PartLocation, position: Option<Point>) {
    //     let parts = self
    //         .parts_locations
    //         .get_mut(&location)
    //         .expect("Failed to get parts");
    //     if !parts.contains_key(&part.part_id) {
    //         println!("added part {}", part.part_id);
    //         parts.insert(part.part_id, part.to_owned());
    //     }
    // }

    pub fn sync_parts(&mut self, found_parts: Vec<&PartDB>, location: &PartLocation) {
        let parts = self
            .parts_locations
            .get_mut(&PartLocation::Car)
            .expect("Didn't get parts")
            .to_owned();

        for (id, part) in &parts {
            if !found_parts.iter().any(|p| p.part_id == part.part_id) && *id != 1 {
                let result = self.move_part(*id, &PartLocation::Car, location, None);
                println!("{:?}", result);
            }
        }
        for part in found_parts {
            // if part is not in carparts
            if !parts.contains_key(&part.part_id) {
                let result = self.move_part(part.part_id, location, &PartLocation::Car, None);
                println!("{:?}", result);
            }
        }
    }

    // pub fn remove_part(&mut self, part_id: i32, location: PartLocation) {
    //     self.parts_locations
    //         .get_mut(&location)
    //         .expect("Didn't get parts")
    //         .remove(&part_id)
    //         .expect("Failed to remove part");
    // }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
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
#[allow(dead_code)]
pub struct PartNew {
    pub tag: String,
    pub point1: Point,
    pub point2: Point,
}
