use std::fs::File;
use std::io::{prelude::*, Cursor};

use bevy::prelude::*;
use bevy::tasks::ParallelIterator;
use bevy::utils::HashMap;
use lazy_static::lazy_static;
use regex::Regex;
use yore::code_pages::CP1252;

use crate::parsers::database_language::{MulleDB, MapData};
use crate::render::scaler::{HIGH_RES_LAYERS, PIXEL_PERFECT_LAYERS};
use crate::systems::mulle_asset_helper::{MulleAssetHelp, MulleAssetHelper, MulleDBHolder, self};
use crate::{despawn_screen, GameState};

pub struct WorldDrivePlugin;

impl Plugin for WorldDrivePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_maps)
            .add_systems(OnEnter(GameState::DaHood), setup_sprite)
            .add_systems(OnExit(GameState::DaHood), despawn_screen::<OnWorldDrive>)
            .add_systems(Update, update_map)
            .add_systems(Update, control_car);
    }
}

fn update_map(
    mut query: Query<&mut Handle<Image>, With<Background>>,
    car_state: Res<MulleCarState>,
    da_hood: Res<MulleWorldData>,
    _asset_server: Res<AssetServer>,
    mulle_asset_helper: Res<MulleAssetHelp>,
) {
    if car_state.is_changed() {
        for mut image_handle in query.iter_mut() {
            *image_handle = mulle_asset_helper
                .get_image_by_name(
                    "cddata.cxt".to_string(),
                    da_hood
                        .maps
                        .get(&car_state.current_map)
                        .unwrap()
                        .map
                        .map_image
                        .clone(),
                )
                .unwrap()
                .clone();
        }
    }
}

fn control_car(
    mut query: Query<&mut Transform, With<Car>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut car_state: ResMut<MulleCarState>,
    da_hood: Res<MulleWorldData>,
) {
    if let Ok(mut car_transform) = query.get_single_mut() {
        // get the car location mut

        // get the current mapmask
        let collission_mask = &da_hood
            .maps
            .get(&car_state.current_map)
            .unwrap()
            .collission_mask
            .data;

        let mut car_location = car_transform.translation.xyz();
        let _orig_car_location = car_transform.translation.xyz();

        if keyboard_input.pressed(KeyCode::Left) {
            car_location.x = car_location.x - 1.;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            car_location.x = car_location.x + 1.;
        }
        if keyboard_input.pressed(KeyCode::Up) {
            car_location.y = car_location.y + 1.;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            car_location.y = car_location.y - 1.;
        }

        let car_y = ((car_location.y * -1.) + 198. + 40.) / 2.;
        let car_x = (car_location.x + 316.) / 2.;

        eprintln!(
            "moving to map: {} mask space {} {}",
            &car_state.current_map, car_x as usize, car_y as usize
        );

        if car_x < COLS as f32 && car_x >= 0. && car_y < ROWS as f32 && car_y >= 0. {
            // eprint!("Tile type is {:02X}", collission_mask[car_y as usize][car_x as usize]);
            if collission_mask[car_y as usize][car_x as usize] != 0xf0 {
                // Correct coordinate space to fit positive numbers only, then divide as the colission space is only half the size
                // collission_mask[((orig_car_location.y+198.+40.)/2.) as usize][((orig_car_location.x+316.)/2.) as usize] == 0xf0  {  // Allow any movement if car is already outside of space

                car_transform.translation = car_location;
                // } else {
                //     eprint!("Drove into illegal tile at {} {}: tile here is {:x}", car_location.x, car_location.y, collission_mask[car_location.x as usize+316/2][car_location.y as usize+198/2]);
            }
        } else {
            // eprint!("car out of bounds!");
        }

        if let Some(transition_points) = TRANSITION_POINTS.get(&car_state.current_map) {
            for point in transition_points {
                if car_y >= point.min_point.y
                    && car_y <= point.max_point.y
                    && car_x >= point.min_point.x
                    && car_x <= point.max_point.x
                {
                    car_state.current_map = point.to_map;
                    if point.flip_x {
                        car_transform.translation.x = car_transform.translation.x * -1.;
                    }
                    if point.flip_y {
                        car_transform.translation.y = car_transform.translation.y * -1. + 80.;
                    }
                }
            }
        }
    }
}

#[derive(Resource)]
struct MulleCarState {
    current_map: u16,
}

