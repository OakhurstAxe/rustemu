

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

                prog_bank_8000: 14,
                prog_bank_a000: 14,
                prog_bank_c000: 15,
                prog_bank_e000: 15,

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
                0x8000..=0x9FFF => {
                    local_location -= 0x8000;
                    self.cart_data.cpu_prog_rom_mmc3[self.prog_bank_8000 as usize][local_location as usize]
                },
                0xA000..=0xBFFF => {
                    local_location -= 0xA000;
                    self.cart_data.cpu_prog_rom_mmc3[self.prog_bank_a000 as usize][local_location as usize]
                },
                0xC000..=0xDFFF => {
                    local_location -= 0xC000;
                    self.cart_data.cpu_prog_rom_mmc3[self.prog_bank_c000 as usize][local_location as usize]
                }
                0xE000..=0xFFFF => {
                    local_location -= 0xe000;
                    self.cart_data.cpu_prog_rom_mmc3[self.prog_bank_e000 as usize][local_location as usize]
                } 
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
                    0 => {
                        match self.char_inversion {
                            0 => {
                                self.char_bank_0000 = byte;
                                self.char_bank_0400 = byte + 1;
                            },
                            1 => {
                                self.char_bank_1000 = byte;
                                self.char_bank_1400 = byte + 1;
                            }
                            _ => {}
                        }
                    },
                    1 => {
                        match self.char_inversion {
                            0 => {
                                self.char_bank_0800 = byte;
                                self.char_bank_0c00 = byte + 1;
                            },
                            1 => {
                                self.char_bank_1800 = byte;
                                self.char_bank_1c00 = byte + 1;
                            }
                            _ => {}
                        }
                    },
                    2 => {
                        match self.char_inversion {
                            0 => {
                                self.char_bank_1000 = byte;
                            },
                            1 => {
                                self.char_bank_0000 = byte;
                            }
                            _ => {}
                        }
                    },
                    3 => {
                        match self.char_inversion {
                            0 => {
                                self.char_bank_1400 = byte;
                            },
                            1 => {
                                self.char_bank_0400 = byte;
                            }
                            _ => {}
                        }
                    },
                    4 => {
                        match self.char_inversion {
                            0 => {
                                self.char_bank_1800 = byte;
                            },
                            1 => {
                                self.char_bank_0800 = byte;
                            }
                            _ => {}
                        }
                    },
                    5 => {
                        match self.char_inversion {
                            0 => {
                                self.char_bank_1c00 = byte;
                            },
                            1 => {
                                self.char_bank_0c00 = byte;
                            }
                            _ => {}
                        }
                    },
                    6 => {
                        match self.prog_bank_mode {
                            0 => {
                                self.prog_bank_c000 = 14;
                                self.prog_bank_e000 = 15;
                                self.prog_bank_8000 = byte;
                            },
                            1 => {
                                self.prog_bank_8000 = 14;
                                self.prog_bank_e000 = 15;
                                self.prog_bank_c000 = byte;
                            },
                            _ => {}
                        }
                    },
                    7 => {
                        match self.prog_bank_mode {
                            0 => {
                                self.prog_bank_c000 = 14;
                                self.prog_bank_e000 = 15;
                                self.prog_bank_a000 = byte;
                            },
                            1 => {
                                self.prog_bank_8000 = 14;
                                self.prog_bank_e000 = 15;
                                self.prog_bank_a000 = byte;
                            },
                            _ => {}
                        }
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
                    self.cart_data.ppu_char_rom_mmc3[self.char_bank_0000 as usize][local_location as usize]
                },
                0x0400..0x0800 =>
                {
                    local_location -= 0x0400;
                    self.cart_data.ppu_char_rom_mmc3[self.char_bank_0400 as usize][local_location as usize]
                },
                0x0800..0x0c00 =>
                {
                    local_location -= 0x0800;
                    self.cart_data.ppu_char_rom_mmc3[self.char_bank_0800 as usize][local_location as usize]
                },
                0x0c00..0x1000 =>
                {
                    local_location -= 0x0c00;
                    self.cart_data.ppu_char_rom_mmc3[self.char_bank_0c00 as usize][local_location as usize]
                },
                0x1000..0x1400 =>
                {
                    local_location -= 0x1000;
                    self.cart_data.ppu_char_rom_mmc3[self.char_bank_1000 as usize][local_location as usize]
                },
                0x1400..0x1800 =>
                {
                    local_location -= 0x1400;
                    self.cart_data.ppu_char_rom_mmc3[self.char_bank_1400 as usize][local_location as usize]
                },
                0x1800..0x1C00 =>
                {
                    local_location -= 0x1800;
                    self.cart_data.ppu_char_rom_mmc3[self.char_bank_1800 as usize][local_location as usize]
                },
                0x1C00..0x2000 =>
                {
                    local_location -= 0x1C00;
                    self.cart_data.ppu_char_rom_mmc3[self.char_bank_1c00 as usize][local_location as usize]
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
