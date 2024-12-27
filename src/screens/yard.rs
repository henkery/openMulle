use crate::render::scaler::PIXEL_PERFECT_LAYERS;
use crate::systems::mulle_asset_helper::{MulleAssetHelp, MulleAssetHelper};
use crate::systems::mulle_point_and_click::{
    deploy_clickables, mulle_clickable_from_name, ClickAction,
};
use crate::{despawn_screen, GameState};
use bevy::prelude::*;

pub struct YardPlugin;

impl Plugin for YardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::YardWithoutCar), setup_yard)
            .add_systems(OnEnter(GameState::YardWithCar), setup_yard)
            .add_systems(
                OnExit(GameState::YardWithCar),
                despawn_screen::<OnYardScreen>,
            )
            .add_systems(
                OnExit(GameState::YardWithoutCar),
                despawn_screen::<OnYardScreen>,
            );
    }
}

// Tag component used to tag entities added on the splash screen
#[derive(Component, Clone)]
struct OnYardScreen;

fn setup_yard(mut commands: Commands, mulle_asset_helper: Res<MulleAssetHelp>) {
    // the sample sprite that will be rendered to the pixel-perfect canvas
    commands.spawn((
        mulle_asset_helper
            .get_image_by_asset_number("04.dxr".to_string(), 145)
            .unwrap()
            .clone(),
        Transform::from_xyz(0., 0., 0.),
        OnYardScreen,
        PIXEL_PERFECT_LAYERS,
    ));

    deploy_clickables::<OnYardScreen>(
        commands,
        &[
            mulle_clickable_from_name(
                vec![ClickAction::GamestateTransition {
                    goal_state: GameState::GarageWithoutCar,
                }],
                "04.dxr",
                13,
                "04.dxr",
                14,
                &mulle_asset_helper,
            ),
            mulle_clickable_from_name(
                vec![ClickAction::GamestateTransition {
                    goal_state: GameState::DaHood,
                }],
                "04.dxr",
                16,
                "04.dxr",
                17,
                &mulle_asset_helper,
            ),
            mulle_clickable_from_name(
                vec![ClickAction::GamestateTransition {
                    goal_state: GameState::GarageWithCar,
                }],
                "04.dxr",
                40,
                "04.dxr",
                41,
                &mulle_asset_helper,
            ),
        ],
        OnYardScreen,
    );
}
