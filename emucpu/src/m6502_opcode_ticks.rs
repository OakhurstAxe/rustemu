
pub mod mopcodeticks {

    #[derive(Default)]
    pub struct OpCodeTicks {
        pub op_code_ticks: Vec<u8>,
    }
    
    impl OpCodeTicks {
        pub fn new () -> OpCodeTicks {
            
            Self {
                op_code_ticks: OpCodeTicks::set_ticks(),
            }
        }
    
        pub fn check_ticks(&self, program_counter: u16, op_code: u16, actual_ticks: u8) {

            if actual_ticks != self.op_code_ticks[op_code as usize] &&
                op_code != 0xd0 && op_code != 0xb0 && op_code != 0xf0 && op_code != 0x10 &&
                op_code != 0x90 && op_code != 0x4c && op_code != 0x30 {
                println!("PC: {:x}, opcode: {:x} tickcount: {} expected: {}", program_counter, op_code, actual_ticks, self.op_code_ticks[op_code as usize]);
            }
        }

        fn set_ticks() -> Vec<u8> {
            let mut op_code_ticks: Vec<u8> = Vec::with_capacity(0x200);
            for _i in 0..0x200 {
                op_code_ticks.push(1);
            }

            op_code_ticks[0x01] = 6;
            op_code_ticks[0x02] = 7;
            op_code_ticks[0x05] = 3;
            op_code_ticks[0x06] = 5;
            op_code_ticks[0x08] = 3;
            op_code_ticks[0x09] = 2;
            op_code_ticks[0x0a] = 2;

            op_code_ticks[0x10] = 2;
            op_code_ticks[0x11] = 5;
            op_code_ticks[0x15] = 4;
            op_code_ticks[0x18] = 2;
            op_code_ticks[0x19] = 4;

            op_code_ticks[0x20] = 6;
            op_code_ticks[0x24] = 3;
            op_code_ticks[0x25] = 3;
            op_code_ticks[0x26] = 5;
            op_code_ticks[0x29] = 2;
            op_code_ticks[0x2a] = 2;
            op_code_ticks[0x2c] = 4;

            op_code_ticks[0x30] = 2;
            op_code_ticks[0x35] = 4;
            op_code_ticks[0x38] = 2;
            op_code_ticks[0x3d] = 4;

            op_code_ticks[0x45] = 3;
            op_code_ticks[0x46] = 5;
            op_code_ticks[0x48] = 3;
            op_code_ticks[0x49] = 2;
            op_code_ticks[0x4a] = 2;
            op_code_ticks[0x4c] = 3;
            op_code_ticks[0x4e] = 6;

            op_code_ticks[0x50] = 2;
            op_code_ticks[0x56] = 6;

            op_code_ticks[0x60] = 6;
            op_code_ticks[0x65] = 3;
            op_code_ticks[0x66] = 5;
            op_code_ticks[0x68] = 4;
            op_code_ticks[0x69] = 2;
            op_code_ticks[0x6a] = 2;

            op_code_ticks[0x70] = 2;
            op_code_ticks[0x75] = 4;
            op_code_ticks[0x78] = 2;
            op_code_ticks[0x79] = 4;
            op_code_ticks[0x7d] = 4;

            op_code_ticks[0x84] = 3;
            op_code_ticks[0x85] = 3;
            op_code_ticks[0x86] = 3;
            op_code_ticks[0x88] = 2;
            op_code_ticks[0x8a] = 2;
            op_code_ticks[0x8c] = 4;
            op_code_ticks[0x8d] = 4;

            op_code_ticks[0x90] = 2;
            op_code_ticks[0x91] = 6;
            op_code_ticks[0x94] = 4;
            op_code_ticks[0x95] = 4;
            op_code_ticks[0x96] = 4;
            op_code_ticks[0x98] = 2;
            op_code_ticks[0x99] = 5;
            op_code_ticks[0x9a] = 2;

            op_code_ticks[0xa0] = 2;
            op_code_ticks[0xa1] = 6;
            op_code_ticks[0xa2] = 2;
            op_code_ticks[0xa4] = 3;
            op_code_ticks[0xa5] = 3;
            op_code_ticks[0xa6] = 3;
            op_code_ticks[0xa8] = 2;
            op_code_ticks[0xa9] = 2;
            op_code_ticks[0xaa] = 2;
            op_code_ticks[0xad] = 4;
            op_code_ticks[0xae] = 4;

            op_code_ticks[0xb0] = 2;
            op_code_ticks[0xb1] = 5;
            op_code_ticks[0xb4] = 4;
            op_code_ticks[0xb5] = 4;
            op_code_ticks[0xb6] = 4;
            op_code_ticks[0xb9] = 4;
            op_code_ticks[0xba] = 2;
            op_code_ticks[0xbc] = 4;
            op_code_ticks[0xbd] = 4;
            op_code_ticks[0xbe] = 4;

            op_code_ticks[0xc0] = 2;
            op_code_ticks[0xc4] = 3;
            op_code_ticks[0xc5] = 3;
            op_code_ticks[0xc6] = 5;
            op_code_ticks[0xc8] = 2;
            op_code_ticks[0xc9] = 2;
            op_code_ticks[0xca] = 2;

            op_code_ticks[0xd0] = 2;
            op_code_ticks[0xd1] = 5;
            op_code_ticks[0xd5] = 4;
            op_code_ticks[0xd6] = 6;
            op_code_ticks[0xd8] = 2;
            op_code_ticks[0xd9] = 4;
            op_code_ticks[0xdd] = 4;

            op_code_ticks[0xe0] = 2;
            op_code_ticks[0xe4] = 3;
            op_code_ticks[0xe5] = 3;
            op_code_ticks[0xe6] = 5;
            op_code_ticks[0xe8] = 2;
            op_code_ticks[0xe9] = 2;
            op_code_ticks[0xea] = 2;
            op_code_ticks[0xec] = 4;

            op_code_ticks[0xf0] = 2;
            op_code_ticks[0xf6] = 6;
            op_code_ticks[0xf8] = 2;
            op_code_ticks[0xf9] = 4;
            op_code_ticks[0xfd] = 4;
            op_code_ticks[0xff] = 7;

            op_code_ticks
        }
    }
}
