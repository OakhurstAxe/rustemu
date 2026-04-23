
pub mod nes {
        
    pub const DATA_SAMPLE_RATE_HZ: usize       = 44100;
    pub const SAMPLES_PER_FRAME: usize         = 750;
    pub const SAMPLES_PER_HALF_FRAME: usize    = 375;
    pub const SAMPLES_PER_QUARTER_FRAME: usize = 187;

    //pub const VOLUME_STEPS: [u16; 16] = [0, 2194, 4390, 6584, 8778, 10972, 13166, 15360, 
    //                                   17554, 19748, 21942, 24136, 26330, 28524, 30718, 32767];
    pub const VOLUME_STEPS: [u8; 16] = [0, 8, 16, 24, 32, 40, 48, 56, 
                                       64, 72, 80, 88, 96, 104, 112, 120];
    pub trait NesApuChannel {

        fn set_channel_settings(&mut self,
                register1: u8, register1flag: bool,
                register2: u8, register2flag: bool,
                register3: u8, register3flag: bool,
                register4: u8, register4flag: bool);

        fn generate_buffer_data(&mut self, sample_count: u32) -> Vec<u8>;

        fn frequency_from_timer(&self, timer: u16) -> u32;
    }
}
