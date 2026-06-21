

pub mod vcs {

    use emucpu::prelude::*;

    use crate::vcs_cartridge::vcs::{VcsCartridge, VcsCartridgeMapper};

    pub struct VcsCartridge4k {
    }

    impl VcsCartridgeMapper for VcsCartridge4k {

        fn execute_tick(&mut self, cart: &VcsCartridge, addr: &mut AddressBus) {

            let mut location = addr.address & 0x1FFF;

            if (0x1000..0x2000).contains(&location) == false {
                return;
            }

            if addr.write {
                eprintln!("Cannot write to VCS 2K cartridges");
            }

            // Read byte
            location -= 0x1000;
            addr.byte = cart.memory[location as usize];
        }
    }    
}