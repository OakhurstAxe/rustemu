
pub mod emu_memory
{
    pub trait MemoryMapper {
        fn cpu_read(&self, location: u16) -> u8;

        fn cpu_write(&mut self, location: u16, byte: u8);
    }
}