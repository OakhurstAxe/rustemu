
pub mod emu_cpu
{
    pub trait BaseCpu {

        fn execute_tick(&mut self);

        fn reset(&mut self);
    }
}