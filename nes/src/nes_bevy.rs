
pub mod nes {

    use std::sync::Arc;

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

    pub struct NesBevy {
    }

    impl NesBevy {
    
        pub fn setup(
            mut commands: Commands, 
            mut images: ResMut<Assets<Image>>,
            rom_file:  ResMut<NesRomFile>,
            windows: Query<&mut Window>) {

            let nes_console = NesConsole::new(rom_file.0.clone());
            commands.insert_resource(Nes(nes_console));

            let image = Image::new_fill(
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
            let mut sprite = Sprite::from_image(handle.clone());
            sprite.custom_size = Some(Vec2::new(windows.single().unwrap().width(), windows.single().unwrap().height()));
            commands.spawn(sprite);
            commands.insert_resource(MyProcGenImage(handle));
        }

        pub fn frame(
            mut commands: Commands, 
            mut audio_assets: ResMut<Assets<AudioSource>>,
            video_handle: Res<MyProcGenImage>,
            mut video_assets: ResMut<Assets<Image>>,
            mut nes_console: ResMut<Nes>
        ) {

            let (video, audio) = nes_console.0.run_frame();
            let image = video_assets.get_mut(&video_handle.0).expect("Image not found");
 
            for y in 0..IMAGE_HEIGHT {
                for x in 0..IMAGE_WIDTH {
                    let color = Color::srgb_u8(
                        video[((y * IMAGE_WIDTH + x) * 3) as usize],
                        video[((y * IMAGE_WIDTH + x) * 3 + 1) as usize],
                        video[((y * IMAGE_WIDTH + x) * 3 + 2) as usize],
                    );
                    _ = image.set_color_at(x, y, color);
                }
            }

            let wav_header: Vec<u8> = vec![ 
                0x52, 0x49, 0x46, 0x46, //b'R', b'I', b'F', b'F', 
                0x0b, 0x03, 0x0, 0x0, 
                0x57, 0x41, 0x56, 0x45, //b'W', b'A', b'V', b'E', 
                0x66, 0x6d, 0x74, 0x20, //b'f', b'm', b't', 0, 
                0x10, 0x0, 0x0, 0x0, 
                0x1, 0x0,
                0x1, 0x0, 
                0x44, 0xac, 0x0, 0x0, 
                0x44, 0xac, 0x0, 0x0, 
                0x1, 0x0, 
                0x8, 0x0, 
                0x64, 0x61, 0x74, 0x61, //b'd', b'a', b't', b'a', 
                0xdf, 0x2, 0x0, 0x0];
                
            let mut buffer: Vec<u8> = vec![0; 780];
            for i in 0..44 {
                buffer[i] = wav_header[i];
            }
            for i in 0..735 {
                buffer[i+44] = ((audio[i] + 1.0) * 127.0) as u8;
            }
            let buffer_array: [u8; 780] = buffer.try_into().unwrap();
            let audio_source = AudioSource{bytes: Arc::new(buffer_array)};
            let audio_handle = audio_assets.add(audio_source);
            commands.spawn((AudioPlayer::new(audio_handle), PlaybackSettings::DESPAWN));
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