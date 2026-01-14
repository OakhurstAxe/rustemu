
pub mod emu_memory {

    pub struct MemoryRam {
        memory: Vec<u8>,
        name: String
    }

    impl MemoryRam {
        pub fn new(_name: String, size: u16) -> MemoryRam{
            MemoryRam {
                memory: vec![0; size as usize],
                name: _name.clone()
            }
        }
    }

    impl crate::base_memory::emu_memory::BaseMemory for MemoryRam{
        fn get_name(&self) -> String {
            self.name.clone()
        }

        fn read(&self, _location: u16) -> u8 {
            self.memory[_location as usize]
        }

        fn write(&mut self, _location: u16, _byte: u8) {
            self.memory[_location as usize] = _byte;
        }

        fn load_data(&mut self, data: &[u8]) {
            self.memory = data.to_vec();
        }
        
        fn check_location(&self, location: u16) {
            if location as usize > self.memory.len() {
                panic!("beyond memory");
            }            
        }
    }
    
}