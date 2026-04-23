

pub mod nes {

    #[derive(Copy, Clone)]
    enum Palette {
        Palette2C03 = 0,
        //Palette_2C05 = 0,
        PaletteRC2C03B = 1,
        PaletteRP2C04_0001 = 2,
        PaletteRP2C04_0002 = 3,
        PaletteRP2C04_0003 = 4,
        PaletteRP2C04_0004 = 5
    }

    const ENTRIES_2C03: [u16; 64] = [
        333, 14,  6,326,403,503,510,420,320,120, 31, 40, 22,  0,  0,  0,
        555,036,027,407,507,704,700,630,430,140, 40, 53, 44,  0,  0,  0,
        777,357,447,637,707,737,740,750,660,360, 70,276, 77,  0,  0,  0,
        777,567,657,757,747,755,764,772,773,572,473,276,467,  0,  0,  0
    ];

    const ENTRIES_RC03B: [u16; 64] = [
        333, 14,  6,326,403,503,510,420,320,100, 31, 40, 22,  0,  0,  0,
        555, 16, 27,407,507,704,700,630,430,140, 40, 53, 44,  0,  0,  0,
        777,357,447,637,707,717,740,750,660,340, 70,276, 77,  0,  0,  0,
        777,547,657,757,747,755,764,772,773,552,473,276,467,  0,  0,  0
    ];
    
    const ENTRIES_RP2C04_0001: [u16; 64] = [
        755,637,700,447, 44,120,222,704,777,333,750,503,403,660,320,777,
        357,653,310,360,467,657,764,027,760,276,000,200,666,444,707, 14,
        3,567,757, 70, 77, 22, 53,507,  0,420,747,510,407,  6,740,  0,
        0,140,555, 31,572,326,770,630, 20, 36, 40,111,773,737,430,473
    ];

    const ENTRIES_RP2C04_0002: [u16; 64] = [
            0,750,430,572,473,737, 44,567,700,407,773,747,777,637,467,040,
            20,357,510,666, 53,360,200,447,222,707,  3,276,657,320,  0,326,
        403,764,740,757, 36,310,555,  6,507,760,333,120,027,  0,660,777,
        653,111, 70,630, 22, 14,704,140,  0, 77,420,770,755,503, 31,444
    ];

    const ENTRIES_RP2C04_0003: [u16; 64] = [
        507,737,473,555, 40,777,567,120, 14,  0,764,320,704,666,653,467,
        447,044,503, 27,140,430,630,053,333,326,  0,  6,700,510,747,755,
        637, 20,  3,770,111,750,740,777,360,403,357,707, 36,444,  0,310,
        77,200,572,757,420, 70,660,222, 31,  0,657,773,407,276,760, 22
    ];

    const ENTRIES_RP2C04_0004: [u16; 64] = [
        430,326, 44,660,  0,755, 14,630,555,310, 70,  3,764,770, 40,572,
        737,200,027,747,  0,222,510,740,653, 53,447,140,403,  0,473,357,
        503,031,420,  6,407,507,333,704,022,666, 36, 20,111,773,444,707,
        757,777,320,700,760,276,777,467,  0,750,637,567,360,657, 77,120
    ];
    
    pub struct NesPalette {
        palette: [[u8; 128 * 3]; 6],
    }

    impl NesPalette {
        
        pub fn new() -> Self {
            let mut temp_instance: NesPalette;
            
            temp_instance = NesPalette { 
                palette: [[0u8; 128 * 3]; 6],
            };
            temp_instance.setup_palettes();

            temp_instance
        }

        pub fn get_color(&self, mut position: usize, palette: usize) -> (u8, u8, u8) {

            if position > 64 {
                return (255, 255, 255);
            }

            position *= 3;
            (self.palette[palette][position],
             self.palette[palette][position + 1],
             self.palette[palette][position + 2])
        }

        fn setup_palettes(&mut self) {
            self.setup_palette(ENTRIES_2C03, Palette::Palette2C03);
            self.setup_palette(ENTRIES_RC03B, Palette::PaletteRC2C03B);
            self.setup_palette(ENTRIES_RP2C04_0001, Palette::PaletteRP2C04_0001);
            self.setup_palette(ENTRIES_RP2C04_0002, Palette::PaletteRP2C04_0002);
            self.setup_palette(ENTRIES_RP2C04_0003, Palette::PaletteRP2C04_0003);
            self.setup_palette(ENTRIES_RP2C04_0004, Palette::PaletteRP2C04_0004);
        }

        fn setup_palette(&mut self, entries: [u16; 64], palette: Palette)
        {
            for index in 0..64 {
                let entry: u16 = entries[index];
                let blue: u16 = entry % 10;
                let green: u16 = ((entry % 100) - blue) / 10;
                let red: u16 = (entry - green - blue) / 100;
                self.set_color((red * 25) as u8, (green * 25) as u8, (blue * 25) as u8, (index * 3) as usize, palette as usize);
            }
        }
        
        fn set_color(&mut self, red: u8, green: u8, blue: u8, position: usize, palette: usize) {            
            self.palette[palette][position] = red;
            self.palette[palette][position+1] = green;
            self.palette[palette][position+2] = blue;
        }
    }
}
