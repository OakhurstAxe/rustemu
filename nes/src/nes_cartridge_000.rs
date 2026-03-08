

pub mod nes {
    use emumemory::memory_rom::emu_memory::MemoryRom;

    use crate::nes_cartridge::nes::NesCartridge;


    pub struct NesCartridge000 {
        cpu_prog_rom_0: Vec<u8>,
        cpu_prog_rom_1: Vec<u8>,
        ppu_char_rom_0: Vec<u8>,
        ppu_char_rom_1: Vec<u8>,
    }

    impl NesCartridge000 {

        pub fn new() -> NesCartridge000 {
            Self {
                cpu_prog_rom_0: vec!(0; 0x4000),
                cpu_prog_rom_1: vec!(0; 0x4000),
                ppu_char_rom_0: vec!(0; 0x2000),
                ppu_char_rom_1: vec!(0; 0x2000),
            }
        }

    }

    impl NesCartridge for NesCartridge000 {    

        fn cpu_read(&self, mut location: u16) -> u8 {

            if location < 0xc000 {

                if location > 0x8000 {
                    location -= 0x8000;
                }
                else {
                    location -= 0x4000;
                }
                
                return self.cpu_prog_rom_0[location as usize];

            }

            location -= 0xc000;
            self.cpu_prog_rom_1[location as usize]
        }
    
        fn cpu_write(&self, _location: u16, _byte: u8) {
            panic!("This cartridge does not support cpu write");
        }

        fn ppu_read(&self, mut location: u16) -> u8 {

            if location < 0x2000 {
                return self.ppu_char_rom_0[location as usize];
            }
            else {
                location -= 0x2000;
                self.ppu_char_rom_1[location as usize]
            }
        }
    
        fn ppu_write(&self, _location: u16, _byte: u8) {
            //panic!("This cartridge does not support ppu write");
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
}
