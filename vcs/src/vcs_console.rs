
pub mod vcs {

    use egui::{ Vec2 };

    use std::fs;
    use std::sync::{ Arc, Mutex };
    use emumemory::memory_mapper::emu_memory::MemoryMapper;
    use emucpu::base_cpu::emu_cpu::BaseCpu;
    use emucpu::m6502::emu_cpu::M6502;
    use crate::vcs_memory::vcs::VcsMemory;
    use crate::vcs_parameters::vcs::VcsParameters;
    use crate::vcs_console_type::vcs::VcsConsoleType;
    use crate::vcs_riot::vcs::VcsRiot;
    use crate::vcs_tia::vcs::VcsTia;

    #[derive(Clone)]
    pub enum Message {
        Tick,
    }

    pub struct VcsConsole {
        vcs_riot: Arc<Mutex<VcsRiot>>,
        vcs_console_type: Arc<Mutex<VcsConsoleType>>,
        vcs_tia: Arc<Mutex<VcsTia>>,
        _vcs_audio: i32,
        cpu: M6502,
        total_ticks: u32,
        ticks_per_frame: u32,
        image: Vec<u8>,
        ctx: egui::Context,
        texture: egui::TextureHandle
    }

    impl VcsConsole {

        pub fn new (ctx: &egui::Context) -> VcsConsole {

            let rom = fs::read("/home/dmax/projects/rust/roms/Combat (NA).a26");
            let parameters: Arc<Mutex<VcsParameters>>;
            parameters = Arc::new(Mutex::new(VcsParameters::new(rom.unwrap())));

            let console_type: Arc<Mutex<VcsConsoleType>> = Arc::new(Mutex::new(VcsConsoleType::new(parameters.lock().unwrap().console_type)));
            let riot: Arc<Mutex<VcsRiot>> = Arc::new(Mutex::new(VcsRiot::new()));
            let tia: Arc<Mutex<VcsTia>> = Arc::new(Mutex::new(VcsTia::new(Arc::clone(&console_type))));
            let memory: Box<dyn MemoryMapper + Send> = Box::new(VcsMemory::new (Arc::clone(&parameters), Arc::clone(&tia), Arc::clone(&riot)));
            let cpu: M6502 = M6502::new(memory);

            let ticks_per_second = console_type.lock().unwrap().ticks_per_second();
            let frames_per_second = console_type.lock().unwrap().get_frames_per_second();
            let x_resolution = console_type.lock().unwrap().get_x_resolution();
            let y_resolution = console_type.lock().unwrap().get_y_resolution();
            let texture = ctx.load_texture("noise", egui::ColorImage::example(), egui::TextureOptions::NEAREST);

            let mut temp_instance = Self {
                vcs_riot: Arc::clone(&riot),
                vcs_console_type: Arc::clone(&console_type),
                vcs_tia: Arc::clone(&tia),
                _vcs_audio: 0,
                cpu: cpu,
                total_ticks: 0,
                ticks_per_frame: (ticks_per_second / frames_per_second as i32) as u32,
                image: Vec::with_capacity(0),
                ctx: ctx.clone(),
                texture: texture
            };

            temp_instance.image = Vec::with_capacity(x_resolution as usize * y_resolution as usize * 4);
            temp_instance.ticks_per_frame = ticks_per_second as u32 / frames_per_second as u32;
            temp_instance.start_up();

            temp_instance

        }

        pub fn update(&self) {
            let mut tsize: Vec2 = self.texture.size_vec2();
            tsize.x = tsize.x * 2.0;
            tsize.y = tsize.y * 2.0;

            let sized_texture = egui::load::SizedTexture::new(&self.texture, tsize);

            egui::CentralPanel::default().show(&self.ctx, |ui| {
                // The central panel the region left after adding TopPanel's and SidePanel's
                ui.add(egui::Image::new(sized_texture).fit_to_exact_size(tsize));
            });
        }


