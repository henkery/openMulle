#![warn(clippy::nursery, clippy::unwrap_used, clippy::style)]
#![allow(clippy::unwrap_used)]
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
mod parsers;
mod render;
mod screens;
mod systems;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<GameState>()
        .add_plugins(systems::mulle_asset_helper::MulleAssetHelperPlugin)
        .add_plugins(render::scaler::ScalerPlugin)
        .add_plugins(systems::mulle_point_and_click::MullePointandClickPlugin)
        .add_plugins(systems::mulle_car::MulleCarPlugin)
        .add_plugins(screens::world_drive::WorldDrivePlugin)
        .add_plugins(screens::garage::GaragePlugin)
        .add_plugins(screens::yard::YardPlugin)
        // .add_plugins(screens::trash_heap::TrashHeapPlugin)
        .add_systems(PostStartup, set_init)
        .run();
}

fn set_init(mut game_state: ResMut<NextState<GameState>>) {
    game_state.set(GameState::Room);
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States, Serialize, Deserialize)]
enum GameState {
    None,
    #[default]
    DaHood,
    GarageWithoutCar,
    GarageWithCar,
    YardWithoutCar,
    YardWithCar,
    TrashHeap,
    Room,
}

// Generic system that takes a component as a parameter, and will despawn all entities with that component
fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
