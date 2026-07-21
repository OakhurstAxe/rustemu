
pub mod nes {

    use bevy::sprite;
use emucpu::prelude::*;
    use emumemory::prelude::*;

    use crate::nes_console::nes::TICKS_PER_FRAME;
    use crate::nes_cartridge::nes::NesCartridge;
    use crate::nes_cartridge_000::nes::NesCartridge000;
    use crate::nes_palette::nes::NesPalette;

    pub const NTSC_X_RESOLUTION: u32 = 256;
    pub const NTSC_Y_RESOLUTION: u32 = 240;

    const PPU_CONTROL_ADDR: u16 =  0x2000;
    const PPU_MASK_ADDR: u16 =     0x2001;
    const PPU_STATUS_ADDR: u16 =   0x2002;
    const PPU_SCROLL_ADDR: u16 =   0x2005;
    const PPU_ADDR: u16 =          0x2006;
    const PPU_DATA_ADDR: u16 =     0x2007;
    const PPU_OAM_ADDR: u16 =      0x2003;
    const PPU_OAM_DATA_ADDR: u16 = 0x2004;
    const _PPU_OAM_DMA_ADDR: u16 =  0x4014;

    const PPU_ATTRIBUTE_ADDR: u16 = 0x23c0;
    const _PPU_ATTRIBUTE_SIZE: u16 = 0x0040;
    const PPU_NAMETABLE_ADDR: u16 = 0x2000;
    const PPU_NAMETABLE_SIZE: u16 = 0x0400;
    const PPU_PATTERN_SIZE: u16 =   0x1000;
    const PPU_PALETTE_ADDR: u16 =   0x3F00;

    const PPU_SPRITE_SIZE: i32 =         0x0004;
    const PPU_SPRITE_PATTERN_SIZE: u16 = 0x0008;

    #[derive(Default)]
    pub struct PpuControlRegister {
        pub nametable_x: u8,
        pub nametable_y: u8,
        pub increment_mode: u8,
        pub pattern_sprite: u8,
        pub pattern_background: u8,
        pub sprite_size: u8,
        pub slave_mode: u8,
        pub enable_nmi: u8,
    }

    impl PpuControlRegister {

        pub fn new() -> PpuControlRegister {
            Self {
                nametable_x: 0,
                nametable_y: 0,
                increment_mode: 0,
                pattern_sprite: 0,
                pattern_background: 0,
                sprite_size: 0,
                slave_mode: 0,
                enable_nmi: 0,
            }
        }

        pub fn reg(&mut self, byte: u8) {
            self.nametable_x = byte & 0x01;
            self.nametable_y = byte & 0x02;
            self.increment_mode = byte & 0x04;
            self.pattern_sprite = byte & 0x08;
            self.pattern_background = byte & 0x10;
            self.sprite_size = byte & 0x20;
            self.slave_mode = byte & 0x40;
            self.enable_nmi = byte & 0x80;
        }

        pub fn get_nametable_x(&self) -> u8 {
            if self.nametable_x > 0 {
                return 1;
            }
            0
        }

        pub fn get_nametable_y(&self) -> u8 {
            if self.nametable_y > 0 {
                return 1;
            }
            0
        }

        pub fn get_increment_mode(&self) -> u8 {
            if self.increment_mode > 0 {
                return 1;
            }
            0
        }

        pub fn get_pattern_sprite(&self) -> u8 {
            if self.pattern_sprite > 0 {
                return 1;
            }
            0
        }

        pub fn get_pattern_background(&self) -> u8 {
            if self.pattern_background > 0 {
                return 1;
            }
            0
        }

        pub fn get_sprite_size(&self) -> u8 {
            if self.sprite_size > 0 {
                return 1;
            }
            0
        }

        pub fn get_slave_mode(&self) -> u8 {
            if self.slave_mode > 0 {
                return 1;
            }
            0
        }

        pub fn get_enable_nmi(&self) -> u8 {
            if self.enable_nmi > 0 {
                return 1;
            }
            0
        }
    }

    #[derive(Default)]
    pub struct PpuSpriteAttributeRegister {
        pub sprite_palette: u8,
        pub unimplemented: u8,
        pub priority: u8,
        pub flip_horizontally: u8,
        pub flip_vertically: u8,
    }

    impl PpuSpriteAttributeRegister {

        pub fn new() -> PpuSpriteAttributeRegister {
            Self {
                sprite_palette: 0,
                unimplemented: 0,
                priority: 0,
                flip_horizontally: 0,
                flip_vertically: 0,
            }
        }