        fn set_texture(&mut self) {
            let width: u32 = self.vcs_console_type.lock().unwrap().get_x_resolution() as u32;
            let height: u32 = self.vcs_console_type.lock().unwrap().get_y_resolution() as u32;
            let image = self.vcs_tia.lock().unwrap().get_screen();

            let orgsize: [usize; 2] = [width as usize, height as usize];

            self.texture.set(
                egui::ColorImage {
                    size: orgsize,
                    source_size: Vec2::new(2 as f32, 2 as f32),
                    pixels: image,
                },
                egui::TextureOptions::NEAREST,
            );
        }

        fn start_up(&mut self) {
            self.cpu.reset();
            self.vcs_riot.lock().unwrap().reset();
            self.vcs_tia.lock().unwrap().reset();

            self.total_ticks = 0;
        }

        pub fn start_next_frame (&mut self) {
            let mut frame_ticks: u32 = 0;

            //vcsAudio_.ExecuteTick();
            while frame_ticks < self.ticks_per_frame {
                if self.total_ticks % 3 == 0 {

                    if !self.vcs_tia.lock().unwrap().is_cpu_blocked() {
                        self.cpu.execute_tick();
                    }
                    self.vcs_riot.lock().unwrap().execute_tick();
                }

                self.vcs_tia.lock().unwrap().execute_tick();
                
                if self.vcs_tia.lock().unwrap().repaint() {
                    self.set_texture();
                    self.update();
                }
                self.total_ticks += 1;
                frame_ticks += 1;
            }
        }

        pub fn input_check(&self) {
            self.ctx.input(|inp| {
                for event in &inp.events {
                    match event {
                        egui::Event::Key { repeat, physical_key, key, pressed, modifiers } => {
                            //println!("Key {:?} was {}", key, if *pressed { "pressed" } else { "released" });
                            // Check for specific keys, e.g., Space
                            if *key == egui::Key::A && *pressed {
                                self.vcs_riot.lock().unwrap().select_pressed(true);
                            }
                            if *key == egui::Key::A && !*pressed {
                                self.vcs_riot.lock().unwrap().select_pressed(false);
                            }
                            if *key == egui::Key::S && *pressed {
                                self.vcs_riot.lock().unwrap().reset_pressed(true);
                            }
                            if *key == egui::Key::S && !*pressed {
                                self.vcs_riot.lock().unwrap().reset_pressed(false);
                            }
                            if *key == egui::Key::ArrowUp && *pressed {
                                self.vcs_riot.lock().unwrap().left_controller_up_down(1);
                            }
                            if *key == egui::Key::ArrowUp && !*pressed {
                                self.vcs_riot.lock().unwrap().left_controller_up_down(0);
                            }                        
                            if *key == egui::Key::ArrowDown && *pressed {
                                self.vcs_riot.lock().unwrap().left_controller_up_down(-1);
                            }
                            if *key == egui::Key::ArrowDown && !*pressed {
                                self.vcs_riot.lock().unwrap().left_controller_up_down(0);
                            }                        
                            if *key == egui::Key::ArrowLeft && *pressed {
                                self.vcs_riot.lock().unwrap().left_controller_left_right(-1);
                            }
                            if *key == egui::Key::ArrowLeft && !*pressed {
                                self.vcs_riot.lock().unwrap().left_controller_left_right(0);
                            }                        
                            if *key == egui::Key::ArrowRight && *pressed {
                                self.vcs_riot.lock().unwrap().left_controller_left_right(1);
                            }
                            if *key == egui::Key::ArrowRight && !*pressed {
                                self.vcs_riot.lock().unwrap().left_controller_left_right(0);
                            }                        
                            if *key == egui::Key::Space && *pressed {
                                self.vcs_tia.lock().unwrap().left_controller_trigger(true);
                            }
                            if *key == egui::Key::Space && !*pressed {
                                self.vcs_tia.lock().unwrap().left_controller_trigger(false);
                            }                        
                        },
                        _ => {}
                    }
                }
            });
        }

    }
}
