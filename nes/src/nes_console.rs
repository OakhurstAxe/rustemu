

pub mod nes {

    use std::sync::{ Arc, RwLock };

    use emucpu::base_cpu::emu_cpu::BaseCpu;
    use emucpu::m6502::emu_cpu::M6502;

    use crate::nes_cartridge::nes::NesCartridge;
    use crate::{nes_cartridge_000::nes::NesCartridge000, nes_memory::nes::NesMemory};
    use crate::nes_ppu::nes::NesPpu;
    use crate::nes_inesfile::nes::INesFile;
    use crate::nes_apu::nes::NesApu;

    const TICKS_PER_FRAME: u16 =  59736;

    pub struct NesAudioEvent {
        pub channel_mix: Vec<u16>,
    }

    pub struct NesConsole {
        cpu: M6502<NesMemory>,
        ppu: Arc<RwLock<NesPpu>>,
        apu: Arc<RwLock<NesApu>>,
        left_controller: u8,
        right_controller: u8,
        _debug: u8,
        pub frame: u32,
    }

    unsafe impl Send for NesConsole {}

    impl NesConsole {

        pub fn new (rom_file: String) -> NesConsole {
            let mut ines_file: INesFile = INesFile::new();
            ines_file.load_file(rom_file);
            let cartridge: Arc<RwLock<NesCartridge000>> = Arc::new(RwLock::new(NesCartridge000::new()));
            cartridge.write().unwrap().load_prog_rom(ines_file.get_prog_rom_data());
            cartridge.write().unwrap().load_char_rom(ines_file.get_char_rom_data());

            let ppu: Arc<RwLock<NesPpu>> = Arc::new(RwLock::new(NesPpu::new(Arc::clone(&cartridge))));
            let apu: Arc<RwLock<NesApu>> = Arc::new(RwLock::new(NesApu::new()));
            let memory = NesMemory::new (Arc::clone(&cartridge), Arc::clone(&ppu), Arc::clone(&apu));
            let cpu: M6502<NesMemory> = M6502::new(memory);

            let mut temp_instance = Self {
                cpu: cpu,
                ppu: Arc::clone(&ppu),
                apu: Arc::clone(&apu),
                left_controller: 0,
                right_controller: 0,
                _debug: 0,
                frame: 0,
            };

            temp_instance.start_up();   
            //temp_instance.cpu.set_nmi();
        
            temp_instance
        }

        fn start_up(&mut self)
        {
            self.cpu.reset();
            self.ppu.write().unwrap().reset();
        }

        fn get_audio(&mut self) -> Vec<u8> {

            let buffer = self.apu.write().unwrap().get_audio_buffer();
            buffer
        }

        pub fn run_frame(&mut self) -> (Vec<u8>, Vec<u8>) {

            self.frame += 1;
            let mut ticks: i32 = 0;
            self.apu.write().unwrap().execute_tick();

            while ticks < TICKS_PER_FRAME as i32 {
              
                if (ticks % 3) == 0 {
                    if self.ppu.read().unwrap().dma_suspend == 0 {
                        self.cpu.execute_tick();
                    }
                    else {
                        self.ppu.write().unwrap().dma_suspend -= 1;
                    }
                }

                self.ppu.write().unwrap().execute_tick();

                if self.ppu.write().unwrap().is_nmi_set() {
                    self.cpu.set_nmi();
                    self.ppu.write().unwrap().reset_nmi();
                }
                self.read_gamepad();
                ticks += 1;
            }
            
            let video = self.ppu.read().unwrap().get_screen();
            let audio = self.get_audio();
            (video, audio)
        }
            
        pub fn left_controler_a(&mut self, value: bool) {
            self.left_controller &= 0xfe;

            if value == true {
                self.left_controller |= 0x01;
            }
        }

        pub fn left_controler_b(&mut self, value: bool) {
            self.left_controller &= 0xfd;

            if value == true {
                self.left_controller |= 0x02;
            }
        }

        pub fn left_controler_select(&mut self, value: bool) {
            self.left_controller &= 0xfb;

            if value == true {
                self.left_controller |= 0x04;
            }
        }

        pub fn left_controler_start(&mut self, value: bool) {
            self.left_controller &= 0xf7;

            if value == true {
                self.left_controller |= 0x08;
            }
        }

        pub fn left_controler_up_down(&mut self, value: i32) {
            self.left_controller &= 0xcf;

            if value < 0 {
                self.left_controller |= 0x10;
            }

            if value > 0 {
                self.left_controller |= 0x20;
            }
        }

        pub fn left_controler_left_right(&mut self, value: i32) {
            self.left_controller &= 0x3f;

            if value < 0 {
                self.left_controller |= 0x40;
            }

            if value > 0 {
                self.left_controller |= 0x80;
            }
        }
            
        fn read_gamepad(&mut self) {

            if self.apu.write().unwrap().is_write_flag_set(0x4016) {
                self.apu.write().unwrap().set_left_controller(self.left_controller);
            }

            if self.apu.write().unwrap().is_write_flag_set(0x4017) {
                self.apu.write().unwrap().set_right_controller(self.right_controller);
            }
        } 

    }
}