        pub fn reg(&mut self, byte: u8) {
            self.sprite_palette = byte & 0x03;
            self.unimplemented = byte & 0x1b;
            self.priority = byte & 0x20;
            self.flip_horizontally = byte & 0x40;
            self.flip_vertically = byte & 0x80;
        }

    }

    pub struct NesPpu {
        palette: NesPalette,
        ppu_name_table: MemoryRam,
        ppu_palette: MemoryRam,
        ppu_oam: MemoryRam,
        cpu_ppu_registers: MemoryRam,
        ppu_io_bus: u8,
        ppu_io_bus_ticks: u32,
        control_register: PpuControlRegister,
        scan_line: i32,
        cycle: i32,
        nmi_set: bool,
        render_sprites: [i8; 9],
        screen: Vec<u8>,
        attribute_byte: u8,
        nametable_address: u16,
        pattern_entry_address: u16,
        ppu_x_scroll_write: bool,
        ppu_x_scroll_read: bool,
        ppu_x_scroll: u8,
        ppu_y_scroll: u8,
        ppu_addr: u16,
        ppu_read_buffer: u8,
        ppu_addr_h: u8,
        ppu_addr_l: u8,
        ppu_addr_count: u8,
        ppu_oam_addr: u8,
        pub dma_suspend: u16,
        _debug: u8,
    }

    impl NesPpu {

        pub fn new () -> NesPpu {
            Self {
                palette: NesPalette::new(),
                ppu_name_table: MemoryRam::new(String::from("PPU Name Table RAM"), 0x1f00),
                ppu_palette: MemoryRam::new(String::from("PPU Palette RAM"), 0x0100),
                ppu_oam: MemoryRam::new(String::from("PPU OAM RAM"), 0x0100),
                cpu_ppu_registers: MemoryRam::new(String::from("PPU Registers"), 0x0008),
                ppu_io_bus: 0,
                ppu_io_bus_ticks: 0,
                control_register: PpuControlRegister::new(),
                scan_line: 0,
                nmi_set: false,
                cycle: 0,
                render_sprites: [-1, -1, -1, -1, -1, -1, -1, -1, -1],
                screen: vec!(0; 61440 * 3),
                attribute_byte: 0,
                nametable_address: 0,
                pattern_entry_address: 0,
                ppu_x_scroll_write: true,
                ppu_x_scroll_read: true,
                ppu_x_scroll: 0,
                ppu_y_scroll: 0,
                ppu_addr: 0,
                ppu_read_buffer: 0,
                ppu_addr_h: 0,
                ppu_addr_l: 0,
                ppu_addr_count: 0,
                ppu_oam_addr: 0,
                dma_suspend: 0,
                _debug: 0,
            }
        }
        
        pub fn execute_tick(&mut self, addr: &mut AddressBus, cartridge: &NesCartridge000,
            tick: i32) {

            if tick % 3 == 0 && (0x2000..0x4000).contains(&addr.address) {
                if addr.write {
                    self.ppu_register_write(addr.address, addr.byte);
                    addr.write = false;
                } else {
                    addr.byte = self.ppu_register_read(addr.address, cartridge);
                }
            }

            if self.ppu_io_bus_ticks == 0 {
                self.ppu_io_bus = 0;
            } else {
                self.ppu_io_bus_ticks -= 1;
            }

            self.cycle += 1;
            if self.cycle >= 340 {
                // Set rendering registers for when scrolling happens
                self.cycle = 0;
                self.scan_line += 1;
                if self.scan_line > 261 {
                    self.scan_line = 0;
                }
            }
            
            if self.scan_line > 0 && self.scan_line <= 240 && self.cycle >= 0  && self.cycle <= 256 {
                self.render_pixel(&cartridge);
            }
            
            if self.scan_line == 241 && self.cycle == 1 {                
                self.cpu_set_vblank(true);
                let ppu_control_addr: u8 = self.cpu_ppu_registers.read(PPU_CONTROL_ADDR % 8); 
                self.control_register.reg(ppu_control_addr);
                if self.control_register.get_enable_nmi() > 0 {
                    // store ppu_addr
                    self.nmi_set = true;
                    self.ppu_x_scroll_read = true;
                    self.ppu_x_scroll_write = true;
                }
            }

            if self.scan_line == 0 && self.cycle == 0 {
                self.set_ppu_sprite_zero_hit(false, 0);
                self.cpu_set_vblank(false);
                self.set_ppu_sprite_overflow(false);
            }

            if self.scan_line == 261 && self.cycle == 1 {
                // restore ppu_addr
            }
        }

