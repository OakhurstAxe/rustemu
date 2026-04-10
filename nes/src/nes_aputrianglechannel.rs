
pub mod nes {
    use crate::nes_apuchannel::nes::NesApuChannel;
    use crate::nes_apuchannel::nes::{ SAMPLES_PER_FRAME, DATA_SAMPLE_RATE_HZ, SAMPLES_PER_QUARTER_FRAME };
        
    pub struct NesApuTriangleChannel {
        timer: u16,
        frequency: u16,
        counter: u16,
        reverse: bool,
        load_counter: u16,
        control_register: u8,
        timer_register: u8,
        length_counter_register: u8,
        total_samples: u64,
        halt_flag: bool,
    }

    impl NesApuTriangleChannel {

        pub fn new() -> NesApuTriangleChannel {
            Self {
                timer: 0,
                frequency: 0,
                counter: 32000,
                reverse: false,
                load_counter: 0,
                control_register: 0,
                timer_register: 0,
                length_counter_register: 0,
                total_samples: 0,
                halt_flag: true,
            }
        }
    }
    impl NesApuChannel for NesApuTriangleChannel {

        fn set_channel_settings(&mut self,
                register1: u8, register1_flag: bool,
                _register2: u8, _register2_flag: bool,
                register3: u8, _register3_flag: bool,
                register4: u8, _register4_flag: bool) {
            
            self.control_register = register1;
            self.timer_register = register3;
            self.length_counter_register = register4;
            
            if register1_flag {
                self.halt_flag = true;
                self.load_counter = (self.control_register & 0x7f) as u16;
            }

            if self.control_register & 0x80 == 0 {
                self.halt_flag = false;
            }
            
            if self.halt_flag == false {
                self.timer = ((self.length_counter_register as u16 & 0x07 << 8) + self.timer_register as u16) as u16;
                self.frequency = self.frequency_from_timer(self.timer) as u16;
            }
        }

        fn frequency_from_timer(& self, timer: u16) -> u32 {
            
            if timer < 8 {
                return 0;
            }

            return (111860 / (timer + 1) as u32) as u32;
        }

        fn generate_buffer_data(&mut self, sample_count: u32) -> Vec<u16> {
            let mut sample_index: u32 = 0;
            let mut buffer: Vec<u16> = vec![0; SAMPLES_PER_FRAME];

            if self.frequency == 0 {
                return buffer;
            }
            
            let wavelength: u32 = (DATA_SAMPLE_RATE_HZ / self.frequency as usize) as u32;
            let step: u16 =  65535 / wavelength as u16;

            while sample_index < sample_count {

                if self.counter as u32 + step as u32 >= 65535 && self.reverse {
                    self.counter = 65535;
                    self.reverse = false;
                }
                else if step >= self.counter && self.reverse == false {
                    self.counter = 0;
                    self.reverse = true;
                }

                if self.load_counter == 0 && self.halt_flag == false {
                    buffer[sample_index as usize] = 32000;
                }                
                else {
                    buffer[sample_index as usize] = self.counter;
                    //buffer[sample_index as usize] = (((self.counter / 2.0) * 32000.0) as u16) + 32000;
                }
                
                if self.reverse {
                    self.counter += step;
                }
                else {
                    self.counter -= step;
                }
                    
                self.total_samples += 1;

                if self.total_samples % SAMPLES_PER_QUARTER_FRAME as u64 == 0 { // 240 Hz counter
                    if self.load_counter > 0 && self.halt_flag == false {
                        self.load_counter -= 1;
                    }
                }

                sample_index += 1;
            } 

            buffer
        }

    }

}


