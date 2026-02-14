
pub mod vcs {

    use emumemory::base_memory::emu_memory::BaseMemory;
    
    pub trait VcsCartridge: BaseMemory {
        
        fn read_a13(&self, location: u16, a13set: bool) -> u8;

        fn read_offset(&self, location: u16, memory_offset: u16) -> u8;
    }

}