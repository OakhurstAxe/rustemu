

pub mod vcs {
    use emumemory::base_memory::emu_memory::BaseMemory;
    use emumemory::memory_rom::emu_memory::MemoryRom;

    use crate::vcs_cartridge::vcs::VcsCartridge;


    pub struct VcsCartridgeF8 {
        pub memory: Vec<u8>,
        pub name: String,
        pub has_super_chip: bool,
        pub memory_offset: u16,
    }

    impl VcsCartridgeF8 {
        
        fn set_memory_offset(&mut self, location: u16) -> bool {
            if location == 0xFF8 {
                self.memory_offset = 0x0000;
                return true;
            }
            else if location == 0xFF9 {
                self.memory_offset = 0x1000;
                return true;
            }
        
            false
        }
    }

    impl BaseMemory for VcsCartridgeF8 {

        fn read(&mut self, location: u16) -> u8 {
            self.set_memory_offset(location);
            <Self as VcsCartridge>::read_offset(self, location, self.memory_offset)
        }

        fn load_data(&mut self, data: &[u8]) {
            self.memory = data.to_vec();
        }

        fn write(&mut self, location: u16, byte: u8) {
            if self.set_memory_offset(location) {
                return;
            }
            self.memory[location as usize] = byte;
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

    impl MemoryRom for VcsCartridgeF8 { }

    impl VcsCartridge for VcsCartridgeF8 {

        fn read_a13(&mut self, location: u16, _a13set: bool) -> u8 {            
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