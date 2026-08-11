

pub mod nes {

    use emucpu::prelude::*;
    
    use crate::nes_cartridge::nes::NesCartridge;
    use crate::nes_cartridge::nes::NesCartridgeTrait;

    pub struct NesCartridge004 {
        cart_data: NesCartridge,
        
        target_register: u8,
        prog_bank_mode: u8,
        char_inversion: u8,

        prog_bank_8000: u8,
        prog_bank_a000: u8,
        prog_bank_c000: u8,
        prog_bank_e000: u8,

        char_bank_0000: u8,
        char_bank_0400: u8,
        char_bank_0800: u8,
        char_bank_0c00: u8,
        char_bank_1000: u8,
        char_bank_1400: u8,
        char_bank_1800: u8,
        char_bank_1c00: u8,
    }

    impl NesCartridge004 {

        pub fn new(cart_data: NesCartridge) -> NesCartridge004 {
            Self { 
                cart_data,
                target_register: 0,
                prog_bank_mode: 0,
                char_inversion: 0,

                prog_bank_8000: 6,
                prog_bank_a000: 7,
                prog_bank_c000: 6,
                prog_bank_e000: 7,

                char_bank_0000: 0,
                char_bank_0400: 0,
                char_bank_0800: 1,
                char_bank_0c00: 0,
                char_bank_1000: 2,
                char_bank_1400: 3,
                char_bank_1800: 4,
                char_bank_1c00: 5,
              }
        }
    }

    impl NesCartridgeTrait for NesCartridge004 {    

        fn execute_tick(&mut self, addr: &mut AddressBus) {

            if addr.address >= 0x6000 {
                if addr.write {
                    self.cpu_write(addr.address, addr.byte);
                    addr.write = false;
                } else {
                    addr.byte = self.cpu_read(addr.address);
                }
            }

        }

        fn cpu_read(&self, location: u16) -> u8 {

            if !(0x6000..=0xFFFF).contains(&location) {
                return 0;
            }

            let mut local_location = location;

            match location {
                0x6000..=0x7FFF => {
                    local_location -= 0x6000;
                    self.cart_data.cpu_prog_ram_0[local_location as usize]
                },
                0x8000..=0xBFFF => {
                    local_location -= 0x8000;

                    match self.prog_bank_8000 {
                        0 => self.cart_data.cpu_prog_rom_0[local_location as usize],
                        1 => self.cart_data.cpu_prog_rom_1[local_location as usize],
                        2 => self.cart_data.cpu_prog_rom_2[local_location as usize],
                        3 => self.cart_data.cpu_prog_rom_3[local_location as usize],
                        4 => self.cart_data.cpu_prog_rom_4[local_location as usize],
                        5 => self.cart_data.cpu_prog_rom_5[local_location as usize],
                        6 => self.cart_data.cpu_prog_rom_6[local_location as usize],
                        7 => self.cart_data.cpu_prog_rom_7[local_location as usize],
                        _ => panic!("Unknwn bank number for $8000 {}", self.prog_bank_8000)
                    }
                },
                0xC000..=0xFFFF => {
                    local_location -= 0xC000;

                    match self.prog_bank_a000 {
                        0 => self.cart_data.cpu_prog_rom_0[local_location as usize],
                        1 => self.cart_data.cpu_prog_rom_1[local_location as usize],
                        2 => self.cart_data.cpu_prog_rom_2[local_location as usize],
                        3 => self.cart_data.cpu_prog_rom_3[local_location as usize],
                        4 => self.cart_data.cpu_prog_rom_4[local_location as usize],
                        5 => self.cart_data.cpu_prog_rom_5[local_location as usize],
                        6 => self.cart_data.cpu_prog_rom_6[local_location as usize],
                        7 => self.cart_data.cpu_prog_rom_7[local_location as usize],
                        _ => panic!("Unknwn bank number for $A000 {}", self.prog_bank_a000)
                    }
                },
                /*
                0xC000..=0xFFFF => {
                    local_location -= 0xC000;
                    match self.prog_bank_c000 {
                        0 => self.cart_data.cpu_prog_rom_0[local_location as usize],
                        1 => self.cart_data.cpu_prog_rom_1[local_location as usize],
                        2 => self.cart_data.cpu_prog_rom_2[local_location as usize],
                        3 => self.cart_data.cpu_prog_rom_3[local_location as usize],
                        4 => self.cart_data.cpu_prog_rom_4[local_location as usize],
                        5 => self.cart_data.cpu_prog_rom_5[local_location as usize],
                        6 => self.cart_data.cpu_prog_rom_6[local_location as usize],
                        7 => self.cart_data.cpu_prog_rom_7[local_location as usize],
                        _ => panic!("Unknwn bank number for $C000 {}", self.prog_bank_c000)
                    }
                }
                0xF000..=0xFFFF => {
                    local_location -= 0xF000;
                    match self.prog_bank_e000 {
                        0 => self.cart_data.cpu_prog_rom_0[local_location as usize],
                        1 => self.cart_data.cpu_prog_rom_1[local_location as usize],
                        2 => self.cart_data.cpu_prog_rom_2[local_location as usize],
                        3 => self.cart_data.cpu_prog_rom_3[local_location as usize],
                        4 => self.cart_data.cpu_prog_rom_4[local_location as usize],
                        5 => self.cart_data.cpu_prog_rom_5[local_location as usize],
                        6 => self.cart_data.cpu_prog_rom_6[local_location as usize],
                        7 => self.cart_data.cpu_prog_rom_7[local_location as usize],
                        _ => panic!("Unknwn bank number for $E000 {}", self.prog_bank_e000)
                    }
                } */
                _ => {
                    0
                }

            }
        }
    
