use crate::render::scaler::PIXEL_PERFECT_LAYERS;
use crate::systems::mulle_asset_helper::{MulleAssetHelp, MulleAssetHelper};
use crate::systems::mulle_point_and_click::{
    deploy_clickables, mulle_clickable_from_name, ClickAction,
};
use crate::{despawn_screen, GameState};
use bevy::prelude::*;

pub struct TrashHeapPlugin;

impl Plugin for TrashHeapPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<TrashState>()
            .add_systems(OnEnter(GameState::TrashHeap), setup_yard)
            .add_systems(OnExit(GameState::TrashHeap), cleanup_trash)
            .add_systems(OnEnter(TrashState::Yellow), setup_trash_yellow)
            .add_systems(
                OnExit(TrashState::Yellow),
                despawn_screen::<OnTrashYellowScreen>,
            )
            .add_systems(OnEnter(TrashState::Red), setup_trash_red)
            .add_systems(OnExit(TrashState::Red), despawn_screen::<OnTrashRedScreen>)
            .add_systems(OnEnter(TrashState::Purple), setup_trash_purple)
            .add_systems(
                OnExit(TrashState::Purple),
                despawn_screen::<OnTrashPurpleScreen>,
            )
            .add_systems(OnEnter(TrashState::Blue), setup_trash_blue)
            .add_systems(
                OnExit(TrashState::Blue),
                despawn_screen::<OnTrashBlueScreen>,
            )
            .add_systems(OnEnter(TrashState::Turquise), setup_trash_turquise)
            .add_systems(
                OnExit(TrashState::Turquise),
                despawn_screen::<OnTrashTurquiseScreen>,
            )
            .add_systems(OnEnter(TrashState::Green), setup_trash_green)
            .add_systems(
                OnExit(TrashState::Green),
                despawn_screen::<OnTrashGreenScreen>,
            );
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
pub enum TrashState {
    #[default]
    None,
    Yellow,
    Red,
    Purple,
    Blue,
    Turquise,
    Green,
}

#[derive(Component, Clone)]
struct OnTrashYellowScreen;
#[derive(Component, Clone)]
struct OnTrashRedScreen;
#[derive(Component, Clone)]
struct OnTrashPurpleScreen;
#[derive(Component, Clone)]
struct OnTrashBlueScreen;
#[derive(Component, Clone)]
struct OnTrashTurquiseScreen;
#[derive(Component, Clone)]
struct OnTrashGreenScreen;

fn setup_yard(mut trash_state: ResMut<NextState<TrashState>>) {
    trash_state.set(TrashState::Yellow);
}

fn cleanup_trash(mut trash_state: ResMut<NextState<TrashState>>) {
    trash_state.set(TrashState::None);
}

fn setup_trash_yellow(mut commands: Commands, mulle_asset_helper: Res<MulleAssetHelp>) {
    commands.spawn((
        SpriteBundle {
            texture: mulle_asset_helper
                .get_image_by_name("02.dxr".to_string(), 66)
                .unwrap()
                .clone(),
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        },
        OnTrashYellowScreen,
        PIXEL_PERFECT_LAYERS,
    ));

    deploy_clickables::<OnTrashYellowScreen>(
        commands,
        &[
            mulle_clickable_from_name(
                ClickAction::ActionGamestateTransition {
                    goal_state: GameState::Garage,
                },
                "02.dxr",
                85,
                "02.dxr",
                86,
                &mulle_asset_helper,
            ),
            mulle_clickable_from_name(
                ClickAction::ActionTrashstateTransition {
                    goal_state: TrashState::Green,
                },
                "02.dxr",
                122,
                "02.dxr",
                123,
                &mulle_asset_helper,
            ),
            mulle_clickable_from_name(
                ClickAction::ActionTrashstateTransition {
                    goal_state: TrashState::Red,
                },
                "02.dxr",
                110,
                "02.dxr",
                111,
                &mulle_asset_helper,
            ),
        ],
        OnTrashYellowScreen,
    );
}

fn setup_trash_red(mut commands: Commands, mulle_asset_helper: Res<MulleAssetHelp>) {
    commands.spawn((
        SpriteBundle {
            texture: mulle_asset_helper
                .get_image_by_name("02.dxr".to_string(), 71)
                .unwrap()
                .clone(),
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        },
        OnTrashRedScreen,
        PIXEL_PERFECT_LAYERS,
    ));

    deploy_clickables::<OnTrashRedScreen>(
        commands,
        &[
            mulle_clickable_from_name(
                ClickAction::ActionGamestateTransition {
                    goal_state: GameState::Garage,
                },
                "02.dxr",
                87,
                "02.dxr",
                88,
                &mulle_asset_helper,
            ),
            mulle_clickable_from_name(
                ClickAction::ActionTrashstateTransition {
                    goal_state: TrashState::Yellow,
                },
                "02.dxr",
                124,
                "02.dxr",
                125,
                &mulle_asset_helper,
            ),
            mulle_clickable_from_name(
                ClickAction::ActionTrashstateTransition {
                    goal_state: TrashState::Purple,
                },
                "02.dxr",
                112,
                "02.dxr",
                113,
                &mulle_asset_helper,
            ),
        ],
        OnTrashRedScreen,
    );
}

