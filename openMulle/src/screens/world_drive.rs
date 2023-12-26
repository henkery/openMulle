use bevy::{
    prelude::*,
    render::{
        camera::RenderTarget,
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
        view::RenderLayers,
    },
    sprite::MaterialMesh2dBundle,
    window::WindowResized,
    app,
};

use crate::GameState;

pub struct WorldDrivePlugin;

impl Plugin for WorldDrivePlugin {
    fn build(&self, app: &mut App) {
        app
            // At start, the menu is not enabled. This will be changed in `menu_setup` when
            // entering the `GameState::WorldDrive` state.
            // Current screen in the menu is handled by an independent state from `GameState`
            .add_systems(OnEnter(GameState::WorldDrive), (setup_camera, setup_sprite))
            .add_systems(Update, (rotate, fit_canvas));
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

fn drive_setup(
    mut commands: Commands
) {
    commands.spawn(Camera2dBundle::default());
}

/// Low-resolution texture that contains the pixel-perfect world.
/// Canvas itself is rendered to the high-resolution world.
#[derive(Component)]
struct Canvas;

/// Camera that renders the pixel-perfect world to the [`Canvas`].
#[derive(Component)]
struct InGameCamera;

/// In-game resolution width.
const RES_WIDTH: u32 = 632;

/// In-game resolution height.
const RES_HEIGHT: u32 = 476;

/// Default render layers for pixel-perfect rendering.
/// You can skip adding this component, as this is the default.
const PIXEL_PERFECT_LAYERS: RenderLayers = RenderLayers::layer(0);

/// Render layers for high-resolution rendering.
const HIGH_RES_LAYERS: RenderLayers = RenderLayers::layer(1);


/// Camera that renders the [`Canvas`] (and other graphics on [`HIGH_RES_LAYERS`]) to the screen.
#[derive(Component)]
struct OuterCamera;

#[derive(Component)]
struct Rotate;

fn setup_sprite(mut commands: Commands, asset_server: Res<AssetServer>) {
    // the sample sprite that will be rendered to the pixel-perfect canvas
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("cst_out_new/CDDATA.CXT/Standalone/644.png"),
            transform: Transform::from_xyz(0., 40., 0.),
            ..default()
        },
        
        PIXEL_PERFECT_LAYERS,
    ));

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("cst_out_new/05.DXR/Internal/25.png"),
            transform: Transform::from_xyz(0., -198., 0.),
            ..default()
        },
        
        PIXEL_PERFECT_LAYERS,
    ));

    // the sample sprite that will be rendered to the high-res "outer world"
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("cst_out_new/05.DXR/Internal/101.png"),
            transform: Transform::from_xyz(-40., -20., 2.),
            ..default()
        },
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

fn setup_camera(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let canvas_size = Extent3d {
        width: RES_WIDTH,
        height: RES_HEIGHT,
        ..default()
    };

    // this Image serves as a canvas representing the low-resolution game screen
    let mut canvas = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size: canvas_size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };

    // fill image.data with zeroes
    canvas.resize(canvas_size);

    let image_handle = images.add(canvas);

    // this camera renders whatever is on `PIXEL_PERFECT_LAYERS` to the canvas
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                // render before the "main pass" camera
                order: -1,
                target: RenderTarget::Image(image_handle.clone()),
                ..default()
            },
            ..default()
        },
        InGameCamera,
        PIXEL_PERFECT_LAYERS,
    ));

    // spawn the canvas
    commands.spawn((
        SpriteBundle {
            texture: image_handle,
            ..default()
        },
        Canvas,
        HIGH_RES_LAYERS,
    ));

    // the "outer" camera renders whatever is on `HIGH_RES_LAYERS` to the screen.
    // here, the canvas and one of the sample sprites will be rendered by this camera
    commands.spawn((Camera2dBundle::default(), OuterCamera, HIGH_RES_LAYERS));
}

/// Rotates entities to demonstrate grid snapping.
fn rotate(time: Res<Time>, mut transforms: Query<&mut Transform, With<Rotate>>) {
    for mut transform in &mut transforms {
        let dt = time.delta_seconds();
        transform.rotate_z(dt);
    }
}

/// Scales camera projection to fit the window (integer multiples only).
fn fit_canvas(
    mut resize_events: EventReader<WindowResized>,
    mut projections: Query<&mut OrthographicProjection, With<OuterCamera>>,
) {
    for event in resize_events.read() {
        let h_scale = event.width / RES_WIDTH as f32;
        let v_scale = event.height / RES_HEIGHT as f32;
        let mut projection = projections.single_mut();
        projection.scale = 1. / h_scale.min(v_scale).round();
    }
}