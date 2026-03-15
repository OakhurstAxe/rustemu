
pub mod nes {
        
    pub const DataSampleRateHz: usize        = 44100;
    pub const SamplesPerFrame: usize         = 736;
    pub const SamplesPerHalfFrame: usize     = 366;
    pub const SamplesPerQuarterFrame: usize  = 183;

    pub trait NesApuChannel {

        fn set_channel_settings(&mut self,
                register1: u8, register1flag: bool,
                register2: u8, register2flag: bool,
                register3: u8, register3flag: bool,
                register4: u8, register4flag: bool);

        fn generate_buffer_data(&mut self, sample_count: u32) -> Vec<u16>;

        fn frequency_from_timer(&self, timer: u16) -> u32;


    }
}
