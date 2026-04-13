
pub mod nes {
    use crate::nes_apuchannel::nes::NesApuChannel;
    use crate::nes_apuchannel::nes::{ SAMPLES_PER_FRAME, DATA_SAMPLE_RATE_HZ, SAMPLES_PER_HALF_FRAME, VOLUME_STEPS };
    
    pub struct NesApuPulseChannel {
        duty: u8,
        halt: bool,
        constant_volume: bool,
        volume: f32,
        sweep_enabled: bool,
        sweep_period: u8,
        sweep_negate: bool,
        sweep_shift: u8,
        timer_low: u16,
        timer_high: u16,
        load_counter: u8,
        timer: u16,
        frequency: u16,
        total_samples: u64,
    }

    impl NesApuPulseChannel {

        pub fn new() -> NesApuPulseChannel {
            Self {
                duty: 0,
                halt: true,
                constant_volume: true,
                volume: 0.0,
                sweep_enabled: false,
                sweep_period: 0,
                sweep_negate: false,
                sweep_shift: 0,
                timer_low: 0,
                timer_high: 0,
                load_counter: 0,
                timer: 0,
                frequency: 0,
                total_samples: 0,
            }
        }
    }
    impl NesApuChannel for NesApuPulseChannel {

        fn set_channel_settings(&mut self,
                register1: u8, register1_flag: bool,
                register2: u8, register2_flag: bool,
                register3: u8, register3_flag: bool,
                register4: u8, register4_flag: bool) {
            
            if register1_flag {
                self.duty = (register1 & 0xC0) >> 6;
                self.halt = (register1 & 0x20) != 0;
                self.constant_volume = (register1 & 0x10) != 0;
                self.volume = VOLUME_STEPS[(register1 & 0x0F) as usize];
            }

            if register2_flag {
                self.sweep_enabled = register2 & 0x80 != 0;
                self.sweep_period = (register2 & 0x70) >> 4;
                self.sweep_negate = register2 & 0x80 != 0;
                self.sweep_shift = register2 & 0x70;
            }

            if register3_flag {
                self.timer_low = register3 as u16;
            }

            if register4_flag {
                self.load_counter = (register4 & 0xF8) >> 3;
                self.timer_high = (register4 & 0x07) as u16;
            }

            self.timer = (self.timer_high << 8) + (self.timer_low);
            self.frequency = self.frequency_from_timer(self.timer) as u16;
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

            if self.frequency == 0 {
                return buffer;
            }
            
            let wavelength: u32 = (DATA_SAMPLE_RATE_HZ / self.frequency as usize) as u32;
            let wavelength_eigth: u32 = wavelength / 8;

            while sample_index < sample_count {
                if self.load_counter == 0 && self.halt == false {
                    buffer[sample_index as usize] = 0.0; // zero
                }
                else if (self.total_samples % wavelength as u64) < (wavelength_eigth * (self.duty + 1) as u32) as u64 {
                    buffer[sample_index as usize] = self.volume;
                }
                else 
                {
                    buffer[sample_index as usize] = - self.volume;
                }
                
                if self.total_samples % SAMPLES_PER_HALF_FRAME as u64 == 0 {// 120 Hz timer
                    /*
                    if (constantVolume_)
                    {
                        if (volumeCounter_ > 0)
                        {
                            volumeCounter_--;
                        }
                        else
                        {
                            volume_ = volumeSteps_[volumeCounter_];
                        }
                    }*/

                    if self.load_counter > 0 && self.halt == false {
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


