
pub mod emu_memory
{

    pub struct MemoryRamFlagged {
        memory: Vec<u8>,
        name: String,
        is_read_flagged: Vec<bool>,
        is_write_flagged: Vec<bool>,
    }

    impl MemoryRamFlagged {

        pub fn new(size: usize, name: String) -> MemoryRamFlagged {
            Self {
                memory: vec![0; size],
                name: name,
                is_read_flagged: vec![false; size],
                is_write_flagged: vec![false; size],
            }
        }

        fn get_name(&self) -> String {
            self.name.clone()
        }

        pub fn read(&mut self, location: u16) -> u8 {
            self.is_read_flagged[location as usize] = true;
            self.memory[location as usize]
        }

        pub fn write(&mut self, location: u16, byte: u8) {
            self.is_write_flagged[location as usize] = true;
            self.memory[location as usize] = byte;
        }

        pub fn is_read_flag_set(&mut self, mut location: u16) -> bool {

            location %= 0x4000;
            let result: bool = self.is_read_flagged[location as usize];
            self.is_read_flagged[location as usize] = false;
            result
        }

        pub fn is_write_flag_set(&mut self, mut location: u16) -> bool {

            location %= 0x4000;
            let result: bool = self.is_write_flagged[location as usize];
            self.is_write_flagged[location as usize] = false;
            result
        }

    }

}
