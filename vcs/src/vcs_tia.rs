
pub mod vcs {

    use std::rc::Rc;
    use std::cell::RefCell;

    use emumemory::{base_memory::emu_memory::BaseMemory, memory_ram::emu_memory::MemoryRam};
    use rand::Rng;
    use crate::{vcs_console_type::{vcs::VcsConsoleType}, vcs_palette::vcs::VcsPalette};

    const REG_VSYNC: u16 =    0x00;
    const REG_VBLANK: u16 =   0x01;
    const REG_WSYNC: u16 =    0x02;
    const REG_RSYNC: u16 =    0x03;
    const REG_NUSIZ0: u16 =   0x04;
    const REG_NUSIZ1: u16 =   0x05;
    const REG_COLUP0 : u16 =  0x06;
    const REG_COLUP1: u16 =   0x07;
    const REG_COLUPF: u16 =   0x08;
    const REG_COLUBK : u16 =  0x09;
    const REG_CTRLPF: u16 =   0x0A;
    const REG_REFP0: u16 =    0x0B;
    const REG_REFP1: u16 =    0x0C;
    const REG_PF0: u16 =      0x0D;
    const REG_PF1: u16 =      0x0E;
    const REG_PF2: u16 =      0x0F;

    const REG_RESP0: u16 =    0x10;
    const REG_RESP1: u16 =    0x11;
    const REG_RESM0: u16 =    0x12;
    const REG_RESM1: u16 =    0x13;
    const REG_RESBL: u16 =    0x14;

    const REG_AUDC0: u16 =    0x15;
    const REG_AUDC1: u16 =    0x16;
    const REG_AUDF0: u16 =    0x17;
    const REG_AUDF1: u16 =    0x18;
    const REG_AUDV0: u16 =    0x19;
    const REG_AUDV1: u16 =    0x1A;

    const REG_GRP0: u16 =     0x1B;
    const REG_GRP1: u16 =     0x1C;
    const REG_ENAM0: u16 =    0x1D;
    const REG_ENAM1: u16 =    0x1E;
    const REG_ENABL: u16 =    0x1F;

    const REG_HMP0: u16 =     0x20;
    const REG_HMP1: u16 =     0x21;
    const REG_HMM0: u16 =     0x22;
    const REG_HMM1: u16 =     0x23;
    const REG_HMBL: u16 =     0x24;
    const REG_VDELP0: u16 =   0x25;
    const REG_VDELP1: u16 =   0x26;
    const REG_VDELBL: u16 =   0x27;
    const REG_RESMP0: u16 =   0x28;
    const REG_RESMP1: u16 =   0x29;
    const REG_HMOVE: u16 =    0x2A;
    const REG_HMCLR: u16 =    0x2B;
    const REG_CXCLR: u16 =    0x2C;

    const REG_CXM0P: u16 =    0x30;
    const REG_CXM1P: u16 =   0x31;
    const REG_CXP0FB: u16 =   0x32;
    const REG_CXP1FB: u16 =   0x33;
    const REG_CXM0FB: u16 =   0x34;
    const REG_CXM1FB: u16 =   0x35;
    const REG_CXBLPF: u16 =   0x36;
    const REG_CXPPMM: u16 =   0x37;
    const REG_INPT0: u16 =    0x38;
    const REG_INPT1: u16 =    0x39;
    const REG_INPT2: u16 =    0x3A;
    const REG_INPT3: u16 =    0x3B;
    const REG_INPT4: u16 =    0x3C;
    const REG_INPT5 : u16 =   0x3D;

    const CLOSE: u16 =        16;
    const MEDIUM: u16 =       40;
    const WIDE: u16 =         72;

