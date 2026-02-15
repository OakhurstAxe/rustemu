

pub mod vcs {
    use emumemory::base_memory::emu_memory::BaseMemory;
    use emumemory::memory_rom::emu_memory::MemoryRom;

    use crate::vcs_cartridge::vcs::VcsCartridge;


    pub struct VcsCartridge4k {
        pub memory: Vec<u8>,
        pub name: String,
        pub has_super_chip: bool
    }

    impl VcsCartridge4k {
    }

    impl BaseMemory for VcsCartridge4k {

        fn read(&self, location: u16) -> u8 {
            <Self as VcsCartridge>::read_offset(self, location, 0)
        }

        fn load_data(&mut self, data: &[u8]) {
            self.memory = data.to_vec();
        }

        fn write(&mut self, _location: u16, _byte: u8) {
            panic!("Cannot write to ROM memory ");
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

    impl MemoryRom for VcsCartridge4k { }

    impl VcsCartridge for VcsCartridge4k {

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