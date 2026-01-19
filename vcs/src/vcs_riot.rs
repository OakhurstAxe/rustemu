
pub mod vcs {

    use emumemory::{base_memory::emu_memory::BaseMemory, memory_ram::emu_memory::MemoryRam};

    const REG_OFFSET: u16 = 0x280;

    const REG_SWCHA:  u16 = 0x280 - REG_OFFSET;
    const REG_SWCNT:  u16 = 0x281 - REG_OFFSET;
    const REG_SWCHB:  u16 = 0x282 - REG_OFFSET;
    const REG_SWBCNT: u16 = 0x283 - REG_OFFSET;

    const REG_INTIM:  u16 = 0x284 - REG_OFFSET;
    const REG_INSTAT: u16 = 0x285 - REG_OFFSET;
    const REG_TIMI1T: u16 = 0x294 - REG_OFFSET;
    const REG_TIM8T:  u16 = 0x295 - REG_OFFSET;
    const REG_TIM64T: u16 = 0x296 - REG_OFFSET;
    const REG_T1024T: u16 = 0x297 - REG_OFFSET;
    
    pub struct VcsRiot {
        riot_ram: MemoryRam,
        system_ram: MemoryRam,
        step: u16,
        step_count: u16,
        overflow_tick: bool,
        select_pressed: bool,
        reset_pressed: bool
    }

    impl VcsRiot {

        pub fn new() -> Self {
            Self {
                riot_ram: MemoryRam::new(String::from("VCS RIOT Registers"), 0x80),
                system_ram: MemoryRam::new(String::from("VCS Ram"), 0x80),
                step: 1,
                step_count: 0,
                overflow_tick: false,
                select_pressed: false,
                reset_pressed: false
            }
        }

        pub fn reset(&mut self) {
            self.step = 1;
            self.riot_ram.write(REG_INSTAT, 0);
            self.riot_ram.write(REG_SWCHA, 255);
            self.riot_ram.write(REG_SWCNT, 255);
            self.riot_ram.write(REG_SWBCNT, 255);
            self.select_pressed = false;
            self.reset_pressed = false;
        }

        pub fn reset_pressed(&mut self, value: bool) {
            if value {
                self.reset_pressed = true;
            }
            else {
                self.reset_pressed = false;
            }
        }

        pub fn select_pressed(&mut self, value: bool)
        {
            if value {
                self.select_pressed = true;
            }
            else {
                self.select_pressed = false;
            }
        }

        // up + down -
        pub fn left_controller_up_down(&mut self, value : i8) {
            let mut byte: u8 = self.riot_ram.read(REG_SWCHA);
            if value < 0 {
                byte |= 0x30;
                byte &= 0xDF;
            }
            else if value > 0 {
                byte |= 0x30;
                byte &= 0xEF;
            }
            else if value == 0 {
                byte |= 0x30;
            }
            self.riot_ram.write(REG_SWCHA, byte);
        }
           
        // left + right -
        pub fn left_controller_left_right(&mut self, value : i8) {
            let mut byte: u8 = self.riot_ram.read(REG_SWCHA);
            if value < 0 {
                byte |= 0xC0;
                byte &= 0xBF;
            }
            else if value > 0 {
                byte |= 0xC0;
                byte &= 0x7F;
            }
            else if value == 0 {
                byte |= 0xC0;
            }
            self.riot_ram.write(REG_SWCHA, byte);
        }

        pub fn execute_tick(&mut self) {
            self.step_count += 1;
            if self.step_count < self.step {
                return;
            }

            self.step_count = 0;
            self.overflow_tick = false;
            let mut timer: u8 = self.riot_ram.read(REG_INTIM);
            let (new_timer, overflow) = timer.overflowing_sub(1);
            timer = new_timer;
            if overflow {
                //timer = 0;
                let mut status_byte: u8 = self.system_ram.read(REG_INSTAT);
                status_byte = status_byte | 0xFF;
                self.riot_ram.write(REG_INSTAT, status_byte);
                self.step = 1;
                self.overflow_tick = true;
            }            
            self.riot_ram.write(REG_INTIM, timer);
        }

        pub fn read(&mut self, location: u16) -> u8 {

            if location == REG_INTIM {
                self.clear_timnnt_underflow();
            }
            else if location == REG_INSTAT {
                self.clear_instat_underflow();
            }

            if location == REG_SWCHB {
                let mut result: u8 = 11;
                if self.select_pressed {
                    result &= 0xFD;
                }
                if self.reset_pressed {
                    result &= 0xFE;
                }
                return result;
            }

            self.riot_ram.read(location)
        }

        pub fn write(&mut self, location: u16, byte: u8) {

            let (byte_minus_1, _overflow) = byte.overflowing_sub(1);

            if location == REG_TIMI1T {
                self.clear_timnnt_underflow();
                self.riot_ram.write(REG_INTIM, byte_minus_1);
                self.step = 1;
                self.step_count = 0;
            }
            else if location == REG_TIM8T {
                self.clear_timnnt_underflow();
                self.riot_ram.write(REG_INTIM, byte_minus_1);
                self.step = 8;
                self.step_count = 0;
            } else if location == REG_TIM64T {
                self.clear_timnnt_underflow();
                self.riot_ram.write(REG_INTIM, byte_minus_1);
                self.step = 64;
                self.step_count = 0;
            }
            else if location == REG_T1024T {
                self.clear_timnnt_underflow();
                self.riot_ram.write(REG_INTIM, byte_minus_1);
                self.step = 1024;
                self.step_count = 0;
            }
            self.riot_ram.write(location, byte);
        }

        pub fn read_ram(&self, location: u16) -> u8 {
            self.system_ram.read(location)
        }
        
        pub fn write_ram(&mut self, location: u16, byte: u8) {
            self.system_ram.write(location, byte);
        }

        fn clear_instat_underflow(&mut self)
        {
            if !self.overflow_tick {
                let mut status_byte: u8 = self.system_ram.read(REG_INSTAT);
                status_byte = status_byte & 0xBF;
                self.riot_ram.write(REG_INSTAT, status_byte);
            }
        }
        
        fn clear_timnnt_underflow(&mut self)
        {
            if !self.overflow_tick {
                let mut status_byte: u8 = self.system_ram.read(REG_INSTAT);
                status_byte = status_byte & 0x7F;
                self.riot_ram.write(REG_INSTAT, status_byte);
            }
        }

    }
}