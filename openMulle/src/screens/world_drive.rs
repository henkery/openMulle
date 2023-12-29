use std::fs::File;

use std::io::prelude::*;

use bevy::asset::io::file;
use bevy::prelude::*;
use bevy::utils::HashMap;
use lazy_static::lazy_static;
use regex::Regex;

use crate::render::scaler::{HIGH_RES_LAYERS, PIXEL_PERFECT_LAYERS};
use crate::systems::mulle_asset_helper::{MulleAssetHelp, MulleAssetHelper};
use crate::{despawn_screen, GameState};

pub struct WorldDrivePlugin;

impl Plugin for WorldDrivePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::DaHood), setup_sprite)
            .add_systems(OnExit(GameState::DaHood), despawn_screen::<OnWorldDrive>)
            .add_systems(Update, update_map)
            .add_systems(Update, control_car);
    }
}

fn update_map(
    mut query: Query<&mut Handle<Image>, With<Background>>,
    car_state: Res<MulleCarState>,
    da_hood: Res<MulleWorldData>,
    asset_server: Res<AssetServer>,
    mulle_asset_helper: Res<MulleAssetHelp>,
) {
    if car_state.is_changed() {
        for mut image_handle in query.iter_mut() {
            *image_handle = asset_server.load(
                mulle_asset_helper
                    .find_member_path_from_actor_name(
                        "cddata.cxt",
                        &da_hood.maps.get(&car_state.current_map).unwrap().map_image,
                        ".png",
                    )
                    .unwrap(),
            );
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
            .topology
            .data;

        let mut car_location = car_transform.translation.xyz();
        let orig_car_location = car_transform.translation.xyz();

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

        let car_y = (((car_location.y * -1.) + 198. + 40.) / 2.);
        let car_x = ((car_location.x + 316.) / 2.);

        eprintln!("moving to mask space {} {}", car_x as usize, car_y as usize);

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
                if car_y > point.min_point.y
                    && car_y < point.max_point.y
                    && car_x > point.min_point.x
                    && car_x < point.max_point.x
                {
                    car_state.current_map = point.to_map;
                } else {
                    eprintln!("missed marker!");
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

fn setup_sprite(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mulle_asset_helper: Res<MulleAssetHelp>,
) {
    // Load worldmap
    let mut da_hood = MulleWorldData {
        name: String::from("da hood"),
        maps: HashMap::new(),
    };

    for mapid in 661..688 {
        da_hood
            .maps
            .insert(mapid, parse_mapdb(&mulle_asset_helper, mapid).unwrap());
    }

    let car_state = MulleCarState { current_map: 661 };

    // Maybe have these only created once?

    // the sample sprite that will be rendered to the pixel-perfect canvas
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load(
                mulle_asset_helper
                    .find_member_path_from_actor_name(
                        "cddata.cxt",
                        da_hood
                            .maps
                            .get(&car_state.current_map)
                            .unwrap()
                            .map_image
                            .as_str(),
                        ".png",
                    )
                    .unwrap()
                    .display()
                    .to_string(),
            ),
            transform: Transform::from_xyz(0., 40., 0.),
            ..default()
        },
        OnWorldDrive,
        Background,
        PIXEL_PERFECT_LAYERS,
    ));

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load(
                mulle_asset_helper
                    .find_member_path("05.dxr", "25", ".png")
                    .unwrap()
                    .display()
                    .to_string(),
            ),
            transform: Transform::from_xyz(0., -198., 0.),
            ..default()
        },
        OnWorldDrive,
        PIXEL_PERFECT_LAYERS,
    ));

    // the sample sprite that will be rendered to the high-res "outer world"
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load(
                mulle_asset_helper
                    .find_member_path("05.dxr", "101", ".png")
                    .unwrap()
                    .display()
                    .to_string(),
            ),
            transform: Transform::from_xyz(-40., -22., 2.),
            ..default()
        },
        OnWorldDrive,
        Car,
        HIGH_RES_LAYERS,
    ));

    commands.insert_resource(da_hood);
    commands.insert_resource(car_state);
}

