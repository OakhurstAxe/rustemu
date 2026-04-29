
pub mod nes {

    use emumemory::memory_ram_flagged::emu_memory::MemoryRamFlagged;

    use crate::nes_apuchannel::nes::NesApuChannel;
    use crate::nes_apupulsechannel::nes::NesApuPulseChannel;
    use crate::nes_aputrianglechannel::nes::NesApuTriangleChannel;
    use crate::nes_apunoisechannel::nes::NesApuNoiseChannel;
    use crate::nes_apuchannel::nes::SAMPLES_PER_FRAME;

    pub struct NesApu {
        apu_io_registers: MemoryRamFlagged,
        left_controller: u8,
        right_controller: u8,
        channel0: NesApuPulseChannel,
        channel1: NesApuPulseChannel,
        channel2: NesApuTriangleChannel,
        channel3: NesApuNoiseChannel,
    }

    impl NesApu {

        pub fn new() -> NesApu {
            Self {
                apu_io_registers: MemoryRamFlagged::new(0x001f, String::from("APU IO Registers")),
                left_controller: 0,
                right_controller: 0,
                channel0: NesApuPulseChannel::new(),
                channel1: NesApuPulseChannel::new(),
                channel2: NesApuTriangleChannel::new(),
                channel3: NesApuNoiseChannel::new(),
            }
        }
        
        pub fn is_read_flag_set(&mut self, location: u16) -> bool {
            self.apu_io_registers.is_read_flag_set(location)
        }

        pub fn is_write_flag_set(&mut self, location: u16) -> bool {
            self.apu_io_registers.is_write_flag_set(location)
        }

        pub fn read(&mut self, location: u16) -> u8 {
            self.apu_io_registers.read(location)
        }

        pub fn write(&mut self, location: u16, byte: u8) {
            self.apu_io_registers.write(location, byte);
        }

        pub fn set_left_controller(&mut self, byte: u8) {
            self.left_controller = byte;
        }

        pub fn set_right_controller(&mut self, byte: u8) {
            self.right_controller = byte;
        }

        pub fn get_left_controller(&mut self) -> u8 {
            let result: u8 = self.left_controller & 0x01;
            self.left_controller >>= 1;
            result
        }

        pub fn get_right_controller(&mut self) -> u8 {
            let result: u8 = self.right_controller & 0x01;
            self.right_controller >>= 1;
            result
        }

        pub fn execute_tick(&mut self) {
            
            let mut register1: u8 = self.apu_io_registers.read(0);
            let mut register1_flag = self.apu_io_registers.is_write_flag_set(0);
            let mut register2: u8 = self.apu_io_registers.read(1);
            let mut register2_flag = self.apu_io_registers.is_write_flag_set(1);
            let mut register3: u8 = self.apu_io_registers.read(2);
            let mut register3_flag = self.apu_io_registers.is_write_flag_set(2);
            let mut register4: u8 = self.apu_io_registers.read(3);
            let mut register4_flag = self.apu_io_registers.is_write_flag_set(3);
            self.channel0.set_channel_settings(register1, register1_flag,
                                            register2, register2_flag,
                                            register3, register3_flag,
                                            register4, register4_flag);

            register1 = self.apu_io_registers.read(4);
            register1_flag = self.apu_io_registers.is_write_flag_set(4);
            register2 = self.apu_io_registers.read(5);
            register2_flag = self.apu_io_registers.is_write_flag_set(5);
            register3 = self.apu_io_registers.read(6);
            register3_flag = self.apu_io_registers.is_write_flag_set(6);
            register4 = self.apu_io_registers.read(7);
            register4_flag = self.apu_io_registers.is_write_flag_set(7);
            self.channel1.set_channel_settings(register1, register1_flag,
                                            register2, register2_flag,
                                            register3, register3_flag,
                                            register4, register4_flag);

            register1 = self.apu_io_registers.read(8);
            register1_flag = self.apu_io_registers.is_write_flag_set(8);
            register2 = self.apu_io_registers.read(9);
            register2_flag = self.apu_io_registers.is_write_flag_set(9);
            register3 = self.apu_io_registers.read(10);
            register3_flag = self.apu_io_registers.is_write_flag_set(10);
            register4 = self.apu_io_registers.read(11);
            register4_flag = self.apu_io_registers.is_write_flag_set(11);
            self.channel2.set_channel_settings(register1, register1_flag,
                                            register2, register2_flag,
                                            register3, register3_flag,
                                            register4, register4_flag);

            register1 = self.apu_io_registers.read(12);
            register1_flag = self.apu_io_registers.is_write_flag_set(12);
            register2 = self.apu_io_registers.read(13);
            register2_flag = self.apu_io_registers.is_write_flag_set(13);
            register3 = self.apu_io_registers.read(14);
            register3_flag = self.apu_io_registers.is_write_flag_set(14);
            register4 = self.apu_io_registers.read(15);
            register4_flag = self.apu_io_registers.is_write_flag_set(15);
            self.channel3.set_channel_settings(register1, register1_flag,
                                            register2, register2_flag,
                                            register3, register3_flag,
                                            register4, register4_flag);
                                        }

        pub fn get_audio_buffer(&mut self) -> Vec<u8> {

            let mut mix:Vec<u8> = Vec::with_capacity(SAMPLES_PER_FRAME);

            let buffer0 = self.channel0.generate_buffer_data(SAMPLES_PER_FRAME as u32).clone();
            let buffer1 = self.channel1.generate_buffer_data(SAMPLES_PER_FRAME as u32).clone();
            let buffer2 = self.channel2.generate_buffer_data(SAMPLES_PER_FRAME as u32).clone();
            let buffer3 = self.channel3.generate_buffer_data(SAMPLES_PER_FRAME as u32).clone();

            for i in 0..SAMPLES_PER_FRAME {
                let volume = buffer0[i];// + buffer1[i];// + buffer2[i];// + buffer3[i];
                mix.push(volume);
            }

            mix
        }

    }
}
