
pub mod vcs {

    use emumemory::prelude::*;
    use emucpu::prelude::*;

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

    const SPRITEOFFSET: u16 =  4;

    pub struct TiaAudio {
        pub v0: u8,
        pub f0: u8,
        pub c0: u8,
        pub v1: u8,
        pub f1: u8,
        pub c1: u8,
    }

    pub struct VcsTia {
        registers: MemoryRam,
        cycle: u16,
        scan_line: u16,
        screen: Vec<u8>,
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
        x_resolution: u32,
        y_resolution: u32,
        _debug: u8
    }

    impl VcsTia {

        pub fn new(console_type: &VcsConsoleType) -> Self {
            let v_blank: u8 = console_type.get_v_blank_lines();
            let x_resolution: u32 = console_type.get_x_resolution();
            let y_resolution: u32 = console_type.get_y_resolution();
            Self {
                registers: MemoryRam::new(String::from("TIA Registers"), 0x7f),
                cycle: 0,
                scan_line: 0,
                screen: vec![0u8; (x_resolution * y_resolution * 3) as usize],
                vcs_palette: VcsPalette::new(console_type),
                w_sync_set: false,
                res_p0_cycle: 0,
                res_p1_cycle: 0,
                res_m0_cycle: 0,
                res_m1_cycle: 0,
                res_bl_cycle: 0,
                grp_0_delay: 0,
                grp_1_delay: 0,
                enable_delay: 0,
                v_blank,
                x_resolution,
                y_resolution,
                _debug: 0
            }
        }

        pub fn get_screen(&self) -> Vec<u8> {
            self.screen.clone()
        }

        pub fn is_cpu_blocked(&self) -> bool {
            self.w_sync_set
        }

        pub fn get_tia_audio(&mut self) -> TiaAudio {
            TiaAudio { 
                v0: self.registers.read(REG_AUDV0),
                f0: self.registers.read(REG_AUDF0),
                c0: self.registers.read(REG_AUDC0),
                v1: self.registers.read(REG_AUDV1),
                f1: self.registers.read(REG_AUDF1),
                c1: self.registers.read(REG_AUDC1)}
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

        pub fn execute_tick(&mut self, addr: &mut AddressBus) {

            self.execute_addr(addr);

            // Move to next pixel
            self.cycle += 1;
            if self.cycle > 67 + self.x_resolution as u16 {
                self.cycle = 0;
                self.scan_line += 1;
            }

            // If on screen, render pixel
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
                } else if size & 0x07 == 7 { // size 4
                    self.res_m0_cycle += 10;
                } else { // size 1
                    self.res_m0_cycle += 3;
                }
            }

            if self.registers.read(REG_RESMP1) & 0x02 > 0 {
                let size: u8 = self.registers.read(REG_NUSIZ1);
                self.res_m1_cycle = self.res_p1_cycle;
                
                if size & 0x07 == 5 { // size 2
                    self.res_m1_cycle += 6;
                } else if size & 0x07 == 7 { // size 4
                    self.res_m1_cycle += 10;
                } else { // size 1
                    self.res_m1_cycle += 3;
                }
            }
            
            // WSYNC 
            if self.cycle == 3 { // Not sure this should be 8, but works pretty good
                self.w_sync_set = false;
            }
        }

        fn execute_addr(&mut self, addr: &mut AddressBus) {

            let mut location = addr.address & 0x1FFF;

            if addr.write {
                if location & 0x1080 == 0 {
                    location &= 0xFF;
                    if location >= 0x40
                    {
                        location -= 0x40;
                    }
                    self.write(location, addr.byte);
                    addr.write = false;
                }
            } else {
                if location & 0x1080 == 0 {
                    location &= 0x0F;
                    location += 0x30;
                    addr.byte = self.read(location);
                }
            }
        }

        fn read(&mut self, location: u16) -> u8 {
            
            if (0x30..=0x3D).contains(&location) == false {
                // Undefined TIA read returns address 0x30
                return self.registers.read(0x30);
            }

            self.registers.read(location)
        }

