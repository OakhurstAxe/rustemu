
pub mod emu_memory
{
    pub trait BaseMemory {
        fn load_data(&mut self, data: &[u8]);

        fn read(&self, location: u16) -> u8;

        fn write(&mut self, location: u16, byte: u8);

        fn get_name(&self) -> String;// {
        //   String::from("Unnamed Memory")
        //};

        fn check_location(&self, location: u16);
    }

    // Macro to implement the trait for multiple types
//    #[macro_export]
//    macro_rules! impl_BaseMemory {
//        ($structname: ident, BaseMemory $($memory:ident),*) => {
//            impl crate::base_memory::emu_memory::BaseMemory for $structname {
//                fn load_data (&mut self, data: &[u8]) {
//                    self.memory = data.to_vec();
//                }

//               fn read (&self, location: u16) -> u8 {
//                    self.check_location(location);
//                    self.memory[location as usize]
//                }

//                fn write (&mut self, location: u16, byte: u8) {
//                    self.check_location(location);
//                    self.memory[location as usize] = byte;
//                }

//                fn check_location(&self, location: u16) {
//                    if location as usize > self.memory.len() {
//                        panic!("beyond memory");
//                    }
//                }
//            }
//        };
//    }

}
