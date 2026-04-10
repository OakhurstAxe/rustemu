
pub mod nes {
    use std::{sync::Arc, sync::Mutex, time::Duration};

    use sdl2::Sdl;
    use sdl2::controller::{Button, GameController};
    use sdl2::event::Event;
    use sdl2::keyboard::Keycode;
    use sdl2::mixer::Chunk;
    use sdl2::pixels::PixelFormatEnum;
    use sdl2::timer::TimerCallback;

    use crate::nes_console::nes::{ NesConsole, NesAudioEvent };
    //use crate::vcs_audio_channel::vcs::{ DATA_SAMPLE_RATE_HZ, PAL_SAMPLES_PER_FRAME, NTSC_SAMPLES_PER_FRAME };
    //use crate::vcs_console_type::vcs::ConsoleType;

    pub struct NesSdlMain {
        sdl_context: Sdl,
    }

    impl NesSdlMain {

        pub fn new() -> NesSdlMain {
            Self {
                sdl_context: sdl2::init().unwrap(),
            }
        }

        pub fn nes_sdl_main(&mut self, rom_file: &str) -> Result<(), String> {

            // Controller (buttons)
            let controller_subsystem = self.sdl_context.game_controller().unwrap();
            let controller_count = controller_subsystem.num_joysticks().unwrap();
            let mut controller: GameController;
            //for j in 0..controller_count {
                controller = controller_subsystem.open(0).unwrap();
                println!("controller {:?}", controller_subsystem.name_for_index(0));
            //}
            _ = controller_subsystem.add_mapping("030000005e0400000700000000010000,Microsoft® SideWinder® Game Pad USB,platform:Linux,crc:4119,a:b0,b:b1,x:b3,y:b4,guide:b8,start:b9,leftshoulder:b6,rightshoulder:b7,dpup:-a1,dpdown:+a1,dpleft:-a0,dpright:+a0,");
            controller_subsystem.set_event_state(true);

            // Events
            let event_subsystem = self.sdl_context.event().unwrap();
            event_subsystem.register_custom_event::<NesAudioEvent>()?;
            let event_sender = event_subsystem.event_sender();

            // VCS Console
            let nes_console: Arc<Mutex<NesConsole>> = Arc::new(Mutex::new(NesConsole::new(rom_file.to_string())));//, event_sender)));
            let nes_console_clone = Arc::clone(&nes_console);

            // Audio
            let frequency = 44100;
            let format = sdl2::mixer::AUDIO_U16;
            let channels = 1;
            let chunk_size: usize = 735;
            _ = sdl2::mixer::open_audio(frequency as i32, format, channels, chunk_size as i32).unwrap();
            sdl2::mixer::allocate_channels(1);
            let mut chunk0: Chunk;

            // Video
            let x_resolution = 256;// console_type.read().unwrap().get_x_resolution();
            let y_resolution = 240;//console_type.read().unwrap().get_y_resolution();
            let video_subsystem = self.sdl_context.video().unwrap();
            let window = video_subsystem.window("NES", 800, 600)
                .position_centered()
                .build()
                .unwrap();
            let mut canvas = window.into_canvas().build().unwrap();
            let mut event_pump = self.sdl_context.event_pump().unwrap();
            let mut surface = sdl2::surface::Surface::new(x_resolution, y_resolution, PixelFormatEnum::RGB24);

            // Frame timer
            let timer_subsystem= self.sdl_context.timer().unwrap();
            let frames_per_second = 30;//vcs_console.lock().unwrap().get_console_type().read().unwrap().get_frames_per_second();
            let callback: TimerCallback = Box::new(move || {
                    let mut nes = nes_console.lock().unwrap();
                    nes.start_next_frame();
                    1000 / frames_per_second
                });
            let delay = 17;
            let _timer = timer_subsystem.add_timer(delay, callback);
            
            'running: loop {
                let new_screen = nes_console_clone.lock().unwrap().is_frame_rendered();
                if new_screen.0 {
                    let sur = surface.as_mut().unwrap();
                    let pixels = sur.without_lock_mut();

                    if let Some(pixels) = pixels {
                        pixels.copy_from_slice(&new_screen.1[..]);
                    }
                    
                    let texture_creator = canvas.texture_creator();
                    let texture = texture_creator.create_texture_from_surface(&sur).unwrap();
                    canvas.clear();
                    _ = canvas.copy(&texture, None, None);
                    canvas.present();

                }
                
                for event in event_pump.poll_iter() {

                    match event {
                        Event::User { type_: _u32, .. } => {
                            if let Some(custom_event) = event.as_user_event_type::<NesAudioEvent>() {
                                let boxdata0: Box<[u16; 735]> = Box::new(custom_event.channel_mix.try_into().unwrap());
                                chunk0 = Chunk::from_raw_buffer(boxdata0.clone()).unwrap();
                                let _result = sdl2::mixer::Channel::all().play(&chunk0, 1);
                            }
                        },
                        Event::Quit {..} |
                        Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                            break 'running
                        },
                        Event::ControllerButtonDown { button: Button::Guide, .. } |
                        Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                            nes_console_clone.lock().unwrap().left_controler_select(true);
                        },
                        Event::ControllerButtonUp { button: Button::Guide, .. } |
                        Event::KeyUp { keycode: Some(Keycode::A), .. } => {
                            nes_console_clone.lock().unwrap().left_controler_select(false);
                        },
                        Event::ControllerButtonDown { button: Button::Start, .. } |
                        Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                            nes_console_clone.lock().unwrap().left_controler_start(true);
                        },
                        Event::ControllerButtonUp { button: Button::Start, .. } |
                        Event::KeyUp { keycode: Some(Keycode::S), .. } => {
                            nes_console_clone.lock().unwrap().left_controler_start(false);
                        },
                        Event::ControllerButtonDown { button: Button::DPadUp, .. } |
                        Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                            nes_console_clone.lock().unwrap().left_controler_up_down(-1);
                        },
                        Event::ControllerButtonUp { button: Button::DPadUp, .. } |
                        Event::KeyUp { keycode: Some(Keycode::Up), .. } => {
                            nes_console_clone.lock().unwrap().left_controler_up_down(0);
                        },
                        Event::ControllerButtonDown { button: Button::DPadDown, .. } |
                        Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                            nes_console_clone.lock().unwrap().left_controler_up_down(1);
                        },
                        Event::ControllerButtonUp { button: Button::DPadDown, .. } |
                        Event::KeyUp { keycode: Some(Keycode::Down), .. } => {
                            nes_console_clone.lock().unwrap().left_controler_up_down(0);
                        },
                        Event::ControllerButtonDown { button: Button::DPadLeft, .. } |
                        Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                            nes_console_clone.lock().unwrap().left_controler_left_right(-1);
                        },
                        Event::ControllerButtonUp { button: Button::DPadLeft, .. } |
                        Event::KeyUp { keycode: Some(Keycode::Left), .. } => {
                            nes_console_clone.lock().unwrap().left_controler_left_right(0);
                        },
                        Event::ControllerButtonDown { button: Button::DPadRight, .. } |
                        Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                            nes_console_clone.lock().unwrap().left_controler_left_right(1);
                        },
                        Event::ControllerButtonUp { button: Button::DPadRight, .. } |
                        Event::KeyUp { keycode: Some(Keycode::Right), .. } => {
                            nes_console_clone.lock().unwrap().left_controler_left_right(0);
                        },
                        Event::ControllerButtonDown { button: Button::A, .. } |
                        Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                            nes_console_clone.lock().unwrap().left_controler_a(true);
                        },
                        Event::ControllerButtonUp { button: Button::A, .. } |
                        Event::KeyUp { keycode: Some(Keycode::Space), .. } => {
                            nes_console_clone.lock().unwrap().left_controler_a(false);
                        },
                        Event::ControllerButtonDown { button: Button::B, .. } |
                        Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                            nes_console_clone.lock().unwrap().left_controler_b(true);
                        },
                        Event::ControllerButtonUp { button: Button::B, .. } |
                        Event::KeyUp { keycode: Some(Keycode::Space), .. } => {
                            nes_console_clone.lock().unwrap().left_controler_b(false);
                        },
                        _ => {}
                    }
                    
                }
                
                // The rest of the game loop goes here...
                std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
            }    
            
            Ok(())
        }
    }
}