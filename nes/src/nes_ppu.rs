
pub mod nes {

    use std::sync::Arc;
    use std::sync::RwLock;

    use emumemory::{memory_mapper::emu_memory::MemoryMapper};
    use emumemory::memory_ram::emu_memory::MemoryRam;
    use emumemory::base_memory::emu_memory::BaseMemory;

    use crate::nes_memory::nes::NesMemory;
    use crate::nes_cartridge::nes::NesCartridge;

    const PPU_CONTROL_ADDR: u16 =  0x2000;
    const PPU_MASK_ADDR: u16 =     0x2001;
    const PPU_STATUS_ADDR: u16 =   0x2002;
    const PPU_SCROLL_ADDR: u16 =   0x2005;
    const PPU_ADDR: u16 =          0x2006;
    const PPU_DATA_ADDR: u16 =     0x2007;
    const PPU_OAM_ADDR: u16 =      0x2003;
    const PPU_OAM_DATA_ADDR: u16 = 0x2004;
    const PPU_OAM_DMA_ADDR: u16 =  0x4014;

    const PPU_ATTRIBUTE_ADDR: u16 = 0x23c0;
    const PPU_ATTRIBUTE_SIZE: u16 = 0x23c0;
    const PPU_NAMETABLE_ADDR: u16 = 0x2000;
    const PPU_NAMETABLE_SIZE: u16 = 0x0400;
    const PPU_PATTERN_SIZE: u16 =   0x1000;
    const PPU_PALETTE_ADDR: u16 =   0x1000;