fn store_colission_mask(
    asset_name: &str,
    mulle_asset_helper: &Res<MulleAssetHelp>,
) -> MapCollissionMask {
    // Since collission masks actually are 2 files for some reason need to read 2!
    // Guess the name of the next file, it ends with -2
    let asset_name_part2 = String::from(asset_name) + "-2";
    let filename1 = mulle_asset_helper
        .find_member_path_with_asset_from_actor_name("cddata.cxt", asset_name, ".txt")
        .unwrap();
    let filename2 = mulle_asset_helper
        .find_member_path_with_asset_from_actor_name("cddata.cxt", &asset_name_part2, ".txt")
        .unwrap();

    // prepare a collission map to dump the contents into
    let mut col_map = MapCollissionMask {
        data: [[0u8; COLS]; ROWS],
    };

    let mut buffer = [0u8; COLS];

    if let Ok(mut file_upper_reader) = File::open(filename1) {
        for row in 0..ROWS / 2 {
            file_upper_reader.read_exact(&mut buffer).unwrap();

            // Copy the buffer to the corresponding row in the 2-dimensional array
            col_map.data[row].copy_from_slice(&buffer);
        }
    }
    if let Ok(mut file_lower_reader) = File::open(filename2) {
        for row in ROWS / 2 + 1..ROWS {
            file_lower_reader.read_exact(&mut buffer).unwrap();

            // Copy the buffer to the corresponding row in the 2-dimensional array
            col_map.data[row].copy_from_slice(&buffer);
        }
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

fn parse_mapdb(mulle_asset_helper: &Res<MulleAssetHelp>, mapnr: u16) -> Option<MapData> {
    // Open requested mapdb entry
    match mulle_asset_helper.find_member_path_with_asset(
        "cddata.cxt",
        mapnr.to_string().as_str(),
        ".txt",
    ) {
        // this can be done better! REDUCE COMPLEXITY!
        Some(mapdb_path) => {
            match File::open(mapdb_path.to_owned()) {
                Ok(mut mapdb_rdr) => {
                    // Once we have a reader on the file, read it into a buffer
                    let mut mapbd_txt = String::new();
                    _ = mapdb_rdr.read_to_string(&mut mapbd_txt);
                    // Create a Regex to parse the general structure
                    let mapdb_re = Regex::new(r#"\[#MapId: (?P<id>[0-9])+, #objects: \[(?P<objects>.*?)\], #MapImage: "(?P<mapimage>[^"]+)", #Topology: "(?P<topology>[^"]+)"]"#).unwrap();
                    if let Some(captures) = mapdb_re.captures(&mapbd_txt) {
                        if let Ok(id) = &captures["id"].parse::<i32>() {
                            // From the regex captures create a MapData object, also immediatly handle the colission mask
                            let map_data = MapData {
                                map_id: id.to_owned(),
                                objects: Vec::<Object>::new(), //TODO fix objects parsing
                                map_image: captures["mapimage"].to_string(),
                                topology: store_colission_mask(
                                    &captures["topology"],
                                    mulle_asset_helper,
                                ), // handle the colission mask
                            };
                            return Some(map_data);
                        }
                    }
                }
                Err(error) => {
                    eprint!("Failed to open mapdb {}: {}", mapdb_path.display(), error)
                }
            }
        }
        None => {
            eprint!("Failed to find mapdb {}", mapnr.to_string())
        }
    }
    None
}

// [#MapId: 1, #objects: [[31, point(146,392), [#InnerRadius:50]], [19, point(390, 205), []], [6, point(120, 350), [#Show:1]]], #MapImage: "30b001v0", #Topology: "30t001v0"]
struct Point {
    x: i32,
    y: i32,
}

enum InnerValue {
    InnerRadius(i32),
    Show(i32),
}

struct Object {
    id: i32,
    point: Point,
    inner_values: Vec<InnerValue>,
}

struct MapData {
    map_id: i32,
    objects: Vec<Object>,
    map_image: String,
    topology: MapCollissionMask,
}

lazy_static! {
    static ref TRANSITION_POINTS: HashMap<u16, Vec::<MapTransition>> = {
        let m = HashMap::from([(
            661,
            Vec::from([
                MapTransition {
                    to_map: 662,
                    min_point: Vec2 { x: 63., y: 197. },
                    max_point: Vec2 { x: 80., y: 197. },
                },
                MapTransition {
                    to_map: 663,
                    min_point: Vec2 { x: 315., y: 79. },
                    max_point: Vec2 { x: 315., y: 89. },
                },
            ]),
        )]);
        m
    };
}

struct MapTransition {
    to_map: u16,
    min_point: Vec2,
    max_point: Vec2,
}

#[derive(Resource)]
struct MulleWorldData {
    name: String,
    maps: HashMap<u16, MapData>,
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