        fn read(&mut self, mut location: u16, cartridge: &NesCartridge000) -> u8 {

            //  Cartridge PPU ROM
            if location < 0x2000 {
                return cartridge.ppu_read(location);
                //return self.cartridge.read().unwrap().ppu_read(location);
            } else if location < 0x3f00 {
                location -= 0x2000;
                return self.ppu_name_table.read(location);
            } else if location < 0x4000 {
                let is_greyscale: bool = self.cpu_ppu_registers.read(0x0001) & 0x01 != 0;
                location -= 0x3f00;
                let mut result: u8 = self.ppu_palette.read(location % 0x20);
                if location.is_multiple_of(4) {
                    if location >= 0x10 {
                        location -= 0x10;
                    }
                    result = self.ppu_palette.read(location);
                }
                result = (result & 0x3f);// + (self.ppu_io_bus & 0xc0);
                if is_greyscale {
                    result &= 0xf0;
                }

                return result;
            }
            
            eprintln!("Invalid PPU read address: {}", location);
            0
        }

        fn write(&mut self, mut location: u16, byte: u8) {

            // Cartridge PPU ROM
            if location < 0x2000 {
                //self.cartridge.write().unwrap().ppu_write(location, byte);
                return;
            }       

            else if location < 0x3f00 {
                location -= 0x2000;
                self.ppu_name_table.write(location, byte);
                //self.ppu_register_write(location, byte);
                return;
            }
            
            else if location < 0x4000 {
                location -= 0x3f00;
                if location.is_multiple_of(4) {
                    if location >= 0x10 {
                        location -= 0x10;
                    }
                    return self.ppu_palette.write(location, byte);
                }
                location %= 0x20;
                self.ppu_palette.write(location, byte);
                return;
            }
            
            eprintln!("Invalid NES memory location for PPU write {}", location);

        }

        fn ppu_register_read(&mut self, mut location: u16, cartridge: &NesCartridge000) -> u8 {
            // Mirroring, and bring to zero
            location %= 8;
            
            if location == 0x02 {
                let byte: u8 = self.cpu_ppu_registers.read(2);
                self.cpu_ppu_registers.write(2, byte & 0x7f);
                self.ppu_addr_count = 0;
                let latch_byte = byte & 0xe0;
                self.ppu_io_bus &= 0x1f;
                self.ppu_io_bus |= latch_byte;
                self.ppu_io_bus_ticks = TICKS_PER_FRAME as u32 * 30;
                return self.ppu_io_bus;
            } else if location == 0x04 {
                let byte = self.oam_read(self.ppu_oam_addr as u16);
                self.ppu_io_bus = byte;
                self.ppu_io_bus_ticks = TICKS_PER_FRAME as u32 * 30;
                return byte;
            } else if location == 0x07 {
                let mut byte = self.ppu_read_buffer;
                self.ppu_read_buffer = self.read(self.ppu_addr, cartridge);
                // No buffer when reading from PPU ram
                if self.ppu_addr >= 0x3f00 {
                    byte = self.read(self.ppu_addr, cartridge);
                    self.ppu_read_buffer = self.read(self.ppu_addr - 0x1000, cartridge);
                }
                self.ppu_io_bus = byte;
                self.ppu_io_bus_ticks = TICKS_PER_FRAME as u32 * 30;
                let controller: u8 = self.cpu_ppu_registers.read(0x0000);
                if (controller & 0x04) != 0 {
                    self.ppu_addr += 32;
                }
                else {
                    self.ppu_addr += 1;
                }
                return byte;
            }

            self.ppu_io_bus
        }

