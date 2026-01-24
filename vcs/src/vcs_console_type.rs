
pub mod vcs {

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub enum ConsoleType { 
        NTSC, 
        PAL, 
        SECAM 
    }

    pub struct VcsConsoleType {
        console_type: ConsoleType
    }

    impl VcsConsoleType {
                
        pub fn new(_type: ConsoleType) -> VcsConsoleType {
            Self {
                console_type: _type,
            }
        }
        
        pub fn get_x_resolution(&self) -> u8{
            160
        }
        
        pub fn get_y_resolution(&self) -> u8 {
            let mut result = 228;

            if self.console_type == ConsoleType::NTSC {
                result = 210; // Should be 192
            }

            result
        }
        
        pub fn get_frames_per_second(&self) -> u8 {
            let mut result = 50;

            if self.console_type == ConsoleType::NTSC {
                result = 60;
            }

            result
        }
        
        pub fn get_v_blank_lines(&self) -> u8 {
            let mut result = 45;

            if self.console_type == ConsoleType::NTSC {
                result = 37;
            }

            result
        }
        
        pub fn ticks_per_second(&self) -> i32 {
            let mut result = 3546894;

            if self.console_type == ConsoleType::NTSC {
                //return 3584160;
                result = 3579545;
            }

            result
        }
        
        pub fn get_video_type(&self) -> ConsoleType {
            self.console_type
        }
    }
}