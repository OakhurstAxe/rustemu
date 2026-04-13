
pub mod nes {
        
    pub const DATA_SAMPLE_RATE_HZ: usize       = 44100;
    pub const SAMPLES_PER_FRAME: usize         = 736;
    pub const SAMPLES_PER_HALF_FRAME: usize    = 366;
    pub const SAMPLES_PER_QUARTER_FRAME: usize = 183;

    //pub const VOLUME_STEPS: [u16; 16] = [0, 2194, 4390, 6584, 8778, 10972, 13166, 15360, 
    //                                   17554, 19748, 21942, 24136, 26330, 28524, 30718, 32767];
    pub const VOLUME_STEPS: [f32; 16] = [0.0, 0.067, 0.134, 0.201, 0.268, 0.335, 0.402, 0.469, 
                                       0.535, 0.602, 0.669, 0.736, 0.803, 0.870, 0.937, 1.0];
    pub trait NesApuChannel {

        fn set_channel_settings(&mut self,
                register1: u8, register1flag: bool,
                register2: u8, register2flag: bool,
                register3: u8, register3flag: bool,
                register4: u8, register4flag: bool);

        fn generate_buffer_data(&mut self, sample_count: u32) -> Vec<f32>;

        fn frequency_from_timer(&self, timer: u16) -> u32;
    }
}
