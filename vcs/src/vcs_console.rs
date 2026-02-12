
pub mod vcs {

    use std::fs;
    use std::sync::{ Arc, RwLock };

    use sdl2::event::{ EventSender };

    use emumemory::memory_mapper::emu_memory::MemoryMapper;
    use emucpu::base_cpu::emu_cpu::BaseCpu;
    use emucpu::m6502::emu_cpu::M6502;
    use crate::vcs_memory::vcs::VcsMemory;
    use crate::vcs_parameters::vcs::VcsParameters;
    use crate::vcs_console_type::vcs::VcsConsoleType;
    use crate::vcs_riot::vcs::VcsRiot;
    use crate::vcs_tia::vcs::VcsTia;
    use crate::vcs_audio::vcs::VcsAudio;

    use crate::vcs_audio_channel::vcs::SAMPLES_PER_FRAME;

    pub struct VcsAudioEvent {
        pub channel_mix: Vec<u16>,
    }

    #[derive(Clone)]
    pub enum Message {
        Select,
        Reset,
        P0UpDown,
        P0LeftRight,
        P0Trigger,
    }

    pub struct VcsConsole {
        vcs_riot: Arc<RwLock<VcsRiot>>,
        vcs_tia: Arc<RwLock<VcsTia>>,
        vcs_audio: VcsAudio,
        cpu: M6502,
        total_ticks: u32,
        ticks_per_frame: u32,
        image: Vec<u8>,
        frame_rendered: bool,
        event_sender: EventSender,
    }

    impl VcsConsole {

        pub fn new (sender: EventSender) -> VcsConsole{

            //let rom = fs::read("/home/dmax/projects/rust/roms/Combat (NA).a26");
            let rom = fs::read("/home/dmax/projects/rust/roms/Adventure (1980) (Atari, Warren Robinett - Sears) (CX2613 - 49-75154) ~.bin");
            let parameters: Arc<RwLock<VcsParameters>>;
            parameters = Arc::new(RwLock::new(VcsParameters::new(rom.unwrap())));

            let console_type: Arc<RwLock<VcsConsoleType>> = Arc::new(RwLock::new(VcsConsoleType::new(parameters.read().unwrap().console_type)));
            let riot: Arc<RwLock<VcsRiot>> = Arc::new(RwLock::new(VcsRiot::new()));
            let tia: Arc<RwLock<VcsTia>> = Arc::new(RwLock::new(VcsTia::new(Arc::clone(&console_type))));
            let memory: Box<dyn MemoryMapper + Send> = Box::new(VcsMemory::new (Arc::clone(&parameters), Arc::clone(&tia), Arc::clone(&riot)));
            let cpu: M6502 = M6502::new(memory);
            let audio: VcsAudio = VcsAudio::new(Arc::clone(&tia));
            let ticks_per_second = console_type.read().unwrap().ticks_per_second();
            let frames_per_second = console_type.read().unwrap().get_frames_per_second();
            let x_resolution = console_type.read().unwrap().get_x_resolution();
            let y_resolution = console_type.read().unwrap().get_y_resolution();

            let mut temp_instance = Self {
                vcs_riot: Arc::clone(&riot),
                vcs_tia: Arc::clone(&tia),
                vcs_audio: audio,
                cpu: cpu,
                total_ticks: 0,
                ticks_per_frame: (ticks_per_second / frames_per_second as i32) as u32,
                image: Vec::with_capacity(0),
                frame_rendered: false,
                event_sender: sender,
            };

            temp_instance.image = Vec::with_capacity(x_resolution as usize * y_resolution as usize * 4);
            temp_instance.ticks_per_frame = ticks_per_second as u32 / frames_per_second as u32;
            temp_instance.start_up();

            temp_instance

        }

        pub fn is_frame_rendered(&mut self) -> (bool, Vec<u8>) {
            let result = self.frame_rendered;
            self.frame_rendered = false;
            (result, self.vcs_tia.read().unwrap().get_screen())
        }

        fn render_frame(&mut self) {
            self.frame_rendered = true;
        }

        fn start_up(&mut self) {
            //self.vcs_audio.write().unwrap().setup();
            self.cpu.reset();
            self.vcs_riot.write().unwrap().reset();
            self.vcs_tia.write().unwrap().reset();

            self.total_ticks = 0;
        }

        fn send_audio_event(&mut self) {
            let channel0 = self.vcs_audio.get_audio_buffer(0);
            let channel1 = self.vcs_audio.get_audio_buffer(1);
            let mut mix:Vec<u16> = Vec::with_capacity(SAMPLES_PER_FRAME);

            for i in 0..SAMPLES_PER_FRAME {
                mix.push((channel0[i] >> 1) + (channel1[i] >> 1));
            }

            let audio_event:VcsAudioEvent = VcsAudioEvent {
                channel_mix: mix.clone(),
            };

            _ = self.event_sender.push_custom_event(audio_event);
        }

        pub fn start_next_frame (&mut self) {
            let mut frame_ticks: u32 = 0;

            self.vcs_audio.execute_tick();
            self.send_audio_event();

            while frame_ticks < self.ticks_per_frame {
                if self.total_ticks % 3 == 0 {

                    if !self.vcs_tia.read().unwrap().is_cpu_blocked() {
                        self.cpu.execute_tick();
                    }
                    self.vcs_riot.write().unwrap().execute_tick();
                }

                self.vcs_tia.write().unwrap().execute_tick();
                
                if self.vcs_tia.write().unwrap().repaint() {
                    self.render_frame();
                }
                self.total_ticks += 1;
                frame_ticks += 1;
            }
        }

        pub fn handle_input(&mut self, event: Message, value: i8) {
            match event {
                Message::Select => {
                    self.vcs_riot.write().unwrap().select_pressed(value != 0);
                },
                Message::Reset => {
                    self.vcs_riot.write().unwrap().reset_pressed(value != 0);
                },
                Message::P0UpDown => {
                    self.vcs_riot.write().unwrap().left_controller_up_down(value);
                },
                Message::P0LeftRight => {
                    self.vcs_riot.write().unwrap().left_controller_left_right(value);
                },
                Message::P0Trigger => {
                    self.vcs_tia.write().unwrap().left_controller_trigger(value != 0);
                },
            }
        }

    }
}