
pub mod vcs {

    use std::sync::{ Arc, Mutex };
    use emumemory::base_memory::emu_memory::BaseMemory;

    
    pub trait VcsCartridge: emumemory::memory_rom::emu_memory::MemoryRom {
        
        fn read_a13(&self, location: u16, a13set: bool) -> u8;

        fn read_offset(&self, location: u16, memory_offset: u16) -> u8;

        fn get_cartridge(vcs_parameters: Arc<Mutex<crate::vcs_parameters::vcs::VcsParameters>>) -> crate::vcs_cartridge2k::vcs::VcsCartridge2k {
            
            let mut cart2k: crate::vcs_cartridge2k::vcs::VcsCartridge2k;

            //if vcs_parameters.mapper == "2K" {
                cart2k = crate::vcs_cartridge2k::vcs::VcsCartridge2k {
                    memory: [].to_vec(),
                    name: String::from("2k Cartridge"),
                    has_super_chip: false
                };

                cart2k.load_data(&vcs_parameters.lock().unwrap().cart_data);
            //}

            cart2k
        }
    }

}