    const SPRITEOFFSET: u16 =  5;
    pub struct VcsTia {
        registers: MemoryRam,
        vcs_console_type: Rc<RefCell<VcsConsoleType>>,
        cycle: u16,
        scan_line: u16,
        screen: Vec<u8>,
        screen_display: Vec<u8>,
        vcs_palette: VcsPalette,
        w_sync_set: bool,
        res_p0_cycle: u16,
        res_p1_cycle: u16,
        res_m0_cycle: u16,
        res_m1_cycle: u16,
        res_bl_cycle: u16,
        grp_0_delay: u8,
        grp_1_delay: u8,
        enable_delay: u8
    }

    impl VcsTia {

        pub fn new(console_type: Rc<RefCell<VcsConsoleType>>) -> Self {
            let x_resolution: u8 = console_type.borrow_mut().get_x_resolution();
            let y_resolution: u8 = console_type.borrow_mut().get_y_resolution();
            Self {
                registers: MemoryRam::new(String::from("TIA Registers"), 0x7f),
                vcs_console_type: Rc::clone(&console_type),
                cycle: 0,
                scan_line: 0,
                screen: vec![0u8; (x_resolution as u32 * y_resolution as u32 * 4) as usize],
                screen_display: vec![0u8; (x_resolution as u32 * y_resolution as u32 * 4) as usize],
                vcs_palette: VcsPalette::new(Rc::clone(&console_type)),
                w_sync_set: false,
                res_p0_cycle: 0,
                res_p1_cycle: 0,
                res_m0_cycle: 0,
                res_m1_cycle: 0,
                res_bl_cycle: 0,
                grp_0_delay: 0,
                grp_1_delay: 0,
                enable_delay: 0,
            }
        }

        pub fn get_screen(&self) -> Vec<u8> {
            self.screen_display.clone()
        }

        pub fn read(&self, location: u16) -> u8 {
            let mut result: u8;
            
            if location < 0x30 || location > 0x3D {
                // Undefined TIA read returns address 0x30
                result = self.registers.read(0x30);
            }

            result = self.registers.read(location);
            result
        }

        pub fn write(&mut self, location: u16, byte: u8) {
            if location > 0x2C {
                // Undefined write, does nothing.
                // Sometimes used to waste specific cycle count.
                return; 
            }
            
            if location == REG_GRP0 {
                if self.registers.read(REG_VDELP0) & 0x01 > 0 {
                    self.grp_0_delay = byte;
                }
                else {
                    self.registers.write(REG_GRP0, byte);
                }

                if self.registers.read(REG_VDELP1) & 0x01 > 0 {
                    self.registers.write(REG_GRP1, self.grp_1_delay);
                    self.grp_1_delay = 0;
                }
            }
            else if location == REG_GRP1
            {
                if self.registers.read(REG_VDELP1) & 0x01 > 0 {
                    self.grp_1_delay = byte;
                }
                else {
                    self.registers.write(REG_GRP1, byte);
                }

                if self.registers.read(REG_VDELP0) & 0x01 > 0 {
                    self.registers.write(REG_GRP0, self.grp_0_delay);
                    self.grp_0_delay = 0;
                }

                if self.registers.read(REG_VDELBL) & 0x01 > 0 {
                    self.registers.write(REG_ENABL, self.enable_delay);
                    self.enable_delay = 0;
                }
            }
            else if location == REG_ENABL {
                if self.registers.read(REG_VDELBL) & 0x01 > 0 {
                    self.enable_delay = 0;
                }
                else {
                    self.registers.write(REG_ENABL, byte);
                }
            }
            else if location == REG_VSYNC {
                if byte & 0x02 == 0 && self.registers.read(REG_VSYNC) & 0x02 > 0 {
                    self.scan_line = 2;
                    self.registers.write(REG_VBLANK, self.registers.read(REG_VBLANK) | 0x02);
                }

                self.registers.write(REG_VSYNC, byte);
            }
            else if location == REG_VBLANK {

                // the scanLine_ > 30 is a hack that seems to work,  Probably a more accurate way to do it
                if byte & 0x02 == 0 && self.registers.read(REG_VBLANK & 0x02) > 0 && self.scan_line > 30 {
                    self.scan_line = 2 + self.vcs_console_type.borrow().get_v_blank_lines() as u16;
                }
                self.registers.write(REG_VBLANK, byte);
            }
            else if location == REG_WSYNC {
                self.w_sync_set = true;
            }
            else if location == REG_RSYNC {
                self.cycle = 0;
            }
            else if location == REG_RESP0 {
                self.res_p0_cycle = self.cycle + SPRITEOFFSET;

                if self.res_p0_cycle < 68 {
                    self.res_p0_cycle = 71;
                }
            }
            else if location == REG_RESP1 {
                self.res_p1_cycle = self.cycle + SPRITEOFFSET;

                if self.res_p1_cycle < 68 {
                    self.res_p1_cycle = 71;
                }
            }
            else if location == REG_RESM0 {
                self.res_m0_cycle = self.cycle + SPRITEOFFSET - 1;

                if self.res_m0_cycle < 68 {
                    self.res_m0_cycle = 71;
                }
            }
            else if location == REG_RESM1 {
                self.res_m1_cycle = self.cycle + SPRITEOFFSET - 1;

                if self.res_m1_cycle < 68 {
                    self.res_m1_cycle = 71;
                }
            }
            else if location == REG_RESBL {
                self.res_bl_cycle = self.cycle + SPRITEOFFSET;

                if self.res_bl_cycle < 68 {
                    self.res_bl_cycle = 71;
                }
            }
            else if location == REG_HMOVE {
                self.apply_movement();
            }
            else if location == REG_HMCLR {
                self.clear_move_registers();
            }
            else if location == REG_CXCLR {
                self.registers.write(REG_CXM0P, 0);
                self.registers.write(REG_CXM1P, 0);
                self.registers.write(REG_CXP0FB, 0);
                self.registers.write(REG_CXP1FB, 0);
                self.registers.write(REG_CXM0FB, 0);
                self.registers.write(REG_CXM1FB, 0);
                self.registers.write(REG_CXBLPF, 0);
                self.registers.write(REG_CXPPMM, 0);
            }
            else {
                self.registers.write(location, byte);
            }
        }

