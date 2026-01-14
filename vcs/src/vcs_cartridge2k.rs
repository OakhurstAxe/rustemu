

pub mod vcs {
    use emumemory::base_memory::emu_memory::BaseMemory;

    use crate::vcs_cartridge::vcs::VcsCartridge;


    pub struct VcsCartridge2k {
        pub memory: Vec<u8>,
        pub name: String,
        pub has_super_chip: bool
    }

    impl VcsCartridge2k {
    }

    impl emumemory::base_memory::emu_memory::BaseMemory for VcsCartridge2k {

        fn read(&self, mut location: u16) -> u8 {
            if location > 0x800 {
                location -= 0x800;
            }
            <Self as VcsCartridge>::read_offset(self, location, 0)
        }

        fn load_data(&mut self, data: &[u8]) {
            self.memory = data.to_vec();
        }

        fn write(&mut self, location: u16, byte: u8) {
            if self.has_super_chip && location < 0x80 {
                self.memory[location as usize + 0x80] = byte;
            }
            else {
                panic!("Cannot write to ROM memory ");
            }
        }

        fn check_location(&self, location: u16) {
            if location as usize > self.memory.len() {
                panic!("Bad cart read location address");
            }
        }

        fn get_name(&self) -> String {
            String::from("ROM cartridge")
        }

    }

    impl emumemory::memory_rom::emu_memory::MemoryRom for VcsCartridge2k { }

    impl crate::vcs_cartridge::vcs::VcsCartridge for VcsCartridge2k {

        fn read_a13(&self, location: u16, _a13set: bool) -> u8 {            
            <Self as BaseMemory>::read(self, location)
        }

        fn read_offset(&self, location: u16, mut memory_offset: u16) -> u8 {
            if location < 0x100 && self.has_super_chip {
                memory_offset = 0;
            }
            self.memory[location as usize + memory_offset as usize]
        }

    }

}