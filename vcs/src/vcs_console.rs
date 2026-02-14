
pub mod vcs {

    use std::fs;
    use std::sync::{ Arc, RwLock };

    use sdl2::event::{ EventSender };

    use emumemory::memory_mapper::emu_memory::MemoryMapper;
    use emucpu::base_cpu::emu_cpu::BaseCpu;
    use emucpu::m6502::emu_cpu::M6502;
    use crate::vcs_audio_channel::vcs::{NTSC_SAMPLES_PER_FRAME, PAL_SAMPLES_PER_FRAME};
    use crate::vcs_memory::vcs::VcsMemory;
    use crate::vcs_parameters::vcs::VcsParameters;
    use crate::vcs_console_type::vcs::VcsConsoleType;
    use crate::vcs_riot::vcs::VcsRiot;
    use crate::vcs_tia::vcs::VcsTia;
    use crate::vcs_audio::vcs::VcsAudio;
    use crate::vcs_console_type::vcs::ConsoleType;

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
        console_type: Arc<RwLock<VcsConsoleType>>,
        vcs_audio: VcsAudio,
        cpu: M6502,
        total_ticks: u32,
        image: Vec<u8>,
        frame_rendered: bool,
        event_sender: EventSender,
    }

    impl VcsConsole {

        pub fn new (rom_file: &str, sender: EventSender) -> VcsConsole{

            let rom = fs::read(rom_file);
            let parameters: Arc<RwLock<VcsParameters>>;
            parameters = Arc::new(RwLock::new(VcsParameters::new(rom.unwrap())));

            let console_type: Arc<RwLock<VcsConsoleType>> = Arc::new(RwLock::new(VcsConsoleType::new(parameters.read().unwrap().console_type)));
            let riot: Arc<RwLock<VcsRiot>> = Arc::new(RwLock::new(VcsRiot::new()));
            let tia: Arc<RwLock<VcsTia>> = Arc::new(RwLock::new(VcsTia::new(Arc::clone(&console_type))));
            let memory: Box<dyn MemoryMapper + Send> = Box::new(VcsMemory::new (Arc::clone(&parameters), Arc::clone(&tia), Arc::clone(&riot)));
            let cpu: M6502 = M6502::new(memory);
            let frames_per_second = console_type.read().unwrap().get_frames_per_second();
            let x_resolution = console_type.read().unwrap().get_x_resolution();
            let y_resolution = console_type.read().unwrap().get_y_resolution();
            let audio: VcsAudio = VcsAudio::new(Arc::clone(&tia), frames_per_second);

            let mut temp_instance = Self {
                vcs_riot: Arc::clone(&riot),
                vcs_tia: Arc::clone(&tia),
                console_type: Arc::clone(&console_type),
                vcs_audio: audio,
                cpu: cpu,
                total_ticks: 0,
                image: Vec::with_capacity(0),
                frame_rendered: false,
                event_sender: sender,
            };

            temp_instance.image = Vec::with_capacity(x_resolution as usize * y_resolution as usize * 4);
            temp_instance.start_up();

            temp_instance

        }

        pub fn get_console_type(&self) -> Arc<RwLock<VcsConsoleType>> {
            Arc::clone(&self.console_type)
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
            let samples_per_frame: usize;
            
            if self.console_type.read().unwrap().get_console_type() == ConsoleType::PAL {
                samples_per_frame = PAL_SAMPLES_PER_FRAME;
            }
            else {
                samples_per_frame = NTSC_SAMPLES_PER_FRAME;
            }

            let mut mix:Vec<u16> = Vec::with_capacity(samples_per_frame);

            for i in 0..samples_per_frame {
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

            while frame_ticks < self.console_type.read().unwrap().get_ticks_per_frame() as u32 {
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