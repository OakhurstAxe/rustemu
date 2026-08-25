

pub mod nes {

    use std::sync::Arc;

use emucpu::prelude::*;
    
    use crate::nes_cartridge::nes::NesCartridge;
    use crate::nes_cartridge::nes::NesCartridgeTrait;

    pub struct NesCartridge001 {
        cart_data: NesCartridge,
        
        load_register: u8,
        shift_register: u8,
        control_register: u8,
        char_bank_0: u8,
        char_bank_1: u8,
        prog_bank: u8,

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

    impl NesCartridge001 {

        pub fn new(cart_data: NesCartridge) -> NesCartridge001 {

            let prog_rom_count: u8 = cart_data.cpu_prog_rom_count as u8;
            let char_rom_count: u8 = cart_data.ppu_char_rom_count as u8;

            Self { 
                cart_data,
                load_register: 0x20,
                shift_register: 0,
                control_register: 0xC0,
                char_bank_0: 0,
                char_bank_1: 0,
                prog_bank: 0,

                prog_bank_8000: 0,
                prog_bank_a000: 0,
                prog_bank_c000: prog_rom_count - 2,
                prog_bank_e000: prog_rom_count - 1,

                char_bank_0000: char_rom_count - 8,
                char_bank_0400: char_rom_count - 7,
                char_bank_0800: char_rom_count - 6,
                char_bank_0c00: char_rom_count - 5,
                char_bank_1000: char_rom_count - 4,
                char_bank_1400: char_rom_count - 3,
                char_bank_1800: char_rom_count - 2,
                char_bank_1c00: char_rom_count - 1,
              }
        }
    }

    impl NesCartridgeTrait for NesCartridge001 {    

        fn is_irq_set(&self) -> bool {
            self.cart_data.irq_set
        }

