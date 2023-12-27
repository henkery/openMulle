use std::path::PathBuf;

use bevy::{
    input::mouse::MouseButtonInput,
    prelude::*,
};
use crate::systems::mulle_point_and_click::{MulleClickables, mulle_clickable};
use crate::{GameState, despawn_screen};
use crate::render::scaler::{HIGH_RES_LAYERS, PIXEL_PERFECT_LAYERS};

pub struct GaragePlugin;

impl Plugin for GaragePlugin {
    fn build(&self, app: &mut App) {
        app
            // At start, the menu is not enabled. This will be changed in `menu_setup` when
            // entering the `GameState::WorldDrive` state.
            // Current screen in the menu is handled by an independent state from `GameState`
            .add_systems(OnEnter(GameState::Garage), setup_garage)
            .add_systems(OnExit(GameState::Garage), despawn_screen::<OnGarageScreen>);
            // Systems to handle the main menu screen
            // .add_systems(OnEnter(MenuState::Main), main_menu_setup)
            // .add_systems(OnExit(MenuState::Main), despawn_screen::<OnMainMenuScreen>)
            // // Systems to handle the settings menu screen
            // .add_systems(OnEnter(MenuState::Settings), settings_menu_setup)
            // .add_systems(
            //     OnExit(MenuState::Settings),
            //     despawn_screen::<OnSettingsMenuScreen>,
            // )
            // // Systems to handle the display settings screen
            // .add_systems(
            //     OnEnter(MenuState::SettingsDisplay),
            //     display_settings_menu_setup,
            // )
            // .add_systems(
            //     Update,
            //     (
            //         setting_button::<DisplayQuality>
            //             .run_if(in_state(MenuState::SettingsDisplay)),
            //     ),
            // )
            // .add_systems(
            //     OnExit(MenuState::SettingsDisplay),
            //     despawn_screen::<OnDisplaySettingsMenuScreen>,
            // )
            // // Systems to handle the sound settings screen
            // .add_systems(OnEnter(MenuState::SettingsSound), sound_settings_menu_setup)
            // .add_systems(
            //     Update,
            //     setting_button::<Volume>.run_if(in_state(MenuState::SettingsSound)),
            // )
            // .add_systems(
            //     OnExit(MenuState::SettingsSound),
            //     despawn_screen::<OnSoundSettingsMenuScreen>,
            // )
            // Common systems to all screens that handles buttons behavior
            // .add_systems(
            //     Update,
            //     (menu_action, button_system).run_if(in_state(GameState::WorldDrive)),
            // );
    }
}

// Tag component used to tag entities added on the splash screen
#[derive(Component)]
struct OnGarageScreen;

fn setup_garage(mut commands: Commands, asset_server: Res<AssetServer>, mut clickables: ResMut<MulleClickables>,) {
    // the sample sprite that will be rendered to the pixel-perfect canvas
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("cst_out_new/03.DXR/Internal/33.png"),
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        },
        OnGarageScreen,
        PIXEL_PERFECT_LAYERS,
    ));

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("cst_out_new/05.DXR/Internal/25.png"),
            transform: Transform::from_xyz(0., -198., 0.),
            ..default()
        },
        OnGarageScreen,
        PIXEL_PERFECT_LAYERS,
    ));

    // the sample sprite that will be rendered to the high-res "outer world"
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("cst_out_new/05.DXR/Internal/101.png"),
            transform: Transform::from_xyz(-40., -20., 2.),
            ..default()
        },
        OnGarageScreen,
        HIGH_RES_LAYERS,
    ));

    clickables.clickables.push(mulle_clickable(
        PathBuf::from("cst_out_new/03.DXR/Internal/34.png"),
        PathBuf::from("cst_out_new/03.DXR/Internal/35.png"),
        (),
        155., 242., 78., 227.));
}

fn mouse_click_system(mut mouse_button_input_events: EventReader<MouseButtonInput>) {
    for event in mouse_button_input_events.read() {
        info!("{:?}", event);
    }
}