
pub mod vcs {
    
    #[derive(PartialEq, Eq)]
    enum ShiftRegister { 
        Four, 
        Five, 
        Nine, 
        FiveToFour, 
        Div2, 
        Div31, 
        Div31Four 
    }

    pub const DATA_SAMPLE_RATE_HZ:usize       = 48000;
    pub const SAMPLES_PER_FRAME:usize         = DATA_SAMPLE_RATE_HZ / 60;
    pub const BUFFER_SIZE:usize               = SAMPLES_PER_FRAME;

    pub struct VcsAudioChannel {
        total_sample: u64,
        volume: u8,
        frequency: u16,
        volume_steps: Vec<u8>,
        m_buffer: Vec<u8>,
        is_shutdown: bool,
        shift_register: ShiftRegister,
        apply_third: bool,
        third_count: u8,
        div2_count: u8,
        div31_count: u8,
        shift_4_register: u16,
        shift_5_register: u16,
        shift_9_register: u16,
    }

    impl VcsAudioChannel {

        pub fn new () -> VcsAudioChannel {

            Self {
                total_sample: 0,
                volume: 0,
                frequency: 0,

                //volume_steps: vec![0.000, 0.067, 0.134, 0.201, 0.268, 0.335, 0.402, 0.469, 0.535, 0.602, 0.669, 0.736, 0.803, 0.870, 0.937, 1.000],
                volume_steps: vec![0, 9, 17, 26, 34, 43, 51, 60, 68, 77, 85, 94, 103, 111, 120, 128],
                m_buffer: vec![0u8; BUFFER_SIZE as usize],
                is_shutdown: false,
                shift_register: ShiftRegister::Four,
                apply_third: false,
                third_count: 0,
                div2_count: 0,
                div31_count: 0,
                shift_4_register: 0xFFFF,
                shift_5_register: 0xFFFF,
                shift_9_register: 0xFFFF,
            }
        }

        pub fn set_channel_settings(&mut self, volume_reg: u8, frequency_reg: u8, noise_reg: u8) {

            self.volume = self.volume_steps[(volume_reg & 0x0F) as usize];
            let frequency_divider: u8 = (frequency_reg & 0x1F) + 1;
            self.frequency = 30000 / frequency_divider as u16;
            self.apply_third = false;

            match noise_reg & 0x0F {
                0 | 11 =>
                    self.volume = 0,
                1 =>
                    if self.shift_register != ShiftRegister::Four {
                        self.shift_register = ShiftRegister::Four;
                    },
                2 =>
                    if self.shift_register != ShiftRegister::Div31Four {
                        self.div31_count = 0;
                        self.shift_register = ShiftRegister::Div31Four;
                    },
                3 =>
                    if self.shift_register != ShiftRegister::FiveToFour {
                        self.shift_register = ShiftRegister::FiveToFour;
                    },
                4 | 5 =>
                    if self.shift_register != ShiftRegister::Div2 {
                        self.div2_count = 0;
                        self.shift_register = ShiftRegister::Div2;
                    },
                6 | 10 =>
                    if self.shift_register != ShiftRegister::Div31 {
                        self.div31_count = 0;
                        self.shift_register = ShiftRegister::Div31;
                    },
                8 => 
                    if self.shift_register != ShiftRegister::Nine {
                        self.shift_register = ShiftRegister::Nine;
                    },
                7 | 9 =>
                    if self.shift_register != ShiftRegister::Five {
                        self.shift_register = ShiftRegister::Five;
                    },
                12 | 13 =>
                    if self.shift_register != ShiftRegister::Div2 {
                        self.div2_count = 0;
                        self.shift_register = ShiftRegister::Div2;
                        self.apply_third = true;
                        self.third_count = 0;
                    },
                14 =>
                    if self.shift_register != ShiftRegister::Div31 {
                        self.div31_count = 0;
                        self.shift_register = ShiftRegister::Div31;
                        self.apply_third = true;
                        self.third_count = 0;                        
                    },
                15 =>
                    if self.shift_register != ShiftRegister::Five {
                        self.shift_register = ShiftRegister::Five;
                        self.apply_third = true;
                        self.third_count = 0;
                    },
                _ => {}

            }
        }

        
        fn shift_four_register(&mut self) {
            let new_bit: u8 = (((self.shift_4_register & 0x0002) >> 1) ^ (self.shift_4_register & 0x0001)) as u8;
            self.shift_4_register = self.shift_4_register >> 1;
            if new_bit > 0 {
                self.shift_4_register |= 0x0008;
            }
            else {
                self.shift_4_register &= 0xFFF7;
            }
        }
        
