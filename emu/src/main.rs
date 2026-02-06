
use std::{sync::Arc, sync::Mutex, time::Duration};

use sdl2::controller::Button;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mixer::Chunk;
use sdl2::pixels::PixelFormatEnum;
use sdl2::timer::TimerCallback;

use vcs::vcs_console::vcs::{ VcsConsole, Message, VcsAudioEvent };
use vcs::vcs_audio_channel::vcs::{ DATA_SAMPLE_RATE_HZ, SAMPLES_PER_FRAME };

pub fn main() -> Result<(), String> {

    let sdl_context = sdl2::init().unwrap();
    
    // Video
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("rust-sdl3 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut surface = sdl2::surface::Surface::new(160, 210, PixelFormatEnum::RGB24);

    // Controller (buttons)
    let controller_subsystem = sdl_context.game_controller().unwrap();
    let _controller = controller_subsystem.open(0).unwrap();
    println!("controller {:?}", controller_subsystem.name_for_index(0));
    controller_subsystem.set_event_state(true);

    // Joystick (Direction pushed)
    let joystick_subsystem = sdl_context.joystick().unwrap();
    let _joystick = joystick_subsystem.open(0).unwrap();
    joystick_subsystem.set_event_state(true);

    // Events
    let event_subsystem = sdl_context.event().unwrap();
    event_subsystem.register_custom_event::<VcsAudioEvent>()?;
    let event_sender = event_subsystem.event_sender();

    // Audio
    let frequency = DATA_SAMPLE_RATE_HZ;
    let format = sdl2::mixer::AUDIO_U16;
    let channels = 1;
    let chunk_size = SAMPLES_PER_FRAME;
    _ = sdl2::mixer::open_audio(frequency as i32, format, channels, chunk_size as i32).unwrap();
    sdl2::mixer::allocate_channels(1);
    let mut chunk0: Chunk;

    // VCS Console
    let vcs_console: Arc<Mutex<VcsConsole>> = Arc::new(Mutex::new(VcsConsole::new(event_sender)));
    let vcs_console_clone = Arc::clone(&vcs_console);
    let vcs_console_clone2 = Arc::clone(&vcs_console);

    // Frame timer
    let timer_subsystem= sdl_context.timer().unwrap();
    let callback: TimerCallback = Box::new(move || {
            let mut vcs = vcs_console.lock().unwrap();
            vcs.start_next_frame();
            17
        });
    let delay = 17;
    let _timer = timer_subsystem.add_timer(delay, callback);

    'running: loop {
        let new_screen = vcs_console_clone.lock().unwrap().is_frame_rendered();
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
                    if let Some(custom_event) = event.as_user_event_type::<VcsAudioEvent>() {
                        let data0:[u16; SAMPLES_PER_FRAME] = custom_event.channel_mix.try_into().unwrap();
                        let boxdata0: Box<[u16; SAMPLES_PER_FRAME]> = Box::new(data0);
                        chunk0 = Chunk::from_raw_buffer(boxdata0.clone()).unwrap();
                        _ = sdl2::mixer::Channel::all().play(&chunk0, 1);
                    }
                },
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                    vcs_console_clone2.lock().unwrap().handle_input(Message::Select, 1);
                },
                Event::KeyUp { keycode: Some(Keycode::A), .. } => {
                    vcs_console_clone2.lock().unwrap().handle_input(Message::Select, 0);
                },
                Event::ControllerButtonDown { button: Button::Start, .. } |
                Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                    vcs_console_clone2.lock().unwrap().handle_input(Message::Reset, 1);
                },
                Event::ControllerButtonUp { button: Button::Start, .. } |
                Event::KeyUp { keycode: Some(Keycode::S), .. } => {
                    vcs_console_clone2.lock().unwrap().handle_input(Message::Reset, 0);
                },
                Event::JoyAxisMotion { axis_idx: 0, value, ..} => {
                    if value > -259 {
                        vcs_console_clone2.lock().unwrap().handle_input(Message::P0LeftRight, 1);
                    }
                    if value < -259 {
                        vcs_console_clone2.lock().unwrap().handle_input(Message::P0LeftRight, -1);
                    }
                    if value == -259 {
                        vcs_console_clone2.lock().unwrap().handle_input(Message::P0LeftRight, 0);
                    }
                }
                Event::JoyAxisMotion { axis_idx: 1, value, ..} => {
                    if value > -259 {
                        vcs_console_clone2.lock().unwrap().handle_input(Message::P0UpDown, -1);
                    }
                    if value < -259 {
                        vcs_console_clone2.lock().unwrap().handle_input(Message::P0UpDown, 1);
                    }
                    if value == -259 {
                        vcs_console_clone2.lock().unwrap().handle_input(Message::P0UpDown, 0);
                    }
                }
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                    vcs_console_clone2.lock().unwrap().handle_input(Message::P0UpDown, 1);
                },
                Event::ControllerButtonUp { button: Button::DPadUp, .. } |
                Event::KeyUp { keycode: Some(Keycode::Up), .. } => {
                    vcs_console_clone2.lock().unwrap().handle_input(Message::P0UpDown, 0);
                },
                Event::ControllerButtonDown { button: Button::DPadDown, .. } |
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    vcs_console_clone2.lock().unwrap().handle_input(Message::P0UpDown, -1);
                },
                Event::ControllerButtonUp { button: Button::DPadDown, .. } |
                Event::KeyUp { keycode: Some(Keycode::Down), .. } => {
                    vcs_console_clone2.lock().unwrap().handle_input(Message::P0UpDown, 0);
                },
                Event::ControllerButtonDown { button: Button::DPadLeft, .. } |
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                    vcs_console_clone2.lock().unwrap().handle_input(Message::P0LeftRight, -1);
                },
                Event::ControllerButtonUp { button: Button::DPadLeft, .. } |
                Event::KeyUp { keycode: Some(Keycode::Left), .. } => {
                    vcs_console_clone2.lock().unwrap().handle_input(Message::P0LeftRight, 0);
                },
                Event::ControllerButtonDown { button: Button::DPadRight, .. } |
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                    vcs_console_clone2.lock().unwrap().handle_input(Message::P0LeftRight, 1);
                },
                Event::ControllerButtonUp { button: Button::DPadRight, .. } |
                Event::KeyUp { keycode: Some(Keycode::Right), .. } => {
                    vcs_console_clone2.lock().unwrap().handle_input(Message::P0LeftRight, 0);
                },
                Event::ControllerButtonDown { button: Button::A, .. } |
                Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                    vcs_console_clone2.lock().unwrap().handle_input(Message::P0Trigger, 1);
                },
                Event::ControllerButtonUp { button: Button::A, .. } |
                Event::KeyUp { keycode: Some(Keycode::Space), .. } => {
                    vcs_console_clone2.lock().unwrap().handle_input(Message::P0Trigger, 0);
                },
                _ => {}
            }
            
        }
        // The rest of the game loop goes here...
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }    

    Ok(())
}