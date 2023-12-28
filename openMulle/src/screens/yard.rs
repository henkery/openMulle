use crate::render::scaler::{HIGH_RES_LAYERS, PIXEL_PERFECT_LAYERS};
use crate::systems::mulle_asset_helper::{MulleAssetHelp, MulleAssetHelper};
use crate::systems::mulle_point_and_click::{
    deploy_clickables, mulle_clickable_from_name, ClickAction,
};
use crate::{despawn_screen, GameState};
use bevy::prelude::*;

pub struct YardPlugin;

impl Plugin for YardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Yard), setup_yard)
            .add_systems(OnExit(GameState::Yard), despawn_screen::<OnYardScreen>);
    }
}

// Tag component used to tag entities added on the splash screen
#[derive(Component, Clone)]
struct OnYardScreen;

fn setup_yard(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mulle_asset_helper: Res<MulleAssetHelp>,
) {
    // the sample sprite that will be rendered to the pixel-perfect canvas
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load(
                mulle_asset_helper
                    .find_member_path("04.dxr", "145", ".png")
                    .unwrap()
                    .display()
                    .to_string(),
            ),
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        },
        OnYardScreen,
        PIXEL_PERFECT_LAYERS,
    ));

    deploy_clickables::<OnYardScreen>(
        commands,
        asset_server,
        &[
            mulle_clickable_from_name(
                ClickAction::ActionGamestateTransition {
                    goal_state: GameState::Garage,
                },
                "04.dxr",
                "13",
                "04.dxr",
                "14",
                &mulle_asset_helper,
            ),
            mulle_clickable_from_name(
                ClickAction::ActionGamestateTransition {
                    goal_state: GameState::DaHood,
                },
                "04.dxr",
                "16",
                "04.dxr",
                "17",
                &mulle_asset_helper,
            ),
            mulle_clickable_from_name(
                ClickAction::ActionGamestateTransition {
                    goal_state: GameState::Garage,
                },
                "04.dxr",
                "40",
                "04.dxr",
                "41",
                &mulle_asset_helper,
            ),
        ],
        OnYardScreen,
    );
}
