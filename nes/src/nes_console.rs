

pub mod nes {

    use std::sync::{ Arc, RwLock };

    use emucpu::base_cpu::emu_cpu::BaseCpu;
    use emucpu::m6502::emu_cpu::M6502;
    use emumemory::memory_mapper::emu_memory::MemoryMapper;

    use sdl2::event::EventSender;

    use crate::nes_cartridge::nes::NesCartridge;
    use crate::{nes_cartridge_000::nes::NesCartridge000, nes_memory::nes::NesMemory};
    use crate::nes_ppu::nes::NesPpu;
    use crate::nes_inesfile::nes::INesFile;
    use crate::nes_apu::nes::NesApu;

    const TICKS_PER_FRAME: u16 =  59736;

    pub struct NesConsole {
        cpu: M6502,
        ppu: Arc<RwLock<NesPpu>>,
        apu: Arc<RwLock<NesApu>>,
        frame_rendered: bool,
        left_controller: u8,
        right_controller: u8,
        debug: u8,
    }

    unsafe impl Send for NesConsole {}

    impl NesConsole {

        pub fn new (rom_file: &str, sender: EventSender) -> NesConsole {
            let mut ines_file: INesFile = INesFile::new();
            ines_file.load_file(rom_file);
            let mut cartridge: Arc<RwLock<dyn NesCartridge>> = Arc::new(RwLock::new(NesCartridge000::new()));
            cartridge.write().unwrap().load_prog_rom(ines_file.get_prog_rom_data());
            cartridge.write().unwrap().load_char_rom(ines_file.get_char_rom_data());

            let ppu: Arc<RwLock<NesPpu>> = Arc::new(RwLock::new(NesPpu::new(Arc::clone(&cartridge))));
            let apu: Arc<RwLock<NesApu>> = Arc::new(RwLock::new(NesApu::new()));
            let memory: Box<dyn MemoryMapper + Send> = Box::new(NesMemory::new (Arc::clone(&cartridge), Arc::clone(&ppu), Arc::clone(&apu)));
            let cpu: M6502 = M6502::new(memory);

            let mut temp_instance = Self {
                cpu: cpu,
                ppu: Arc::clone(&ppu),
                apu: Arc::clone(&apu),
                frame_rendered: false,
                left_controller: 0,
                right_controller: 0,
                debug: 0,
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

        pub fn start_next_frame(&mut self) {

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
            self.frame_rendered = true;
            //nesMainWindow_->DrawFrame(ppu_->GetScreen());
        }
            
        pub fn is_frame_rendered(&mut self) -> (bool, Vec<u8>) {
            let result = self.frame_rendered;
            self.frame_rendered = false;
            (result, self.ppu.read().unwrap().get_screen())
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
