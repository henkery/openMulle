use crate::render::scaler::{HIGH_RES_LAYERS, PIXEL_PERFECT_LAYERS};
use crate::systems::mulle_asset_helper::{MulleAssetHelp, MulleAssetHelper};
use crate::systems::mulle_point_and_click::{
    deploy_clickables, destroy_clickables, mulle_clickable_from_name, ClickAction,
};
use crate::{despawn_screen, GameState};
use bevy::prelude::*;

pub struct GaragePlugin;

impl Plugin for GaragePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Garage), setup_garage)
            .add_systems(OnExit(GameState::Garage), despawn_screen::<OnGarageScreen>);
        // .add_systems(OnExit(GameState::Garage), destroy_clickables::<OnGarageScreen>);
    }
}

// Tag component used to tag entities added on the splash screen
#[derive(Component, Clone)]
struct OnGarageScreen;

fn setup_garage(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mulle_asset_helper: Res<MulleAssetHelp>,
) {
    // the sample sprite that will be rendered to the pixel-perfect canvas
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load(
                mulle_asset_helper
                    .find_member_path("03.dxr", "33", ".png")
                    .unwrap()
                    .display()
                    .to_string(),
            ),
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        },
        OnGarageScreen,
        PIXEL_PERFECT_LAYERS,
    ));

    deploy_clickables::<OnGarageScreen>(
        commands,
        asset_server,
        &[
            mulle_clickable_from_name(
                ClickAction::ActionGamestateTransition {
                    goal_state: GameState::TrashHeap,
                },
                "03.dxr",
                "34",
                "03.dxr",
                "35",
                &mulle_asset_helper,
            ),
            mulle_clickable_from_name(
                ClickAction::ActionGamestateTransition {
                    goal_state: GameState::Yard,
                },
                "03.dxr",
                "36",
                "03.dxr",
                "37",
                &mulle_asset_helper,
            ),
            mulle_clickable_from_name(
                ClickAction::ActionGamestateTransition {
                    goal_state: GameState::Yard,
                },
                "03.dxr",
                "38",
                "03.dxr",
                "39",
                &mulle_asset_helper,
            ),
        ],
        OnGarageScreen,
    );
}
