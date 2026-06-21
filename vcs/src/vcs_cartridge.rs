
pub mod vcs {

    use emumemory::prelude::*;
    use emucpu::prelude::*;

    pub trait VcsCartridge: BaseMemory + Send + Sync {
        
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

}