        fn ppu_register_write(&mut self, mut location: u16, byte: u8) {

            location %= 8;
            self.ppu_io_bus = byte;
            self.ppu_io_bus_ticks = TICKS_PER_FRAME as u32 * 30;

            if location == 0x00 {
                let control_byte = self.cpu_ppu_registers.read(0);
                let status_byte = self.cpu_ppu_registers.read(2);
                self.cpu_ppu_registers.write(0, byte);

                if (control_byte & 0x80) == 0 && (byte & 0x80) != 0  && (status_byte & 0x80) != 0 {
                    self.nmi_set = true;
                }
            } else if location == 0x01 {
                self.cpu_ppu_registers.write(1, byte);
            } else if location == 0x03 {
                self.ppu_oam_addr = byte;
            } else if location == 0x04 {
                self.oam_write(self.ppu_oam_addr as u16, byte);
                self.ppu_oam_addr += 1;
            } else if location == 0x05 {
                self.ppu_scroll_write(byte);
            } else if location == 0x06 {
                self.ppu_addr_count += 1;
                
                if self.ppu_addr_count == 1 {
                    self.ppu_addr_h = byte;
                }

                if self.ppu_addr_count == 2 {
                    self.ppu_addr_l = byte;
                    self.ppu_addr = ((self.ppu_addr_h as u16) << 8) + self.ppu_addr_l as u16;
                    self.ppu_addr_count = 0;
                }
            } else if location == 0x07 { 
                self.write(self.ppu_addr, byte);
                let controller: u8 = self.cpu_ppu_registers.read(0x0000);
                if (controller & 0x04) != 0 {
                    self.ppu_addr += 32;
                }
                else {
                    self.ppu_addr += 1;
                }
            }

        }
        
        pub fn oam_read(&mut self, location: u16) -> u8 {

            if location < 256 {
                return self.ppu_oam.read(location);
            }
            panic!("Invalid OAM memory location for PPU read {}", location);
        
        }

        pub fn oam_write(&mut self, location: u16, byte: u8) {

            if location < 256 {
                self.ppu_oam.write(location, byte);
                return;
            }
            panic!("Invalid OAM memory location for PPU read {}", location);
        
        }

        pub fn set_ppu_sprite_overflow(&mut self, value: bool) {

            let mut byte: u8 = self.cpu_ppu_registers.read(2) & 0xdf;

            if value {
                byte = self.cpu_ppu_registers.read(2) | 0x20;
            }
            
            self.cpu_ppu_registers.write(2, byte);
        }

        pub fn set_ppu_sprite_zero_hit(&mut self, value: bool, sprite_pos: i32) {

            if value == false {
                let byte: u8 = self.cpu_ppu_registers.read(2) & 0xbf;
                self.cpu_ppu_registers.write(2, byte);
            }

            let background_rendered = self.cpu_ppu_registers.read(1) & 0x08 != 0;
            let sprite_rendered = self.cpu_ppu_registers.read(1) & 0x10 != 0;
            let background_mask = self.cpu_ppu_registers.read(1) & 0x02 != 0 || self.cycle >= 8;
            let sprite_mask = self.cpu_ppu_registers.read(1) & 0x04 != 0 || self.cycle >= 8;

            if value && background_rendered && sprite_rendered && self.cycle < 255
                && background_mask && (sprite_mask || sprite_pos > 0) {
                let byte = self.cpu_ppu_registers.read(2) | 0x40;
                self.cpu_ppu_registers.write(2, byte);
            }
        }

        pub fn ppu_scroll_read(&mut self) -> u8 {
            if self.ppu_x_scroll_read {
                self.ppu_x_scroll_read = false;
                self.ppu_x_scroll
            }
            else {
                self.ppu_x_scroll_read = true;
                self.ppu_y_scroll
            }
        }

        pub fn ppu_scroll_write(&mut self, byte: u8) {
            if self.ppu_x_scroll_write {
                self.ppu_x_scroll = byte;
                self.ppu_x_scroll_write = false;
            }
            else {
                self.ppu_y_scroll = byte;
                self.ppu_x_scroll_write = true;
            }
        }

        pub fn cpu_set_vblank(&mut self, value: bool) {

            let mut byte: u8 = self.cpu_ppu_registers.read(2);
            
            if value {
                byte |= 0x80;
            }
            else {
                byte &= 0x7f;
            }
            self.cpu_ppu_registers.write(2, byte);
        }