        pub fn reset(&mut self) {
            self.cycle = 0;
            self.scan_line = 0;
            self.w_sync_set = false;
            self.res_p0_cycle = 0;
            self.res_p1_cycle = 0;
            
            for i in 0..0x7f {
                self.registers.write(i, 0);
            }

            self.registers.write(REG_INPT0, 255);
            self.registers.write(REG_INPT1, 255);
            self.registers.write(REG_INPT2, 255);
            self.registers.write(REG_INPT3, 255);
            self.registers.write(REG_INPT4, 255);
            self.registers.write(REG_INPT5, 255);
        }

        pub fn execute_tick(&mut self) {
            self.cycle += 1;
            if self.cycle > 67 + self.vcs_console_type.borrow().get_x_resolution() as u16 {
                // Set rendering registers for when scrolling happens
                self.cycle = 0;
                self.scan_line += 1;
            }
            
            if (self.scan_line > (2 + self.vcs_console_type.borrow().get_v_blank_lines() as u16)) && 
                (self.scan_line <= (2 + self.vcs_console_type.borrow().get_v_blank_lines() as u16 + 
                    self.vcs_console_type.borrow().get_y_resolution() as u16)) && (self.cycle > 67) {
                self.render_pixel();
            }
            
            if self.registers.read(REG_RESMP0) & 0x02 > 0 {
                let size: u8 = self.registers.read(REG_NUSIZ0);
                self.res_m0_cycle = self.res_p0_cycle;
                if size & 0x07 == 5 { // size 2
                    self.res_m0_cycle += 6;
                }
                else if size & 0x07 == 7 { // size 4
                    self.res_m0_cycle += 10;
                }
                else // size 1
                {
                    self.res_m0_cycle += 3;
                }
            }

            if self.registers.read(REG_RESMP1) & 0x02 > 0 {
                let size: u8 = self.registers.read(REG_NUSIZ1);
                self.res_m1_cycle = self.res_p1_cycle;
                if size & 0x07 == 5 { // size 2
                    self.res_m1_cycle += 6;
                }
                else if size & 0x07 == 7 { // size 4
                    self.res_m1_cycle += 10;
                }
                else { // size 1
                    self.res_m1_cycle += 3;
                }
            }
            
            // WSYNC 
            if self.cycle == 3 { // Not sure this should be 8, but works pretty good
                self.w_sync_set = false;
            }
        }