#[derive(Component)]
struct Car;

#[derive(Component)]
struct Background;

// Tag component used to tag entities added on the splash screen
#[derive(Component)]
struct OnWorldDrive;

fn init_maps(mulle_asset_helper: Res<MulleAssetHelp>, mut commands: Commands) {
    // Load worldmap
    let mut da_hood = MulleWorldData {
        name: String::from("da hood"),
        maps: HashMap::new(),
    };

    for mapid in 661..688 {
        let map = parse_mapdb(mulle_asset_helper.get_mulle_db_by_asset_number("cddata.cxt".to_owned(), mapid as u32).unwrap()).unwrap();
        let topo = map.topology.clone();
        da_hood
            .maps
            .insert(mapid, MapCollissionData { 
                map: map,
                collission_mask: store_colission_mask(&topo, &mulle_asset_helper)
    });
    }

    let car_state = MulleCarState { current_map: 676 };

    commands.insert_resource(da_hood);
    commands.insert_resource(car_state);
}

fn setup_sprite(
    mut commands: Commands,
    mulle_asset_helper: Res<MulleAssetHelp>,
    da_hood: Res<MulleWorldData>,
    car_state: Res<MulleCarState>,
) {
    // Maybe have these only created once?

    // the sample sprite that will be rendered to the pixel-perfect canvas
    commands.spawn((
        SpriteBundle {
            texture: mulle_asset_helper
                .get_image_by_name(
                    "cddata.cxt".to_string(),
                    da_hood
                        .maps
                        .get(&car_state.current_map)
                        .unwrap()
                        .map
                        .map_image
                        .clone(),
                )
                .unwrap()
                .clone(),
            transform: Transform::from_xyz(0., 40., 0.),
            ..default()
        },
        OnWorldDrive,
        Background,
        PIXEL_PERFECT_LAYERS,
    ));

    commands.spawn((
        SpriteBundle {
            texture: mulle_asset_helper
                .get_image_by_asset_number("05.dxr".to_string(), 25)
                .unwrap()
                .clone(),
            transform: Transform::from_xyz(0., -198., 0.),
            ..default()
        },
        OnWorldDrive,
        PIXEL_PERFECT_LAYERS,
    ));

    // the sample sprite that will be rendered to the high-res "outer world"
    commands.spawn((
        SpriteBundle {
            texture: mulle_asset_helper
                .get_image_by_asset_number("05.dxr".to_string(), 101)
                .unwrap()
                .clone(),
            transform: Transform::from_xyz(5., 30., 2.),
            ..default()
        },
        OnWorldDrive,
        Car,
        HIGH_RES_LAYERS,
    ));
}

fn store_colission_mask(
    asset_name: &str,
    mulle_asset_helper: &Res<MulleAssetHelp>,
) -> MapCollissionMask {
    // Since collission masks actually are 2 files for some reason need to read 2!
    // Guess the name of the next file, it ends with -2
    let asset_name_part2 = String::from(asset_name) + "-2";
    let mut cursor_file_1 = Cursor::new(
        CP1252
            .encode(
                mulle_asset_helper
                    .get_mulle_text_by_name("cddata.cxt".to_string(), asset_name.to_string())
                    .unwrap()
                    .text
                    .as_str(),
            )
            .unwrap(),
    );
    let mut cursor_file_2 = Cursor::new(
        CP1252
            .encode(
                mulle_asset_helper
                    .get_mulle_text_by_name("cddata.cxt".to_string(), asset_name_part2.to_string())
                    .unwrap()
                    .text
                    .as_str(),
            )
            .unwrap(), //TODO this could be very expensive
    );

    // prepare a collission map to dump the contents into
    let mut col_map = MapCollissionMask {
        data: [[0u8; COLS]; ROWS],
    };

    let mut buffer = [0u8; COLS];

    for row in 0..ROWS / 2 {
        cursor_file_1.read_exact(&mut buffer).unwrap();

        // Copy the buffer to the corresponding row in the 2-dimensional array
        col_map.data[row].copy_from_slice(&buffer);
    }
    for row in ROWS / 2 + 1..ROWS {
        cursor_file_2.read_exact(&mut buffer).unwrap();

        // Copy the buffer to the corresponding row in the 2-dimensional array
        col_map.data[row].copy_from_slice(&buffer);
    }

    // let mut filehandle = File::create(format!("{}.txt", asset_name)).unwrap();

    // for y in 0..ROWS {
    //     for x in 0..COLS {
    //         write!(filehandle, "{:02X} ", col_map.data[y][x]);
    //     }
    //     write!(filehandle, "\n");
    // }

    col_map
}

