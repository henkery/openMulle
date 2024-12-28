use std::fs::{self, File};

use crate::render::scaler::PIXEL_PERFECT_LAYERS;
use crate::systems::mulle_asset_helper::{MulleAssetHelp, MulleAssetHelper};
use crate::systems::mulle_point_and_click::{
    deploy_clickables, mulle_clickable_from_name, MulleClickable, MulleClickableSerializable,
};
use crate::{despawn_screen, GameState};
use bevy::prelude::*;
use bevy::utils::hashbrown::hash_map::Values;
use bevy::utils::hashbrown::HashMap;
use serde::{Deserialize, Serialize};

pub struct YardPlugin;

impl Plugin for YardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, setup_rooms)
            .add_systems(
                PreUpdate,
                despawn_screen::<OnRoomScreen>.run_if(resource_changed::<RoomState>),
            )
            .add_systems(OnExit(GameState::Room), despawn_screen::<OnRoomScreen>)
            .add_systems(Update, build_room.run_if(resource_changed::<RoomState>));
    }
}

// Tag component used to tag entities added on the splash screen
#[derive(Component, Clone)]
struct OnRoomScreen;

#[derive(Resource, Serialize, Deserialize)]
pub struct RoomState {
    pub current_room: String,
}

#[derive(Serialize, Deserialize)]
struct RoomsSerializable {
    rooms: HashMap<String, MulleRoomSerializable>,
}

#[derive(Resource)]
struct Rooms {
    rooms: HashMap<String, MulleRoom>,
}

trait RoomsConvert {
    fn convert(
        rooms_serializable: RoomsSerializable,
        mulle_asset_helper: &bevy::prelude::Res<'_, MulleAssetHelp>,
    ) -> Rooms;
}

impl RoomsConvert for Rooms {
    fn convert(
        rooms_serializable: RoomsSerializable,
        mulle_asset_helper: &bevy::prelude::Res<'_, MulleAssetHelp>,
    ) -> Rooms {
        let mut rooms: HashMap<String, MulleRoom> = HashMap::default();

        for (k, v) in rooms_serializable.rooms {
            rooms.insert(k, MulleRoom::convert(v, mulle_asset_helper));
        }

        Self { rooms }
    }
}

fn setup_rooms(mut commands: Commands, mulle_asset_helper: Res<MulleAssetHelp>) {
    commands.insert_resource(RoomState {
        current_room: "yard".into(),
    });

    let json_content = fs::read_to_string("assets/rooms.json").expect("Failed to read rooms file!");

    let rooms: RoomsSerializable =
        serde_json::from_str(&json_content).expect("Failed to parse json file");

    commands.insert_resource(Rooms::convert(rooms, &mulle_asset_helper));
}

fn build_room(
    mut commands: Commands,
    mulle_asset_helper: Res<MulleAssetHelp>,
    rooms: Res<Rooms>,
    roomstate: Res<RoomState>,
) {
    print!("Switched room {}", roomstate.current_room);
    let room = rooms
        .rooms
        .get(&roomstate.current_room)
        .unwrap_or_else(|| panic!("Failed to get requested room: {}", roomstate.current_room));
    // Render background
    commands.spawn((
        mulle_asset_helper
            .get_image_by_asset_number(
                room.background_asset_ref.to_string(),
                room.background_asset_number,
            )
            .unwrap()
            .clone(),
        Transform::from_xyz(0., 0., 0.),
        OnRoomScreen,
        PIXEL_PERFECT_LAYERS,
    ));

    deploy_clickables::<OnRoomScreen, Values<String, MulleClickable>>(
        commands,
        room.clickables.values(),
        OnRoomScreen,
    );
}

#[derive(Deserialize, Serialize)]
struct MulleRoomSerializable {
    background_asset_ref: String,
    background_asset_number: u32,
    clickables: HashMap<String, MulleClickableSerializable>,
}

struct MulleRoom {
    background_asset_ref: String,
    background_asset_number: u32,
    clickables: HashMap<String, MulleClickable>,
}

trait MulleRoomConvert {
    fn convert(
        value: MulleRoomSerializable,
        mulle_asset_helper: &bevy::prelude::Res<'_, MulleAssetHelp>,
    ) -> MulleRoom;
}

impl MulleRoomConvert for MulleRoom {
    fn convert(
        value: MulleRoomSerializable,
        mulle_asset_helper: &bevy::prelude::Res<'_, MulleAssetHelp>,
    ) -> Self {
        let clickables_vec: Vec<(String, MulleClickable)> = value
            .clickables
            .iter()
            .map(|(name, clickable)| {
                (
                    name.clone(),
                    mulle_clickable_from_name(
                        clickable.click.clone(),
                        &clickable.sprite_default_asset_dir,
                        clickable.sprite_default_asset_number,
                        &clickable.sprite_hover_asset_dir,
                        clickable.sprite_hover_asset_number,
                        mulle_asset_helper,
                    ),
                )
            })
            .collect();
        let mut clickables: HashMap<String, MulleClickable> = HashMap::default();
        for (k, v) in clickables_vec {
            clickables.insert(k, v);
        }
        Self {
            background_asset_number: value.background_asset_number,
            background_asset_ref: value.background_asset_ref,
            clickables,
        }
    }
}
