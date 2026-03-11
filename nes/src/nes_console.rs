

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

    const TICKS_PER_FRAME: u16 =  59736;

    pub struct NesConsole {
        cpu: M6502,
        ppu: Arc<RwLock<NesPpu>>,
        //apu: NesApu,
        frame_rendered: bool,
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
            let memory: Box<dyn MemoryMapper + Send> = Box::new(NesMemory::new (Arc::clone(&cartridge), Arc::clone(&ppu)));
            let cpu: M6502 = M6502::new(memory);

            let mut temp_instance = Self {
                cpu: cpu,
                ppu: Arc::clone(&ppu),
                frame_rendered: false,
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
            //apu_->ExecuteTick();
            
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
//                ReadGamepad();
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

        /*
        fn read_gamepad(&mut self) {

            if self.nes_memory.cpu_write_flagged(0x4016) {
                //self.nes_memory.set_left_controller(nesMainWindow_->leftController_);
            }

            if self.nes_memory.cpu_write_flagged(0x4017) {
                //nesMemory_->SetRightController(nesMainWindow_->rightController_);
            }
        } 
        */   
    }
}
