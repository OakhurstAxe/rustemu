
pub mod nes {

    use std::sync::Arc;
    use std::sync::RwLock;
    use std::rc::Rc;
    use std::cell::RefCell;

    use emumemory::{memory_mapper::emu_memory::MemoryMapper, memory_ram::emu_memory::MemoryRam};
    use emumemory::base_memory::emu_memory::BaseMemory;
    use crate::nes_cartridge::nes::NesCartridge;
    use crate::nes_cartridge_000::nes::NesCartridge000;
    use crate::nes_ppu::nes::NesPpu;
    use crate::nes_apu::nes::NesApu;

    pub struct NesMemory {
        cartridge: Arc<RwLock<NesCartridge000>>,
        cpu_work_ram: MemoryRam,
        pub ppu: NesPpu,
        apu: Arc<RwLock<NesApu>>,
        read_bus: u8,
        ppu_dma_address: u16,
        ppu_dma_write: u16,
        apu_dma_address: u16,
        apu_dma_write: u16,
        _debug: u8,
    }   

    impl NesMemory { 
        pub fn new(cartridge: Arc<RwLock<NesCartridge000>>, ppu: NesPpu, apu: Arc<RwLock<NesApu>>) -> NesMemory {
            Self {
                cartridge: cartridge,
                cpu_work_ram: MemoryRam::new(String::from("CPU Work RAM"), 0x0800),
                ppu: ppu,
                apu: apu,
                read_bus: 0,
                ppu_dma_address: 0,
                ppu_dma_write: 0,
                apu_dma_address: 0,
                apu_dma_write: 0,
                _debug: 0,
            }
        }

    }

    unsafe impl Send for NesMemory {}

    impl MemoryMapper for NesMemory {

        fn get_dma_write(&self) -> u16 {
            self.ppu_dma_write + self.apu_dma_write
        }

        fn execute_tick(&mut self) {

            // Execute PPU DMA
            if self.apu_dma_write > 0 {
                let byte: u8 = self.cpu_read(self.apu_dma_address);
                self.read_bus = byte;
                self.apu_dma_address += 1;
                self.apu_dma_write -= 1;
                if self.apu_dma_write == 0 {
                    self.apu.write().unwrap().write(0, 0);
                }

            }
            else if self.ppu_dma_write > 0 {
                let byte: u8 = self.cpu_read(self.ppu_dma_address);
                self.ppu.oam_write(256 - self.ppu_dma_write, byte);
                self.read_bus = byte;
                self.ppu_dma_address += 1;
                self.ppu_dma_write -= 1;
            }
        }

        fn cpu_read(&mut self, mut location: u16) -> u8 {

            // Working RAM
            if location < 0x2000 {
                location = location % 0x800;  // mirroring
                let byte = self.cpu_work_ram.read(location);
                self.read_bus = byte;
                return byte;
            }   

            // PPU Registers
            else if location < 0x4000 {
                let byte = self.ppu.ppu_register_read(location);
                self.read_bus = byte;
                return byte;
            }

            // APU and IO Registers
            else if location > 0x4000 && location < 0x4018 {                
                location -= 0x4000;

                if location == 0x16 {
                    let byte = (self.apu.write().unwrap().get_left_controller() & 0x1f) + (self.read_bus & 0xe0);
                    self.read_bus = byte;
                    return byte;
                }

                if location == 0x17 {
                    let byte = (self.apu.write().unwrap().get_right_controller() & 0x1f) + (self.read_bus & 0xe0);
                    self.read_bus = byte;
                    return byte;
                }

                if location == 0x15 {
                    return self.read_bus;
                }

                let byte = self.apu.write().unwrap().read(location);
                self.read_bus = byte;
                return byte;
            }

            // Cartridge RAM/ROM
            else if location >= 0x6000 {
                let byte = self.cartridge.read().unwrap().cpu_read(location);
                self.read_bus = byte;
                return byte;
            }

            self.read_bus
            
        }

        fn cpu_write(&mut self, mut location: u16, byte: u8) -> bool {
            
            self.read_bus = byte;

            // Working RAM
            if location < 0x2000 {
                location = location % 0x800;  // mirroring
                self.cpu_work_ram.write(location, byte);
                return true;
            }
            
            // PPU Registers
            else if location < 0x4000 {
                self.ppu.ppu_register_write(location, byte);
                return true;
            }
            
            // APU and IO Registers            
            else if location < 0x401f {

                // PPU DMA
                if location == 0x4014 {
                    self.ppu_dma_write = 256;
                    self.ppu_dma_address = (byte as u16) << 8;
                    return true;
                }

                if location == 0x4015 && ((byte & 0x10) > 0) {
                    let apu_address: u16 = self.apu.write().unwrap().read(0x12) as u16;
                    self.apu_dma_address = 0xC0 + (apu_address << 6);
                    let length = self.apu.write().unwrap().read(0x13) as u16;
                    self.apu_dma_write = (length << 4) + 1;
                }
                
                location -= 0x4000;
                self.apu.write().unwrap().write(location, byte);
                return true;
            }
            
            // Cartridge RAM/ROM
            self.cartridge.write().unwrap().cpu_write(location, byte);

            true
        }

    }
}
