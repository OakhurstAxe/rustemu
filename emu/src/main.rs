

use std::{sync::Arc, sync::Mutex, time::Duration};

use sdl2::timer::TimerCallback;
use sdl2::pixels::PixelFormatEnum;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::controller::Button;

use vcs::vcs_console::vcs::{ VcsConsole, Message };

pub fn main() -> Result<(), String> {

    let sdl_context = sdl2::init().unwrap();
    
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("rust-sdl3 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut surface = sdl2::surface::Surface::new(160, 210, PixelFormatEnum::RGB24);
    
    let vcs_console: Arc<Mutex<VcsConsole>> = Arc::new(Mutex::new(VcsConsole::new()));
    let vcs_console_clone = Arc::clone(&vcs_console);
    let vcs_console_clone2 = Arc::clone(&vcs_console);

    let timer_subsystem= sdl_context.timer().unwrap();
    let callback: TimerCallback = Box::new(move || {
            let mut vcs = vcs_console.lock().unwrap();
            vcs.start_next_frame();
            17
        });
    let delay = 17;
    let _timer = timer_subsystem.add_timer(delay, callback);
 
    let controller_subsystem = sdl_context.game_controller().unwrap();
    let _controller = controller_subsystem.open(0).unwrap();
    println!("controller {:?}", controller_subsystem.name_for_index(0));
    controller_subsystem.set_event_state(true);

    let joystick_subsystem = sdl_context.joystick().unwrap();
    let _joystick = joystick_subsystem.open(0).unwrap();
    joystick_subsystem.set_event_state(true);

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