        fn write(&mut self, location: u16, byte: u8) {

            // Undefined write, does nothing.
            // Sometimes used to waste specific cycle count.
            if location > 0x2C {
                return; 
            }

            match location {
                REG_GRP0 => {
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

                    if (self.registers.read(REG_VDELBL) & 0x01) > 0 {
                        self.registers.write(REG_ENABL, self.enable_delay);
                        self.enable_delay = 0;
                    }
                },
                REG_GRP1 => {
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
                },
                REG_ENABL => {
                    if (self.registers.read(REG_VDELBL) & 0x01) > 0 {
                        self.enable_delay = byte;
                    }
                    else {
                        self.registers.write(REG_ENABL, byte);
                    }
                },
                REG_VSYNC => {
                    if (byte & 0x02 == 0) && (self.registers.read(REG_VSYNC) & 0x02) > 0 {
                        self.scan_line = 2;
                        let byte = self.registers.read(REG_VBLANK) | 0x02;
                        self.registers.write(REG_VBLANK, byte);
                    }

                    self.registers.write(REG_VSYNC, byte);
                },
                REG_VBLANK => {
                    // the scanLine_ > 30 is a hack that seems to work,  Probably a more accurate way to do it
                    if (byte & 0x02) == 0 && (self.registers.read(REG_VBLANK) & 0x02 > 0) && self.scan_line > 30 {
                        self.scan_line = 2 + self.v_blank as u16;
                    }
                    self.registers.write(REG_VBLANK, byte);
                },
                REG_WSYNC => {
                    self.w_sync_set = true;
                },
                REG_RSYNC => {
                    self.cycle = 0;
                },
                REG_RESP0 => {
                    self.res_p0_cycle = self.cycle + SPRITEOFFSET;
                    if self.res_p0_cycle < 68 {
                        self.res_p0_cycle = 71;
                    }
                },
                REG_RESP1 => {
                    self.res_p1_cycle = self.cycle + SPRITEOFFSET;

                    if self.res_p1_cycle < 68 {
                        self.res_p1_cycle = 71;
                    }
                },
                REG_RESM0 => {
                    self.res_m0_cycle = self.cycle + SPRITEOFFSET - 1;

                    if self.res_m0_cycle < 68 {
                        self.res_m0_cycle = 71;
                    }
                },
                REG_RESM1 => {
                    self.res_m1_cycle = self.cycle + SPRITEOFFSET - 1;

                    if self.res_m1_cycle < 68 {
                        self.res_m1_cycle = 71;
                    }
                },
                REG_RESBL => {
                    self.res_bl_cycle = self.cycle + SPRITEOFFSET;

                    if self.res_bl_cycle < 68 {
                        self.res_bl_cycle = 71;
                    }
                },
                REG_HMOVE => {
                    self.apply_movement();
                },
                REG_HMCLR => {
                    self.clear_move_registers();
                },
                REG_CXCLR => {
                    self.registers.write(REG_CXM0P, 0);
                    self.registers.write(REG_CXM1P, 0);
                    self.registers.write(REG_CXP0FB, 0);
                    self.registers.write(REG_CXP1FB, 0);
                    self.registers.write(REG_CXM0FB, 0);
                    self.registers.write(REG_CXM1FB, 0);
                    self.registers.write(REG_CXBLPF, 0);
                    self.registers.write(REG_CXPPMM, 0);
                },
                _ => {
                    self.registers.write(location, byte);
                }
            }
        }

        fn move_object(&mut self, mov: u8, object_cycle: u16) -> u16 {
            let mut move_value: i8 = ((mov & 0x70) >> 4) as i8;
            if mov & 0x80 > 0 {
                // twos compliment
                move_value = (move_value as u8 | 0xf8) as i8;
            }
            let mut new_value = object_cycle.overflowing_sub(move_value as u16).0;

            if new_value > 68 + self.x_resolution as u16 {
                new_value = 68;
            }
            if new_value < 68 {
                new_value = 68 + self.x_resolution as u16;
            }
            new_value
        }

