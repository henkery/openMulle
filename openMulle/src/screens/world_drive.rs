use bevy::prelude::*;

use crate::{GameState, despawn_screen};
use crate::render::scaler::{HIGH_RES_LAYERS, PIXEL_PERFECT_LAYERS};

pub struct WorldDrivePlugin;

impl Plugin for WorldDrivePlugin {
    fn build(&self, app: &mut App) {
        app
            // At start, the menu is not enabled. This will be changed in `menu_setup` when
            // entering the `GameState::WorldDrive` state.
            // Current screen in the menu is handled by an independent state from `GameState`
            .add_systems(OnEnter(GameState::WorldDrive), setup_sprite)
            .add_systems(OnExit(GameState::WorldDrive), despawn_screen::<OnWorldDrive>);
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

#[derive(Component)]
struct Rotate;

// Tag component used to tag entities added on the splash screen
#[derive(Component)]
struct OnWorldDrive;

fn setup_sprite(mut commands: Commands, asset_server: Res<AssetServer>) {
    // the sample sprite that will be rendered to the pixel-perfect canvas
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("cst_out_new/CDDATA.CXT/Standalone/644.png"),
            transform: Transform::from_xyz(0., 40., 0.),
            ..default()
        },
        OnWorldDrive,
        PIXEL_PERFECT_LAYERS,
    ));

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("cst_out_new/05.DXR/Internal/25.png"),
            transform: Transform::from_xyz(0., -198., 0.),
            ..default()
        },
        OnWorldDrive,
        PIXEL_PERFECT_LAYERS,
    ));

    // the sample sprite that will be rendered to the high-res "outer world"
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("cst_out_new/05.DXR/Internal/101.png"),
            transform: Transform::from_xyz(-40., -20., 2.),
            ..default()
        },
        OnWorldDrive,
        Rotate,
        HIGH_RES_LAYERS,
    ));
}

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