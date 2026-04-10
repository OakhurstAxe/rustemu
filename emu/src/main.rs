
// When compiling natively:
use std::time::Duration;

use bevy::{ prelude::*};
use bevy_file_dialog::prelude::*;

use ui::*;
use nes::nes_bevy::nes::{NesBevy, NesRomFile};

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum EmuAppState {
    #[default]
    Menu,
    NesGame,
    VcsGame,
}

pub struct NesPlugin<S: States> {
    pub state: S,
}

impl<S: States> Plugin for NesPlugin<S> {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(NesRomFile::new("roms/Donkey_kong.nes".to_string()))
        .insert_resource(Time::<Fixed>::from_duration(Duration::from_millis(17)))
        .add_systems(OnEnter(EmuAppState::NesGame), NesBevy::setup.run_if(in_state(EmuAppState::NesGame)))
        .add_systems(FixedUpdate, NesBevy::draw.run_if(in_state(EmuAppState::NesGame)))
        .add_systems(Update, NesBevy::gamepad_system.run_if(in_state(EmuAppState::NesGame)));
    }
}

fn main() {

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(FileDialogPlugin::new().with_pick_file::<NesRomFile>())
        .add_plugins(NesPlugin {state: EmuAppState::NesGame})
        .init_state::<EmuAppState>()
        .add_systems(Startup, setup)
        .add_systems(OnEnter(EmuAppState::Menu), setup_menu)
        .add_systems(Update, menu.run_if(in_state(EmuAppState::Menu)))
        .add_systems(Update, file_picked)
        .add_systems(OnExit(EmuAppState::Menu), cleanup_menu)
        .run();
}

fn file_picked(
    mut ev_picked: MessageReader<DialogFilePicked<NesRomFile>>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<EmuAppState>>) {


    for ev in ev_picked.read() {
        //eprintln!("File picked, path {:?}", ev.path);
        commands.insert_resource(NesRomFile::new(ev.path.clone().into_os_string().into_string().unwrap()));
        next_state.set(EmuAppState::NesGame);
    }
}

fn menu(
    mut commands: Commands,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                commands.dialog().add_filter("NES", &["nes"])
                    .pick_file_path::<NesRomFile>();
                *color = PRESSED_BUTTON.into();
                //next_state.set(EmuAppState::NesGame);
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

mod ui {
    use crate::*;

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
        let button_entity = commands
            .spawn((
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
                )],
            ))
            .id();
        commands.insert_resource(MenuData { button_entity });
    }

}