        fn apply_movement(&mut self) {
            let mut mov = self.registers.read(REG_HMP0);
            self.res_p0_cycle = self.move_object(mov, self.res_p0_cycle);
            mov = self.registers.read(REG_HMP1);
            self.res_p1_cycle = self.move_object(mov, self.res_p1_cycle);
            mov = self.registers.read(REG_HMM0);
            self.res_m0_cycle = self.move_object(mov, self.res_m0_cycle);
            mov = self.registers.read(REG_HMM1);
            self.res_m1_cycle = self.move_object(mov, self.res_m1_cycle);
            mov = self.registers.read(REG_HMBL);
            self.res_bl_cycle = self.move_object(mov, self.res_bl_cycle);
        }
        
        fn clear_move_registers(&mut self) {
            self.registers.write(REG_HMP0, 0);
            self.registers.write(REG_HMP1, 0);
            self.registers.write(REG_HMM0, 0);
            self.registers.write(REG_HMM1, 0);
            self.registers.write(REG_HMBL, 0);
        }

        fn get_player_pixel(&self, graphics_player: u8, player_size: u8, reflect_player: u8, 
            color: u8, player_cycle: u16) -> i16 {

            if graphics_player == 0 {
                return -1;
            }

            let mut result: i16 = -1;            
            let mut sprite_data: u8 = graphics_player;

            if (reflect_player & 0x08) == 0 {
                sprite_data = VcsTia::reverse_bits(sprite_data);
            }
            
            let mut position2_cycle: u16 = player_cycle;
            let mut position3_cycle: u16 = player_cycle;
            let mut size_multiple: u8 = 1;

            match player_size & 0x07 {
                0 => { size_multiple = 1; },
                1 => { position2_cycle = player_cycle + CLOSE; },
                2 => { position2_cycle = player_cycle + MEDIUM; },
                3 => {
                    position2_cycle = player_cycle + CLOSE;
                    position3_cycle = position2_cycle + CLOSE;
                },
                4 => { position2_cycle = player_cycle + WIDE; },
                5 => { size_multiple = 2; },
                6 => {
                    position2_cycle = player_cycle + MEDIUM;
                    position3_cycle = position2_cycle + MEDIUM;
                },
                7 => { size_multiple = 4; },
                _ => {}
            }

            let value = self.cycle.overflowing_sub(player_cycle).0;
            let mut shift: u32 = (value/ size_multiple as u16) as u32;
            if shift < 8 && (((sprite_data >> shift) & 0x01) > 0) {
                result = color as i16;
                return result;
            }
            let value2 = self.cycle.overflowing_sub(position2_cycle).0;
            shift = (value2/ size_multiple as u16) as u32;
            if shift < 8 && (((sprite_data >> shift) & 0x01) > 0) {
                result = color as i16;
                return result;
            }
            let value3 = self.cycle.overflowing_sub(position3_cycle).0;
            shift = (value3/ size_multiple as u16) as u32;
            if shift < 8 && (((sprite_data >> shift) & 0x01) > 0) {
                result = color as i16;
                return result;
            }                    

            result
        }

