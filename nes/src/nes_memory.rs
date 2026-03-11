
pub mod nes {

    use std::sync::Arc;
    use std::sync::RwLock;

    use emumemory::{memory_mapper::emu_memory::MemoryMapper, memory_ram::emu_memory::MemoryRam};
    use emumemory::base_memory::emu_memory::BaseMemory;
    use emumemory::memory_ram_flagged::emu_memory::MemoryRamFlagged;
    use crate::nes_cartridge::nes::NesCartridge;
    use crate::nes_ppu::nes::NesPpu;

    pub struct NesMemory {
        cartridge: Arc<RwLock<dyn NesCartridge>>,
        cpu_work_ram: MemoryRam,
        cpu_apu_io_registers: MemoryRamFlagged,
        left_controller: u8,
        right_controller: u8,
        ppu: Arc<RwLock<NesPpu>>,
        ppu_addr_count: i32,
        ppu_addr_h: u8,
        ppu_addr_l: u8,
        //ppu_addr: u16,
        ppu_oam_addr: u8,
        dma_suspend: u8,
        debug: u8,
    }   

    impl NesMemory { 
        pub fn new(cartridge: Arc<RwLock<dyn NesCartridge>>, ppu: Arc<RwLock<NesPpu>>) -> NesMemory {
            Self {
                cartridge: cartridge,
                cpu_work_ram: MemoryRam::new(String::from("CPU Work RAM"), 0x0800),
                cpu_apu_io_registers: MemoryRamFlagged::new(0x001f, String::from("APU IO Registers")),
                ppu: ppu,
                left_controller: 0,
                right_controller: 0,
                ppu_addr_count: 0,
                ppu_addr_h: 0,
                ppu_addr_l: 0,
                //ppu_addr: 0,
                ppu_oam_addr: 0,
                dma_suspend: 0,
                debug: 0,
            }
        }

        pub fn cpu_read_flagged(&mut self, mut location: u16) -> bool {

            if location < 0x4000 {
                panic!("Bad location address {} for read flag", location);
            }            
            else if location < 0x4020 {
                location -= 0x4000;
                return self.cpu_apu_io_registers.is_read_flag_set(location);
            }
            panic!("Bad location address {} for read flag", location);
        }

        pub fn cpu_write_flagged(&mut self, mut location: u16) -> bool {

            if location < 0x4000 {
                panic!("Bad location address {} for write flag", location);
            }            
            else if location < 0x4020 {
                location -= 0x4000;
                return self.cpu_apu_io_registers.is_write_flag_set(location);
            }

            panic!("Bad location address {} for write flag", location);
        }

        fn set_left_controller(&mut self, byte: u8) {
            self.left_controller = byte;
        }

        fn set_right_controller(&mut self, byte: u8) {
            self.right_controller = byte;
        }
    
    }

    unsafe impl Send for NesMemory {}

    impl MemoryMapper for NesMemory {

        fn cpu_read(&mut self, mut location: u16) -> u8 {

            let original_location: u16 = location;
            
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
                    let result: u8 = ((self.left_controller & 0x01));
                    self.left_controller >>= 1;
                    return result;
                }

                if location == 0x17 {
                    let result: u8 = ((self.right_controller & 0x01));
                    self.right_controller >>= 1;
                    return result;
                }

                return self.cpu_apu_io_registers.read(location);
            }

            // Cartridge RAM/ROM
            return self.cartridge.read().unwrap().cpu_read(location);
            
        }

        fn cpu_write(&mut self, mut location: u16, byte: u8) {
            
            let original_location: u16 = location;
            
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
                self.cpu_apu_io_registers.write(location, byte);
                return;
            }
            
            // Cartridge RAM/ROM
            self.cartridge.write().unwrap().cpu_write(location, byte);

        }

    }
}
