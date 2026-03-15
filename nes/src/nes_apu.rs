
pub mod nes {

    use emumemory::memory_ram_flagged::emu_memory::MemoryRamFlagged;

    use crate::nes_apuchannel::nes::NesApuChannel;
    use crate::nes_aputrianglechannel::nes::NesApuTriangleChannel;

    pub struct NesApu {
        apu_io_registers: MemoryRamFlagged,
        left_controller: u8,
        right_controller: u8,
        channels: [Box<dyn NesApuChannel>; 4],
    }

    impl NesApu {

        pub fn new() -> NesApu {
            Self {
                apu_io_registers: MemoryRamFlagged::new(0x001f, String::from("APU IO Registers")),
                left_controller: 0,
                right_controller: 0,
                channels: [Box::new(NesApuTriangleChannel::new()),
                    Box::new(NesApuTriangleChannel::new()),
                    Box::new(NesApuTriangleChannel::new()),
                    Box::new(NesApuTriangleChannel::new())],
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
            
            for i in 1..4 {
                let register1: u8 = self.apu_io_registers.read(i*4);
                let register1_flag = self.apu_io_registers.is_write_flag_set(i*4);
                let register2: u8 = self.apu_io_registers.read(i*4);
                let register2_flag = self.apu_io_registers.is_write_flag_set(i*4);
                let register3: u8 = self.apu_io_registers.read(i*4);
                let register3_flag = self.apu_io_registers.is_write_flag_set(i*4);
                let register4: u8 = self.apu_io_registers.read(i*4);
                let register4_flag = self.apu_io_registers.is_write_flag_set(i*4); 

                self.channels[i as usize].set_channel_settings(register1, register1_flag,
                                                register2, register2_flag,
                                                register3, register3_flag,
                                                register4, register4_flag);
            }
        }

    }
}
