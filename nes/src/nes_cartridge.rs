pub mod nes {

    use emucpu::prelude::*;
    
    use crate::nes_inesfile::nes::INesFile;
    use crate::nes_cartridge_000::nes::NesCartridge000;
    use crate::nes_cartridge_004::nes::NesCartridge004;

    pub trait NesCartridgeTrait: Sync {        
        fn execute_tick(&mut self, addr: &mut AddressBus);
        fn cpu_read(&self, location: u16) -> u8;
        fn cpu_write(&mut self, location: u16, byte: u8);
        fn ppu_read(&mut self, location: u16) -> u8;
        fn ppu_write(&self, location: u16, byte: u8);
        fn is_irq_set(&self) -> bool;
        fn reset_irq(&mut self);
    }

    pub struct NesCartridge {
        pub cpu_prog_ram_0: Vec<u8>,
        pub cpu_prog_rom: Vec<Vec<u8>>,
        pub ppu_char_rom: Vec<Vec<u8>>,
        pub irq_set: bool,
    }

    impl NesCartridge {
        pub fn new(ines: &INesFile) -> NesCartridge {
            let mut cart = NesCartridge {
                cpu_prog_ram_0: vec![0; 0x2000],
                cpu_prog_rom: vec![Vec::new(); 16],
                ppu_char_rom: vec![Vec::new(); 128],
                irq_set: false,
            };

            cart.load_prog_rom(ines.get_prog_rom_data());
            cart.load_char_rom(ines.get_char_rom_data());
            cart
        }

        fn load_prog_rom(&mut self, data: Vec<u8>) {
            
            let rom_count = data.len() / 0x2000;

            for i in 0..rom_count {
                self.cpu_prog_rom[i] = data[(i*0x2000)..(i*0x2000)+0x2000].to_vec();
            }

            // if 0x4000 size, double copy prog roms
            if data.len() == 0x4000 {
                for i in rom_count..rom_count*2 {
                    self.cpu_prog_rom[i] = data[((i - rom_count)*0x2000)..((i - rom_count)*0x2000)+0x2000].to_vec();
                }
            }
        }
    
        fn load_char_rom(&mut self, data: Vec<u8>) {
            
            let rom_count = data.len() / 0x400;

            for i in 0..rom_count {
                self.ppu_char_rom[i] = data[(i*0x400)..(i*0x400)+0x400].to_vec();
            }
        }
    }

    pub struct NesCartridgeSelector {}

    impl NesCartridgeSelector {

        pub fn select_cartridge(ines: &INesFile, cart_data: NesCartridge) -> Box<dyn NesCartridgeTrait> {
            
            let mapper_id = ines.get_memory_mapper();

            match mapper_id {
                0 =>  {
                    println!("Cart mapper 000");
                    Box::new(NesCartridge000::new(cart_data))
                },
                4 => {
                    println!("Cart mapper 004");
                    Box::new(NesCartridge004::new(cart_data))
                },
                _ => panic!("Unsupported mapper ID: {}", mapper_id),
            }
        }
    }
}