lazy_static! {
    static ref MAPDB_REGEX: Regex = Regex::new(r#"\[#MapId: (?P<id>[0-9])+, #objects: \[(?P<objects>.*?)\], #MapImage: "(?P<mapimage>[^"]+)", #Topology: "(?P<topology>[^"]+)"]"#).unwrap();
}

fn parse_mapdb(mulle_db: &MulleDBHolder) -> Option<MapData> {
    if let MulleDB::MapData(mapdata) = &mulle_db.db {
        return Some(mapdata.clone())
    }
    None

    // // Open requested mapdb entry
    // match mulle_asset_helper.get_mulle_text_by_asset_number("cddata.cxt".to_owned(), mapnr as u32) {
    //     // this can be done better! REDUCE COMPLEXITY!
    //     Some(mapdb_mulle_text) => {
    //         // Once we have a reader on the file, read it into a buffer
    //         let mapbd_txt = mapdb_mulle_text.text.to_owned();
    //         // Create a Regex to parse the general structure
    //         if let Some(captures) = MAPDB_REGEX.captures(&mapbd_txt) {
    //             if let Ok(id) = &captures["id"].parse::<i32>() {
    //                 // From the regex captures create a MapData object, also immediatly handle the colission mask
    //                 let map_data = MapData {
    //                     map_id: id.to_owned(),
    //                     objects: Vec::<Object>::new(), //TODO fix objects parsing
    //                     map_image: captures["mapimage"].to_string(),
    //                     topology: store_colission_mask(&captures["topology"], mulle_asset_helper), // handle the colission mask
    //                 };
    //                 return Some(map_data);
    //             }
    //         }
    //     }
    //     None => {
    //         eprint!("Failed to find mapdb {}", mapnr.to_string())
    //     }
    // }
    // None
}

// [#MapId: 1, #objects: [[31, point(146,392), [#InnerRadius:50]], [19, point(390, 205), []], [6, point(120, 350), [#Show:1]]], #MapImage: "30b001v0", #Topology: "30t001v0"]

lazy_static! {
    static ref TRANSITION_POINTS: HashMap<u16, Vec::<MapTransition>> = {
        let m = HashMap::from([
            (661,
            Vec::from([
                MapTransition {
                    to_map: 667,
                    min_point: Vec2 { x: 63., y: 197. },
                    max_point: Vec2 { x: 80., y: 197. },
                    flip_y: true,
                    flip_x: false
                },
                MapTransition {
                    to_map: 662,
                    min_point: Vec2 { x: 315., y: 79. },
                    max_point: Vec2 { x: 315., y: 89. },
                    flip_y: false,
                    flip_x: true
                },
            ]),
            ),
            (662,
            Vec::from([
                MapTransition {
                    to_map: 661,
                    min_point: Vec2 { x: 0., y: 78. },
                    max_point: Vec2 { x: 0., y: 89. },
                    flip_x: true,
                    flip_y: false
                },
                MapTransition {
                    to_map: 668,
                    min_point: Vec2 { x: 242., y: 197. },
                    max_point: Vec2 { x: 254., y: 197. },
                    flip_x: false,
                    flip_y: true
                },
                MapTransition {
                    to_map: 663,
                    min_point: Vec2 { x: 315., y: 132. },
                    max_point: Vec2 { x: 315., y: 142. },
                    flip_x: false,
                    flip_y: true
                }
            ])
            ),
            (667,
                Vec::from([
                    MapTransition {
                        to_map: 661,
                        min_point: Vec2 { x: 63., y: 1. },
                        max_point: Vec2 { x: 80., y: 1. },
                        flip_x: false,
                        flip_y: true
                    },
                    // MapTransition {
                    //     to_map: ??
                    //     min_point: Vec { x: 201., y: 197. },
                    //     max_point: Vec { x: 211., y: 197. }
                    //     flip_x: false,
                    //     flip_y: true
                    // }//TODO fix this
                ])
            ),
            (668,
            Vec::from([
                MapTransition {
                    to_map: 662,
                    min_point: Vec2 { x: 242., y: 1. },
                    max_point: Vec2 { x: 254., y: 1. },
                    flip_x: false,
                    flip_y: true,
                },
                MapTransition {
                    to_map: 674,
                    min_point: Vec2 { x: 176., y:  197. },
                    max_point: Vec2 { x: 186., y:  197. },
                    flip_x: false,
                    flip_y: true
                },
                MapTransition {
                    to_map: 669,
                    min_point: Vec2 { x: 315., y: 26. },
                    max_point: Vec2 { x: 315., y: 35. },
                    flip_x: true,
                    flip_y: false
                }
            ])
            ),
            (669,
            Vec::from([
                MapTransition {
                    to_map: 668,
                    min_point: Vec2 { x: 0., y: 26. },
                    max_point: Vec2 { x: 0., y: 35. },
                    flip_x: true,
                    flip_y: false
                },
                MapTransition {
                    to_map: 663,
                    min_point: Vec2 { x: 100., y: 0. },
                    max_point: Vec2 { x: 111., y: 0. },
                    flip_x: false,
                    flip_y: true
                },
                MapTransition {
                    to_map: 670,
                    min_point: Vec2 { x: 315., y: 132. },
                    max_point: Vec2 { x: 315., y: 142. },
                    flip_x: true,
                    flip_y: false,
                },
                MapTransition {
                    to_map: 675,
                    min_point: Vec2 { x: 102., y: 197. },
                    max_point: Vec2 { x: 112., y: 197. },
                    flip_x: false,
                    flip_y: true
                }
            ])
            ),
            (670,
            Vec::from([
                MapTransition {
                    to_map: 669,
                    min_point: Vec2 { x: 0., y: 132. },
                    max_point: Vec2 { x: 0., y: 142. },
                    flip_x: true,
                    flip_y: false
                },
                MapTransition {
                    to_map: 676,
                    min_point: Vec2 { x: 147., y: 197. },
                    max_point: Vec2 { x: 155., y: 197. },
                    flip_x: false,
                    flip_y: true
                }
            ])
            ),
            (671,
            Vec::from([
                MapTransition {
                    to_map: 677,
                    min_point: Vec2 { x: 142., y: 197. },
                    max_point: Vec2 { x: 151., y: 197. },
                    flip_x: false,
                    flip_y: true
                }
            ])
            ),
            (674,
            Vec::from([
                MapTransition {
                    to_map: 675,
                    min_point: Vec2 { x: 315., y: 55.},
                    max_point: Vec2 { x: 315., y: 65.},
                    flip_x: true,
                    flip_y: false
                },
                MapTransition {
                    to_map: 668,
                    min_point: Vec2 { x: 176., y:  0. },
                    max_point: Vec2 { x: 186., y:  0. },
                    flip_x: false,
                    flip_y: true
                }
            ])
            ),
            (675,
            Vec::from([
                MapTransition {
                    to_map: 669,
                    min_point: Vec2 { x: 102., y: 0. },
                    max_point: Vec2 { x: 112., y: 0. },
                    flip_x: false,
                    flip_y: true
                },
                MapTransition {
                    to_map: 674,
                    min_point: Vec2 { x: 0., y: 55.},
                    max_point: Vec2 { x: 0., y: 65.},
                    flip_x: true,
                    flip_y: false
                },
                MapTransition {
                    to_map: 676,
                    min_point: Vec2 { x: 315., y: 31. },
                    max_point: Vec2 { x: 315., y: 37. },
                    flip_x: true,
                    flip_y: false
                }
            ])
            ),
            (676,
            Vec::from([
                MapTransition {
                    to_map: 675,
                    min_point: Vec2 { x: 0., y: 31. },
                    max_point: Vec2 { x: 0., y: 37. },
                    flip_x: true,
                    flip_y: false
                },
                MapTransition {
                    to_map: 677,
                    min_point: Vec2 { x: 315., y: 115. },
                    max_point: Vec2 { x: 315., y: 124. },
                    flip_x: true,
                    flip_y: false
                },
                MapTransition {
                    to_map: 670,
                    min_point: Vec2 { x: 147., y: 0. },
                    max_point: Vec2 { x: 155., y: 0. },
                    flip_x: false,
                    flip_y: true
                }
            ])
            ),
            (677,
            Vec::from([
                MapTransition {
                    to_map: 671,
                    min_point: Vec2 { x: 142., y: 0. },
                    max_point: Vec2 { x: 151., y: 0. },
                    flip_x: false,
                    flip_y: true
                },
                MapTransition {
                    to_map: 678,
                    min_point: Vec2 { x: 315., y: 61. },
                    max_point: Vec2 { x: 315., y: 69. },
                    flip_x: true,
                    flip_y: false
                },
                MapTransition {
                    to_map: 683,
                    min_point: Vec2 { x: 77., y: 197. },
                    max_point: Vec2 { x: 93., y: 197. },
                    flip_x: false,
                    flip_y: true
                }
            ])
            ),
            (678,
            Vec::from([
                MapTransition {
                    to_map: 677,
                    min_point: Vec2 { x: 0., y: 61. },
                    max_point: Vec2 { x: 0., y: 69. },
                    flip_x: true,
                    flip_y: false
                },
                MapTransition {
                    to_map: 684,
                    min_point: Vec2 { x: 115., y: 197. },
                    max_point: Vec2 { x: 124., y: 197. },
                    flip_x: false,
                    flip_y: true
                }
            ])
            ),
            (683,
            Vec::from([
                MapTransition {
                    to_map: 677,
                    min_point: Vec2 { x: 77., y: 0. },
                    max_point: Vec2 { x: 93., y: 0. },
                    flip_x: false,
                    flip_y: true
                },
                MapTransition {
                    to_map: 684,
                    min_point: Vec2 { x: 315., y: 116. },
                    max_point: Vec2 { x: 315., y: 120. },
                    flip_x: true,
                    flip_y: false
                }
            ])
            ),
            (684,
            Vec::from([
                MapTransition {
                    to_map: 678,
                    min_point: Vec2 { x: 115., y: 0. },
                    max_point: Vec2 { x: 124., y: 0. },
                    flip_x: false,
                    flip_y: true
                },
                MapTransition {
                    to_map: 683,
                    min_point: Vec2 { x: 0., y: 116. },
                    max_point: Vec2 { x: 0., y: 120. },
                    flip_x: true,
                    flip_y: false
                },
                MapTransition {
                    to_map: 688,
                    min_point: Vec2 { x: 96., y: 197. },
                    max_point: Vec2 { x: 110., y: 197. },
                    flip_x: false,
                    flip_y: true
                }
            ])
            ),
            (688,
            Vec::from([
                MapTransition {
                    to_map: 684,
                    min_point: Vec2 { x: 96., y: 0. },
                    max_point: Vec2 { x: 110., y: 0. },
                    flip_x: false,
                    flip_y: true
                }
            ])
            )
        ]);
        m
    };
}

struct MapTransition {
    to_map: u16,
    min_point: Vec2,
    max_point: Vec2,
    flip_y: bool,
    flip_x: bool,
}

#[derive(Resource)]
struct MulleWorldData {
    name: String,
    maps: HashMap<u16, MapCollissionData>,
}

struct MapCollissionData {
    map: MapData, 
    collission_mask: MapCollissionMask
}

struct MapCollissionMask {
    data: [[u8; COLS]; ROWS],
}

const ROWS: usize = 198;
const COLS: usize = 316;

// MAP THINGS
// CDDATA.CXT files 515 to 561 contain "map objects"
// objects 661 to 688 contain "MAPDB" definitions
// MAPDB specifies "objects" "image" and "collision"
// "collision" maps seem to be 1 byte per pixel bitmaps presumably to indicate where you can drive, these objects are 693 to 748. they always come in pairs of two (ex. 693 is upper 694 is lower)
// in collission bytes the lower nibble seems to be used to indicate the "height" of terrain, the upper nibble marks "special terrains" such as 0x10 for rubble and 0x20 for mud

// /// Spawns a capsule mesh on the pixel-perfect layer.
// fn setup_car(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<ColorMaterial>>,
// ) {
//     commands.spawn((
//         MaterialMesh2dBundle {
//             mesh: meshes.add(Mesh::from(shape::Capsule::default())).into(),
//             transform: Transform::from_xyz(40., 0., 2.).with_scale(Vec3::splat(32.)),
//             material: materials.add(ColorMaterial::from(Color::BLACK)),
//             ..default()
//         },
//         Rotate,
//         PIXEL_PERFECT_LAYERS,
//     ));
// }
