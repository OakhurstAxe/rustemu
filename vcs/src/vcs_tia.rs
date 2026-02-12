
pub mod vcs {

    use std::sync::{ Arc, RwLock };

    use emumemory::{base_memory::emu_memory::BaseMemory, memory_ram::emu_memory::MemoryRam};
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
    const REG_CXM1P: u16 =    0x31;
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
        enable_delay: u8,
        v_blank: u8,
        x_resolution: u8,
        y_resolution: u8,
        _debug: u8
    }

    impl VcsTia {

        pub fn new(console_type: Arc<RwLock<VcsConsoleType>>) -> Self {
            let v_blank: u8 = console_type.read().unwrap().get_v_blank_lines();
            let x_resolution: u8 = console_type.read().unwrap().get_x_resolution();
            let y_resolution: u8 = console_type.read().unwrap().get_y_resolution();
            Self {
                registers: MemoryRam::new(String::from("TIA Registers"), 0x7f),
                cycle: 0,
                scan_line: 0,
                screen: vec![0u8; (x_resolution as u32 * y_resolution as u32 * 3) as usize],
                screen_display: vec![0u8; (x_resolution as u32 * y_resolution as u32 * 3) as usize],
                vcs_palette: VcsPalette::new(Arc::clone(&console_type)),
                w_sync_set: false,
                res_p0_cycle: 0,
                res_p1_cycle: 0,
                res_m0_cycle: 0,
                res_m1_cycle: 0,
                res_bl_cycle: 0,
                grp_0_delay: 0,
                grp_1_delay: 0,
                enable_delay: 0,
                v_blank: v_blank,
                x_resolution: x_resolution,
                y_resolution: y_resolution,
                _debug: 0
            }
        }

        pub fn get_screen(&self) -> Vec<u8> {
            self.screen_display.clone()
        }

        pub fn left_controller_trigger(&mut self, value: bool) {
            let mut reg_input_4 = self.registers.read(REG_INPT4);
            if value {
                reg_input_4 &= 0x7f;
            }
            else {
                reg_input_4 |= 0x80;                
            }
            self.registers.write(REG_INPT4, reg_input_4);
        }

        pub fn read(&self, location: u16) -> u8 {
            
            if location < 0x30 || location > 0x3D {
                // Undefined TIA read returns address 0x30
                return self.registers.read(0x30);
            }

            self.registers.read(location)
        }

        pub fn write(&mut self, location: u16, byte: u8) {
            if location > 0x2C {
                // Undefined write, does nothing.
                // Sometimes used to waste specific cycle count.
                return; 
            }

            if location == REG_GRP0 {
                if (self.registers.read(REG_VDELP0) & 0x01) > 0 {
                    self.grp_0_delay = byte;
                }
                else {
                    self.registers.write(REG_GRP0, byte);
                }

                if (self.registers.read(REG_VDELP1) & 0x01) > 0 {
                    self.registers.write(REG_GRP1, self.grp_1_delay);
                    self.grp_1_delay = 0;
                }
            }
            else if location == REG_GRP1
            {
                if (self.registers.read(REG_VDELP1) & 0x01) > 0 {
                    self.grp_1_delay = byte;
                }
                else {
                    self.registers.write(REG_GRP1, byte);
                }

                if (self.registers.read(REG_VDELP0) & 0x01) > 0 {
                    self.registers.write(REG_GRP0, self.grp_0_delay);
                    self.grp_0_delay = 0;
                }

                if (self.registers.read(REG_VDELBL) & 0x01) > 0 {
                    self.registers.write(REG_ENABL, self.enable_delay);
                    self.enable_delay = 0;
                }
            }
            else if location == REG_ENABL {
                if (self.registers.read(REG_VDELBL) & 0x01) > 0 {
                    self.enable_delay = byte;
                }
                else {
                    self.registers.write(REG_ENABL, byte);
                }
            }
            else if location == REG_VSYNC {
                if (byte & 0x02 == 0) && (self.registers.read(REG_VSYNC) & 0x02) > 0 {
                    self.scan_line = 2;
                    self.registers.write(REG_VBLANK, self.registers.read(REG_VBLANK) | 0x02);
                }

                self.registers.write(REG_VSYNC, byte);
            }
            else if location == REG_VBLANK {

                // the scanLine_ > 30 is a hack that seems to work,  Probably a more accurate way to do it
                if (byte & 0x02) == 0 && (self.registers.read(REG_VBLANK) & 0x02 > 0) && self.scan_line > 30 {
                    self.scan_line = 2 + self.v_blank as u16;
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

            self.registers.write(REG_AUDC0, 0);
            self.registers.write(REG_AUDC1, 0);
            self.registers.write(REG_AUDF0, 0);
            self.registers.write(REG_AUDF1, 0);
            self.registers.write(REG_AUDV0, 0);
            self.registers.write(REG_AUDV1, 0);
        }

        pub fn execute_tick(&mut self) {
            self.cycle += 1;
            if self.cycle > 67 + self.x_resolution as u16 {
                // Set rendering registers for when scrolling happens
                self.cycle = 0;
                self.scan_line += 1;
            }
            
            if (self.scan_line > (2 + self.v_blank as u16)) && 
                (self.scan_line <= (2 + self.v_blank as u16 + 
                    self.y_resolution as u16)) && (self.cycle > 67) {
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
                self.screen_display.copy_from_slice(&self.screen[..]);
            }
            self.cycle == 0 && self.scan_line == 3
        }

        fn move_object(&mut self, mov: u8, object_cycle: u16) -> u16 {
            let mut move_value: i8 = ((mov & 0x70) >> 4) as i8;
            if mov & 0x80 > 0 {
                // twos compliment
                move_value = (move_value as u8 | 0xf8) as i8;
            }
            let (mut new_value, _overflow) = object_cycle.overflowing_sub(move_value as u16);

            if new_value > 68 + self.x_resolution as u16 {
                new_value = 68;
            }
            if new_value < 68 {
                new_value = 68 + self.x_resolution as u16;
            }
            new_value
        }

        fn apply_movement(&mut self) {
            self.res_p0_cycle = self.move_object(self.registers.read(REG_HMP0), self.res_p0_cycle);
            self.res_p1_cycle = self.move_object(self.registers.read(REG_HMP1), self.res_p1_cycle);
            self.res_m0_cycle = self.move_object(self.registers.read(REG_HMM0), self.res_m0_cycle);
            self.res_m1_cycle = self.move_object(self.registers.read(REG_HMM1), self.res_m1_cycle);
            self.res_bl_cycle = self.move_object(self.registers.read(REG_HMBL), self.res_bl_cycle);
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

            let (value, mut _overflow) = self.cycle.overflowing_sub(player_cycle);
            let mut shift: u32 = (value/ size_multiple as u16) as u32;
            if shift < 8 && (((sprite_data >> shift) & 0x01) > 0) {
                result = color as i16;
                return result;
            }
            let (value2, _overflow) = self.cycle.overflowing_sub(position2_cycle);
            shift = (value2/ size_multiple as u16) as u32;
            if shift < 8 && (((sprite_data >> shift) & 0x01) > 0) {
                result = color as i16;
                return result;
            }
            let (value3, _overflow) = self.cycle.overflowing_sub(position3_cycle);
            shift = (value3/ size_multiple as u16) as u32;
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

            if (control_playfield & 0x02) > 0 {
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
                    else if screen_x <= self.x_resolution as u16{
                        byte = (self.registers.read(REG_PF0) >> 4) & 0x0f;
                        byte = VcsTia::reverse_bits(byte) >> 4;
                        let shift: u8 = ((screen_x - 144) >> 2) as u8;
                        byte = (byte >> shift) & 0x01;
                        if byte > 0 {
                            result = playfield_color as i16;
                        }
                    }
                }
                else {
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
                    else if screen_x < self.x_resolution as u16 {
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

        fn get_missle_pixel(&mut self, enable: u8, missle_reset: u8, missle_size: u8, missle_color: u8, missle_cycle: u16) -> i16 {
            let mut result: i16 = -1;
            
            if (enable & 0x02) > 0 && (missle_reset & 0x02) == 0 {
                let mut position_2_cycle: u16 = missle_cycle;
                let mut position_3_cycle: u16 = missle_cycle;
                let mut size: u8 = missle_size;

                if size & 0x07 == 1 {
                    position_2_cycle = missle_cycle + CLOSE;
                }
                else if size & 0x07 == 2 {
                    position_2_cycle = missle_cycle + MEDIUM;
                }
                else if size & 0x07 == 3 {
                    position_2_cycle = missle_cycle + CLOSE;
                    position_3_cycle = missle_cycle + MEDIUM;
                }
                else if size & 0x07 == 4 {
                    position_2_cycle = missle_cycle + WIDE;
                }
                else if size & 0x07 == 6 {
                    position_2_cycle = missle_cycle + MEDIUM;
                    position_3_cycle = missle_cycle + WIDE;
                }
                size = (size & 0x30) >> 4;

                match size {
                    0 => size = 1,
                    1 => size = 2,
                    2 => size = 4,
                    3 => size = 8,
                    _ => ()
                }

                if missle_cycle <= self.cycle && missle_cycle + size as u16 > self.cycle {
                    result  = missle_color as i16;
                }
                if position_2_cycle <= self.cycle && position_2_cycle + size as u16 > self.cycle {
                    result  = missle_color as i16;
                }
                if position_3_cycle <= self.cycle && position_3_cycle + size as u16 > self.cycle {
                    result  = missle_color as i16;
                }
            }

            result
        }

        fn get_ball_pixel(&mut self) -> i16{
            let mut result: i16 = -1;
            
            if self.registers.read(REG_ENABL) & 0x02 > 0 {
                let mut size: u8 = self.registers.read(REG_CTRLPF);
                size = (size & 0x30) >> 4;
                match size {
                    1 => size = 1,
                    2 => size = 2,
                    4 => size = 4,
                    8 => size = 8,
                    _ => ()
                }

                if self.res_bl_cycle <= self.cycle && self.res_bl_cycle + size as u16 >= self.cycle {
                    result = self.registers.read(REG_COLUPF) as i16;
                }
            }
            
            result
        }

        fn render_pixel(&mut self) {
            let screen_x: u16 = self.cycle - 68;
            let screen_y: u16 = self.scan_line - (3 + self.v_blank) as u16;
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
            let m0_pixel: i16 = self.get_missle_pixel(self.registers.read(REG_ENAM0), self.registers.read(REG_RESM0), self.registers.read(REG_NUSIZ0), self.registers.read(REG_COLUP0), self.res_m0_cycle);
            let m1_pixel: i16 = self.get_missle_pixel(self.registers.read(REG_ENAM1), self.registers.read(REG_RESM1), self.registers.read(REG_NUSIZ1), self.registers.read(REG_COLUP1), self.res_m1_cycle);
            let ball_pixel: i16 = self.get_ball_pixel();
            
            let current_pixel: usize = (screen_y * self.x_resolution as u16 + screen_x) as usize;

            // Don't display pixel if PF has priority and is set
            if pf_above {                
                // Ball
                if ball_pixel >= 0 && current_color == -1 {
                    current_color = ball_pixel as i16;
                }
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
            if m0_pixel >= 0 && current_color == -1 {
                current_color = m0_pixel as i16;
            }
            
            // P1
            if p1_pixel >= 0 && current_color == -1 {
                current_color = p1_pixel as i16;
            }
            // M1
            if m1_pixel >= 0 && current_color == -1 {
                current_color = m1_pixel as i16;
            }

            // Ball
            if ball_pixel >= 0 && current_color == -1 {
                current_color = ball_pixel as i16;
            }

            // Playfield
            if playfield_pixel >= 0 && current_color == -1 {
                current_color = playfield_pixel;
            }
            // Background
            if current_color == -1 {
                current_color = background as i16;
            }

            let (color_r, color_g, color_b) = self.vcs_palette.get_color(current_color as usize);
            self.screen[current_pixel * 3]     = color_r;
            self.screen[current_pixel * 3 + 1] = color_g;
            self.screen[current_pixel * 3 + 2] = color_b;

            self.check_collisions(playfield_pixel, p0_pixel, p1_pixel, m0_pixel, m1_pixel, ball_pixel);
        }

        fn check_collisions(&mut self, playfield_pixel: i16, p0_pixel: i16, p1_pixel: i16, 
            m0_pixel: i16, m1_pixel: i16, ball_pixel: i16) {
            // Collisions
            let mut collision: u8 = self.registers.read(REG_CXM0P);

            if m0_pixel >= 0 && p1_pixel >= 0 {
                collision |= 0x80;
            }
            if m0_pixel >= 0 && p0_pixel >= 0 {
                collision |= 0x40;
            }
            self.registers.write(REG_CXM0P, collision);

            collision = self.registers.read(REG_CXM1P);
            if m1_pixel >= 0 && p0_pixel >= 0 {
                collision |= 0x80;
            }
            if m1_pixel >= 0 && p1_pixel >= 0 {
                collision |= 0x40;
            }
            self.registers.write(REG_CXM1P, collision);

            collision = self.registers.read(REG_CXP0FB);
            if p0_pixel >= 0 && playfield_pixel >= 0 {
                collision |= 0x80;
            }
            if p0_pixel >= 0 && ball_pixel >= 0 {
                collision |= 0x40;
            }
            self.registers.write(REG_CXP0FB, collision);

            collision = self.registers.read(REG_CXP1FB);
            if p1_pixel >= 0 && playfield_pixel >= 0 {
                collision |= 0x80;
            }
            if p1_pixel >= 0 && ball_pixel >= 0 {
                collision |= 0x40;
            }
            self.registers.write(REG_CXP1FB, collision);
            
            collision = self.registers.read(REG_CXM0FB);
            if m0_pixel >= 0 && playfield_pixel >= 0 {
                collision |= 0x80;
            }
            if m0_pixel >= 0 && ball_pixel >= 0 {
                collision |= 0x40;
            }
            self.registers.write(REG_CXM0FB, collision);

            collision = self.registers.read(REG_CXM1FB);
            if m1_pixel >= 0 && playfield_pixel >= 0 {
                collision |= 0x80;
            }
            if m1_pixel >= 0 && ball_pixel >= 0 {
                collision |= 0x40;
            }
            self.registers.write(REG_CXM1FB, collision);
            
            collision = self.registers.read(REG_CXBLPF);
            if ball_pixel >= 0 && playfield_pixel >= 0 {
                collision |= 0x80;
            }
            self.registers.write(REG_CXBLPF, collision);

            collision = self.registers.read(REG_CXPPMM);
            if p0_pixel >= 0 && p1_pixel >= 0 {
                collision |= 0x80;
            }
            if m0_pixel >= 0 && m1_pixel >= 0 {
                collision |= 0x40;
            }
            self.registers.write(REG_CXPPMM, collision);
        }

        pub fn is_cpu_blocked(&self) -> bool {
            let mut result: bool = false;

            if self.w_sync_set {
                result = true;
            }

            result
        }

        pub fn get_audio_c0(&self) -> u8 {
            self.registers.read(REG_AUDC0)
        }
        pub fn get_audio_c1(&self) -> u8 {
            self.registers.read(REG_AUDC1)
        }
        pub fn get_audio_f0(&self) -> u8 {
            self.registers.read(REG_AUDF0)
        }
        pub fn get_audio_f1(&self) -> u8 {
            self.registers.read(REG_AUDF1)
        }
        pub fn get_audio_v0(&self) -> u8 {
            self.registers.read(REG_AUDV0)
        }
        pub fn get_audio_v1(&self) -> u8 {
            self.registers.read(REG_AUDV1)
        }

        fn reverse_bits(n: u8) -> u8 {
            let mut input: u8 = n;
            let mut ans: u8 = 0;
            for i in (0..8).rev() {
                ans |= (input & 1) << i;
                input = input >> 1;
            }
            ans
        }

    }
}