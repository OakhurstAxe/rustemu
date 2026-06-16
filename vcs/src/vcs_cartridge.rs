
pub mod vcs {

    use emumemory::prelude::*;
    use emucpu::prelude::*;

    pub trait VcsCartridge: BaseMemory {
        
        fn read_a13(&mut self, location: u16, a13set: bool) -> u8;

        fn read_offset(&self, location: u16, memory_offset: u16) -> u8;

        fn execute_tick(&mut self, addr: &mut AddressBus) {

            let mut location = addr.address & 0x1FFF;

            if addr.write {
                if location >= 0x1000 && location < 0x2000 {
                    eprintln!("Cannot write to VCS standard cartridges");
                }
            }
            else {
                if location >= 0x1000 && location < 0x2000 {
                    location -= 0x1000;
                    let a_13_set = (0x2000 & location) > 0;
                    addr.byte = self.read_a13(location, a_13_set);
                }
            }
        }

    }

}