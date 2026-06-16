
pub mod vcs {

    use std::fs;
    use std::sync::RwLock;

    use emucpu::prelude::*;
    
    use crate::vcs_audio_channel::vcs::{NTSC_SAMPLES_PER_FRAME, PAL_SAMPLES_PER_FRAME};
    use crate::vcs_parameters::vcs::VcsParameters;
    use crate::vcs_console_type::vcs::VcsConsoleType;
    use crate::vcs_riot::vcs::VcsRiot;
    use crate::vcs_tia::vcs::VcsTia;
    use crate::vcs_audio::vcs::VcsAudio;
    use crate::vcs_console_type::vcs::ConsoleType;

    use crate::vcs_cartridge::vcs::VcsCartridge;
    use crate::vcs_cartridge_detector::vcs::VcsCartridgeDetector;

    pub struct VcsConsole {
        vcs_riot: VcsRiot,
        vcs_tia: VcsTia,
        vcs_audio: VcsAudio,
        vcs_cartridge: Box<dyn VcsCartridge + Send + Sync>,
        console_type: VcsConsoleType,
        total_ticks: u32,
        cpu_runner: Runner,
        addr: AddressBus,
        inframe: RwLock<bool>,
    }

    impl VcsConsole {

        pub fn new (rom_file: String) -> VcsConsole{

            let rom = fs::read(rom_file);
            let parameters: VcsParameters = VcsParameters::new(rom.unwrap());

            let console_type: VcsConsoleType = VcsConsoleType::new(parameters.console_type);
            let vcs_riot: VcsRiot = VcsRiot::new();
            let vcs_tia: VcsTia = VcsTia::new(&console_type);
            let vcs_cartridge: Box<dyn VcsCartridge + Send + Sync> = VcsCartridgeDetector::detect_cartridge(&parameters);
            let frames_per_second = console_type.get_frames_per_second();
            let vcs_audio: VcsAudio = VcsAudio::new(frames_per_second);

            let mut temp_instance = Self {
                vcs_riot,
                vcs_tia,
                vcs_audio,
                vcs_cartridge,
                console_type,
                total_ticks: 0,
                cpu_runner: Runner::new(),
                addr: AddressBus { address: 0 , write: false, byte: 0, is_accumulator: false },
                inframe: RwLock::new(false),
            };

            temp_instance.start_up();
            temp_instance
        }

        fn start_up(&mut self) {
            //self.vcs_audio.write().unwrap().setup();
            //self.cpu.reset();
            self.vcs_riot.reset();
            self.vcs_tia.reset();

            self.total_ticks = 0;
        }

        fn get_audio(&mut self) -> Vec<f32> {
            let channel0 = self.vcs_audio.get_audio_buffer(0);
            let channel1 = self.vcs_audio.get_audio_buffer(1);
            let mut samples_per_frame: usize = NTSC_SAMPLES_PER_FRAME;
            
            if self.console_type.get_console_type() == ConsoleType::PAL {
                samples_per_frame = PAL_SAMPLES_PER_FRAME;
            }

            let mut mix:Vec<f32> = Vec::with_capacity(samples_per_frame);

            for i in 0..samples_per_frame {
                mix.push((channel0[i] + channel1[i]) / 2.0);
            }

            mix
        }

        pub fn run_frame (&mut self) -> (Option<Vec<u8>>, Option<Vec<f32>>) {
            let mut frame_ticks: u32 = 0;

            if *self.inframe.read().unwrap() {
                return (None, None);
            }
            *self.inframe.write().unwrap() = true;

            self.vcs_audio.execute_frame(self.vcs_tia.get_tia_audio());

            while frame_ticks < self.console_type.get_ticks_per_frame() {
                
                self.vcs_cartridge.execute_tick(&mut self.addr);
                self.vcs_tia.execute_tick(&mut self.addr);
                
                if self.total_ticks.is_multiple_of(3) {

                    self.vcs_tia.execute_addr(&mut self.addr);
                    self.vcs_riot.execute_tick(&mut self.addr);

                    if !self.vcs_tia.is_cpu_blocked() {
                        self.cpu_runner.execute_tick(&mut self.addr);
                    }
                }

                self.vcs_tia.repaint();
                self.total_ticks = self.total_ticks.overflowing_add(1).0;
                frame_ticks += 1;
            }

            let video = self.vcs_tia.get_screen();
            let audio = self.get_audio();

            *self.inframe.write().unwrap() = false;

            (Some(video), Some(audio))
        }

        pub fn left_controler_select(&mut self, value: bool) {
            self.vcs_riot.select_pressed(value);
        }

        pub fn left_controler_start(&mut self, value: bool) {
            self.vcs_riot.reset_pressed(value);
        }

        pub fn left_controler_a(&mut self, value: bool) {
            self.vcs_tia.left_controller_trigger(value);
        }

        pub fn left_controler_up_down(&mut self, value: i8) {
            self.vcs_riot.left_controller_up_down(value);
        }

        pub fn left_controler_left_right(&mut self, value: i8) {
            self.vcs_riot.left_controller_left_right(value);
        }
    }

}
