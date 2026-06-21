
pub mod vcs {

    use emumemory::prelude::*;
    use emucpu::prelude::*;

    pub struct VcsCartridge {
        pub memory: Vec<u8>,
        pub name: String,
        pub has_super_chip: bool,
        pub memory_offset: u16,
    }

    impl VcsCartridge {

        pub fn new(rom: &Vec<u8>) -> VcsCartridge {
            Self {
                memory: rom.clone(),
                name: String::from("VCS Cartridge"),
                has_super_chip: false,
                memory_offset: 0,
            }
        }
    }

    pub trait VcsCartridgeMapper: Send + Sync {

        fn execute_tick(&mut self, cart: &VcsCartridge, addr: &mut AddressBus);
    }

    /*

        fn read_a13(&mut self, location: u16, a13set: bool) -> u8;

        fn read_offset(&self, location: u16, memory_offset: u16) -> u8;

        fn execute_tick(&mut self, addr: &mut AddressBus) {

            let mut location = addr.address & 0x1FFF;
            let address_range = 0x1000..0x2000;

            if addr.write {
                if address_range.contains(&location) {
                    eprintln!("Cannot write to VCS standard cartridges");
                }
            }
            else {
                if address_range.contains(&location) {
                    location -= 0x1000;
                    let a_13_set = (0x2000 & location) > 0;
                    addr.byte = self.read_a13(location, a_13_set);
                }
            }
        }
    }
 */

}