
pub mod emu_memory
{
    pub trait MemoryMapper {
            
        fn get_dma_write(&self) -> u16;

        fn execute_tick(&mut self);

        fn cpu_read(&mut self, location: u16) -> u8;

        fn cpu_write(&mut self, location: u16, byte: u8) -> bool;

    }
}