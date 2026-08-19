

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

        fn is_irq_set(&self) -> bool {
            false
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

        fn cpu_read(&self, mut location: u16) -> u8 {

            match location {
                (0x6000..0x8000) => {
                    let local_location = location - 0x6000;
                    self.cart_data.cpu_prog_ram_0[local_location as usize]
                },
                (0x8000..=0xFFFF) => {
                    let local_location = location - 0x8000;
                    let index = local_location / 0x2000;
                    self.cart_data.cpu_prog_rom[index as usize][(local_location - (0x2000 * index)) as usize]
                }
                _ => {0}
            }
        }
    
        fn cpu_write(&mut self, location: u16, _byte: u8) {
            eprintln!("This cartridge does not support cpu write {}", location);
        }

        fn ppu_read(&mut self, mut location: u16) -> u8 {

            let index = location / 0x400;
            self.cart_data.ppu_char_rom[index as usize][(location - (0x400 * index)) as usize]
        }
    
        fn ppu_write(&self, location: u16, _byte: u8) {
            eprintln!("This cartridge does not support ppu write {}", location);
        }

    }
}
