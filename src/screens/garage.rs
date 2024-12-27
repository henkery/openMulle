use crate::render::scaler::PIXEL_PERFECT_LAYERS;
use crate::systems::mulle_asset_helper::{MulleAssetHelp, MulleAssetHelper};
use crate::systems::mulle_point_and_click::{
    deploy_clickables, mulle_clickable_from_name, ClickAction,
};
use crate::{despawn_screen, GameState};
use bevy::prelude::*;

pub struct GaragePlugin;

impl Plugin for GaragePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GarageWithCar), setup_garage)
            .add_systems(OnEnter(GameState::GarageWithoutCar), setup_garage)
            .add_systems(
                OnExit(GameState::GarageWithoutCar),
                despawn_screen::<OnGarageScreen>,
            )
            .add_systems(
                OnExit(GameState::GarageWithCar),
                despawn_screen::<OnGarageScreen>,
            );
        // .add_systems(OnExit(GameState::Garage), destroy_clickables::<OnGarageScreen>);
    }
}

// Tag component used to tag entities added on the splash screen
#[derive(Component, Clone)]
struct OnGarageScreen;

fn setup_garage(mut commands: Commands, mulle_asset_helper: Res<MulleAssetHelp>) {
    // the sample sprite that will be rendered to the pixel-perfect canvas
    commands.spawn((
        SpriteBundle {
            sprite: Sprite::from(
                mulle_asset_helper
                    .get_image_by_asset_number("03.dxr".to_string(), 33)
                    .unwrap()
                    .clone(),
            ),
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        },
        OnGarageScreen,
        PIXEL_PERFECT_LAYERS,
    ));

    deploy_clickables::<OnGarageScreen>(
        commands,
        &[
            mulle_clickable_from_name(
                vec![ClickAction::GamestateTransition {
                    goal_state: GameState::TrashHeap,
                }],
                "03.dxr",
                34,
                "03.dxr",
                35,
                &mulle_asset_helper,
            ),
            mulle_clickable_from_name(
                vec![ClickAction::GamestateTransition {
                    goal_state: GameState::YardWithCar,
                }],
                "03.dxr",
                36,
                "03.dxr",
                37,
                &mulle_asset_helper,
            ),
            mulle_clickable_from_name(
                vec![ClickAction::GamestateTransition {
                    goal_state: GameState::YardWithoutCar,
                }],
                "03.dxr",
                38,
                "03.dxr",
                39,
                &mulle_asset_helper,
            ),
        ],
        OnGarageScreen,
    );
}