        pub fn repaint(&mut self) -> bool {
            if self.cycle == 0 && self.scan_line == 3 {
                self.screen_display = self.screen.clone();
            }
            self.cycle == 0 && self.scan_line == 3
        }

        fn move_object(&mut self, mov: u8) -> u16 {
            let mut move_value: u8 = ((mov & 0x70) >> 4) as u8;
            if move_value & 0x80 > 0 {
                // twos compliment
                move_value = move_value | 0xf8;
            }
            move_value as u16
        }

        fn apply_movement(&mut self) {
            self.res_p0_cycle -= self.move_object(self.registers.read(REG_HMP0));
            self.res_p1_cycle -= self.move_object(self.registers.read(REG_HMP1));
            self.res_m0_cycle -= self.move_object(self.registers.read(REG_HMM0));
            self.res_m1_cycle -= self.move_object(self.registers.read(REG_HMM1));
            self.res_bl_cycle -= self.move_object(self.registers.read(REG_HMBL));
        }
        
        fn clear_move_registers(&mut self) {
            self.registers.write(REG_HMP0, 0);
            self.registers.write(REG_HMP1, 0);
            self.registers.write(REG_HMM0, 0);
            self.registers.write(REG_HMM1, 0);
            self.registers.write(REG_HMBL, 0);
        }

        fn get_player_pixel(&self, graphics_player: u8, player_size: u8, reflect_player: u8, color: u8, player_cycle: u16) -> i16 {
            let mut result: i16 = -1;
            
            let mut sprite_data: u8 = graphics_player;
            if sprite_data == 0 {
                return result;
            }
            if (reflect_player & 0x08) == 0 {
                sprite_data = VcsTia::reverse_bits(sprite_data);
            }
            
            let mut position2_cycle: u16 = player_cycle;
            let mut position3_cycle: u16 = player_cycle;
            let mut size_multiple: u8 = 1;
            let size: u8 = player_size;
            if (size & 0x07) == 0 {
                size_multiple = 1;
            }
            else if (size & 0x07) == 1 {
                position2_cycle = player_cycle + CLOSE;
            }
            else if (size & 0x07) == 2 {
                position2_cycle = player_cycle + MEDIUM;
            }
            else if (size & 0x07) == 3 {
                position2_cycle = player_cycle + CLOSE;
                position3_cycle = position2_cycle + CLOSE;
            }
            else if (size & 0x07) == 4 {
                position2_cycle = player_cycle + WIDE;
            }
            else if (size & 0x07) == 5 {
                size_multiple = 2;
            }
            else if (size & 0x07) == 6 {
                position2_cycle = player_cycle + MEDIUM;
                position3_cycle = position2_cycle + MEDIUM;
            }
            else if (size & 0x07) == 7 {
                size_multiple = 4;
            }

            let (mut value, mut _overflow) = self.cycle.overflowing_sub(player_cycle);
            let mut shift: u32 = (value/ size_multiple as u16) as u32;
            if shift < 8 && (((sprite_data >> shift) & 0x01) > 0) {
                result = color as i16;
                return result;
            }
            (value, _overflow) = self.cycle.overflowing_sub(position2_cycle);
            shift = (value/ size_multiple as u16) as u32;
            if shift < 8 && (((sprite_data >> shift) & 0x01) > 0) {
                result = color as i16;
                return result;
            }
            (value, _overflow) = self.cycle.overflowing_sub(position3_cycle);
            shift = (value/ size_multiple as u16) as u32;
            if shift < 8 && (((sprite_data >> shift) & 0x01) > 0) {
                result = color as i16;
                return result;
            }                    

            result
        }