        fn reset_irq(&mut self) {
            self.cart_data.irq_set = false;
        }

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
                    self.cart_data.cpu_prog_rom[self.prog_bank_8000 as usize][local_location as usize]
                },
                0xA000..=0xBFFF => {
                    local_location -= 0xA000;
                    self.cart_data.cpu_prog_rom[self.prog_bank_a000 as usize][local_location as usize]
                },
                0xC000..=0xDFFF => {
                    local_location -= 0xC000;
                    self.cart_data.cpu_prog_rom[self.prog_bank_c000 as usize][local_location as usize]
                }
                0xE000..=0xFFFF => {
                    local_location -= 0xe000;
                    self.cart_data.cpu_prog_rom[self.prog_bank_e000 as usize][local_location as usize]
                } 
                _ => {
                    0
                }

            }
        }
    
        fn cpu_write(&mut self, location: u16, byte: u8) {
            
            if byte & 0x80 > 0 {
                self.load_register = 0x20;
                return;
            } else {
                self.load_register >>= 1;
                self.load_register |= (byte & 0x01) << 4;
            }
            
            if self.load_register & 0x01 == 1 &&
                location >= 0x8000 {
                match location {
                    (0x8000..0xA000) => {
                        self.control_register = (self.load_register & 0x1E) >> 1;
println!("Control reg {:x}", self.control_register);
                    }
                    (0xA000..0xC000) => {
                        self.char_bank_0 = (self.load_register & 0x1E) >> 1;
                        self.set_char_bank_0();
println!("char bank 0 {:x}", self.char_bank_0);
                    }
                    (0xC000..0xE000) => {
                        self.char_bank_1 = (self.load_register & 0x1E) >> 1;
                        self.set_char_bank_1();
println!("char bank 1 {:x}", self.char_bank_1);
                    }
                    (0xE000..=0xFFFF) => {
                        self.prog_bank = (self.load_register & 0x1E) >> 1;
                        self.set_prog_bank();
println!("Prog bank {:x}", self.prog_bank);
                    }
                    _ => {}
                }
                self.load_register = 0x20;
            }

        }

       
        fn ppu_read(&mut self, location: u16) -> u8 {

            let mut local_location = location;
            
            if (0x2000..0x3000).contains(&location) {
                local_location -= 0x2000;
            }

            match local_location {
                0x0000..0x0400 =>
                {
                    self.cart_data.ppu_char_rom[self.char_bank_0000 as usize][local_location as usize]
                },
                0x0400..0x0800 =>
                {
                    local_location -= 0x0400;
                    self.cart_data.ppu_char_rom[self.char_bank_0400 as usize][local_location as usize]
                },
                0x0800..0x0c00 =>
                {
                    local_location -= 0x0800;
                    self.cart_data.ppu_char_rom[self.char_bank_0800 as usize][local_location as usize]
                },
                0x0c00..0x1000 =>
                {
                    local_location -= 0x0c00;
                    self.cart_data.ppu_char_rom[self.char_bank_0c00 as usize][local_location as usize]
                },
                0x1000..0x1400 =>
                {
                    local_location -= 0x1000;
                    self.cart_data.ppu_char_rom[self.char_bank_1000 as usize][local_location as usize]
                },
                0x1400..0x1800 =>
                {
                    local_location -= 0x1400;
                    self.cart_data.ppu_char_rom[self.char_bank_1400 as usize][local_location as usize]
                },
                0x1800..0x1C00 =>
                {
                    local_location -= 0x1800;
                    self.cart_data.ppu_char_rom[self.char_bank_1800 as usize][local_location as usize]
                },
                0x1C00..0x2000 =>
                {
                    local_location -= 0x1C00;
                    self.cart_data.ppu_char_rom[self.char_bank_1c00 as usize][local_location as usize]
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

    impl NesCartridge001 {

        fn set_char_bank_0(&mut self) {
            match self.control_register & 0x10 >> 4 {
                0 => {
                    self.char_bank_0000 = self.char_bank_0 * 4;
                    self.char_bank_0400 = (self.char_bank_0 * 4) + 1;
                    self.char_bank_0800 = (self.char_bank_0 * 4) + 2;
                    self.char_bank_0c00 = (self.char_bank_0 * 4) + 3;
                    self.char_bank_1000 = (self.char_bank_0 * 4) + 4;
                    self.char_bank_1400 = (self.char_bank_0 * 4) + 5;
                    self.char_bank_1800 = (self.char_bank_0 * 4) + 6;
                    self.char_bank_1c00 = (self.char_bank_0 * 4) + 7;
                },
                1 => {
                    self.char_bank_0000 = self.char_bank_0 * 4;
                    self.char_bank_0400 = (self.char_bank_0 * 4) + 1;
                    self.char_bank_0800 = (self.char_bank_0 * 4) + 2;
                    self.char_bank_0c00 = (self.char_bank_0 * 4) + 3;
                },
                _ => {}
            }
        }

        fn set_char_bank_1(&mut self) {
            self.char_bank_1000 = self.char_bank_1 * 4;
            self.char_bank_1400 = (self.char_bank_1 * 4) + 1;
            self.char_bank_1800 = (self.char_bank_1 * 4) + 2;
            self.char_bank_1c00 = (self.char_bank_1 * 4) + 3;
        }

        fn set_prog_bank(&mut self) {

            match self.control_register & 0x0C {
                (0..=1) => {
                    self.prog_bank_8000 = self.prog_bank;
                    self.prog_bank_a000 = self.prog_bank + 1;
                    self.prog_bank_c000 = self.prog_bank + 2;
                    self.prog_bank_e000 = self.prog_bank + 3;
                },
                2 => {
                    self.prog_bank_8000 = 0;
                    self.prog_bank_a000 = 1;
                    self.prog_bank_c000 = self.prog_bank;
                    self.prog_bank_e000 = self.prog_bank + 1;
                },
                3 => {
                    self.prog_bank_8000 = self.prog_bank;
                    self.prog_bank_a000 = self.prog_bank + 1;
                    self.prog_bank_c000 = (self.cart_data.cpu_prog_rom_count - 2) as u8;
                    self.prog_bank_e000 = (self.cart_data.cpu_prog_rom_count - 1) as u8;
                },
                _ => {}
            }
        }

    }
}
