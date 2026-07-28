pub mod nes {

    use emucpu::prelude::*;
    use emumemory::prelude::*;

    use crate::nes_console::nes::TICKS_PER_FRAME;
    use crate::nes_cartridge::nes::NesCartridge;
    use crate::nes_cartridge_000::nes::NesCartridge000;
    use crate::nes_palette::nes::NesPalette;
    use crate::nes_ppu::nes::NesPpu;

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
    pub struct VideoBus {
        pub byte: u8,
        pub timer: u32,
    }

    impl VideoBus {
        fn new() -> VideoBus {
            Self {
                byte: 0,
                timer: 0,
            }
        }

        fn set_byte(&mut self, byte: u8) {
            self.byte = byte;
            self.timer = TICKS_PER_FRAME as u32 * 30;
        }

        fn execute_tick(&mut self) {
            if self.timer > 0 {
                self.timer -= 1;
            }

            if self.timer == 0 {
                self.byte = 0;
            }
        }
    }

    #[derive(Default)]
    pub struct PpuControlRegister {
        byte: u8,
    }

    impl PpuControlRegister {
        pub fn new(byte: u8) -> PpuControlRegister {
            Self {
                byte,
            }
        }

        pub fn base_nametable_address(&self) -> u16 {
            match self.byte & 0x03 {
                0 => 0x2000,
                1 => 0x2400,
                2 => 0x2800,
                3 => 0x2C00,
                _ => 0x2000,
            }
        }

        pub fn base_attributetable_address(&self) -> u16 {
            match self.byte & 0x03 {
                0 => 0x23C0,
                1 => 0x27C0,
                2 => 0x2BC0,
                3 => 0x2FC0,
                _ => 0x23C0,
            }
        }

        pub fn vram_address_increment(&self) -> u16 {
            match self.byte & 0x04 {
                0 => 1,
                _ => 32,
            }
        }

        pub fn sprite_pattern_table_address(&self) -> u16 {
            match self.byte & 0x08 {
                0 => 0x0000,
                _ => 0x1000,
            }
        }

        pub fn background_pattern_table_address(&self) -> u16 {
            match self.byte & 0x10 {
                0 => 0x0000,
                _ => 0x1000,
            }
        }

        pub fn sprite_size(&self) -> u8 {
            match self.byte & 0x20 {
                0 => 8,
                _ => 16,
            }
        }

        pub fn ppu_master_slave_select(&self) -> u8 {
            match self.byte & 0x40 {
                0 => 0,
                _ => 1,
            }
        }

        pub fn vblank_nmi_enable(&self) -> bool {
            match self.byte & 0x80 {
                0 => false,
                _ => true,
            }
        }
    }

    #[derive(Default)]
    pub struct PpuMaskRegister {
        byte: u8,
    }

    impl PpuMaskRegister {
        pub fn new(byte: u8) -> PpuMaskRegister {
            Self {
                byte,
            }
        }

        pub fn is_greyscale(&self) -> bool {
            self.byte & 0x01 != 0
        }

        pub fn show_background_leftmost_8_pixels(&self) -> bool {
            self.byte & 0x02 != 0
        }

        pub fn show_sprites_leftmost_8_pixels(&self) -> bool {
            self.byte & 0x04 != 0
        }

        pub fn show_background(&self) -> bool {
            self.byte & 0x08 != 0
        }

        pub fn show_sprites(&self) -> bool {
            self.byte & 0x10 != 0
        }

        pub fn emphasize_red(&self) -> bool {
            self.byte & 0x20 != 0
        }

        pub fn emphasize_green(&self) -> bool {
            self.byte & 0x40 != 0
        }
        
        pub fn emphasize_blue(&self) -> bool {
            self.byte & 0x80 != 0
        }
    }

    #[derive(Default)]
    pub struct PpuStatusRegister {
        byte: u8,
    }

    impl PpuStatusRegister {
        pub fn new(byte: u8) -> PpuStatusRegister {
            Self {
                byte,
            }
        }

        pub fn sprite_overflow_flag(&self) -> bool {
            match self.byte & 0x20 {
                0 => false,
                1 => true,
                _ => false,
            }
        }

        pub fn sprite_zero_hit_flag(&self) -> bool {
            match self.byte & 0x40 {
                0 => false,
                1 => true,
                _ => false,
            }
        }

        pub fn vblank_flag(&self) -> bool {
            match self.byte & 0x40 {
                0 => false,
                1 => true,
                _ => false,
            }
        }
    }
    

    #[derive(Default, Copy, Clone)]
    pub struct Sprite {
        pub sprite_id: i8,
        pub y_pos: u8,
        pub x_pos: u8,
        pub tile: u8,
        pub attribute: u8,
    }

    #[derive(Default)]
    pub struct SpriteAttribute {
        byte: u8,
    }

    impl SpriteAttribute {
        pub fn new(byte: u8) -> SpriteAttribute {
            Self {
                byte,
            }
        }

        pub fn get_palette(&self)-> u8 { self.byte & 0x03 }
        pub fn get_priority(&self)-> u8 { (self.byte & 0x20) >> 6 }
        pub fn get_flip_horizontal(&self)-> bool { self.byte & 0x40 != 0 }
        pub fn get_flip_verticle(&self)-> bool { self.byte & 0x80 != 0 }
    }

    pub struct NesPpu2 {
        pub palette: NesPalette,
        pub video_bus: VideoBus,
        pub registers: MemoryRam,
        pub oam: MemoryRam,
        pub name_table: MemoryRam,
        pub ppu_palette: MemoryRam,
        pub nmi_set: bool,
        pub ppu_addr_first: bool,
        pub ppu_scroll_x: u8,
        pub ppu_scroll_y: u8,
        pub ppu_addr: u16,
        pub screen: Vec<u8>,
        pub cycle: i32,
        pub scan_line: i32,
        pub nametable_hold_byte: u8,
        pub attribute_hold_byte: u8,
        pub pattern_low_hold_byte: u8,
        pub pattern_high_hold_byte: u8,
        pub nametable_byte: u8,
        pub attribute_byte: u8,
        pub pattern_low_byte: u8,
        pub pattern_high_byte: u8,
        pub sprites: [Sprite; 8],
    }

    impl NesPpu2 {

        pub fn new () -> NesPpu2 {
            Self {
                palette: NesPalette::new(),
                video_bus: VideoBus::new(),
                registers: MemoryRam::new(String::from("PPU Registers"), 0x0008),
                oam: MemoryRam::new(String::from("PPU OAM"), 0x0100),
                name_table: MemoryRam::new(String::from("PPU Name Table"), 0x1000),
                ppu_palette: MemoryRam::new(String::from("PPU Palette"), 0x0020),
                nmi_set: false,
                ppu_addr_first: true,
                ppu_scroll_x: 0,
                ppu_scroll_y: 0,
                ppu_addr: 0,
                screen: vec!(0; 61440 * 3),
                cycle: 0,
                scan_line: 0,
                nametable_hold_byte: 0,
                attribute_hold_byte: 0,
                pattern_low_hold_byte: 0,
                pattern_high_hold_byte: 0,
                nametable_byte: 0,
                attribute_byte: 0,
                pattern_low_byte: 0,
                pattern_high_byte: 0,
                sprites: [Sprite::default(); 8],
            }
        }
    }

    pub struct NesPpu2Runner {
        ppu: NesPpu2,
    }

    impl NesPpu2Runner {
    
        pub fn new () -> NesPpu2Runner {
            Self {
                ppu: NesPpu2::new(),
            }
        }

        pub fn execute_memory(&mut self, addr: &mut AddressBus, cartridge: &NesCartridge000) {

            if (0x2000..0x4000).contains(&addr.address) {
                if addr.write {
                    self.ppu_register_write(addr.address, addr.byte);
                    addr.write = false;
                } else {
                    addr.byte = self.ppu_register_read(addr.address, cartridge);
                }
            }                
        }

        fn ppu_register_write(&mut self, mut location: u16, byte: u8) {

            location %= 8;
            self.ppu.video_bus.set_byte(byte);

            match location {
                0x00 => {
                    self.ppu.registers.write(0, byte);
                    let control_register = PpuControlRegister::new(self.ppu.registers.read(0));
                    let status_register = PpuStatusRegister::new(self.ppu.registers.read(2));
                    if byte & 0x80 != 0 && control_register.vblank_nmi_enable() && status_register.vblank_flag() {
                        self.ppu.nmi_set = true;
                    }
                },
                0x01 => self.ppu.registers.write(1, byte),
                0x03 => self.ppu.registers.write(3, byte),
                0x04 => {
                    self.ppu.registers.write(4, byte);
                    let oam_address = self.ppu.registers.read(3);
                    self.oam_write(oam_address, byte);
                    self.ppu.registers.write(3, (oam_address.wrapping_add(1)));
                },
                0x05 => {
                    if self.ppu.ppu_addr_first {
                        self.ppu.ppu_scroll_x = byte;
                        self.ppu.ppu_addr_first = false;
                    } else {
                        self.ppu.ppu_scroll_y = byte;
                        self.ppu.ppu_addr_first = true;
                    }
                },
                0x06 => {
                    if self.ppu.ppu_addr_first {
                        self.ppu.ppu_addr = ((byte & 0x3F) as u16) << 8;
                        self.ppu.ppu_addr_first = false;
                    } else {
                        self.ppu.ppu_addr |= byte as u16;
                        self.ppu.ppu_addr_first = true;
                    }
                },
                0x07 => {
                    self.write(self.ppu.ppu_addr, byte);
                    let control_register = PpuControlRegister::new(self.ppu.registers.read(0));
                    self.ppu.ppu_addr = self.ppu.ppu_addr.wrapping_add(control_register.vram_address_increment());
                },
                _ => {}
            }
        }

        fn ppu_register_read(&mut self, mut location: u16, cartridge: &NesCartridge000) -> u8 {
            // Mirroring, and bring to zero
            location %= 8;
            
            match location {
                0x00 => {
                    let byte = self.ppu.registers.read(2);
                    self.ppu.video_bus.set_byte(byte);
                    byte
                },
                0x02 => {
                    let byte = self.ppu.registers.read(2);
                    // Clear the vblank flag
                    self.ppu.registers.write(2, byte & 0x7f);
                    self.ppu.video_bus.set_byte(byte);
                    byte
                },
                0x04 => {
                    let oam_address = self.ppu.registers.read(3);
                    let byte = self.oam_read(oam_address);
                    self.ppu.video_bus.set_byte(byte);
                    byte
                },
                0x07 => {
                // No buffer when reading from PPU ram
                    if self.ppu.ppu_addr >= 0x3f00 {
                        let byte = self.read(self.ppu.ppu_addr, cartridge);
                        let ppu_byte = self.read(self.ppu.ppu_addr - 0x1000, cartridge);
                        self.ppu.video_bus.set_byte(ppu_byte);
                        return byte;
                    }
                    let byte = self.ppu.video_bus.byte;
                    let control_register = PpuControlRegister::new(self.ppu.registers.read(0));
                    self.ppu.ppu_addr = self.ppu.ppu_addr.wrapping_add(control_register.vram_address_increment());
                    byte
                },
                _ => {0}
            }

        }

        pub fn oam_read(&mut self, location: u8) -> u8 {
            self.ppu.oam.read(location as u16)
        }

        pub fn oam_write(&mut self, location: u8, byte: u8) {
            self.ppu.oam.write(location as u16, byte);
        }

        fn read(&mut self, mut location: u16, cartridge: &NesCartridge000) -> u8 {

            //  Cartridge PPU ROM
            match location {
                0x0000..=0x1FFF => {
                    return cartridge.ppu_read(location);
                },
                0x2000..=0x3EFF => {
                    location -= 0x2000;
                    return self.ppu.name_table.read(location);
                },
                0x3F00..=0x3FFF => {
                    let mask_register = PpuMaskRegister::new(self.ppu.registers.read(1));
                    location -= 0x3F00;
                    
                    let mut result: u8 = self.ppu.ppu_palette.read(location % 0x20);
                    if location.is_multiple_of(4) {
                        if location >= 0x10 {
                            location -= 0x10;
                        }
                        result = self.ppu.ppu_palette.read(location);
                    }
                    result = result & 0x3f;

                    if mask_register.is_greyscale() {
                        result &= 0xf0;
                    }
                    return result;
                },
                _ => {
                    eprintln!("Invalid PPU read address: {}", location);
                }
            }
            0
        }

        fn write(&mut self, mut location: u16, byte: u8) {

            match location {
                0x0000..=0x1FFF => {
                    // Cartridge PPU ROM
                    return;
                },
                0x2000..=0x3EFF => {
                    location -= 0x2000;
                    self.ppu.name_table.write(location, byte);
                    return;
                },
                0x3F00..=0x3FFF => {
                    location -= 0x3F00;
                    if location.is_multiple_of(4) {
                        if location >= 0x10 {
                            location -= 0x10;
                        }
                        return self.ppu.ppu_palette.write(location, byte);
                    }
                    location %= 0x20;
                    self.ppu.ppu_palette.write(location, byte);
                    return;
                },
                _ => {}
            }
            eprintln!("Invalid NES memory location for PPU write {}", location);
        }

        pub fn execute_tick(&mut self, addr: &mut AddressBus, cartridge: &NesCartridge000) {

            self.ppu.video_bus.execute_tick();

            self.ppu.cycle += 1;
            if self.ppu.cycle >= 340 {
                // Set rendering registers for when scrolling happens
                self.ppu.cycle = 0;
                self.ppu.scan_line += 1;
                if self.ppu.scan_line > 261 {
                    self.ppu.scan_line = 0;
                }
            }

            if self.ppu.scan_line > 0 && self.ppu.scan_line <= 240 && self.ppu.cycle >= 0  && self.ppu.cycle <= 256 {
                self.render_pixel(&cartridge);
            }
            
            if self.ppu.scan_line == 241 && self.ppu.cycle == 1 {
                self.cpu_set_vblank(true);
                let control_register = PpuControlRegister::new(self.ppu.registers.read(0));
                if control_register.vblank_nmi_enable() {
                    // store ppu_addr
                    self.ppu.nmi_set = true;
//                    self.ppu.ppu_x_scroll_read = true;
//                    self.ppu_x_scroll_write = true;
                }
            }

            if self.ppu.scan_line == 0 && self.ppu.cycle == 0 {
//                self.set_ppu_sprite_zero_hit(false, 0);
                self.cpu_set_vblank(false);
//                self.set_ppu_sprite_overflow(false);
            }

        }

        pub fn set_ppu_sprite_overflow(&mut self, value: bool) {
            let mut byte: u8 = self.ppu.registers.read(2) & 0xdf;

            if value {
                byte = self.ppu.registers.read(2) | 0x20;
            }
            
            self.ppu.registers.write(2, byte);
        }

        pub fn cpu_set_vblank(&mut self, value: bool) {

            let mut byte: u8 = self.ppu.registers.read(2);
            
            if value {
                byte |= 0x80;
            }
            else {
                byte &= 0x7f;
            }
            self.ppu.registers.write(2, byte);
        }

        fn render_pixel(&mut self, cartridge: &NesCartridge000) {

            let (sprite_pixel, sprite_priority, is_sprite_zero, sprite_pos) = self.get_sprite_pixel(self.ppu.scan_line as u16, self.ppu.cycle as i16, &cartridge);
            let background_pixel: u8 = self.get_background_pixel(self.ppu.scan_line as u16, self.ppu.cycle as i16, &cartridge);
            let backdrop: u8 = self.read(PPU_PALETTE_ADDR, cartridge);

            if self.ppu.cycle >= 0  && self.ppu.cycle < 256 && self.ppu.scan_line >= 0 && self.ppu.scan_line < 240 {

                let mut color = backdrop;

                if background_pixel != 0 {
                    color = background_pixel;
                }
                if sprite_pixel != 0 && (sprite_priority == 0 || background_pixel == 0) {
                    color = sprite_pixel;
                }

                if sprite_pixel != 0 && background_pixel != 0 && is_sprite_zero
                    && self.ppu.scan_line < 239 {
                    //self.set_ppu_sprite_zero_hit(true, sprite_pos);
                }

                let (red, green, blue) = self.ppu.palette.get_color(color as usize, 0);

                self.ppu.screen[((self.ppu.scan_line * 256 + self.ppu.cycle) * 3) as usize] = red;
                self.ppu.screen[(((self.ppu.scan_line * 256 + self.ppu.cycle) * 3) + 1) as usize] = green;
                self.ppu.screen[(((self.ppu.scan_line * 256 + self.ppu.cycle) * 3) + 2) as usize] = blue;
            }

        }

        pub fn get_sprite_pixel(&mut self, screen_row: u16, screen_column: i16, cartridge: &NesCartridge000) -> (u8, u8, bool, i32) {

            let control_register = PpuControlRegister::new(self.ppu.registers.read(0));

            if self.ppu.cycle == 255 {

                for i in 0..=7 {
                    self.ppu.sprites[i as usize].sprite_id = -1;
                }

                let mut sprite_count: i8 = 0;

                for i in 0..64 {
                    let y_pos: u8 = self.oam_read(i * 4);

                    if (screen_row >= y_pos as u16) && (screen_row - y_pos as u16) < 8 {
                        self.ppu.sprites[sprite_count as usize].sprite_id = i as i8;
                        self.ppu.sprites[sprite_count as usize].y_pos = y_pos;
                        self.ppu.sprites[sprite_count as usize].tile = self.oam_read(i * 4 + 1);
                        self.ppu.sprites[sprite_count as usize].attribute = self.oam_read(i * 4 + 2);
                        self.ppu.sprites[sprite_count as usize].x_pos = self.oam_read(i * 4 + 3);

                        sprite_count += 1;

                        if sprite_count == 8 {
                            // sprite overflow
                            self.set_ppu_sprite_overflow(true);
                            break;
                        }
                    }            
                }
            }
            
            let mut priority: u8 = 0;

            for i in 0..=7 {

                if self.ppu.sprites[i].sprite_id == -1 {
                    continue;
                }

                let x_pos = self.ppu.sprites[i].x_pos + 8;
                if (self.ppu.cycle - x_pos as i32) < 0 || self.ppu.cycle - x_pos as i32 > 7 {
                    continue;
                }

                let y_pos = self.ppu.sprites[i].y_pos;

                let sprite_attribute = SpriteAttribute::new(self.ppu.sprites[i].attribute);

                let mut pattern_address: u16 = ((self.ppu.sprites[i].tile as u16) << 4) + (self.ppu.scan_line as u8 - y_pos - 1) as u16;
                if sprite_attribute.get_flip_verticle() {
                    pattern_address = (pattern_address + 7 - self.ppu.scan_line  as u16 - y_pos  as u16);
                }
                let mut sprite_lsb = self.read(pattern_address, cartridge);
                let mut sprite_msb = self.read(pattern_address + 8, cartridge);

                if sprite_attribute.get_flip_horizontal() {
                    sprite_lsb = NesPpu2Runner::reverse_bits(sprite_lsb);
                    sprite_msb = NesPpu2Runner::reverse_bits(sprite_msb);
                }

                priority = sprite_attribute.get_priority();

                let slide = self.ppu.cycle - x_pos as i32;
                sprite_lsb <<= slide;
                sprite_msb <<= slide;

                let pixel: u8 = ((sprite_msb & 0x80) >> 6) + ((sprite_lsb & 0x80) >> 7);
                let palette_address: u16 = ((sprite_attribute.get_palette() + 0x04) << 2) as u16 + pixel as u16;

                if pixel != 0{
                    let color: u8 = self.read(PPU_PALETTE_ADDR + palette_address, cartridge);
                    return (color, priority, self.ppu.sprites[i].sprite_id != 0, 0);//sprite_pos);
                }
            }

            (0, priority, false, 0)
        }

        fn get_background_pixel(&mut self, mut screen_row: u16, mut screen_column: i16, cartridge: &NesCartridge000) -> u8 {

            self.get_bg_attribute_bytes(screen_row, screen_column, cartridge);

            let pixel =  ((self.ppu.pattern_high_byte & 0x80) >> 6) + ((self.ppu.pattern_low_byte & 0x80) >> 7);
            let palette_address: u16 = ((self.ppu.attribute_byte & 0x03) << 2) as u16 + pixel as u16;
            let color: u8 = self.read(PPU_PALETTE_ADDR + palette_address, cartridge);

            self.ppu.pattern_low_byte <<= 1;
            self.ppu.pattern_high_byte <<= 1;

            color
        }

        fn get_bg_attribute_bytes(&mut self, mut screen_row: u16, mut screen_column: i16, cartridge: &NesCartridge000) {

            let control_register = PpuControlRegister::new(self.ppu.registers.read(0));
            let tile_row: u16 = self.ppu.scan_line as u16 / 8;
            let tile_column: u16 = self.ppu.cycle as u16 / 8;

            match self.ppu.cycle % 8 {
                // Copy registers, Nametable
                0 => {
                    self.ppu.nametable_byte = self.ppu.nametable_hold_byte;
                    self.ppu.attribute_byte = self.ppu.attribute_hold_byte;
                    self.ppu.pattern_low_byte = self.ppu.pattern_low_hold_byte;
                    self.ppu.pattern_high_byte = self.ppu.pattern_high_hold_byte;

                    let mut nametable_table_address: u16 = control_register.base_nametable_address();
                    nametable_table_address += ((tile_row * 32) + tile_column);
                    self.ppu.nametable_hold_byte = self.read(nametable_table_address, cartridge);
                },
                // Attribute byte
                2 => {
                    let attr_x = tile_column / 4;
                    let attr_y = tile_row / 4;
                    let attr_x_shift = (tile_column % 4) / 2;
                    let attr_y_shift = (tile_row % 4) / 2;
                    let shift = (attr_y_shift * 2 + attr_x_shift) * 2;
                    let attribute_table_address = control_register.base_attributetable_address() + (attr_x + attr_y * 8);
                    self.ppu.attribute_hold_byte = self.read(attribute_table_address, cartridge);
                    self.ppu.attribute_hold_byte >>= shift;
                },
                // Pattern lsb
                4 => {
                    let pattern_address: u16 = control_register.background_pattern_table_address() + (self.ppu.nametable_hold_byte as u16 * 16) + (self.ppu.scan_line % 8) as u16;
                    self.ppu.pattern_low_hold_byte = self.read(pattern_address, cartridge);
                },
                // Patterm msb
                6 => {
                    let pattern_address: u16 = control_register.background_pattern_table_address() + (self.ppu.nametable_hold_byte as u16 * 16)  + (self.ppu.scan_line % 8) as u16 + 8;
                    self.ppu.pattern_high_hold_byte = self.read(pattern_address, cartridge);
                },
                _ => {}
            }
        }

        pub fn is_nmi_set(&mut self) -> bool {
            self.ppu.nmi_set
        }

        pub fn reset_nmi(&mut self) {
            self.ppu.nmi_set = false;
        }

        pub fn get_screen(&mut self) -> Vec<u8> {
            self.ppu.screen.clone()
        }

        fn reverse_bits(mut n: u8) -> u8 {
            let mut ans: u8 = 0;

            for i in (0..=7).rev() {
                ans |= (n & 1) <<i;
                n >>= 1;
            }
            ans
        }

    }
}