        fn get_playfield_pixel(&self) -> i16 {
            let screen_x: u16 = self.cycle - 68;
            let control_playfield: u8 = self.registers.read(REG_CTRLPF);
            let mut playfield_color: u8 = self.registers.read(REG_COLUPF);
            let mut byte: u8;
            let mut result: i16 = -1;

            if control_playfield & 0x02 > 0 {
                if screen_x < 80 {
                    playfield_color = self.registers.read(REG_COLUP0);
                }
                else {
                    playfield_color = self.registers.read(REG_COLUP1);
                }
            }

            if screen_x < 16 {
                byte = (self.registers.read(REG_PF0) >> 4) & 0x0f;
                byte = (byte >> (screen_x >> 2)) & 0x01;
                if byte > 0 {
                    result = playfield_color as i16;
                }
            }
            else if screen_x < 48 {
                byte = self.registers.read(REG_PF1);
                byte = VcsTia::reverse_bits(byte);
                let shift: u8 = ((screen_x - 16) >> 2) as u8;
                byte = (byte >> shift) & 0x01;
                if byte > 0 {
                    result = playfield_color as i16;
                }
            }
            else if screen_x < 80 {
                byte = self.registers.read(REG_PF2);
                let shift: u8 = ((screen_x - 48) >> 2) as u8;
                byte = (byte >> shift) & 0x01;
                if byte > 0 {
                    result = playfield_color as i16;
                }
            }
            if screen_x >= 80 {
                if control_playfield & 0x01 > 0 {
                    if screen_x < 112 {
                        byte = self.registers.read(REG_PF2);
                        byte = VcsTia::reverse_bits(byte);
                        let shift: u8 = ((screen_x - 80) >> 2) as u8;
                        byte = (byte >> shift) & 0x01;
                        if byte > 0 {
                            result = playfield_color as i16;
                        }
                    }
                    else if screen_x < 144 {
                        byte = self.registers.read(REG_PF1);
                        let shift: u8 = ((screen_x - 112) >> 2) as u8;
                        byte = (byte >> shift) & 0x01;
                        if byte > 0 {
                            result = playfield_color as i16;
                        }
                    }
                    else if screen_x <= self.vcs_console_type.borrow_mut().get_x_resolution() as u16{
                        byte = (self.registers.read(REG_PF0) >> 4) & 0x0f;
                        byte = VcsTia::reverse_bits(byte) >> 4;
                        let shift: u8 = ((screen_x - 144) >> 2) as u8;
                        byte = (byte >> shift) & 0x01;
                        if byte > 0 {
                            result = playfield_color as i16;
                        }
                    }
                }
                else
                {
                    if screen_x < 96 {
                        byte = (self.registers.read(REG_PF0) >> 4) & 0x0f;
                        let shift: u8 = ((screen_x - 80) >> 2) as u8;
                        byte = (byte >> shift) & 0x01;
                        if byte > 0 {
                            result = playfield_color as i16;
                        }
                    }
                    else if screen_x < 128 {
                        byte = self.registers.read(REG_PF1);
                        byte = VcsTia::reverse_bits(byte);
                        let shift: u8 = ((screen_x - 96) >> 2) as u8;
                        byte = (byte >> shift) & 0x01;
                        if byte > 0 {
                            result = playfield_color as i16;
                        }
                    }
                    else if screen_x < self.vcs_console_type.borrow_mut().get_x_resolution() as u16 {
                        byte = self.registers.read(REG_PF2);
                        let shift: u8 = ((screen_x - 128) >> 2) as u8;
                        byte = (byte >> shift) & 0x01;
                        if byte > 0 {
                            result = playfield_color as i16;
                        }                
                    }
                }
            }
            
            return result;
        }