        pub fn get_sprite_pixel(&mut self, screen_row: u16, screen_column: i16, cartridge: &NesCartridge000) -> (u8, u8, bool, i32) {


            if screen_column == 0 {
                self.render_sprites = [-1, -1, -1, -1, -1, -1, -1, -1, -1];
                let mut sprite_count: i8 = 0;

                for i in 0..64 {
                    let y_pos: u8 = self.oam_read(i * 4);

                    if (screen_row >= y_pos as u16) && (screen_row - y_pos as u16) < 8 {
                        self.render_sprites[sprite_count as usize] = i as i8;
                        sprite_count += 1;

                        if sprite_count == 9 {
                            // sprite overflow
                            self.set_ppu_sprite_overflow(true);
                            break;
                        }
                    }            
                }
            }
            
            let mut priority: u8 = 0;

            let mut sprite_attribute: PpuSpriteAttributeRegister = PpuSpriteAttributeRegister::new();
            for i in 0..=7 {

                let sprite_pos: i32 = self.render_sprites[i] as i32;
                if sprite_pos == -1 {
                    continue;
                }

                let x_pos: u16 = self.oam_read((sprite_pos * PPU_SPRITE_SIZE + 3) as u16) as u16;
                if (screen_column - x_pos as i16) < 0 || screen_column - x_pos as i16 > 7 {
                    continue;
                }                       

                let y_pos: i32 = self.oam_read((sprite_pos * PPU_SPRITE_SIZE) as u16) as i32;

                let pattern_address: u16 = (self.oam_read((sprite_pos * PPU_SPRITE_SIZE + 1) as u16) as u16) << 4;
                let mut sprite_pattern_address: u16 = pattern_address + ((screen_row - y_pos as u16) & 0x07);

                sprite_attribute.reg(self.oam_read((sprite_pos * PPU_SPRITE_SIZE + 2) as u16));
                if sprite_attribute.flip_vertically > 0 {// flip verticle
                    sprite_pattern_address = pattern_address + ((7 - screen_row - y_pos as u16) & 0x07);
                }

                priority = sprite_attribute.priority;

                let mut sprite_lsb = cartridge.ppu_read(sprite_pattern_address);
                let mut sprite_msb = cartridge.ppu_read(sprite_pattern_address + PPU_SPRITE_PATTERN_SIZE);

                if sprite_attribute.flip_horizontally > 0 {
                    sprite_lsb = NesPpu::reverse_bits(sprite_lsb);
                    sprite_msb = NesPpu::reverse_bits(sprite_msb);
                }

                let slide = screen_column - x_pos as i16;
                sprite_lsb <<= slide;
                sprite_msb <<= slide;

                let pixel: u8 = ((sprite_msb >> 6) & 0x02) + ((sprite_lsb >> 7) & 0x01);
                let palette: u8 = ((sprite_attribute.sprite_palette) + 0x04) << 2;

                if pixel != 0{
                    let color: u8 = self.read(PPU_PALETTE_ADDR + palette as u16 + pixel as u16, cartridge);
                    return (color, priority, self.render_sprites[i] == 0, sprite_pos);
                }
            }

            (0, priority, false, 0)
        }

        fn render_pixel(&mut self, cartridge: &NesCartridge000) {
            let (sprite_pixel, sprite_priority, is_sprite_zero, sprite_pos): (u8, u8, bool, i32) = self.get_sprite_pixel(self.scan_line as u16, self.cycle as i16, &cartridge);
            let background_pixel: u8 = self.get_background_pixel(self.scan_line as u16, self.cycle as i16, &cartridge);
            let backdrop: u8 = self.read(PPU_PALETTE_ADDR, cartridge);

            if self.cycle >= 0  && self.cycle < 256 && self.scan_line >= 0 && self.scan_line < 240 {

                let mut color = backdrop;
                if background_pixel != 0 {
                    color = background_pixel;
                }
                
                if sprite_pixel != 0 && (sprite_priority == 0 || background_pixel == 0) {
                    color = sprite_pixel;
                }

                if sprite_pixel != 0 && background_pixel != 0 && is_sprite_zero
                    && self.scan_line < 239 {
                    self.set_ppu_sprite_zero_hit(true, sprite_pos);
                }

                let (red, green, blue) = self.palette.get_color(color as usize, 0);

                self.screen[((self.scan_line * 256 + self.cycle) * 3) as usize] = red;
                self.screen[(((self.scan_line * 256 + self.cycle) * 3) + 1) as usize] = green;
                self.screen[(((self.scan_line * 256 + self.cycle) * 3) + 2) as usize] = blue;
            }
        }

