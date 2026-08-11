

pub mod nes {

    use emucpu::prelude::*;
    
    use crate::nes_cartridge::nes::NesCartridge;
    use crate::nes_cartridge::nes::NesCartridgeTrait;

    pub struct NesCartridge000 {
        cart_data: NesCartridge,
    }

    impl NesCartridge000 {

        pub fn new(cart_data: NesCartridge) -> NesCartridge000 {
            Self { 
                cart_data,
            }
        }
    }

    impl NesCartridgeTrait for NesCartridge000 {    

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

        fn cpu_read(&self, mut location: u16) -> u8 {

            if location < 0xc000 {

                if location >= 0x8000 {
                    location -= 0x8000;
                }
                else {
                    location -= 0x4000;
                }
                
                self.cart_data.cpu_prog_rom_0[location as usize]
            } else {
                location -= 0xc000;
                self.cart_data.cpu_prog_rom_1[location as usize]
            }
        }
    
        fn cpu_write(&mut self, _location: u16, _byte: u8) {
            //eprintln!("This cartridge does not support cpu write {}", location);
        }

        fn ppu_read(&self, mut location: u16) -> u8 {

            if location < 0x2000 {
                self.cart_data.ppu_char_rom_0[location as usize]
            } else {
                location -= 0x2000;
                self.cart_data.ppu_char_rom_1[location as usize]
            }
        }
    
        fn ppu_write(&self, location: u16, _byte: u8) {
            eprintln!("This cartridge does not support ppu write {}", location);
        }

    }
}