        fn shift_five_register(&mut self) {
            let new_bit: u8 = (((self.shift_5_register & 0x0004) >> 2) ^ (self.shift_5_register & 0x0001)) as u8;
            self.shift_5_register = self.shift_5_register >> 1;
            if new_bit > 0 {
                self.shift_5_register |= 0x0010;
            }
            else {
                self.shift_5_register &= 0xFFEF;
            }
        }
        
        fn shift_nine_register(&mut self) {
            let new_bit: u8 = (((self.shift_9_register & 0x0010) >> 4) ^ (self.shift_9_register & 0x0001)) as u8;
            self.shift_9_register = self.shift_9_register >> 1;
            if new_bit > 0 {
                self.shift_9_register |= 0x0100;
            }
            else {
                self.shift_9_register &= 0xFEFF;
            }
        }

        fn shift_registers(&mut self) {
            if self.apply_third {
                self.third_count += 1;
                if self.third_count < 2 {
                    return;
                }
                self.third_count = 0;
            }
            
            let mut should_shift_four: bool = true;
            if self.shift_register == ShiftRegister::FiveToFour {
                should_shift_four = (self.shift_5_register & 0x0001) > 0;
            }

            if self.shift_register == ShiftRegister::Div31Four {
                if self.div31_count != 17 && self.div31_count != 30 {
                    should_shift_four = false;
                }
            }

            if should_shift_four {
                self.shift_four_register();
            }

            self.shift_five_register();
            self.shift_nine_register();

            if self.shift_register == ShiftRegister::Div2 {
                if self.div2_count == 0 {
                    self.div2_count = 1;
                }
                else {
                    self.div2_count = 0;
                }
            }
            else if self.shift_register == ShiftRegister::Div31 || self.shift_register == ShiftRegister::Div31Four {
                self.div31_count += 1;

                if self.div31_count >= 31 {
                    self.div31_count = 0;
                }
            }
        }

        fn generate_buffer_data(&mut self, sample_count: usize) -> &[u8] {

            if self.frequency == 0 {
                if sample_count > BUFFER_SIZE as usize {
                    panic!("Audio Sample larger than buffer size");
                }
                self.m_buffer[0..sample_count as usize].fill(0);
                return self.m_buffer.as_slice().into();
            }

            let wavelength: u32 = ((DATA_SAMPLE_RATE_HZ / self.frequency as usize)) as u32;
            let mut sample_index: u32 = 0;

            while sample_index < sample_count as u32{

                if (self.total_sample % wavelength as u64) == 0 {
                    self.shift_registers();
                }

                if self.shift_register == ShiftRegister::Four 
                    || self.shift_register == ShiftRegister::Div31Four
                    || self.shift_register == ShiftRegister::FiveToFour {

                        if self.shift_4_register & 0x0001 == 1 {
                            self.m_buffer[sample_index as usize] = self.volume + 127;
                        }
                        else {
                            self.m_buffer[sample_index as usize] = self.volume;
                        }
                    }
                    else if self.shift_register == ShiftRegister::Five {

                        if self.shift_5_register & 0x0001 == 1 {
                            self.m_buffer[sample_index as usize] = self.volume + 127;
                        }
                        else {
                            self.m_buffer[sample_index as usize] = self.volume;
                        }
                    }
                    else if self.shift_register == ShiftRegister::Nine {

                        if self.shift_9_register & 0x0001 == 1 {
                            self.m_buffer[sample_index as usize] = self.volume + 127;
                        }
                        else {
                            self.m_buffer[sample_index as usize] = self.volume;
                        }
                    }
                    else if self.shift_register == ShiftRegister::Div2 {

                        if self.div2_count == 1 {
                            self.m_buffer[sample_index as usize] = self.volume + 127;
                        }
                        else {
                            self.m_buffer[sample_index as usize] = self.volume;
                        }
                    }
                    else if self.shift_register == ShiftRegister::Div31 {

                        if self.div31_count < 18 {
                            self.m_buffer[sample_index as usize] = self.volume + 127;
                        }
                        else {
                            self.m_buffer[sample_index as usize] = self.volume;
                        }
                    }

                    sample_index += 1;
                    self.total_sample += 1;

                    if sample_index > BUFFER_SIZE as u32 {
                        panic!("Audio sample index larger than buffer size");
                    }

            }

            self.m_buffer.as_slice().into()
        }

        pub fn callback(&mut self, size: usize) -> Vec<u8> {
                
            let mut buffer: Vec<u8> = vec![0; size];

            if self.is_shutdown {
                return buffer;
            }

            buffer.copy_from_slice(self.generate_buffer_data(size));
            buffer
            // copy to stream
        }

    }

}
