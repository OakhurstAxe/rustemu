pub mod nes {

    use emucpu::prelude::*;
    
    use crate::nes_inesfile::nes::INesFile;
    use crate::nes_cartridge_000::nes::NesCartridge000;
    use crate::nes_cartridge_004::nes::NesCartridge004;

    pub trait NesCartridgeTrait: Sync {
        
        fn execute_tick(&mut self, addr: &mut AddressBus);

        fn cpu_read(&self, location: u16) -> u8;

        fn cpu_write(&mut self, location: u16, byte: u8);

        fn ppu_read(&self, location: u16) -> u8;

        fn ppu_write(&self, location: u16, byte: u8);

    }

    pub struct NesCartridge {
        pub cpu_prog_ram_0: Vec<u8>,

        pub cpu_prog_rom_0: Vec<u8>,
        pub cpu_prog_rom_1: Vec<u8>,
        pub cpu_prog_rom_2: Vec<u8>,
        pub cpu_prog_rom_3: Vec<u8>,
        pub cpu_prog_rom_4: Vec<u8>,
        pub cpu_prog_rom_5: Vec<u8>,
        pub cpu_prog_rom_6: Vec<u8>,
        pub cpu_prog_rom_7: Vec<u8>,

        pub cpu_prog_rom_mmc3: Vec<Vec<u8>>,
        pub ppu_char_rom_mmc3: Vec<Vec<u8>>,
        
        pub ppu_char_rom_0: Vec<u8>,
        pub ppu_char_rom_1: Vec<u8>,
        pub ppu_char_rom_2: Vec<u8>,
        pub ppu_char_rom_3: Vec<u8>,
        pub ppu_char_rom_4: Vec<u8>,
        pub ppu_char_rom_5: Vec<u8>,
        pub ppu_char_rom_6: Vec<u8>,
        pub ppu_char_rom_7: Vec<u8>,
        pub ppu_char_rom_8: Vec<u8>,
        pub ppu_char_rom_9: Vec<u8>,
        pub ppu_char_rom_10: Vec<u8>,
        pub ppu_char_rom_11: Vec<u8>,
        pub ppu_char_rom_12: Vec<u8>,
        pub ppu_char_rom_13: Vec<u8>,
        pub ppu_char_rom_14: Vec<u8>,
        pub ppu_char_rom_15: Vec<u8>,
        pub ppu_char_rom_16: Vec<u8>,
    }

    impl NesCartridge {
        pub fn new(ines: &INesFile) -> NesCartridge {
            let mut cart = NesCartridge {
                cpu_prog_ram_0: vec![0; 0x2000],

                cpu_prog_rom_0: Vec::new(),
                cpu_prog_rom_1: Vec::new(),
                cpu_prog_rom_2: Vec::new(),
                cpu_prog_rom_3: Vec::new(),
                cpu_prog_rom_4: Vec::new(),
                cpu_prog_rom_5: Vec::new(),
                cpu_prog_rom_6: Vec::new(),
                cpu_prog_rom_7: Vec::new(),
                cpu_prog_rom_mmc3: vec![Vec::new(); 16],
                ppu_char_rom_mmc3: vec![Vec::new(); 128],

                ppu_char_rom_0: Vec::new(),
                ppu_char_rom_1: Vec::new(),
                ppu_char_rom_2: Vec::new(),
                ppu_char_rom_3: Vec::new(),
                ppu_char_rom_4: Vec::new(),
                ppu_char_rom_5: Vec::new(),
                ppu_char_rom_6: Vec::new(), 
                ppu_char_rom_7: Vec::new(), 
                ppu_char_rom_8: Vec::new(), 
                ppu_char_rom_9: Vec::new(), 
                ppu_char_rom_10: Vec::new(),
                ppu_char_rom_11: Vec::new(),
                ppu_char_rom_12: Vec::new(),
                ppu_char_rom_13: Vec::new(),
                ppu_char_rom_14: Vec::new(),
                ppu_char_rom_15: Vec::new(),
                ppu_char_rom_16: Vec::new(),
            };

            cart.load_prog_rom(ines.get_prog_rom_data());
            cart.load_char_rom(ines.get_char_rom_data());
            cart
        }

        fn load_prog_rom(&mut self, data: Vec<u8>) {
            
            match data.len() {
                0x4000 => {
                    self.cpu_prog_rom_0 = data[0..0x4000].to_vec();
                    self.cpu_prog_rom_1 = data[0..0x4000].to_vec();
                },
                0x8000 => {
                    self.cpu_prog_rom_0 = data[0..0x4000].to_vec();
                    self.cpu_prog_rom_1 = data[0x4000..0x8000].to_vec();
                },
                0x20000 => {
                    for i in 0..16 {
                        self.cpu_prog_rom_mmc3[i] = data[(i*0x2000)..(i*0x2000)+0x2000].to_vec();
                    }
                },
                _ => {
                    panic!("Unsupported program ROM size: {:x}", data.len());
                }
            }
        }
    
        fn load_char_rom(&mut self, data: Vec<u8>) {
            
            match data.len() {
                0x2000 => {
                    self.ppu_char_rom_0 = data[0..0x2000].to_vec();
                    self.ppu_char_rom_1 = data[0..0x2000].to_vec();
                },
                0x4000 => {
                    self.ppu_char_rom_0 = data[0..0x2000].to_vec();
                    self.ppu_char_rom_1 = data[0x2000..0x4000].to_vec();
                },
                0x20000 => {
                    for i in 0..128 {
                        self.ppu_char_rom_mmc3[i] = data[(i*0x400)..(i*0x400)+0x400].to_vec();
                    }
                    /*
                    self.ppu_char_rom_0 = data[0..0x2000].to_vec();
                    self.ppu_char_rom_1 = data[0x2000..0x4000].to_vec();
                    self.ppu_char_rom_2 = data[0x4000..0x6000].to_vec();
                    self.ppu_char_rom_3 = data[0x6000..0x8000].to_vec();
                    self.ppu_char_rom_4 = data[0x8000..0xA000].to_vec();
                    self.ppu_char_rom_5 = data[0xA000..0xC000].to_vec();
                    self.ppu_char_rom_6 = data[0xC000..0xE000].to_vec();
                    self.ppu_char_rom_7 = data[0x10000..0x12000].to_vec();
                    self.ppu_char_rom_8 = data[0x12000..0x14000].to_vec();
                    self.ppu_char_rom_9 = data[0x14000..0x16000].to_vec();
                    self.ppu_char_rom_10 = data[0x16000..0x18000].to_vec();
                    self.ppu_char_rom_11 = data[0x18000..0x1A000].to_vec();
                    self.ppu_char_rom_12 = data[0x1A000..0x1C000].to_vec();
                    self.ppu_char_rom_13 = data[0x1C000..0x1E000].to_vec();
                    self.ppu_char_rom_14 = data[0x1E000..0x20000].to_vec();
                    self.ppu_char_rom_15 = data[0x1E000..0x20000].to_vec();
                    self.ppu_char_rom_16 = data[0x1E000..0x20000].to_vec();
                     */
                },
                _ => {
                    panic!("Unsupported character ROM size: {:x}", data.len());
                }
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