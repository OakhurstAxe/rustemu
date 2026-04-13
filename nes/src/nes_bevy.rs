
pub mod nes {

    use std::time::Duration;
    use std::ops::DerefMut;

    use bevy::audio::AddAudioSource;
    use bevy::audio::AudioPlugin;
    use bevy::audio::Source;
    use bevy::asset::RenderAssetUsages;
    use bevy::color::{palettes::css};
    use bevy::prelude::*;
    use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};

    use crate::nes_console::nes::NesConsole;

    const IMAGE_WIDTH: u32 = 256;
    const IMAGE_HEIGHT: u32 = 240;

    /// Store the image handle that we will draw to, here.
    #[derive(Resource)]
    pub struct MyProcGenImage(Handle<Image>);

    #[derive(Resource)]
    pub struct MyProcGenAudio(Handle<NesAudio>);

    #[derive(Resource)]
    pub struct Nes(NesConsole);

    #[derive(Resource)]
    pub struct NesRomFile(String);

    impl NesRomFile {

        pub fn new (rom_file: String) -> NesRomFile {
            Self {
                0: rom_file,
            }
        }
    }

    #[derive(Asset, TypePath)]
    pub struct NesAudio {
        pub buffer: Vec<f32>,
        pub current_position: u32,
    }

    impl NesAudio {
        pub fn new(audio: Vec<f32>) -> NesAudio {
            Self {
                buffer: audio,
                current_position: 0,
            }
        }

        pub fn set_buffer(&mut self, audio: Vec<f32>) {
            self.buffer = audio;
            self.current_position = 0;
        }
    }

    impl Iterator for NesAudio {
        type Item = f32;

        fn next(&mut self) -> Option<Self::Item> {
            
            if self.current_position > 734 {
                self.current_position = 0;
            }

            let mut val = self.buffer[self.current_position as usize];
            //val = 0.7;
            //if self.current_position %100 < 50 {
            //    val = -0.7;
           //}

            self.current_position += 1;

            Some(val)
        }
    }

    impl Source for NesAudio {
        fn current_frame_len(&self) -> Option<usize> {
            None
        }

        fn channels(&self) -> u16 {
            1
        }

        fn sample_rate(&self) -> u32 {
            44100
        }

        fn total_duration(&self) -> Option<Duration> {
            None
        }
    }

    impl Decodable for NesAudio {
        type DecoderItem = <NesAudio as Iterator>::Item;

        type Decoder = NesAudio;

        fn decoder(&self) -> Self::Decoder {
            NesAudio::new(self.buffer.clone())
        }
    }

    pub struct NesBevy {
    }

    impl NesBevy {
    
        pub fn setup(
            mut commands: Commands, 
            mut assets: ResMut<Assets<NesAudio>>,
            mut images: ResMut<Assets<Image>>,
            rom_file:  ResMut<NesRomFile>) {

            let audio_handle = assets.add(NesAudio::new(vec![0.0; 736]));
            commands.spawn(AudioPlayer(audio_handle.clone()));
            commands.insert_resource(MyProcGenAudio(audio_handle));

            commands.spawn(Camera2d);

            let nes_console = NesConsole::new(rom_file.0.clone());//"roms/Donkey_kong.nes");
            commands.insert_resource(Nes(nes_console));

            // Create an image that we are going to draw into
            let image = Image::new_fill(
                // 2D image of size 256x256
                Extent3d {
                    width: IMAGE_WIDTH,
                    height: IMAGE_HEIGHT,
                    depth_or_array_layers: 1,
                },
                TextureDimension::D2,
                &(css::BLACK.to_u8_array()),
                TextureFormat::Rgba8UnormSrgb,
                RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
            );

            let handle = images.add(image);
            commands.spawn(Sprite::from_image(handle.clone()));
            commands.insert_resource(MyProcGenImage(handle));
        }

        pub fn draw(
            mut commands: Commands, 
            mut assets: ResMut<Assets<NesAudio>>,
            audio_handle: Res<MyProcGenAudio>,
            my_handle: Res<MyProcGenImage>,
            mut images: ResMut<Assets<Image>>,
            mut nes_console: ResMut<Nes>
        ) {

            nes_console.0.start_next_frame();
            let new_screen = nes_console.0.is_frame_rendered();
            let screen = new_screen.1;

            // Get the image from Bevy's asset storage.
            let image = images.get_mut(&my_handle.0).expect("Image not found");
 
            for y in 0..IMAGE_HEIGHT {
                for x in 0..IMAGE_WIDTH {
                    let color = Color::srgb_u8(
                        screen[((y * 256 + x) * 3) as usize],
                        screen[((y * 256 + x) * 3 + 1) as usize],
                        screen[((y * 256 + x) * 3 + 2) as usize],
                    );
                    _ = image.set_color_at(x, y, color);
                }
            }

            let audio = nes_console.0.get_audio();
            let nes_audio = assets.get_mut(&audio_handle.0).expect("Audio not found");
            nes_audio.buffer = audio;
            commands.spawn(AudioPlayer(audio_handle.0.clone()));
        }

        pub fn gamepad_system(gamepads: Query<(Entity, &Gamepad)>,
            mut nes_console: ResMut<Nes>
        ) {
            for (_entity, gamepad) in &gamepads {
                if gamepad.just_pressed(GamepadButton::Select) {
                    nes_console.0.left_controler_select(true);
                } else if gamepad.just_released(GamepadButton::Select) {
                    nes_console.0.left_controler_select(false);
                }

                if gamepad.just_pressed(GamepadButton::Start) {
                    nes_console.0.left_controler_start(true);
                } else if gamepad.just_released(GamepadButton::Start) {
                    nes_console.0.left_controler_start(false);
                }

                if gamepad.just_pressed(GamepadButton::South) {
                    nes_console.0.left_controler_a(true);
                } else if gamepad.just_released(GamepadButton::South) {
                    nes_console.0.left_controler_a(false);
                }

                if gamepad.just_pressed(GamepadButton::North) {
                    nes_console.0.left_controler_b(true);
                } else if gamepad.just_released(GamepadButton::North) {
                    nes_console.0.left_controler_b(false);
                }

                let left_stick_x = gamepad.get(GamepadAxis::LeftStickX).unwrap();
                if left_stick_x > 0.01 {
                    nes_console.0.left_controler_left_right(1);
                }
                else if left_stick_x < -0.01 {
                    nes_console.0.left_controler_left_right(-1);
                }
                else {
                    nes_console.0.left_controler_left_right(0);
                }

                let left_stick_y = gamepad.get(GamepadAxis::LeftStickY).unwrap();
                if left_stick_y > 0.01 {
                    nes_console.0.left_controler_up_down(-1);
                }
                else if left_stick_y < -0.01 {
                    nes_console.0.left_controler_up_down(1);
                }
                else {
                    nes_console.0.left_controler_up_down(0);
                }
            }
        }
    }
}