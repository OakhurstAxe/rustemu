
// When compiling natively:
use std::time::Duration;

use bevy::prelude::*;
use rfd::FileDialog;

use nes::prelude::*;
use vcs::prelude::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum EmuAppState {
    #[default]
    Menu,
    NesGame,
    VcsGame,
}

#[derive(Component)]
struct NesButton;
pub struct NesPlugin<S: States> {
    pub state: S,
}

#[derive(Component)]
struct VcsButton;
pub struct VcsPlugin<S: States> {
    pub state: S,
}

impl<S: States> Plugin for NesPlugin<S> {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(Time::<Fixed>::from_duration(Duration::from_millis(17)))
        .add_systems(OnEnter(EmuAppState::NesGame), NesBevy::setup.run_if(in_state(EmuAppState::NesGame)))
        .add_systems(FixedUpdate, NesBevy::frame.run_if(in_state(EmuAppState::NesGame)))
        .add_systems(Update, NesBevy::gamepad_system.run_if(in_state(EmuAppState::NesGame)));
    }
}

impl<S: States> Plugin for VcsPlugin<S> {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(Time::<Fixed>::from_duration(Duration::from_millis(17)))
        .add_systems(OnEnter(EmuAppState::VcsGame), VcsBevy::setup.run_if(in_state(EmuAppState::VcsGame)))
        .add_systems(FixedUpdate, VcsBevy::frame.run_if(in_state(EmuAppState::VcsGame)))
        .add_systems(Update, VcsBevy::gamepad_system.run_if(in_state(EmuAppState::VcsGame)));
    }
}

fn main() {

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Rust Emulator".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins(NesPlugin {state: EmuAppState::NesGame})
        .add_plugins(VcsPlugin {state: EmuAppState::VcsGame})
        .init_state::<EmuAppState>()
        .add_systems(Startup, setup)
        .add_systems(OnEnter(EmuAppState::Menu), setup_menu)
        .add_systems(Update, nes_menu.run_if(in_state(EmuAppState::Menu)))
        .add_systems(Update, vcs_menu.run_if(in_state(EmuAppState::Menu)))
        .add_systems(OnExit(EmuAppState::Menu), cleanup_menu)
        .run();
}

fn nes_menu(
    mut commands: Commands,
    mut next_state: ResMut<NextState<EmuAppState>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<NesButton>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                let files = FileDialog::new()
                    .add_filter("NES", &["nes"])
                    .pick_file();
                if let Some(file) = files {
                    commands.insert_resource(NesRomFile::new(file.into_os_string().into_string().unwrap()));
                    next_state.set(EmuAppState::NesGame);
                }
                *color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}


fn vcs_menu(
    mut commands: Commands,
    mut next_state: ResMut<NextState<EmuAppState>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<VcsButton>)
        >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                let files = FileDialog::new()
                    .add_filter("BIN", &["bin"])
                    .pick_file();
                if let Some(file) = files {                    
                    commands.insert_resource(VcsRomFile::new(file.into_os_string().into_string().unwrap()));
                    next_state.set(EmuAppState::VcsGame);
                }
                *color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

fn cleanup_menu(mut commands: Commands, menu_data: Res<MenuData>) {
    commands.entity(menu_data.button_entity).despawn();
}

#[derive(Resource)]
pub struct MenuData {
    pub button_entity: Entity,
}

pub const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.35, 0.35);

pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    
}

pub fn setup_menu(mut commands: Commands) {
    let button_entity = 
    commands.spawn((
                        Node {
                    // center button
                    width: percent(100),
                    height: percent(100),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
            children![(
                Button,
                NesButton,
                Node {
                    width: px(150),
                    height: px(65),
                    // horizontally center child text
                    justify_content: JustifyContent::Center,
                    // vertically center child text
                    align_items: AlignItems::Center,
                    ..default()
                },

                BackgroundColor(NORMAL_BUTTON),
                children![(
                    Text::new("NES"),
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                )]
            ),
            (
                Button,
                VcsButton,
                Node {
                    width: px(150),
                    height: px(65),
                    // horizontally center child text
                    justify_content: JustifyContent::Center,
                    // vertically center child text
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(NORMAL_BUTTON),
                children![(
                    Text::new("VCS"),
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                )]
            )],
        ))
        .id();
    commands.insert_resource(MenuData { button_entity });
}

