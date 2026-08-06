pub mod nes {

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
            self.byte & 0x80 != 0
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
        pub x_pos: u16,
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

    pub struct NesPpu {
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

    impl NesPpu {

        pub fn new () -> NesPpu {

            let palette = NesPalette::new();
            let ppu_palette = MemoryRam::new(String::from("PPU Palette"), 0x0100);

                Self {
                palette: palette,
                video_bus: VideoBus::new(),
                registers: MemoryRam::new(String::from("PPU Registers"), 0x0008),
                oam: MemoryRam::new(String::from("PPU OAM"), 0x0100),
                name_table: MemoryRam::new(String::from("PPU Name Table"), 0x1F00),
                ppu_palette: ppu_palette,
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

    #[derive(Default)]
    pub struct NesPpuRunner {
    }

    impl NesPpuRunner {
    
        pub fn execute_memory(ppu: &mut NesPpu, addr: &mut AddressBus, cartridge: &NesCartridge000) {

            if (0x2000..0x4000).contains(&addr.address) {
                if addr.write {
                    Self::ppu_register_write(ppu, addr);
                    addr.write = false;
                } else {
                    addr.byte = Self::ppu_register_read(ppu,addr.address, cartridge);
                }
            }                
        }

        fn ppu_register_write(ppu: &mut NesPpu, addr: &mut AddressBus) {


            let mut location = addr.address;
            location %= 8;

            if location != 0x06 {
                ppu.video_bus.set_byte(addr.byte);
            }

            match location {
                0x00 => {
                    ppu.registers.write(0, addr.byte);
                    let control_register = PpuControlRegister::new(ppu.registers.read(0));
                    let status_register = PpuStatusRegister::new(ppu.registers.read(2));
                    if addr.byte & 0x80 != 0 && control_register.vblank_nmi_enable() && status_register.vblank_flag() {
                        ppu.nmi_set = true;
                    }
                },
                0x01 => ppu.registers.write(1, addr.byte),
                0x03 => ppu.registers.write(3, addr.byte),
                0x04 => {
                    ppu.registers.write(4, addr.byte);
                    let oam_address = ppu.registers.read(3);
                    Self::oam_write(ppu, oam_address, addr.byte);
                    ppu.registers.write(3, oam_address.wrapping_add(1));
                },
                0x05 => {
                    if ppu.ppu_addr_first {
                        ppu.ppu_scroll_x = addr.byte;
                        ppu.ppu_addr_first = false;
                    } else {
                        ppu.ppu_scroll_y = addr.byte;
                        ppu.ppu_addr_first = true;
                    }
                },
                0x06 => {
                    if ppu.ppu_addr_first {
                        ppu.ppu_addr = ((addr.byte & 0x3F) as u16) << 8;
                        ppu.ppu_addr_first = false;
                    } else {
                        ppu.ppu_addr |= addr.byte as u16;
                        ppu.ppu_addr_first = true;
                    }
                },
                0x07 => {
                    Self::write(ppu, ppu.ppu_addr, addr.byte);
                    let control_register = PpuControlRegister::new(ppu.registers.read(0));
                    ppu.ppu_addr = ppu.ppu_addr.wrapping_add(control_register.vram_address_increment());
                },
                _ => {}
            }
        }

        fn ppu_register_read(ppu: &mut NesPpu, mut location: u16, cartridge: &NesCartridge000) -> u8 {
            // Mirroring, and bring to zero
            location %= 8;
            
            match location {
                0x02 => {
                    let mut byte = ppu.registers.read(2);
                    // Clear the vblank flag
                    ppu.registers.write(2, byte & 0x7f);
                    byte &= 0xe0;
                    byte |= ppu.video_bus.byte & 0x1f;
                    ppu.video_bus.set_byte(byte);
                    byte
                },
                0x04 => {
                    let oam_address = ppu.registers.read(3);
                    let byte = Self::oam_read(ppu, oam_address);
                    ppu.video_bus.set_byte(byte);
                    byte
                },
                0x07 => {
                    // No buffer when reading from PPU ram
                    let byte = ppu.video_bus.byte;
                    let ppu_byte = Self::read(ppu, ppu.ppu_addr, cartridge);
                    if ppu.ppu_addr >= 0x3f00 {
                        return ppu_byte;
                    }
                    ppu.video_bus.set_byte(ppu_byte);
                    let control_register = PpuControlRegister::new(ppu.registers.read(0));
                    ppu.ppu_addr = ppu.ppu_addr.wrapping_add(control_register.vram_address_increment());
                    byte
                },
                _ => {
                    ppu.video_bus.byte
                }
            }

        }

        pub fn oam_read(ppu: &mut NesPpu, location: u8) -> u8 {
            ppu.oam.read(location as u16)
        }

        pub fn oam_write(ppu: &mut NesPpu, location: u8, byte: u8) {
            ppu.oam.write(location as u16, byte);
        }

        fn read(ppu: &mut NesPpu, mut location: u16, cartridge: &NesCartridge000) -> u8 {

            //  Cartridge PPU ROM
            match location {
                0x0000..=0x1FFF => {
                    return cartridge.ppu_read(location);
                },
                0x2000..=0x3EFF => {
                    location -= 0x2000;
                    return ppu.name_table.read(location);
                },
                0x3F00..=0x3FFF => {
                    let mask_register = PpuMaskRegister::new(ppu.registers.read(1));
                    location -= 0x3F00;
                    
                    let mut result: u8 = ppu.ppu_palette.read(location % 0x20);
                    if location.is_multiple_of(4) {
                        if location >= 0x10 {
                            location -= 0x10;
                        }
                        result = ppu.ppu_palette.read(location);
                    }
                    result &= 0x3f;

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

        fn write(ppu: &mut NesPpu, mut location: u16, byte: u8) {

            match location {
                0x0000..=0x1FFF => {
                    // Cartridge PPU ROM
                    return;
                },
                0x2000..=0x3EFF => {
                    location -= 0x2000;
                    ppu.name_table.write(location, byte);
                    return;
                },
                0x3F00..=0x3FFF => {
                    location -= 0x3F00;
                    if location.is_multiple_of(4) {
                        if location >= 0x10 {
                            location -= 0x10;
                        }
                        return ppu.ppu_palette.write(location, byte);
                    }
                    location %= 0x20;
                    ppu.ppu_palette.write(location, byte);
                    return;
                },
                _ => {}
            }
            eprintln!("Invalid NES memory location for PPU write {}", location);
        }

        pub fn execute_tick(ppu: &mut NesPpu, cartridge: &NesCartridge000) {

            ppu.video_bus.execute_tick();

            ppu.cycle += 1;
            if ppu.cycle > 340 {
                // Set rendering registers for when scrolling happens
                ppu.cycle = 0;
                ppu.scan_line += 1;
                if ppu.scan_line > 260 {
                    ppu.scan_line = 0;
                }
            }

            Self::render_pixel(ppu, cartridge);
            
            if ppu.scan_line == 241 && ppu.cycle == 1 {
                Self::cpu_set_vblank(ppu, true);
                let control_register = PpuControlRegister::new(ppu.registers.read(0));
                if control_register.vblank_nmi_enable() {
                    // store ppu_addr
                    ppu.nmi_set = true;
//                    self.ppu.ppu_x_scroll_read = true;
//                    self.ppu_x_scroll_write = true;
                }
            }

            if ppu.scan_line == 260 && ppu.cycle == 1 {
                Self::cpu_set_vblank(ppu, false);
                Self::set_ppu_sprite_zero_hit(ppu, false, 0, 0);
                Self::set_ppu_sprite_overflow(ppu, false);
            }

        }

        fn set_ppu_sprite_overflow(ppu: &mut NesPpu, value: bool) {
            let mut byte: u8 = ppu.registers.read(2) & 0xdf;

            if value {
                byte = ppu.registers.read(2) | 0x20;
            }
            
            ppu.registers.write(2, byte);
        }

        fn cpu_set_vblank(ppu: &mut NesPpu, value: bool) {

            let mut byte: u8 = ppu.registers.read(2);
            
            if value {
                byte |= 0x80;
            }
            else {
                byte &= 0x7f;
            }
            ppu.registers.write(2, byte);
        }

        fn render_pixel(ppu: &mut NesPpu, cartridge: &NesCartridge000) {

            let screen_x = ppu.cycle;
            let screen_y = ppu.scan_line;
            Self::get_bg_attribute_bytes(ppu, screen_x + 8, screen_y, cartridge);

            if (8..256).contains(&screen_x) && (0..240).contains(&screen_y) {

                let ppu_mask = PpuMaskRegister::new(ppu.registers.read(1));

                let (mut sprite_pixel, mut sprite_priority, mut is_sprite_zero) = (0, 0, false);
                if ppu_mask.show_sprites() && (ppu_mask.show_sprites_leftmost_8_pixels() || screen_x >= 8)  {
                    (sprite_pixel, sprite_priority, is_sprite_zero) = Self::get_sprite_pixel(ppu, screen_y as u16, screen_x as u16, cartridge);
                }

                let mut background_pixel: u8 = 0;
                if ppu_mask.show_background() && (ppu_mask.show_background_leftmost_8_pixels() || screen_x >= 8) {
                    background_pixel = Self::get_background_pixel(ppu, cartridge);
                }

                let backdrop: u8 = Self::read(ppu, PPU_PALETTE_ADDR, cartridge);

                let mut color = backdrop;

                if background_pixel != 0 {
                    color = background_pixel;
                }
                if sprite_pixel != 0 && (sprite_priority == 0 || background_pixel == 0) {
                    color = sprite_pixel;
                }

                if sprite_pixel != 0 && background_pixel != 0 && is_sprite_zero {
                    Self::set_ppu_sprite_zero_hit(ppu, true, screen_x, screen_y);
                }

                let (red, green, blue) = ppu.palette.get_color(color as usize, 0);

                ppu.screen[((screen_y * 256 + screen_x) * 3) as usize] = red;
                ppu.screen[(((screen_y * 256 + screen_x) * 3) + 1) as usize] = green;
                ppu.screen[(((screen_y * 256 + screen_x) * 3) + 2) as usize] = blue;
            }

        }

        fn get_sprite_pixel(ppu: &mut NesPpu, screen_y: u16, screen_x: u16, cartridge: &NesCartridge000) -> (u8, u8, bool) {
            
            let mut priority: u8 = 0;

            for i in 0..=7 {

                if ppu.sprites[i].sprite_id == -1 {
                    continue;
                }

                let x_pos = ppu.sprites[i].x_pos;
                if (screen_x as i16 - x_pos as i16) < 0 || screen_x - x_pos > 7 {
                    continue;
                }

                let y_pos = ppu.sprites[i].y_pos;

                let sprite_attribute = SpriteAttribute::new(ppu.sprites[i].attribute);

                let mut pattern_address: u16 = ((ppu.sprites[i].tile as u16) << 4) + (ppu.scan_line as u8 - y_pos - 1) as u16;
                if sprite_attribute.get_flip_verticle() {
                    pattern_address = pattern_address + 7 - screen_y - y_pos  as u16;
                }
                let mut sprite_lsb = Self::read(ppu, pattern_address, cartridge);
                let mut sprite_msb = Self::read(ppu, pattern_address + 8, cartridge);

                if sprite_attribute.get_flip_horizontal() {
                    sprite_lsb = NesPpuRunner::reverse_bits(sprite_lsb);
                    sprite_msb = NesPpuRunner::reverse_bits(sprite_msb);
                }

                priority = sprite_attribute.get_priority();

                let slide = screen_x - x_pos;
                sprite_lsb <<= slide;
                sprite_msb <<= slide;

                let pixel: u8 = ((sprite_msb & 0x80) >> 6) + ((sprite_lsb & 0x80) >> 7);
                let palette_address: u16 = ((sprite_attribute.get_palette() + 0x04) << 2) as u16 + pixel as u16;

                if pixel != 0{
                    let color: u8 = Self::read(ppu, PPU_PALETTE_ADDR + palette_address, cartridge);
                    return (color, priority, ppu.sprites[i].sprite_id == 0);
                }
            }

            (0, priority, false)
        }

        fn get_background_pixel(ppu: &mut NesPpu, cartridge: &NesCartridge000) -> u8 {

            let pixel =  ((ppu.pattern_high_byte & 0x80) >> 6) + ((ppu.pattern_low_byte & 0x80) >> 7);
            let palette_address: u16 = ((ppu.attribute_byte & 0x03) << 2) as u16 + pixel as u16;
            ppu.pattern_low_byte <<= 1;
            ppu.pattern_high_byte <<= 1;
            
            let color: u8 = Self::read(ppu, PPU_PALETTE_ADDR + palette_address, cartridge);

            color
        }

        fn get_bg_attribute_bytes(ppu: &mut NesPpu, screen_x: i32, screen_y: i32, cartridge: &NesCartridge000) {

            let control_register = PpuControlRegister::new(ppu.registers.read(0));
            let tile_row: u16 = (screen_y as u16 + ppu.ppu_scroll_y as u16) / 8;
            let mut tile_column: u16 = (screen_x as u16 + ppu.ppu_scroll_x as u16) / 8;

            let mut addr_add: i32 = 0x0;
            if tile_column >= 32 {
                tile_column -= 32;
                addr_add = 0x400;

                if control_register.base_nametable_address() == 0x2400  || 
                    control_register.base_nametable_address() == 0x2C00 {
                    addr_add = -0x400;
                }
            }

            if ppu.cycle < 256 {
                match (ppu.cycle + (ppu.ppu_scroll_x & 0x07) as i32) % 8 {
                    // Copy registers, Nametable
                    0 => {
                        ppu.nametable_byte = ppu.nametable_hold_byte;
                        ppu.attribute_byte = ppu.attribute_hold_byte;
                        ppu.pattern_low_byte = ppu.pattern_low_hold_byte;
                        ppu.pattern_high_byte = ppu.pattern_high_hold_byte;

                        let mut nametable_table_address: u16 = control_register.base_nametable_address();
                        nametable_table_address = (nametable_table_address as i32 + addr_add) as u16;
                        nametable_table_address += (tile_row * 32) + tile_column;
                        ppu.nametable_hold_byte = Self::read(ppu, nametable_table_address, cartridge);
                    },
                    // Attribute byte
                    2 => {
                        let attr_x = tile_column / 4;
                        let attr_y = tile_row / 4;
                        let attr_x_shift = (tile_column % 4) / 2;
                        let attr_y_shift = (tile_row % 4) / 2;
                        let shift = (attr_y_shift * 2 + attr_x_shift) * 2;
                        let attribute_table_address = (addr_add + control_register.base_attributetable_address() as i32 + (attr_x + attr_y * 8) as i32) as u16;
                        ppu.attribute_hold_byte = Self::read(ppu, attribute_table_address, cartridge);
                        ppu.attribute_hold_byte >>= shift;
                    },
                    // Pattern lsb
                    4 => {
                        let pattern_address: u16 = control_register.background_pattern_table_address() + (ppu.nametable_hold_byte as u16 * 16) + (ppu.scan_line % 8) as u16;
                        ppu.pattern_low_hold_byte = Self::read(ppu, pattern_address, cartridge);
                    },
                    // Patterm msb
                    6 => {
                        let pattern_address: u16 = control_register.background_pattern_table_address() + (ppu.nametable_hold_byte as u16 * 16)  + (ppu.scan_line % 8) as u16 + 8;
                        ppu.pattern_high_hold_byte = Self::read(ppu, pattern_address, cartridge);
                    },
                    _ => {}
                }

            } else if ppu.cycle == 256 {

                for i in 0..=7 {
                    ppu.sprites[i as usize].sprite_id = -1;
                }

                let mut sprite_count: i8 = 0;

                for i in 0..64 {
                    let y_pos: u8 = Self::oam_read(ppu, i * 4);

                    if y_pos == 255 {
                        continue;
                    }

                    if (ppu.scan_line >= y_pos as i32) && (ppu.scan_line <= y_pos as i32 + 7) {

                        // sprite overflow
                        if sprite_count >= 8 {
                            Self::set_ppu_sprite_overflow(ppu, true);
                            break;
                        }

                        ppu.sprites[sprite_count as usize].sprite_id = i as i8;
                        ppu.sprites[sprite_count as usize].y_pos = y_pos;
                        ppu.sprites[sprite_count as usize].tile = Self::oam_read(ppu, i * 4 + 1);
                        ppu.sprites[sprite_count as usize].attribute = Self::oam_read(ppu, i * 4 + 2);
                        ppu.sprites[sprite_count as usize].x_pos = Self::oam_read(ppu, i * 4 + 3) as u16;

                        sprite_count += 1;
                    }            
                }
            }

        }

        fn set_ppu_sprite_zero_hit(ppu: &mut NesPpu, value: bool, screen_x: i32, screen_y: i32) {

            if value == false {
                let byte: u8 = ppu.registers.read(2) & 0xbf;
                ppu.registers.write(2, byte);
                return;
            }

            if value && screen_y < 240 && screen_x < 255{
                let byte = ppu.registers.read(2) | 0x40;
                ppu.registers.write(2, byte);
            }
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