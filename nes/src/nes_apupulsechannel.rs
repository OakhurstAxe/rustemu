
pub mod nes {
    use crate::nes_apuchannel::nes::NesApuChannel;
    use crate::nes_apuchannel::nes::{ SamplesPerFrame, DataSampleRateHz, SamplesPerHalfFrame, VOLUMESTEPS };
    
    pub struct NesApuPulseChannel {
        timer: u16,
        frequency: u16,
        load_counter: u8,
        duty_register: u8,
        timer_register: u8,
        length_counter_register: u8,
        constant_volume: bool,
        sweep_register: u8,
        duty_value: u8,
        total_samples: u64,
        halt_flag: bool,
        volume_counter: u8,
        volume: u16,
    }

    impl NesApuPulseChannel {

        pub fn new() -> NesApuPulseChannel {
            Self {
                timer: 0,
                frequency: 0,
                load_counter: 0,
                duty_register: 0,
                timer_register: 0,
                length_counter_register: 0,
                constant_volume: false,
                sweep_register: 0,
                duty_value: 0,
                total_samples: 0,
                halt_flag: true,
                volume_counter: 0,
                volume: 0,
            }
        }
    }
    impl NesApuChannel for NesApuPulseChannel {

        fn set_channel_settings(&mut self,
                register1: u8, register1_flag: bool,
                register2: u8, register2_flag: bool,
                register3: u8, register3_flag: bool,
                register4: u8, _register4_flag: bool) {
            
            self.duty_register = register1;
            self.timer_register = register3;
            self.length_counter_register = register4;

            self.constant_volume = self.duty_register & 0x10 > 0;
            self.duty_value = (self.duty_register & 0xC0) >> 6;
            self.halt_flag = self.duty_register & 0x20 > 0;

            if self.constant_volume == false {
                self.volume = VOLUMESTEPS[(self.duty_register & 0x0F) as usize];
            }
            
            if register1_flag {
                self.volume_counter = self.duty_register & 0x0F;
            }
            
            if register3_flag {
                self.load_counter = self.length_counter_register & 0xF8;

                if self.load_counter == 1 {
                    self.load_counter = 255;
                }
            }

            if register2_flag {
                self.sweep_register = register2;
            }

            self.timer = ((self.length_counter_register) as u16 & 0x07 << 8) + (self.timer_register) as u16;
            self.frequency = self.frequency_from_timer(self.timer) as u16;
        }

        fn frequency_from_timer(& self, timer: u16) -> u32 {
            
            if timer < 8 {
                return 0;
            }

            return (111860 / (timer + 1) as u32) as u32;
        }

        fn generate_buffer_data(&mut self, sample_count: u32) -> Vec<u16> {
            let mut sample_index: u32 = 0;
            let mut buffer: Vec<u16> = vec![0; SamplesPerFrame];

            if self.frequency == 0 {
                return buffer;
            }
            
            let wavelength: u32 = (DataSampleRateHz / self.frequency as usize) as u32;
            let wavelength_eigth: u32 = wavelength / 8;

            while sample_index < sample_count {
                if self.load_counter == 0 && self.halt_flag == false {
                    buffer[sample_index as usize] = 32767; // zero
                }
                else if (self.total_samples % wavelength as u64) < (wavelength_eigth * (self.duty_value + 1) as u32) as u64 {
                    buffer[sample_index as usize] = self.volume + 32767;
                }
                else 
                {
                    buffer[sample_index as usize] = 32767 - self.volume;
                }
                
                if self.total_samples % SamplesPerHalfFrame as u64 == 0 {// 120 Hz timer
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


