pub mod nes {

    use emucpu::prelude::*;
    
    use crate::nes_inesfile::nes::INesFile;
    use crate::nes_cartridge_000::nes::NesCartridge000;

    pub trait NesCartridgeTrait: Sync {
        
        fn execute_tick(&mut self, addr: &mut AddressBus, cartridge: &NesCartridge);

        fn cpu_read(&self, location: u16, cartridge: &NesCartridge) -> u8;

        fn cpu_write(&self, location: u16, byte: u8, cartridge: &NesCartridge);

        fn ppu_read(&self, location: u16, cartridge: &NesCartridge) -> u8;

        fn ppu_write(&self, location: u16, byte: u8, cartridge: &NesCartridge);

    }

    pub struct NesCartridge {
        pub cpu_prog_rom_0: Vec<u8>,
        pub cpu_prog_rom_1: Vec<u8>,
        pub ppu_char_rom_0: Vec<u8>,
        pub ppu_char_rom_1: Vec<u8>,
    }

    impl NesCartridge {
        pub fn new(ines: &INesFile) -> NesCartridge {
            let mut cart = NesCartridge {
                cpu_prog_rom_0: Vec::new(),
                cpu_prog_rom_1: Vec::new(),
                ppu_char_rom_0: Vec::new(),
                ppu_char_rom_1: Vec::new(),

            };

            cart.load_prog_rom(ines.get_prog_rom_data());
            cart.load_char_rom(ines.get_char_rom_data());
            cart
        }

        fn load_prog_rom(&mut self, data: Vec<u8>) {
            
            if data.len() == 0x4000 {
                self.cpu_prog_rom_0 = data[0..0x4000].to_vec();
                self.cpu_prog_rom_1 = data[0..0x4000].to_vec();
            }
            if data.len() == 0x8000 {
                self.cpu_prog_rom_0 = data[0..0x4000].to_vec();
                self.cpu_prog_rom_1 = data[0x4000..0x8000].to_vec();
            }
        }
    
        fn load_char_rom(&mut self, data: Vec<u8>) {
            
            if data.len() == 0x2000 {
                self.ppu_char_rom_0 = data[0..0x2000].to_vec();
                self.ppu_char_rom_1 = data[0..0x2000].to_vec();
            }
            if data.len() == 0x4000 {
                self.ppu_char_rom_0 = data[0..0x2000].to_vec();
                self.ppu_char_rom_1 = data[0x2000..0x4000].to_vec();
            }
        }
    }

    pub struct NesCartridgeSelector {}

    impl NesCartridgeSelector {

        pub fn select_cartridge(ines: &INesFile) -> Box<dyn NesCartridgeTrait> {
            
            let mapper_id = ines.get_memory_mapper();

            match mapper_id {
                0 => Box::new(NesCartridge000::new()),
                _ => panic!("Unsupported mapper ID: {}", mapper_id),
            }
        }
    }
}