        fn get_bg_attribute_bytes(&mut self, mut screen_row: u16, mut screen_column: i16, cartridge: &NesCartridge000) {

            if screen_column % 8 == 0 {
                let ppu_control_addr: u8 = self.cpu_ppu_registers.read(PPU_CONTROL_ADDR % 8); 
                self.control_register.reg(ppu_control_addr);

                let mut nametable_x: u8 = self.control_register.get_nametable_x();
                let mut nametable_y: u8 = self.control_register.get_nametable_y();

                if screen_column >= 256 {
                    screen_column -= 256;
                    
                    if nametable_x == 0 {
                        nametable_x = 1;
                    }
                    else
                    {
                        nametable_x = 0;
                    }
                }

                if screen_row >= 240 {
                    screen_row -= 240;

                    if nametable_y == 0 {
                        nametable_y = 1;
                    }
                    else {
                        nametable_y = 0;
                    }
                }

                // attribute byte
                let attribute_table_address: u16 = PPU_ATTRIBUTE_ADDR + (nametable_x * PPU_NAMETABLE_SIZE as u8) as u16 + 
                    (nametable_y * PPU_NAMETABLE_SIZE as u8 * 2) as u16;
                let attribute_address: u16 = ((screen_row / 32) * 8 + (screen_column / 32) as u16) + attribute_table_address;
                self.attribute_byte = self.read(attribute_address, cartridge);

                // nametable byte
                let tile_row: u16 = screen_row / 8;
                let tile_column: u16 = screen_column as u16/ 8;
                let nametable_table_address: u16 = PPU_NAMETABLE_ADDR + (nametable_x * PPU_NAMETABLE_SIZE as u8) as u16 + 
                    (nametable_y * PPU_NAMETABLE_SIZE as u8 * 2) as u16;
                self.nametable_address = (((tile_row) * 32) + (tile_column)) + nametable_table_address;

                // pattern table byte
                let addr: u16 = self.read(self.nametable_address, cartridge) as u16;
                let addr2 = addr << 4;
                let pattering: u16 = self.control_register.get_pattern_background() as u16;
                self.pattern_entry_address = (addr2  + (screen_row % 8)) + (PPU_PATTERN_SIZE * pattering);
            }
        }
        
        fn get_background_pixel(&mut self, mut screen_row: u16, mut screen_column: i16, cartridge: &NesCartridge000) -> u8 {

            self.get_bg_attribute_bytes(screen_row, screen_column, cartridge);

            let mut attribute_shift: u8 = 0;
            let x_scroll: u8 = self.ppu_x_scroll;
            let y_scroll: u8 = self.ppu_y_scroll;
            screen_column += x_scroll as i16;
            screen_row += y_scroll as u16;
            
            if ((screen_row % 32) < 16) && (screen_column % 32) < 16 {
                attribute_shift = 0;
            }
            else if ((screen_row % 32) < 16) && (screen_column % 32) >= -16 {
                attribute_shift = 2;
            }
            else if ((screen_row % 32) >= 16) && (screen_column % 32) < 16 {
                attribute_shift = 4;
            }
            else if ((screen_row % 32) >= 16) && (screen_column % 32) >= 16 {
                attribute_shift = 6;
            }

            let attribute_value = ((self.attribute_byte  >> attribute_shift) & 0x03) as u16;

            let _char1 = cartridge.ppu_read(self.pattern_entry_address);// self.read(self.pattern_entry_address);
            let _char2 = cartridge.ppu_read(self.pattern_entry_address + 0x08);//self.read(self.pattern_entry_address + 0x08);
            let char_table_entry_lsb = cartridge.ppu_read(self.pattern_entry_address) << (screen_column % 8);
            let char_table_entry_msb = cartridge.ppu_read(self.pattern_entry_address + 0x08) << (screen_column % 8);
            let pixel: u16 =  (((char_table_entry_msb & 0x80) >> 6) + ((char_table_entry_lsb & 0x80) >> 7)) as u16;
            let pixel_address: u16 = PPU_PALETTE_ADDR + pixel + (attribute_value << 2);
            let color: u8 = self.read(pixel_address, cartridge);

            color
        }

        fn reverse_bits(mut n: u8) -> u8 {
            let mut ans: u8 = 0;

            for i in (0..=7).rev() {
                ans |= (n & 1) <<i;
                n >>= 1;
            }

            ans
        }
        
        pub fn is_nmi_set(&self) -> bool {
            self.nmi_set
        }
        
        pub fn reset_nmi(&mut self) {
            self.nmi_set = false;
        }

        pub fn reset(&mut self) {
            self.write(PPU_CONTROL_ADDR, 0x80);
            self.write(PPU_MASK_ADDR, 0);
            self.write(PPU_STATUS_ADDR, 0);
            self.write(PPU_OAM_ADDR, 0);
            self.write(PPU_OAM_DATA_ADDR, 0);
            self.write(PPU_SCROLL_ADDR, 0);
            self.write(PPU_ADDR, 0);
            self.write(PPU_DATA_ADDR, 0);
            self.scan_line = -1;
            self.cycle = 0;
            self.nmi_set = false;
        }

        pub fn get_screen(&self) -> Vec<u8> {
            self.screen.clone()
        }

    }

}