        fn render_pixel(&mut self) {
            let screen_x: u16 = self.cycle - 68;
            let screen_y: u16 = self.scan_line - (3 + self.vcs_console_type.borrow_mut().get_v_blank_lines()) as u16;
            let background: u8 = self.registers.read(REG_COLUBK);
            let mut current_color: i16 = -1;
            
            // Playfield
            let playfield_pixel: i16 = self.get_playfield_pixel();
            let mut pf_above: bool = false;
            if self.registers.read(REG_CTRLPF) & 0x04 > 0 {
                pf_above = true;
            }
            
            // Get each pixel for collision detection
            let p0_pixel: i16 = self.get_player_pixel(self.registers.read(REG_GRP0), self.registers.read(REG_NUSIZ0), self.registers.read(REG_REFP0), self.registers.read(REG_COLUP0), self.res_p0_cycle);
            let p1_pixel: i16 = self.get_player_pixel(self.registers.read(REG_GRP1), self.registers.read(REG_NUSIZ1), self.registers.read(REG_REFP1), self.registers.read(REG_COLUP1), self.res_p1_cycle);
            //int16_t m0Pixel = GetMisslePixel(memory_[REG_ENAM0], memory_[REG_RESMP0], memory_[REG_NUSIZ0], memory_[REG_COLUP0], resM0Cycle_);
            //int16_t m1Pixel = GetMisslePixel(memory_[REG_ENAM1], memory_[REG_RESMP1], memory_[REG_NUSIZ1], memory_[REG_COLUP1], resM1Cycle_);
            //int16_t ballPixel = GetBallPixel();
            
            let current_pixel: u32 = (screen_y * self.vcs_console_type.borrow().get_x_resolution() as u16 + screen_x) as u32;

            // Don't display pixel if PF has priority and is set
            if pf_above {                
                // Ball
                //if (ballPixel >= 0 && currentColor == -1)
                //{
                  //  currentColor = (uint8_t)ballPixel;
                //}
                // Playfield
                if playfield_pixel >= 0 && current_color == -1 {
                    current_color = playfield_pixel;
                }
            }
            // P0
            if p0_pixel >= 0 && current_color == -1 {
                current_color = p0_pixel as i16;
            }
            // M0
            //if (m0Pixel >= 0 && currentColor == -1)
            //{
            //    currentColor = (uint8_t)m0Pixel;
            //}
            
            // P1
            if p1_pixel >= 0 && current_color == -1 {
                current_color = p1_pixel as i16;
            }
            // M1
            //if (m1Pixel >= 0 && currentColor == -1)
            //{
            //    currentColor = (uint8_t)m1Pixel;
            //}

            // Ball
            //if (ballPixel >= 0 && currentColor == -1)
            //{
            //    currentColor = (uint8_t)ballPixel;
            //}
            // Playfield
            if playfield_pixel >= 0 && current_color == -1 {
                current_color = playfield_pixel;
            }
            // Background
            if current_color == -1 {
                current_color = background as i16;
            }

            let color: u32 = self.vcs_palette.get_color(current_color as u8);
            self.screen[current_pixel as usize * 4] = ((color & 0xff0000) >> 16) as u8;
            self.screen[current_pixel as usize * 4 + 1] = ((color & 0x00ff00) >> 8) as u8;
            self.screen[current_pixel as usize * 4 + 2] = ((color & 0x0000ff)) as u8;
            self.screen[current_pixel as usize * 4 + 3] = 255;

            //CheckCollisions(playfieldPixel,
            //    p0Pixel, p1Pixel, m0Pixel, m1Pixel,
            //    ballPixel);
        }

        pub fn is_cpu_blocked(&self) -> bool {
            let mut result: bool = false;

            if self.w_sync_set {
                result = true;
            }

            result
        }

        fn reverse_bits(n: u8) -> u8 {
            let mut input: u8 = n;
            let mut ans: u8 = 0;
            for i in 7..0 {
                ans |= (input & 1) << i;
                input = input >> 1;
            }
            return ans;
        }

    }
}