        fn get_playfield_pixel(&mut self) -> i16 {
            let screen_x: u16 = self.cycle - 68;
            let control_playfield: u8 = self.registers.read(REG_CTRLPF);
            let mut playfield_color: u8 = self.registers.read(REG_COLUPF);
            let mut byte: u8;
            let mut result: i16 = -1;

            if (control_playfield & 0x02) > 0 {
                if screen_x < 80 {
                    playfield_color = self.registers.read(REG_COLUP0);
                } else {
                    playfield_color = self.registers.read(REG_COLUP1);
                }
            }

            match (screen_x, control_playfield & 0x01 > 0) {
                (..16, _) => {
                    byte = (self.registers.read(REG_PF0) >> 4) & 0x0f;
                    byte = (byte >> (screen_x >> 2)) & 0x01;
                },
                (16..48, _) => {
                    byte = self.registers.read(REG_PF1);
                    byte = VcsTia::reverse_bits(byte);
                    byte = (byte >> ((screen_x - 16) >> 2)) & 0x01;
                },
                (48..80, _) => {
                    byte = self.registers.read(REG_PF2);
                    byte = (byte >> ((screen_x - 48) >> 2)) & 0x01;
                },
                (80..112, true) => {
                    byte = self.registers.read(REG_PF2);
                    byte = VcsTia::reverse_bits(byte);
                    byte = (byte >> ((screen_x - 80) >> 2)) & 0x01;
                },
                (112..144, true) => {
                    byte = self.registers.read(REG_PF1);
                    byte = (byte >> ((screen_x - 112) >> 2)) & 0x01;
                },
                (144.., true) => {
                    byte = (self.registers.read(REG_PF0) >> 4) & 0x0f;
                    byte = VcsTia::reverse_bits(byte) >> 4;
                    byte = (byte >> ((screen_x - 144) >> 2)) & 0x01;
                },
                (80..96, false) => {
                    byte = (self.registers.read(REG_PF0) >> 4) & 0x0f;
                    byte = (byte >> ((screen_x - 80) >> 2)) & 0x01;
                },
                (96..128, false) => {
                    byte = self.registers.read(REG_PF1);
                    byte = VcsTia::reverse_bits(byte);
                    byte = (byte >> ((screen_x - 96) >> 2)) & 0x01;
                },
                (128.., false) => {
                    byte = self.registers.read(REG_PF2);
                    byte = (byte >> ((screen_x - 128) >> 2)) & 0x01;
                }
            }

            if byte > 0 {
                result = playfield_color as i16;
            }                
            
            result
        }

