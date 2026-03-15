
pub mod nes {

    use std::sync::Arc;
    use std::sync::RwLock;

    use emumemory::{memory_mapper::emu_memory::MemoryMapper, memory_ram::emu_memory::MemoryRam};
    use emumemory::base_memory::emu_memory::BaseMemory;
    use crate::nes_cartridge::nes::NesCartridge;
    use crate::nes_ppu::nes::NesPpu;
    use crate::nes_apu::nes::NesApu;

    pub struct NesMemory {
        cartridge: Arc<RwLock<dyn NesCartridge>>,
        cpu_work_ram: MemoryRam,
        ppu: Arc<RwLock<NesPpu>>,
        apu: Arc<RwLock<NesApu>>,
        dma_suspend: u8,
        _debug: u8,
    }   

    impl NesMemory { 
        pub fn new(cartridge: Arc<RwLock<dyn NesCartridge>>, ppu: Arc<RwLock<NesPpu>>, apu: Arc<RwLock<NesApu>>) -> NesMemory {
            Self {
                cartridge: cartridge,
                cpu_work_ram: MemoryRam::new(String::from("CPU Work RAM"), 0x0800),
                ppu: ppu,
                apu: apu,
                dma_suspend: 0,
                _debug: 0,
            }
        }
    }

    unsafe impl Send for NesMemory {}

    impl MemoryMapper for NesMemory {

        fn cpu_read(&mut self, mut location: u16) -> u8 {

            // Working RAM
            if location < 0x2000 {
                location = location % 0x800;  // mirroring
                return self.cpu_work_ram.read(location);
            }   

            // PPU Registers
            else if location < 0x4000 {
                return self.ppu.write().unwrap().ppu_register_read(location);
            }

            // APU and IO Registers
            else if location < 0x4020 {                
                location -= 0x4000;

                if location == 0x16 {
                    return self.apu.write().unwrap().get_left_controller();
                }

                if location == 0x17 {
                    return self.apu.write().unwrap().get_right_controller();
                }

                return self.apu.write().unwrap().read(location);
            }

            // Cartridge RAM/ROM
            return self.cartridge.read().unwrap().cpu_read(location);
            
        }

        fn cpu_write(&mut self, mut location: u16, byte: u8) {
            
            // Working RAM
            if location < 0x2000 {
                location = location % 0x800;  // mirroring
                self.cpu_work_ram.write(location, byte);
                return;
            }
            
            // PPU Registers
            else if location < 0x4000 {
                self.ppu.write().unwrap().ppu_register_write(location, byte);
                return;
            }
            
            // APU and IO Registers            
            else if location < 0x4020 {

                if location == 0x4014 {
                    self.dma_suspend = 154;
                    self.ppu.write().unwrap().dma_suspend += 154;
//                    self.cpu.DmaSuspend();
                    let cpu_addr: u16 = (byte as u16) << 8;

                    for i in 0..256 {
                        let sprite_data: u8 = self.cpu_read(cpu_addr + i);
                        self.ppu.write().unwrap().oam_write(i, sprite_data);
                    }
                    
                    return;
                }
                
                location -= 0x4000;
                self.apu.write().unwrap().write(location, byte);
                return;
            }
            
            // Cartridge RAM/ROM
            self.cartridge.write().unwrap().cpu_write(location, byte);

        }

    }
}