fn setup_trash_purple(mut commands: Commands, mulle_asset_helper: Res<MulleAssetHelp>) {
    commands.spawn((
        SpriteBundle {
            texture: mulle_asset_helper
                .get_image_by_name("02.dxr".to_string(), 70)
                .unwrap()
                .clone(),
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        },
        OnTrashPurpleScreen,
        PIXEL_PERFECT_LAYERS,
    ));

    deploy_clickables::<OnTrashPurpleScreen>(
        commands,
        &[
            mulle_clickable_from_name(
                ClickAction::ActionGamestateTransition {
                    goal_state: GameState::Garage,
                },
                "02.dxr",
                89,
                "02.dxr",
                90,
                &mulle_asset_helper,
            ),
            mulle_clickable_from_name(
                ClickAction::ActionTrashstateTransition {
                    goal_state: TrashState::Red,
                },
                "02.dxr",
                126,
                "02.dxr",
                127,
                &mulle_asset_helper,
            ),
            mulle_clickable_from_name(
                ClickAction::ActionTrashstateTransition {
                    goal_state: TrashState::Blue,
                },
                "02.dxr",
                114,
                "02.dxr",
                115,
                &mulle_asset_helper,
            ),
        ],
        OnTrashPurpleScreen,
    );
}

fn setup_trash_blue(mut commands: Commands, mulle_asset_helper: Res<MulleAssetHelp>) {
    commands.spawn((
        SpriteBundle {
            texture: mulle_asset_helper
                .get_image_by_name("02.dxr".to_string(), 69)
                .unwrap()
                .clone(),
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        },
        OnTrashBlueScreen,
        PIXEL_PERFECT_LAYERS,
    ));

    deploy_clickables::<OnTrashBlueScreen>(
        commands,
        &[
            mulle_clickable_from_name(
                ClickAction::ActionGamestateTransition {
                    goal_state: GameState::Garage,
                },
                "02.dxr",
                91,
                "02.dxr",
                92,
                &mulle_asset_helper,
            ),
            mulle_clickable_from_name(
                ClickAction::ActionTrashstateTransition {
                    goal_state: TrashState::Green,
                },
                "02.dxr",
                116,
                "02.dxr",
                117,
                &mulle_asset_helper,
            ),
            mulle_clickable_from_name(
                ClickAction::ActionTrashstateTransition {
                    goal_state: TrashState::Purple,
                },
                "02.dxr",
                128,
                "02.dxr",
                129,
                &mulle_asset_helper,
            ),
        ],
        OnTrashBlueScreen,
    );
}

fn setup_trash_turquise(mut commands: Commands, mulle_asset_helper: Res<MulleAssetHelp>) {
    commands.spawn((
        SpriteBundle {
            texture: mulle_asset_helper
                .get_image_by_name("02.dxr".to_string(), 68)
                .unwrap()
                .clone(),
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        },
        OnTrashTurquiseScreen,
        PIXEL_PERFECT_LAYERS,
    ));

    deploy_clickables::<OnTrashTurquiseScreen>(
        commands,
        &[
            mulle_clickable_from_name(
                ClickAction::ActionGamestateTransition {
                    goal_state: GameState::Garage,
                },
                "02.dxr",
                93,
                "02.dxr",
                94,
                &mulle_asset_helper,
            ),
            mulle_clickable_from_name(
                ClickAction::ActionTrashstateTransition {
                    goal_state: TrashState::Blue,
                },
                "02.dxr",
                130,
                "02.dxr",
                131,
                &mulle_asset_helper,
            ),
            mulle_clickable_from_name(
                ClickAction::ActionTrashstateTransition {
                    goal_state: TrashState::Green,
                },
                "02.dxr",
                118,
                "02.dxr",
                119,
                &mulle_asset_helper,
            ),
        ],
        OnTrashTurquiseScreen,
    );
}

fn setup_trash_green(mut commands: Commands, mulle_asset_helper: Res<MulleAssetHelp>) {
    commands.spawn((
        SpriteBundle {
            texture: mulle_asset_helper
                .get_image_by_name("02.dxr".to_string(), 72)
                .unwrap()
                .clone(),
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        },
        OnTrashGreenScreen,
        PIXEL_PERFECT_LAYERS,
    ));

    deploy_clickables::<OnTrashGreenScreen>(
        commands,
        &[
            mulle_clickable_from_name(
                ClickAction::ActionGamestateTransition {
                    goal_state: GameState::Garage,
                },
                "02.dxr",
                95,
                "02.dxr",
                96,
                &mulle_asset_helper,
            ),
            mulle_clickable_from_name(
                ClickAction::ActionTrashstateTransition {
                    goal_state: TrashState::Turquise,
                },
                "02.dxr",
                120,
                "02.dxr",
                121,
                &mulle_asset_helper,
            ),
            mulle_clickable_from_name(
                ClickAction::ActionTrashstateTransition {
                    goal_state: TrashState::Yellow,
                },
                "02.dxr",
                132,
                "02.dxr",
                133,
                &mulle_asset_helper,
            ),
        ],
        OnTrashGreenScreen,
    );
}
