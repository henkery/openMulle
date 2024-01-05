use bevy::prelude::*;
mod render;
mod screens;
mod systems;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state::<GameState>()
        .add_systems(Startup, setup)
        .add_plugins(systems::mulle_asset_helper::MulleAssetHelperPlugin)
        .add_plugins(render::scaler::ScalerPlugin)
        .add_plugins(systems::mulle_point_and_click::MullePointandClickPlugin)
        .add_plugins(screens::world_drive::WorldDrivePlugin)
        .add_plugins(screens::garage::GaragePlugin)
        .add_plugins(screens::yard::YardPlugin)
        .add_plugins(screens::trash_heap::TrashHeapPlugin)
        .run();
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
enum GameState {
    #[default]
    DaHood,
    Garage,
    Yard,
    TrashHeap,
}

fn setup(mut game_state: ResMut<NextState<GameState>>) {
    game_state.set(GameState::Yard);
}

// Generic system that takes a component as a parameter, and will despawn all entities with that component
fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
