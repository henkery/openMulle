use crate::render::scaler::PIXEL_PERFECT_LAYERS;
use crate::systems::mulle_asset_helper::{MulleAssetHelp, MulleAssetHelper};
use crate::systems::mulle_point_and_click::{
    deploy_clickables, mulle_clickable_from_name, ClickAction, MulleClickable,
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

fn setup_garage(mut commands: Commands, mulle_asset_helper: Res<MulleAssetHelp>) {}
