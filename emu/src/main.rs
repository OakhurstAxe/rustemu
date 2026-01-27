

extern crate sdl3;

use std::{sync::Arc, time::Duration};
use std::sync::{ Mutex };

use sdl3::pixels::{ PixelFormat };
use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use sdl3::timer;
use vcs::vcs_console::vcs::{ VcsConsole, Message };

pub fn main() -> Result<(), String> {

    let sdl_context = sdl3::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl3 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas();
    let mut event_pump = sdl_context.event_pump().unwrap();
    
    let vcs_console: Arc<Mutex<VcsConsole>> = Arc::new(Mutex::new(VcsConsole::new()));
    let vcs_console_clone = Arc::clone(&vcs_console);

    
    let _timer = timer::add_timer(17,  Box::new(move || {
            let mut vcs = vcs_console_clone.lock().unwrap();
            vcs.start_next_frame();
            17
        }),
    );
 
    let texture_creator = canvas.texture_creator();
    let mut surface = sdl3::surface::Surface::new(160, 210, PixelFormat::RGB24);
    
    'running: loop {
        let new_screen = vcs_console.lock().unwrap().is_frame_rendered();
        if new_screen.0 {
            canvas.clear();
            let sur = surface.as_mut().unwrap();
            unsafe {
                let pixels = sur.without_lock_mut();
                if let Some(pixels) = pixels {
                    pixels.copy_from_slice(&new_screen.1[..]);
                }
            }
            let surface_wrap = sur;
            let text = 
                sdl3::render::Texture::from_surface(&surface_wrap, &texture_creator);
            let _ = canvas.copy(&text.unwrap(), None, None);
            canvas.present();
        }

        for event in event_pump.poll_iter() {

            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                    vcs_console.lock().unwrap().handle_input(Message::Select, 1);
                },
                Event::KeyUp { keycode: Some(Keycode::A), .. } => {
                    vcs_console.lock().unwrap().handle_input(Message::Select, 0);
                },
                Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                    vcs_console.lock().unwrap().handle_input(Message::Reset, 1);
                },
                Event::KeyUp { keycode: Some(Keycode::S), .. } => {
                    vcs_console.lock().unwrap().handle_input(Message::Reset, 0);
                },
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                    vcs_console.lock().unwrap().handle_input(Message::P0UpDown, 1);
                },
                Event::KeyUp { keycode: Some(Keycode::Up), .. } => {
                    vcs_console.lock().unwrap().handle_input(Message::P0UpDown, 0);
                },
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    vcs_console.lock().unwrap().handle_input(Message::P0UpDown, -1);
                },
                Event::KeyUp { keycode: Some(Keycode::Down), .. } => {
                    vcs_console.lock().unwrap().handle_input(Message::P0UpDown, 0);
                },
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                    vcs_console.lock().unwrap().handle_input(Message::P0LeftRight, -1);
                },
                Event::KeyUp { keycode: Some(Keycode::Left), .. } => {
                    vcs_console.lock().unwrap().handle_input(Message::P0LeftRight, 0);
                },
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                    vcs_console.lock().unwrap().handle_input(Message::P0LeftRight, 1);
                },
                Event::KeyUp { keycode: Some(Keycode::Right), .. } => {
                    vcs_console.lock().unwrap().handle_input(Message::P0LeftRight, 0);
                },
                Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                    vcs_console.lock().unwrap().handle_input(Message::P0Trigger, 1);
                },
                Event::KeyUp { keycode: Some(Keycode::Space), .. } => {
                    vcs_console.lock().unwrap().handle_input(Message::P0Trigger, 0);
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }    

    Ok(())
}

