
pub mod nes {

    use std::fs;

    use crate::nes_cartridge::nes::NesCartridge;
    use crate::nes_cartridge_000::nes::NesCartridge000;

    pub struct INesFile {
        prog_rom_data: Vec<u8>,
        prog_rom_size_lsb: u16,
        char_rom_data: Vec<u8>,
        char_rom_size_lsb: u16,
        cartridge_flags: u8,
        console_type_flags: u8,
        mapper_flags: u8,
        memory_mapper: u16,
        rom_size_flags: u8,
        header: [u8; 4],
        ram_eprom_size_flags: u8,
        char_rom_size_flags: u8,
        trainer: Vec<u8>,
    }

    impl INesFile {

        pub fn new() -> INesFile {
            Self {
                prog_rom_data: vec![0; 0],
                prog_rom_size_lsb: 0,
                char_rom_data: vec![0; 0],
                char_rom_size_lsb: 0,
                cartridge_flags: 0,
                console_type_flags: 0,
                mapper_flags: 0,
                memory_mapper: 0,
                rom_size_flags: 0,
                header: [0, 0, 0, 0],
                ram_eprom_size_flags: 0,
                char_rom_size_flags: 0,
                trainer: vec![0; 0],
            }
        }

        pub fn get_nes_cargridge(&self) -> Box<dyn NesCartridge> {
            Box::new(NesCartridge000::new())
        }

        pub fn get_prog_rom_data(&self) -> Vec<u8> {
            self.prog_rom_data.clone()
        }
        
        fn get_prog_rom_size(&self) -> u8 {
            self.prog_rom_data.len() as u8
        }
        
        pub fn get_char_rom_data(&self) -> Vec<u8> {
            self.char_rom_data.clone()
        }
        
        fn get_char_rom_size(&self) -> u8 {
            self.char_rom_data.len() as u8
        }

        fn get_memory_mapper(&self) -> u16 {
            self.memory_mapper
        }

        pub fn load_file(&mut self, file_name: String) {            
            let file_data: Vec<u8> = fs::read(file_name).unwrap();

            let mut position = 0;
//            self.header = file_data[0..3].to_vec();
            position += 4;
            self.prog_rom_size_lsb = file_data[position] as u16;
            position += 1;
            self.char_rom_size_lsb = file_data[position] as u16;
            position += 1;
            self.cartridge_flags = file_data[position];
            position += 1;
            self.console_type_flags = file_data[position];
            position += 1;
            self.mapper_flags = file_data[position];
            position += 1;
            self.rom_size_flags = file_data[position];
            position += 1;
            self.ram_eprom_size_flags = file_data[position];
            position += 1;
            self.char_rom_size_flags = file_data[position];
            position += 5;

            self.memory_mapper = ((((self.mapper_flags & 0x0F) as u16) << 8) +
                (((self.console_type_flags & 0x0F) as u16) << 4) +
                (self.cartridge_flags & 0x0F) as u16) as u16;
            
            let prog_rom_size: u16 = ((((self.rom_size_flags & 0x0F) as u16) << 8) + self.prog_rom_size_lsb) as u16;
            let mut char_rom_size: u16 = ((((self.rom_size_flags & 0xF0) as u16) << 8) + self.char_rom_size_lsb) as u16;

            if char_rom_size == 0 {
                char_rom_size = 1;
            }

            if (self.cartridge_flags & 0x04) != 0 {
                self.trainer = file_data[position..(position + 512)].to_vec();
                position += 512;
            }

            self.prog_rom_data = file_data[position..(position + 0x4000 * prog_rom_size as usize)].to_vec();
            position += 0x4000 * prog_rom_size as usize;
            self.char_rom_data = file_data[position..(position + 0x2000 * char_rom_size as usize)].to_vec();
        }

    }

}
