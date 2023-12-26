use bevy::{prelude::*};
mod screens;
mod render;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state::<GameState>()
        .add_systems(Startup, setup)
        .add_plugins(screens::world_drive::WorldDrivePlugin)
        .run();
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
enum GameState {
    #[default]
    WorldDrive,
    Finished,
}

// #[derive(Resource, Default)]
// struct RpgSpriteFolder(Handle<LoadedFolder>);

// fn load_textures(mut commands: Commands, asset_server: Res<AssetServer>) {
//     // load multiple, individual sprites from a folder
//     commands.insert_resource(RpgSpriteFolder(asset_server.load_folder("cst_out_new/00.CXT/Standalone")));
// }

// fn check_textures(
//     mut next_state: ResMut<NextState<AppState>>,
//     rpg_sprite_folder: ResMut<RpgSpriteFolder>,
//     mut events: EventReader<AssetEvent<LoadedFolder>>,
// ) {
//     // Advance the `AppState` once all sprite handles have been loaded by the `AssetServer`
//     for event in events.read() {
//         if event.is_loaded_with_dependencies(&rpg_sprite_folder.0) {
//             next_state.set(AppState::Finished);
//         }
//     }
// }

// #[derive(Component)]
// struct AnimationIndices {
//     first: usize,
//     last: usize,
// }


// #[derive(Component, Deref, DerefMut)]
// struct AnimationTimer(Timer);

// fn animate_sprite(
//     time: Res<Time>,
//     mut query: Query<(
//         &AnimationIndices,
//         &mut AnimationTimer,
//         &mut TextureAtlasSprite,
//     )>,
// ) {
//     for (indices, mut timer, mut sprite) in &mut query {
//         timer.tick(time.delta());
//         if timer.just_finished() {
//             sprite.index = if sprite.index == indices.last {
//                 indices.first
//             } else {
//                 sprite.index + 1
//             };
//         }
//     }
// }

fn setup(
    mut game_state: ResMut<NextState<GameState>>,
    // mut commands: Commands,
    // rpg_sprite_handles: Res<RpgSpriteFolder>,
    // asset_server: Res<AssetServer>,
    // loaded_folders: Res<Assets<LoadedFolder>>,
    // mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    // mut textures: ResMut<Assets<Image>>,
) {
    game_state.set(GameState::WorldDrive);
    // let mut texture_atlas_builder = TextureAtlasBuilder::default();
    // texture_atlas_builder = texture_atlas_builder.max_size(Vec2 { x:5000000.0, y: 5000000.0 });
    // let loaded_folder = loaded_folders.get(&rpg_sprite_handles.0).unwrap();
    // for handle in loaded_folder.handles.iter() {
    //     let id = handle.id().typed_unchecked::<Image>();
    //     let Some(texture) = textures.get(id) else {
    //         warn!(
    //             "{:?} did not resolve to an `Image` asset.",
    //             handle.path().unwrap()
    //         );
    //         continue;
    //     };

    //     texture_atlas_builder.add_texture(id, texture);
    // }

    // let texture_atlas = texture_atlas_builder.finish(&mut textures).unwrap();
    // let atlas_handle = texture_atlases.add(texture_atlas);

    // let animation_indices = AnimationIndices { first: 0, last: 5 };

    // commands.spawn(Camera2dBundle::default());
    // commands.spawn((
    //     SpriteSheetBundle {
    //         texture_atlas: atlas_handle,
    //         sprite: TextureAtlasSprite::new(animation_indices.first),
    //         transform: Transform::from_scale(Vec3::splat(6.0)),
    //         ..default()
    //     },
    //     animation_indices,
    //     AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    // ));

}