pub mod nes {

    use emumemory::base_memory::emu_memory::BaseMemory;
    
    pub trait NesCartridge {
        
        fn cpu_read(&self, location: u16) -> u8;

        fn cpu_write(&self, location: u16, byte: u8);

        fn ppu_read(&self, location: u16) -> u8;

        fn ppu_write(&self, location: u16, byte: u8);

        fn load_prog_rom(&mut self, data: Vec<u8>);

        fn load_char_rom(&mut self, data: Vec<u8>);

    }

}