    const PPU_SPRITE_SIZE: i8 =          0x0004;
    const PPU_SPRITE_PATTERN_SIZE: u16 = 0x0008;

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

    }

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
        ppu_name_table: MemoryRam,
        ppu_palette: MemoryRam,
        ppu_oam: MemoryRam,
        pub cpu_ppu_registers: MemoryRam,
        cartridge: Arc<RwLock<dyn NesCartridge>>,
        control_register: PpuControlRegister,
        scan_line: i32,
        cycle: i32,
        nmi_set: bool,
        render_sprites: [i8; 8],
        screen: Vec<u8>,
        attribute_byte: u8,
        nametable_address: u16,
        pattern_entry_address: u16,
        char_table_entry_lsb: u8,
        char_table_entry_msb: u8,
        pub ppu_x_scroll_write: bool,
        pub ppu_x_scroll_read: bool,
        pub ppu_x_scroll: u8,
        pub ppu_y_scroll: u8,
    }

    impl NesPpu {

        pub fn new (cartridge: Arc<RwLock<dyn NesCartridge>>) -> NesPpu {
            Self {
                ppu_name_table: MemoryRam::new(String::from("PPU Name Table RAM"), 0x1f00),
                ppu_palette: MemoryRam::new(String::from("PPU Palette RAM"), 0x0100),
                ppu_oam: MemoryRam::new(String::from("PPU OAM RAM"), 0x0100),
                cpu_ppu_registers: MemoryRam::new(String::from("PPU Registers"), 0x0008),
                cartridge: cartridge,
                control_register: PpuControlRegister::new(),
                scan_line: 0,
                nmi_set: false,
                cycle: 0,
                render_sprites: [-1, -1, -1, -1, -1, -1, -1, -1],
                screen: vec!(0; 61440),
                attribute_byte: 0,
                nametable_address: 0,
                pattern_entry_address: 0,
                char_table_entry_lsb: 0,
                char_table_entry_msb: 0,
                ppu_x_scroll_write: true,
                ppu_x_scroll_read: true,
                ppu_x_scroll: 0,
                ppu_y_scroll: 0,
            }
        }
        
        pub fn read(&mut self, mut location: u16) -> u8 {

            //  Cartridge PPU ROM
            if location < 0x2000 {
                return self.cartridge.read().unwrap().ppu_read(location);
            }
            
            else if location < 0x3f00 {
                location -= 0x2000;
                return self.ppu_name_table.read(location);
            }
            
            else if location < 0x4000 {
                location -= 0x3f00;

                if (location % 4) == 0 {
                    self.ppu_palette.read(0);
                }

                return self.ppu_palette.read(location);
            }
            
            panic!("Invalid read address");
        }

        pub fn write(&mut self, mut location: u16, byte: u8) {

            // Cartridge PPU ROM
            if location < 0x2000 {
                self.cartridge.write().unwrap().ppu_write(location, byte);
                return;
            }       

            else if location < 0x3f00 {
                location -= 0x2000;
                self.ppu_name_table.write(location, byte);
                return;
            }
            
            else if location < 0x4000 {
                location -= 0x3f00;

                if (location % 4) == 0 {
                    self.ppu_palette.write(0, byte);
                }

                self.ppu_palette.write(location, byte);
                return;
            }
            
            panic!("Invalid NES memory location for PPU write {}", location);

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

        pub fn set_ppu_sprite_overflow(&mut self, value: u8) {

            let byte: u8;

            if value > 0 {
                byte = self.cpu_ppu_registers.read(2) | 0x20;
            }
            else {
                byte = self.cpu_ppu_registers.read(2) & 0xdf;
            }
            
            self.cpu_ppu_registers.write(2, byte);
        }

        pub fn set_ppu_sprite_zero_hit(&mut self, value: u8) {

            let byte: u8;

            if value > 0 {
                byte = self.cpu_ppu_registers.read(2) & 0x40;
            }
            else {
                byte = self.cpu_ppu_registers.read(2) & 0xbf;
            }

            self.cpu_ppu_registers.write(2, byte);
        }

        pub fn ppu_scroll_read(&mut self) -> u8 {
            if self.ppu_x_scroll_read {
                self.ppu_x_scroll_read = false;
                return self.ppu_x_scroll;
            }
            else {
                self.ppu_x_scroll_read = true;
                return self.ppu_y_scroll;
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

        pub fn cpu_set_vblank(&mut self, value: u8) {

            let mut byte: u8 = self.cpu_ppu_registers.read(2);
            
            if value == 1 {
                byte |= 0x80;
            }
            else {
                byte &= 0x7f;
            }
            self.cpu_ppu_registers.write(2, byte);
        }

        fn render_pixel(&mut self) {
            let screen_scan_line = self.scan_line - 1;
            let screen_cycle: i32 = self.cycle;

            if screen_cycle == 0 {
                self.render_sprites = [-1, -1, -1, -1, -1, -1, -1, -1];
                let mut sprite_count: i8 = 0;

                for i in 0..64 {
                    if sprite_count == 8 {
                        // sprite overflow
                        self.set_ppu_sprite_overflow(1);
                        break;
                    }

                    let y_pos: u8 = self.oam_read(i * 4);

                    if screen_scan_line - y_pos as i32 > 0 && screen_scan_line - y_pos as i32 <= 8 {
                        self.render_sprites[sprite_count as usize] = i as i8;
                        sprite_count += 1;
                    }            
                }
            }
            
            if screen_cycle == 254 {
                let background: u8 = self.read(PPU_PALETTE_ADDR);

                for i in (7..0).rev() {
                    let mut sprite_attribute: PpuSpriteAttributeRegister = PpuSpriteAttributeRegister::new();
                    let sprite_pos: i8 = self.render_sprites[i];

                    if sprite_pos == -1 {
                        continue;
                    }

                    let y_pos: u8 = self.oam_read((sprite_pos * PPU_SPRITE_SIZE) as u16) + 1;
                    let pattern_address: u16 = self.oam_read((sprite_pos as u16 * PPU_SPRITE_SIZE as u16 + 1) << 4) as u16;
                    sprite_attribute.reg(self.oam_read((sprite_pos * PPU_SPRITE_SIZE + 2) as u16));
                    let x_pos: u8 = self.oam_read((sprite_pos * PPU_SPRITE_SIZE + 3) as u16);
                    let mut sprite_lsb: u8 = 0;
                    let mut sprite_msb: u8 = 0;
                    let mut sprite_pattern_address: u16 = pattern_address + ((screen_scan_line - y_pos as i32) & 0x07) as u16;

                    if sprite_attribute.flip_vertically > 0 {// flip verticle
                        sprite_pattern_address = pattern_address + ((7 - screen_scan_line - y_pos as i32) & 0x07) as u16;
                    }

                    sprite_lsb = self.read(sprite_pattern_address);
                    sprite_msb = self.read(sprite_pattern_address + PPU_SPRITE_PATTERN_SIZE);

                    if sprite_attribute.flip_horizontally > 0 {
                        sprite_lsb = NesPpu::reverse_bits(sprite_lsb);
                        sprite_msb = NesPpu::reverse_bits(sprite_msb);
                    }

                    for j in x_pos..x_pos + 8 {
                        let pixel: u8 = ((sprite_msb >> 6) & 0x02) + ((sprite_lsb >> 7) & 0x01);
                        let palette: u8 = ((sprite_attribute.sprite_palette) + 0x04) << 2;

                        if pixel != 0{
                            let color: u8 = self.read(PPU_PALETTE_ADDR + palette as u16 + pixel as u16);

                            if background != color {

                                if sprite_pos == 0 && self.screen[(screen_scan_line * 256 + j as i32) as usize] != background {
                                    self.set_ppu_sprite_zero_hit(1);
                                }

                                if sprite_attribute.priority == 0 || self.screen[(screen_scan_line * 256 + j as i32) as usize] == background {
                                    self.screen[(screen_scan_line * 256 + j as i32) as usize] = color;
                                }
                            }
                        }
                        sprite_lsb = sprite_lsb << 1;
                        sprite_msb = sprite_msb << 1;
                    }
                }
            }
            
            let background_pixel: u8 = self.get_background_pixel(screen_scan_line as u16, screen_cycle as i16);

            if self.cycle >= 0  && self.cycle < 256 {
                self.screen[(screen_scan_line * 256 + screen_cycle) as usize] = background_pixel;
            }
        }

        fn get_background_pixel(&mut self, mut screen_row: u16, mut screen_column: i16) -> u8 {
            let mut attribute_value: u8 = 0;
            let mut attribute_shift: u8 = 0;
            let ppu_control_addr: u8 = self.read(PPU_CONTROL_ADDR); 
            self.control_register.reg(ppu_control_addr);
            let x_scroll: u8 = self.ppu_x_scroll;
            let y_scroll: u8 = self.ppu_y_scroll;
            let mut nametable_x: u8 = self.control_register.nametable_x;
            let mut nametable_y: u8 = self.control_register.nametable_y;
            screen_column += x_scroll as i16;
            screen_row += y_scroll as u16;
            
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

            // Get attribute value
            // Should be one of: $23C0, $27C0, $2BC0, or $2FC0
            let attribute_table_address: u16 = PPU_ATTRIBUTE_ADDR + (nametable_x * PPU_NAMETABLE_SIZE as u8) as u16 + 
                (nametable_y * PPU_NAMETABLE_SIZE as u8 * 2) as u16;
            let attribute_address: u16 = ((screen_row / 32) * 8 + (screen_column / 32) as u16) + attribute_table_address;
            self.attribute_byte = self.read(attribute_address);

            if ((screen_row % 32) < 16) && (screen_column % 32) < 16 {
                attribute_shift = 0;
            }
            else if ((screen_row % 32) < 16) && (screen_column % 32) >=- 16 {
                attribute_shift = 2;
            }
            else if ((screen_row % 32) >= 16) && (screen_column % 32) < 16 {
                attribute_shift = 4;
            }
            else if ((screen_row % 32) >= 16) && (screen_column % 32) >= 16 {
                attribute_shift = 6;
            }

            attribute_value = (self.attribute_byte  >> attribute_shift) & 0x03;

            let tile_row: u16 = screen_row / 8;
            let tile_column: u16 = screen_column as u16/ 8;
            // Should be one of: $2000, $2400, $2800, or $2C00
            let nametable_table_address: u16 = PPU_NAMETABLE_ADDR + (nametable_x * PPU_NAMETABLE_SIZE as u8) as u16 + 
                (nametable_y * PPU_NAMETABLE_SIZE as u8 * 2) as u16;
            self.nametable_address = (((tile_row) * 32) + (tile_column)) + nametable_table_address;
            self.pattern_entry_address = (((self.read(self.nametable_address) << 4)  + (screen_row % 8) as u8) + 
                PPU_PATTERN_SIZE as u8 * self.control_register.pattern_background as u8) as u16;
            self.char_table_entry_lsb = self.read(self.pattern_entry_address) << (screen_column % 8);
            self.char_table_entry_msb = self.read(self.pattern_entry_address + 0x08) << (screen_column % 8);
            let pixel: u8 =  ((self.char_table_entry_msb & 0x80) >> 6) + ((self.char_table_entry_lsb & 0x80) >> 7);
            let pixel_address: u16 = PPU_PALETTE_ADDR + pixel as u16 + (attribute_value << 2) as u16;
            let color: u8 = self.read(pixel_address);

            color
        }
        
        fn reverse_bits(mut n: u8) -> u8 {
            let mut ans: u8 = 0;

            for i in (7..0).rev() {
                ans |= (n & 1) <<i;
                n >>= 1;
            }

            ans
        }
        
        pub fn execute_tick(&mut self) {

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
                self.render_pixel();
            }
            
            if self.scan_line == 241 && self.cycle == 1 {                
                self.cpu_set_vblank(1);
                let ppu_control_addr: u8 = self.read(PPU_CONTROL_ADDR); 
                self.control_register.reg(ppu_control_addr);
                if self.control_register.enable_nmi > 0 {
                    self.nmi_set = true;
                    self.ppu_x_scroll_read = true;
                    self.ppu_x_scroll_write = true;
                }
            }

            if self.scan_line == 261 && self.cycle == 1 {
                self.cpu_set_vblank(0);
                self.set_ppu_sprite_overflow(0);
                self.set_ppu_sprite_zero_hit(0);
            }
        }

        pub fn is_nmi_set(&self) -> bool {
            self.nmi_set
        }
        
        pub fn reset_nmi(&mut self) {
            self.nmi_set = false;
        }

        fn reset(&mut self) {
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