        fn cpu_write(&mut self, location: u16, byte: u8) {
            
            if location.is_multiple_of(2) {
                self.target_register = byte & 0x07;
                self.prog_bank_mode = byte & 0x40;
                self.char_inversion = byte & 0x80;
            } else {
                
                match self.target_register {
                    0x07 => {
                        self.prog_bank_8000 = byte & 0x07;
                    },
                    0x08 => {
                        self.prog_bank_a000 = byte & 0x07;
                    }
                    _ => {}
                }

                match self.char_inversion {
                    0 => {
                        self.char_bank_0000 = 0;
                        self.char_bank_0400 = 1;
                        self.char_bank_0800 = 1;
                        self.char_bank_0c00 = 2;
                        self.char_bank_1000 = 2;
                        self.char_bank_1400 = 3;
                        self.char_bank_1800 = 4;
                        self.char_bank_1c00 = 5;
                    },
                    1 => {
                        self.char_bank_0000 = 2;
                        self.char_bank_0400 = 3;
                        self.char_bank_0800 = 4;
                        self.char_bank_0c00 = 5;
                        self.char_bank_1000 = 0;
                        self.char_bank_1400 = 1;
                        self.char_bank_1800 = 1;
                        self.char_bank_1c00 = 2;
                    },
                    _ => {
                    },
                }

                match self.prog_bank_mode {
                    0 => {
                        self.prog_bank_8000 = 6;
                        self.prog_bank_a000 = 7;
                        self.prog_bank_c000 = 6;
                        self.prog_bank_e000 = 7;
                    },
                    1 => {
                        self.prog_bank_8000 = 6;
                        self.prog_bank_a000 = 7;
                        self.prog_bank_c000 = 6;
                        self.prog_bank_e000 = 7;
                    },
                    _ => {}
                }
            }
        }