        fn get_missle_pixel(&mut self, enable: u8, missle_reset: u8, missle_size: u8, 
            missle_color: u8, missle_cycle: u16) -> i16 {
            
            if (enable & 0x02) == 0 || (missle_reset & 0x02) != 0 {
                return -1;
            }

            let mut result: i16 = -1;            
            let mut position_2_cycle: u16 = missle_cycle;
            let mut position_3_cycle: u16 = missle_cycle;
            let mut size: u8 = missle_size;

            match size & 0x07 {
                1 => {
                    position_2_cycle = missle_cycle + CLOSE;
                },
                2 => {
                    position_2_cycle = missle_cycle + MEDIUM;
                },
                3 => {
                    position_2_cycle = missle_cycle + CLOSE;
                    position_3_cycle = missle_cycle + MEDIUM;
                },
                4 => {
                    position_2_cycle = missle_cycle + WIDE;
                },
                6 => {
                    position_2_cycle = missle_cycle + MEDIUM;
                    position_3_cycle = missle_cycle + WIDE;
                },
                _ => {}
            }

            match (size & 0x30) >> 4 {
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

            result
        }

        fn get_ball_pixel(&mut self) -> i16{

            if self.registers.read(REG_ENABL) & 0x02 == 0 {
                return -1;
            }

            let mut result: i16 = -1;
            
            let mut size: u8 = self.registers.read(REG_CTRLPF);
            match (size & 0x30) >> 4 {
                1 => size = 1,
                2 => size = 2,
                4 => size = 4,
                8 => size = 8,
                _ => ()
            }

            if self.res_bl_cycle <= self.cycle && self.res_bl_cycle + size as u16 >= self.cycle {
                result = self.registers.read(REG_COLUPF) as i16;
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
            let pf_above: bool = self.registers.read(REG_CTRLPF) & 0x04 > 0;
            
            // Get each pixel for collision detection
            // Player 0
            let mut graphics_player = self.registers.read(REG_GRP0);
            let mut player_size = self.registers.read(REG_NUSIZ0);
            let mut reflect_player = self.registers.read(REG_REFP0);
            let mut color = self.registers.read(REG_COLUP0);
            let p0_pixel: i16 = self.get_player_pixel(graphics_player, player_size, reflect_player, color, self.res_p0_cycle);
           
            // Player 1
            graphics_player = self.registers.read(REG_GRP1);
            player_size = self.registers.read(REG_NUSIZ1);
            reflect_player = self.registers.read(REG_REFP1);
            color = self.registers.read(REG_COLUP1);
            let p1_pixel: i16 = self.get_player_pixel(graphics_player, player_size, reflect_player, color, self.res_p1_cycle);
           
            // Missle 0
            let mut enable = self.registers.read(REG_ENAM0);
            let mut missle_reset = self.registers.read(REG_RESM0);
            let mut missle_size = self.registers.read(REG_NUSIZ0);
            let mut missle_color = self.registers.read(REG_COLUP0);
            let m0_pixel: i16 = self.get_missle_pixel(enable, missle_reset, missle_size, missle_color, self.res_m0_cycle);
           
            // Missle 1
            enable = self.registers.read(REG_ENAM1);
            missle_reset = self.registers.read(REG_RESM1);
            missle_size = self.registers.read(REG_NUSIZ1);
            missle_color = self.registers.read(REG_COLUP1);
            let m1_pixel: i16 = self.get_missle_pixel(enable, missle_reset, missle_size, missle_color, self.res_m1_cycle);
           
            // Ball
            let ball_pixel: i16 = self.get_ball_pixel();
            
            let current_pixel: usize = (screen_y * self.x_resolution as u16 + screen_x) as usize;

            // P0
            if p0_pixel >= 0 && current_color == -1 {
                current_color = p0_pixel;
            }
            // M0
            if m0_pixel >= 0 && current_color == -1 {
                current_color = m0_pixel;
            }
            
            // P1
            if p1_pixel >= 0 && current_color == -1 {
                current_color = p1_pixel;
            }
            // M1
            if m1_pixel >= 0 && current_color == -1 {
                current_color = m1_pixel;
            }

            // Playfield
            if playfield_pixel >= 0 && current_color == -1 {
                current_color = playfield_pixel;
            }

            // Ball
            if ball_pixel >= 0 && current_color == -1 {
                current_color = ball_pixel;
            }
            // Don't display pixel if PF has priority and is set
            if pf_above {                
                // Playfield
                if playfield_pixel >= 0 {//&& current_color == ball_pixel {
                    current_color = playfield_pixel;
                }
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

        fn check_single_collision(&mut self, aggressor_pixel: i16, first_target_pixel: i16,
            second_target_pixel: i16, register: u16) {
            let mut collision: u8 = self.registers.read(register);
            if aggressor_pixel >= 0 && first_target_pixel >= 0 {
                collision |= 0x80;
            }
            if aggressor_pixel >= 0 && second_target_pixel >= 0 {
                collision |= 0x40;
            }
            self.registers.write(register, collision);
        }

        fn check_collisions(&mut self, playfield_pixel: i16, p0_pixel: i16, p1_pixel: i16, 
            m0_pixel: i16, m1_pixel: i16, ball_pixel: i16) {

            // Collisions
            self.check_single_collision(m0_pixel, p1_pixel, p0_pixel, REG_CXM0P);
            self.check_single_collision(m1_pixel, p0_pixel, p1_pixel, REG_CXM1P);
            self.check_single_collision(p0_pixel, playfield_pixel, ball_pixel, REG_CXP0FB);
            self.check_single_collision(p1_pixel, playfield_pixel, ball_pixel, REG_CXP1FB);
            self.check_single_collision(m0_pixel, playfield_pixel, ball_pixel, REG_CXM0FB);
            self.check_single_collision(m1_pixel, playfield_pixel, ball_pixel, REG_CXM1FB);
            self.check_single_collision(ball_pixel, playfield_pixel, -1, REG_CXBLPF);
            self.check_single_collision(p0_pixel, p1_pixel, -1, REG_CXPPMM);
            self.check_single_collision(m0_pixel, -1, m1_pixel, REG_CXPPMM);
        }

        fn reverse_bits(n: u8) -> u8 {
            let mut input: u8 = n;
            let mut ans: u8 = 0;
            for i in (0..8).rev() {
                ans |= (input & 1) << i;
                input >>= 1;
            }
            ans
        }

    }
}