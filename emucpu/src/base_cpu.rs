
pub mod emu_cpu
{
    use emumemory::memory_mapper::emu_memory::MemoryMapper;

    pub trait BaseCpu {

        fn execute_tick(&mut self, memory: &mut dyn MemoryMapper);

        fn reset(&mut self, memory: &dyn MemoryMapper);
    }
}