        fn ppu_read(&self, location: u16) -> u8 {

            let mut local_location = location;
            
            if (0x2000..0x3000).contains(&location) {
                local_location -= 0x2000;
            }

            match local_location {
                0x0000..0x0400 =>
                {
                    match self.char_bank_0000 {
                        0 => self.cart_data.ppu_char_rom_1[local_location as usize],
                        1 => self.cart_data.ppu_char_rom_2[local_location as usize],
                        2 => self.cart_data.ppu_char_rom_3[local_location as usize],
                        3 => self.cart_data.ppu_char_rom_4[local_location as usize],
                        4 => self.cart_data.ppu_char_rom_5[local_location as usize],
                        5 => self.cart_data.ppu_char_rom_6[local_location as usize],
                        6 => self.cart_data.ppu_char_rom_7[local_location as usize],
                        7 => self.cart_data.ppu_char_rom_8[local_location as usize],
                        _ => panic!("Unknwn bank number for char $0000 {}", self.char_bank_0000)
                    }
                },
                0x0400..0x0800 =>
                {
                    match self.char_bank_0400 {
                        0 => self.cart_data.ppu_char_rom_1[local_location as usize],
                        1 => self.cart_data.ppu_char_rom_2[local_location as usize],
                        2 => self.cart_data.ppu_char_rom_3[local_location as usize],
                        3 => self.cart_data.ppu_char_rom_4[local_location as usize],
                        4 => self.cart_data.ppu_char_rom_5[local_location as usize],
                        5 => self.cart_data.ppu_char_rom_6[local_location as usize],
                        6 => self.cart_data.ppu_char_rom_7[local_location as usize],
                        7 => self.cart_data.ppu_char_rom_8[local_location as usize],
                        _ => panic!("Unknwn bank number for char $0400 {}", self.char_bank_0400)
                    }
                },
                0x0800..0x0C00 =>
                {
                    match self.char_bank_0c00 {
                        0 => self.cart_data.ppu_char_rom_1[local_location as usize],
                        1 => self.cart_data.ppu_char_rom_2[local_location as usize],
                        2 => self.cart_data.ppu_char_rom_3[local_location as usize],
                        3 => self.cart_data.ppu_char_rom_4[local_location as usize],
                        4 => self.cart_data.ppu_char_rom_5[local_location as usize],
                        5 => self.cart_data.ppu_char_rom_6[local_location as usize],
                        6 => self.cart_data.ppu_char_rom_7[local_location as usize],
                        7 => self.cart_data.ppu_char_rom_8[local_location as usize],
                        _ => panic!("Unknwn bank number for char $0c00 {}", self.char_bank_0c00)
                    }
                },
                0x0C00..0x1000 =>
                {
                    match self.char_bank_0c00 {
                        0 => self.cart_data.ppu_char_rom_1[local_location as usize],
                        1 => self.cart_data.ppu_char_rom_2[local_location as usize],
                        2 => self.cart_data.ppu_char_rom_3[local_location as usize],
                        3 => self.cart_data.ppu_char_rom_4[local_location as usize],
                        4 => self.cart_data.ppu_char_rom_5[local_location as usize],
                        5 => self.cart_data.ppu_char_rom_6[local_location as usize],
                        6 => self.cart_data.ppu_char_rom_7[local_location as usize],
                        7 => self.cart_data.ppu_char_rom_8[local_location as usize],
                        _ => panic!("Unknwn bank number for char $0c00 {}", self.char_bank_0c00)
                    }
                },
                0x1000..0x1400 =>
                {
                    match self.char_bank_1000 {
                        0 => self.cart_data.ppu_char_rom_1[local_location as usize],
                        1 => self.cart_data.ppu_char_rom_2[local_location as usize],
                        2 => self.cart_data.ppu_char_rom_3[local_location as usize],
                        3 => self.cart_data.ppu_char_rom_4[local_location as usize],
                        4 => self.cart_data.ppu_char_rom_5[local_location as usize],
                        5 => self.cart_data.ppu_char_rom_6[local_location as usize],
                        6 => self.cart_data.ppu_char_rom_7[local_location as usize],
                        7 => self.cart_data.ppu_char_rom_8[local_location as usize],
                        _ => panic!("Unknwn bank number for char $1000 {}", self.char_bank_1000)
                    }
                },
                0x1400..0x1800 =>
                {
                    match self.char_bank_1400 {
                        0 => self.cart_data.ppu_char_rom_1[local_location as usize],
                        1 => self.cart_data.ppu_char_rom_2[local_location as usize],
                        2 => self.cart_data.ppu_char_rom_3[local_location as usize],
                        3 => self.cart_data.ppu_char_rom_4[local_location as usize],
                        4 => self.cart_data.ppu_char_rom_5[local_location as usize],
                        5 => self.cart_data.ppu_char_rom_6[local_location as usize],
                        6 => self.cart_data.ppu_char_rom_7[local_location as usize],
                        7 => self.cart_data.ppu_char_rom_8[local_location as usize],
                        _ => panic!("Unknwn bank number for char $1400 {}", self.char_bank_1400)
                    }
                },
                0x1800..0x1C00 =>
                {
                    match self.char_bank_1800 {
                        0 => self.cart_data.ppu_char_rom_1[local_location as usize],
                        1 => self.cart_data.ppu_char_rom_2[local_location as usize],
                        2 => self.cart_data.ppu_char_rom_3[local_location as usize],
                        3 => self.cart_data.ppu_char_rom_4[local_location as usize],
                        4 => self.cart_data.ppu_char_rom_5[local_location as usize],
                        5 => self.cart_data.ppu_char_rom_6[local_location as usize],
                        6 => self.cart_data.ppu_char_rom_7[local_location as usize],
                        7 => self.cart_data.ppu_char_rom_8[local_location as usize],
                        _ => panic!("Unknwn bank number for char $1800 {}", self.char_bank_1800)
                    }
                },
                0x1C00..0x2000 =>
                {
                    match self.char_bank_1c00 {
                        0 => self.cart_data.ppu_char_rom_1[local_location as usize],
                        1 => self.cart_data.ppu_char_rom_2[local_location as usize],
                        2 => self.cart_data.ppu_char_rom_3[local_location as usize],
                        3 => self.cart_data.ppu_char_rom_4[local_location as usize],
                        4 => self.cart_data.ppu_char_rom_5[local_location as usize],
                        5 => self.cart_data.ppu_char_rom_6[local_location as usize],
                        6 => self.cart_data.ppu_char_rom_7[local_location as usize],
                        7 => self.cart_data.ppu_char_rom_8[local_location as usize],
                        _ => panic!("Unknwn bank number for char $1c00 {}", self.char_bank_1c00)
                    }
                },
                _ => {
                    0
                }
            }
        }
    
        fn ppu_write(&self, location: u16, _byte: u8) {
            eprintln!("This cartridge does not support ppu write {}", location);
        }

    }
}
