
pub mod nes {
    use crate::nes_apuchannel::nes::NesApuChannel;
    use crate::nes_apuchannel::nes::{ SAMPLES_PER_FRAME, DATA_SAMPLE_RATE_HZ, SAMPLES_PER_HALF_FRAME };
        
    pub const NOISE_TIMER: [u16; 16] = [4, 8, 16, 32, 64, 96, 128, 160, 
                                       202, 254, 380, 508, 762, 1016, 2034, 4068];

    pub struct NesApuNoiseChannel {
        halt_flag: bool,
        const_volume: bool,
        volume: u16,
        noise_mode: bool,
        noise_period: u16,
        noise_timer: u16,
        length_counter: u16,
        frequency: u16,
        load_counter: u16,
        total_samples: u64,
    }

    impl NesApuNoiseChannel {

        pub fn new() -> NesApuNoiseChannel {
            Self {
                halt_flag: true,
                const_volume: false,
                volume: 0,
                noise_mode: false,
                noise_period: 0,
                noise_timer: 0,
                length_counter: 0,
                frequency: 0,
                load_counter: 0,
                total_samples: 0,
            }
        }
    }

    impl NesApuChannel for NesApuNoiseChannel {

        fn set_channel_settings(&mut self,
                register1: u8, register1_flag: bool,
                _register2: u8, _register2_flag: bool,
                register3: u8, register3_flag: bool,
                register4: u8, register4_flag: bool) {
            
            if register1_flag {
                self.halt_flag = (register1 & 0x20) != 0;
                self.const_volume = (register1 & 0x10) != 0;
                self.volume = (register1 & 0x0F) as u16;
            }

            if register3_flag {
                self.noise_mode = (register3 & 0x80) != 0;
                self.noise_period = (register3 & 0x0F) as u16;
                self.noise_timer = NOISE_TIMER[self.noise_period as usize];
            }

            if register4_flag {
                self.length_counter = ((register4 & 0xF8) >> 3) as u16;
            }
        }

        fn frequency_from_timer(& self, timer: u16) -> u32 {
            
            if timer < 8 {
                return 0;
            }

            return (111860 / (timer + 1) as u32) as u32;
        }

        fn generate_buffer_data(&mut self, sample_count: u32) -> Vec<f32> {
            let mut sample_index: u32 = 0;
            let mut buffer: Vec<f32> = vec![0.0; SAMPLES_PER_FRAME];
            self.frequency = self.frequency_from_timer(self.noise_timer) as u16;

            if self.frequency == 0
            {
                return buffer;
            }

            let mut wavelength: u16 = (DATA_SAMPLE_RATE_HZ as u16 / self.frequency) as u16;

            while sample_index < sample_count {

                if self.length_counter == 0 && self.halt_flag == false {
                    buffer[sample_index as usize] = 0.5;
                }
                else if (self.total_samples % wavelength as u64) < (wavelength >> 1) as u64 {
                    buffer[sample_index as usize] = 1.0;
                }
                else {
                    buffer[sample_index as usize] = 0.5;
                }

                if self.total_samples % SAMPLES_PER_HALF_FRAME as u64 == 0 { // 120 Hz timer
                    
                    let mut feedback: u8 = (self.noise_timer & 0x01) as u8;

                    if self.noise_mode {
                        feedback = (!feedback != !1) as u8;
                    }
                    else {
                        feedback = (!feedback != !(self.noise_timer & 0x20) as u8) as u8;
                    }
                    self.noise_timer = self.noise_timer >> 1;
                    self.noise_timer |= (feedback as u16) << 15;
                    self.frequency = self.frequency_from_timer(self.noise_timer) as u16;
                    wavelength = (DATA_SAMPLE_RATE_HZ / self.frequency as usize) as u16;
                    
                    if self.load_counter > 0 && self.halt_flag == false {
                        self.load_counter -= 1;
                    }
                }

                sample_index += 1;
                self.total_samples += 1;
            }

            buffer
        }

    }

}


