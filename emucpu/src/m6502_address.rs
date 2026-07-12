


pub mod maddress{

    use crate::m6502::emu_cpu::M6502;
    use crate::m6502::emu_cpu::AddressBus;

    pub struct AddressOpCodes {}
    impl AddressOpCodes {
        pub fn get_address_methods() -> Vec<Box<dyn AddressMethod>> {
            let mut op_code_lookup: Vec<Box<dyn AddressMethod>> = Vec::with_capacity(0x200);
            for _i in 0..0x200 {
                op_code_lookup.push(Box::new(AddressMethodError {}));
            }
            op_code_lookup[0x00] = Box::new(AddressMethodNull {});
            op_code_lookup[0x01] = Box::new(AddressMethodIndirectX {});
            op_code_lookup[0x02] = Box::new(AddressMethodNull {});
            op_code_lookup[0x03] = Box::new(AddressMethodIndirectX {});
            op_code_lookup[0x04] = Box::new(AddressMethodZero {});
            op_code_lookup[0x05] = Box::new(AddressMethodZero {});
            op_code_lookup[0x06] = Box::new(AddressMethodZero {});
            op_code_lookup[0x07] = Box::new(AddressMethodZero {});
            op_code_lookup[0x08] = Box::new(AddressMethodNull {});
            op_code_lookup[0x09] = Box::new(AddressMethodImmediate {});
            op_code_lookup[0x0a] = Box::new(AddressMethodAccumulator {});
            op_code_lookup[0x0b] = Box::new(AddressMethodImmediate {});
            op_code_lookup[0x0c] = Box::new(AddressMethodAbsolute {});
            op_code_lookup[0x0d] = Box::new(AddressMethodAbsolute {});
            op_code_lookup[0x0e] = Box::new(AddressMethodAbsolute {});
            op_code_lookup[0x0f] = Box::new(AddressMethodAbsolute {});

            op_code_lookup[0x10] = Box::new(AddressMethodImmediate {});
            op_code_lookup[0x11] = Box::new(AddressMethodIndirectY {});
            op_code_lookup[0x12] = Box::new(AddressMethodNull {});
            op_code_lookup[0x13] = Box::new(AddressMethodIndirectY {});
            op_code_lookup[0x14] = Box::new(AddressMethodZeroX {});
            op_code_lookup[0x15] = Box::new(AddressMethodZeroX {});
            op_code_lookup[0x16] = Box::new(AddressMethodZeroX {});
            op_code_lookup[0x17] = Box::new(AddressMethodZeroX {});
            op_code_lookup[0x18] = Box::new(AddressMethodNull {});
            op_code_lookup[0x19] = Box::new(AddressMethodAbsoluteY {});
            op_code_lookup[0x1a] = Box::new(AddressMethodNull {});
            op_code_lookup[0x1b] = Box::new(AddressMethodAbsoluteY {});
            op_code_lookup[0x1c] = Box::new(AddressMethodAbsoluteX {});
            op_code_lookup[0x1d] = Box::new(AddressMethodAbsoluteX {});
            op_code_lookup[0x1e] = Box::new(AddressMethodAbsoluteX {});
            op_code_lookup[0x1f] = Box::new(AddressMethodAbsoluteX {});

            op_code_lookup[0x20] = Box::new(AddressMethodAbsolute {});
            op_code_lookup[0x21] = Box::new(AddressMethodIndirectX {});
            op_code_lookup[0x22] = Box::new(AddressMethodIndirectX {});
            op_code_lookup[0x23] = Box::new(AddressMethodIndirectX {});
            op_code_lookup[0x24] = Box::new(AddressMethodZero {});
            op_code_lookup[0x25] = Box::new(AddressMethodZero {});
            op_code_lookup[0x26] = Box::new(AddressMethodZero {});
            op_code_lookup[0x27] = Box::new(AddressMethodZero {});
            op_code_lookup[0x28] = Box::new(AddressMethodNull {});
            op_code_lookup[0x29] = Box::new(AddressMethodImmediate {});
            op_code_lookup[0x2a] = Box::new(AddressMethodAccumulator {});
            op_code_lookup[0x2b] = Box::new(AddressMethodImmediate {});
            op_code_lookup[0x2c] = Box::new(AddressMethodAbsolute {});
            op_code_lookup[0x2d] = Box::new(AddressMethodAbsolute {});
            op_code_lookup[0x2e] = Box::new(AddressMethodAbsolute {});
            op_code_lookup[0x2f] = Box::new(AddressMethodAbsolute {});

            op_code_lookup[0x30] = Box::new(AddressMethodImmediate {});
            op_code_lookup[0x31] = Box::new(AddressMethodIndirectY {});
            op_code_lookup[0x32] = Box::new(AddressMethodNull {});
            op_code_lookup[0x33] = Box::new(AddressMethodIndirectY {});
            op_code_lookup[0x34] = Box::new(AddressMethodZeroX {});
            op_code_lookup[0x35] = Box::new(AddressMethodZeroX {});
            op_code_lookup[0x36] = Box::new(AddressMethodZeroX {});
            op_code_lookup[0x37] = Box::new(AddressMethodZeroX {});
            op_code_lookup[0x38] = Box::new(AddressMethodNull {});
            op_code_lookup[0x39] = Box::new(AddressMethodAbsoluteY {});
            op_code_lookup[0x3a] = Box::new(AddressMethodNull {});
            op_code_lookup[0x3b] = Box::new(AddressMethodAbsoluteY {});
            op_code_lookup[0x3c] = Box::new(AddressMethodAbsoluteX {});
            op_code_lookup[0x3d] = Box::new(AddressMethodAbsoluteX {});
            op_code_lookup[0x3e] = Box::new(AddressMethodAbsoluteX {});
            op_code_lookup[0x3f] = Box::new(AddressMethodAbsoluteX {});

            op_code_lookup[0x40] = Box::new(AddressMethodNull {});
            op_code_lookup[0x41] = Box::new(AddressMethodIndirectX {});
            op_code_lookup[0x42] = Box::new(AddressMethodNull {});
            op_code_lookup[0x43] = Box::new(AddressMethodIndirectX {});
            op_code_lookup[0x44] = Box::new(AddressMethodZero {});
            op_code_lookup[0x45] = Box::new(AddressMethodZero {});
            op_code_lookup[0x46] = Box::new(AddressMethodZero {});
            op_code_lookup[0x47] = Box::new(AddressMethodZero {});
            op_code_lookup[0x48] = Box::new(AddressMethodNull {});
            op_code_lookup[0x49] = Box::new(AddressMethodImmediate {});
            op_code_lookup[0x4a] = Box::new(AddressMethodAccumulator {});
            op_code_lookup[0x4b] = Box::new(AddressMethodImmediate {});
            op_code_lookup[0x4c] = Box::new(AddressMethodAbsolute {});
            op_code_lookup[0x4d] = Box::new(AddressMethodAbsolute {});
            op_code_lookup[0x4e] = Box::new(AddressMethodAbsolute {});
            op_code_lookup[0x4f] = Box::new(AddressMethodAbsolute {});

            op_code_lookup[0x50] = Box::new(AddressMethodImmediate {});
            op_code_lookup[0x51] = Box::new(AddressMethodIndirectY {});
            op_code_lookup[0x52] = Box::new(AddressMethodNull {});
            op_code_lookup[0x53] = Box::new(AddressMethodIndirectY {});
            op_code_lookup[0x54] = Box::new(AddressMethodZeroX {});
            op_code_lookup[0x55] = Box::new(AddressMethodZeroX {});
            op_code_lookup[0x56] = Box::new(AddressMethodZeroX {});
            op_code_lookup[0x57] = Box::new(AddressMethodZeroX {});
            op_code_lookup[0x58] = Box::new(AddressMethodNull {});
            op_code_lookup[0x59] = Box::new(AddressMethodAbsoluteY {});
            op_code_lookup[0x5a] = Box::new(AddressMethodNull {});
            op_code_lookup[0x5b] = Box::new(AddressMethodAbsoluteY {});
            op_code_lookup[0x5c] = Box::new(AddressMethodAbsoluteX {});
            op_code_lookup[0x5d] = Box::new(AddressMethodAbsoluteX {});
            op_code_lookup[0x5e] = Box::new(AddressMethodAbsoluteX {});
            op_code_lookup[0x5f] = Box::new(AddressMethodAbsoluteX {});

            op_code_lookup[0x60] = Box::new(AddressMethodNull {});
            op_code_lookup[0x61] = Box::new(AddressMethodIndirectX {});
            op_code_lookup[0x62] = Box::new(AddressMethodNull {});
            op_code_lookup[0x63] = Box::new(AddressMethodIndirectX {});
            op_code_lookup[0x64] = Box::new(AddressMethodZero {});
            op_code_lookup[0x65] = Box::new(AddressMethodZero {});
            op_code_lookup[0x66] = Box::new(AddressMethodZero {});
            op_code_lookup[0x67] = Box::new(AddressMethodZero {});
            op_code_lookup[0x68] = Box::new(AddressMethodNull {});
            op_code_lookup[0x69] = Box::new(AddressMethodImmediate {});
            op_code_lookup[0x6a] = Box::new(AddressMethodAccumulator {});
            op_code_lookup[0x6b] = Box::new(AddressMethodImmediate {});
            op_code_lookup[0x6c] = Box::new(AddressMethodIndirect {});
            op_code_lookup[0x6d] = Box::new(AddressMethodAbsolute {});
            op_code_lookup[0x6e] = Box::new(AddressMethodAbsolute {});
            op_code_lookup[0x6f] = Box::new(AddressMethodAbsolute {});

            op_code_lookup[0x70] = Box::new(AddressMethodImmediate {});
            op_code_lookup[0x71] = Box::new(AddressMethodIndirectY {});
            op_code_lookup[0x72] = Box::new(AddressMethodNull {});
            op_code_lookup[0x73] = Box::new(AddressMethodIndirectY {});
            op_code_lookup[0x74] = Box::new(AddressMethodZeroX {});
            op_code_lookup[0x75] = Box::new(AddressMethodZeroX {});
            op_code_lookup[0x76] = Box::new(AddressMethodZeroX {});
            op_code_lookup[0x77] = Box::new(AddressMethodZeroX {});
            op_code_lookup[0x78] = Box::new(AddressMethodNull {});
            op_code_lookup[0x79] = Box::new(AddressMethodAbsoluteY {});
            op_code_lookup[0x7a] = Box::new(AddressMethodNull {});
            op_code_lookup[0x7b] = Box::new(AddressMethodAbsoluteY {});
            op_code_lookup[0x7c] = Box::new(AddressMethodAbsoluteX {});
            op_code_lookup[0x7d] = Box::new(AddressMethodAbsoluteX {});
            op_code_lookup[0x7e] = Box::new(AddressMethodAbsoluteX {});
            op_code_lookup[0x7f] = Box::new(AddressMethodAbsoluteX {});

            op_code_lookup[0x80] = Box::new(AddressMethodImmediate {});
            op_code_lookup[0x81] = Box::new(AddressMethodIndirectX {});
            op_code_lookup[0x82] = Box::new(AddressMethodImmediate {});
            op_code_lookup[0x83] = Box::new(AddressMethodIndirectX {});
            op_code_lookup[0x84] = Box::new(AddressMethodZero {});
            op_code_lookup[0x85] = Box::new(AddressMethodZero {});
            op_code_lookup[0x86] = Box::new(AddressMethodZero {});
            op_code_lookup[0x87] = Box::new(AddressMethodZero {});
            op_code_lookup[0x88] = Box::new(AddressMethodNull {});
            op_code_lookup[0x89] = Box::new(AddressMethodImmediate {});
            op_code_lookup[0x8a] = Box::new(AddressMethodNull {});
            op_code_lookup[0x8b] = Box::new(AddressMethodImmediate {});
            op_code_lookup[0x8c] = Box::new(AddressMethodAbsolute {});
            op_code_lookup[0x8d] = Box::new(AddressMethodAbsolute {});
            op_code_lookup[0x8e] = Box::new(AddressMethodAbsolute {});
            op_code_lookup[0x8f] = Box::new(AddressMethodAbsolute {});

            op_code_lookup[0x90] = Box::new(AddressMethodImmediate {});
            op_code_lookup[0x91] = Box::new(AddressMethodIndirectY {});
            op_code_lookup[0x92] = Box::new(AddressMethodNull {});
            op_code_lookup[0x93] = Box::new(AddressMethodIndirectY {});
            op_code_lookup[0x94] = Box::new(AddressMethodZeroX {});
            op_code_lookup[0x95] = Box::new(AddressMethodZeroX {});
            op_code_lookup[0x96] = Box::new(AddressMethodZeroY {});
            op_code_lookup[0x97] = Box::new(AddressMethodZeroY {});
            op_code_lookup[0x98] = Box::new(AddressMethodNull {});
            op_code_lookup[0x99] = Box::new(AddressMethodAbsoluteY {});
            op_code_lookup[0x9a] = Box::new(AddressMethodNull {});
            op_code_lookup[0x9b] = Box::new(AddressMethodAbsoluteY {});
            op_code_lookup[0x9c] = Box::new(AddressMethodAbsoluteX {});
            op_code_lookup[0x9d] = Box::new(AddressMethodAbsoluteX {});
            op_code_lookup[0x9e] = Box::new(AddressMethodAbsoluteY {});
            op_code_lookup[0x9f] = Box::new(AddressMethodAbsoluteY {});

            op_code_lookup[0xa0] = Box::new(AddressMethodImmediate {});
            op_code_lookup[0xa1] = Box::new(AddressMethodIndirectX {});
            op_code_lookup[0xa2] = Box::new(AddressMethodImmediate {});
            op_code_lookup[0xa3] = Box::new(AddressMethodIndirectX {});
            op_code_lookup[0xa4] = Box::new(AddressMethodZero {});
            op_code_lookup[0xa5] = Box::new(AddressMethodZero {});
            op_code_lookup[0xa6] = Box::new(AddressMethodZero {});
            op_code_lookup[0xa7] = Box::new(AddressMethodZero {});
            op_code_lookup[0xa8] = Box::new(AddressMethodNull {});
            op_code_lookup[0xa9] = Box::new(AddressMethodImmediate {});
            op_code_lookup[0xaa] = Box::new(AddressMethodNull {});
            op_code_lookup[0xab] = Box::new(AddressMethodImmediate {});
            op_code_lookup[0xac] = Box::new(AddressMethodAbsolute {});
            op_code_lookup[0xad] = Box::new(AddressMethodAbsolute {});
            op_code_lookup[0xae] = Box::new(AddressMethodAbsolute {});
            op_code_lookup[0xaf] = Box::new(AddressMethodAbsolute {});

            op_code_lookup[0xb0] = Box::new(AddressMethodImmediate {});
            op_code_lookup[0xb1] = Box::new(AddressMethodIndirectY {});
            op_code_lookup[0xb2] = Box::new(AddressMethodNull {});
            op_code_lookup[0xb3] = Box::new(AddressMethodIndirectY {});
            op_code_lookup[0xb4] = Box::new(AddressMethodZeroX {});
            op_code_lookup[0xb5] = Box::new(AddressMethodZeroX {});
            op_code_lookup[0xb6] = Box::new(AddressMethodZeroY {});
            op_code_lookup[0xb7] = Box::new(AddressMethodZeroY {});
            op_code_lookup[0xb8] = Box::new(AddressMethodNull {});
            op_code_lookup[0xb9] = Box::new(AddressMethodAbsoluteY {});
            op_code_lookup[0xba] = Box::new(AddressMethodNull {});
            op_code_lookup[0xbb] = Box::new(AddressMethodAbsoluteY {});
            op_code_lookup[0xbc] = Box::new(AddressMethodAbsoluteX {});
            op_code_lookup[0xbd] = Box::new(AddressMethodAbsoluteX {});
            op_code_lookup[0xbe] = Box::new(AddressMethodAbsoluteY {});
            op_code_lookup[0xbf] = Box::new(AddressMethodIndirectY {});

            op_code_lookup[0xc0] = Box::new(AddressMethodImmediate {});
            op_code_lookup[0xc1] = Box::new(AddressMethodIndirectX {});
            op_code_lookup[0xc2] = Box::new(AddressMethodImmediate {});
            op_code_lookup[0xc3] = Box::new(AddressMethodIndirectX {});
            op_code_lookup[0xc4] = Box::new(AddressMethodZero {});
            op_code_lookup[0xc5] = Box::new(AddressMethodZero {});
            op_code_lookup[0xc6] = Box::new(AddressMethodZero {});
            op_code_lookup[0xc7] = Box::new(AddressMethodZero {});
            op_code_lookup[0xc8] = Box::new(AddressMethodNull {});
            op_code_lookup[0xc9] = Box::new(AddressMethodImmediate {});
            op_code_lookup[0xca] = Box::new(AddressMethodNull {});
            op_code_lookup[0xcb] = Box::new(AddressMethodImmediate {});
            op_code_lookup[0xcc] = Box::new(AddressMethodAbsolute {});
            op_code_lookup[0xcd] = Box::new(AddressMethodAbsolute {});
            op_code_lookup[0xce] = Box::new(AddressMethodAbsolute {});
            op_code_lookup[0xcf] = Box::new(AddressMethodAbsolute {});

            op_code_lookup[0xd0] = Box::new(AddressMethodImmediate {});
            op_code_lookup[0xd1] = Box::new(AddressMethodIndirectY {});
            op_code_lookup[0xd2] = Box::new(AddressMethodIndirectY {});
            op_code_lookup[0xd3] = Box::new(AddressMethodIndirectY {});
            op_code_lookup[0xd4] = Box::new(AddressMethodZeroX {});
            op_code_lookup[0xd5] = Box::new(AddressMethodZeroX {});
            op_code_lookup[0xd6] = Box::new(AddressMethodZeroX {});
            op_code_lookup[0xd7] = Box::new(AddressMethodZeroX {});
            op_code_lookup[0xd8] = Box::new(AddressMethodNull {});
            op_code_lookup[0xd9] = Box::new(AddressMethodAbsoluteY {});
            op_code_lookup[0xda] = Box::new(AddressMethodNull {});
            op_code_lookup[0xdb] = Box::new(AddressMethodAbsoluteY {});
            op_code_lookup[0xdc] = Box::new(AddressMethodAbsoluteX {});
            op_code_lookup[0xdd] = Box::new(AddressMethodAbsoluteX {});
            op_code_lookup[0xde] = Box::new(AddressMethodAbsoluteX {});
            op_code_lookup[0xdf] = Box::new(AddressMethodAbsoluteX {});

            op_code_lookup[0xe0] = Box::new(AddressMethodImmediate {});
            op_code_lookup[0xe1] = Box::new(AddressMethodIndirectX {});
            op_code_lookup[0xe2] = Box::new(AddressMethodImmediate {});
            op_code_lookup[0xe3] = Box::new(AddressMethodIndirectX {});
            op_code_lookup[0xe4] = Box::new(AddressMethodZero {});
            op_code_lookup[0xe5] = Box::new(AddressMethodZero {});
            op_code_lookup[0xe6] = Box::new(AddressMethodZero {});
            op_code_lookup[0xe7] = Box::new(AddressMethodZero {});
            op_code_lookup[0xe8] = Box::new(AddressMethodNull {});
            op_code_lookup[0xe9] = Box::new(AddressMethodImmediate {});
            op_code_lookup[0xea] = Box::new(AddressMethodNull {});
            op_code_lookup[0xeb] = Box::new(AddressMethodImmediate {});
            op_code_lookup[0xec] = Box::new(AddressMethodAbsolute {});
            op_code_lookup[0xed] = Box::new(AddressMethodAbsolute {});
            op_code_lookup[0xee] = Box::new(AddressMethodAbsolute {});
            op_code_lookup[0xef] = Box::new(AddressMethodAbsolute {});

            op_code_lookup[0xf0] = Box::new(AddressMethodImmediate {});
            op_code_lookup[0xf1] = Box::new(AddressMethodIndirectY {});
            op_code_lookup[0xf2] = Box::new(AddressMethodNull {});
            op_code_lookup[0xf3] = Box::new(AddressMethodIndirectY {});
            op_code_lookup[0xf4] = Box::new(AddressMethodZeroX {});
            op_code_lookup[0xf5] = Box::new(AddressMethodZeroX {});
            op_code_lookup[0xf6] = Box::new(AddressMethodZeroX {});
            op_code_lookup[0xf7] = Box::new(AddressMethodZeroX {});
            op_code_lookup[0xf8] = Box::new(AddressMethodNull {});
            op_code_lookup[0xf9] = Box::new(AddressMethodAbsolute {});
            op_code_lookup[0xfa] = Box::new(AddressMethodNull {});
            op_code_lookup[0xfb] = Box::new(AddressMethodAbsoluteY {});
            op_code_lookup[0xfc] = Box::new(AddressMethodAbsoluteX {});
            op_code_lookup[0xfd] = Box::new(AddressMethodAbsoluteX {});
            op_code_lookup[0xfe] = Box::new(AddressMethodAbsoluteX {});
            op_code_lookup[0xff] = Box::new(AddressMethodAbsoluteX {});

            op_code_lookup[0x100] = Box::new(AddressMethodNull {});
            op_code_lookup[0x101] = Box::new(AddressMethodNull {});
            op_code_lookup[0x102] = Box::new(AddressMethodNull {});
            op_code_lookup
        }
    }

    pub trait AddressMethod: Sync + Send {
        fn execute(&self, cpu: &mut M6502, addr: &mut AddressBus, step: u8) -> bool {
            let mut result = false;
            match step {
                0 => result = self.step_0(cpu, addr),
                1 => result = self.step_1(cpu, addr),
                2 => result = self.step_2(cpu, addr),
                3 => result = self.step_3(cpu, addr),
                4 => result = self.step_4(cpu, addr),
                _ => {}
            }
            result            
        }
        fn step_0(&self, _cpu: &mut M6502, _addr: &mut AddressBus) -> bool { true }
        fn step_1(&self, _cpu: &mut M6502, _addr: &mut AddressBus) -> bool { true }
        fn step_2(&self, _cpu: &mut M6502, _addr: &mut AddressBus) -> bool { true }
        fn step_3(&self, _cpu: &mut M6502, _addr: &mut AddressBus) -> bool { true }
        fn step_4(&self, _cpu: &mut M6502, _addr: &mut AddressBus) -> bool { true }
    }

    pub struct AddressMethodError {}
    impl AddressMethod for AddressMethodError {
        fn step_0(&self, _cpu: &mut M6502, _addr: &mut AddressBus) -> bool {
            panic!("Error address method");
        }
    }

    pub struct AddressMethodNull {}
    impl AddressMethod for AddressMethodNull {
        fn step_0(&self, cpu: &mut M6502, _addr: &mut AddressBus) -> bool {
            cpu.lookup_address.address = 0;
            true
        }
    }

    pub struct AddressMethodAccumulator {}
    impl AddressMethod for AddressMethodAccumulator {
        fn step_0(&self, cpu: &mut M6502, addr: &mut AddressBus) -> bool {
            addr.address = cpu.accumulator as u16;
            addr.byte = cpu.accumulator;
            addr.is_accumulator = true;
            cpu.lookup_address.address = addr.address;
            cpu.lookup_address.byte = addr.byte;
            cpu.lookup_address.is_accumulator = addr.is_accumulator;
            true
        }
    }

    pub struct AddressMethodImmediate {}
    impl AddressMethod for AddressMethodImmediate {
        fn step_0(&self, cpu: &mut M6502, addr: &mut AddressBus) -> bool {
            addr.address = cpu.program_counter;
            cpu.program_counter += 1;
            cpu.lookup_address.address = addr.address;
            cpu.lookup_address.byte = addr.byte;
            cpu.lookup_address.is_accumulator = addr.is_accumulator;
            true
        }
    }

    pub struct AddressMethodZero {}
    impl AddressMethod for AddressMethodZero {
        fn step_0(&self, cpu: &mut M6502, addr: &mut AddressBus) -> bool {
            addr.address = cpu.program_counter;
            cpu.program_counter += 1;
            false
        }
        fn step_1(&self, cpu: &mut M6502, addr: &mut AddressBus) -> bool {
            addr.address = addr.byte as u16;
            cpu.lookup_address.address = addr.byte as u16;
            cpu.lookup_address.byte = addr.byte;
            cpu.lookup_address.is_accumulator = addr.is_accumulator;
            true
        }
    }

    pub struct AddressMethodZeroX {}
    impl AddressMethod for AddressMethodZeroX {
        fn step_0(&self, cpu: &mut M6502, addr: &mut AddressBus) -> bool {
            addr.address = cpu.program_counter;
            cpu.program_counter += 1;
            false
        }
        fn step_1(&self, cpu: &mut M6502, addr: &mut AddressBus) -> bool {
            addr.address = (addr.byte.overflowing_add(cpu.register_x).0) as u16;
            false        
        }
        fn step_2(&self, cpu: &mut M6502, addr: &mut AddressBus) -> bool {
            cpu.lookup_address.address = addr.address;
            cpu.lookup_address.byte = addr.byte;
            cpu.lookup_address.is_accumulator = addr.is_accumulator;
            true
        }
    }

    pub struct AddressMethodZeroY {}
    impl AddressMethod for AddressMethodZeroY {
        fn step_0(&self, cpu: &mut M6502, addr: &mut AddressBus) -> bool {
            addr.address = cpu.program_counter;
            cpu.program_counter += 1;
            false
        }
        fn step_1(&self, cpu: &mut M6502, addr: &mut AddressBus) -> bool {
            addr.address = (addr.byte.overflowing_add(cpu.register_y).0) as u16;
            false        
        }
        fn step_2(&self, cpu: &mut M6502, addr: &mut AddressBus) -> bool {
            cpu.lookup_address.address = addr.address;
            cpu.lookup_address.byte = addr.byte;
            cpu.lookup_address.is_accumulator = addr.is_accumulator;
            true
        }
    }

    pub struct AddressMethodAbsolute {}
    impl AddressMethod for AddressMethodAbsolute {
        fn step_0(&self, cpu: &mut M6502, addr: &mut AddressBus) -> bool {
            addr.address = cpu.program_counter;
            cpu.program_counter += 1;
            false
        }
        fn step_1(&self, cpu: &mut M6502, addr: &mut AddressBus) -> bool {
            cpu.lookup_address.address = addr.byte as u16;
            addr.address = cpu.program_counter;
            cpu.program_counter += 1;
            false
        }
        fn step_2(&self, cpu: &mut M6502, addr: &mut AddressBus) -> bool {
            cpu.lookup_address.address |= (addr.byte as u16) << 8;
            cpu.lookup_address.byte = addr.byte;
            cpu.lookup_address.is_accumulator = addr.is_accumulator;
            addr.address = cpu.lookup_address.address;
            true
        }
    }

    pub struct AddressMethodAbsoluteX {}
    impl AddressMethod for AddressMethodAbsoluteX {
        fn step_0(&self, cpu: &mut M6502, addr: &mut AddressBus) -> bool {
            addr.address = cpu.program_counter;
            cpu.program_counter += 1;
            false
        }
        fn step_1(&self, cpu: &mut M6502, addr: &mut AddressBus) -> bool {
            addr.address = cpu.program_counter;
            cpu.program_counter += 1;
            cpu.lookup_address.address = addr.byte as u16;
            false
        }
        fn step_2(&self, cpu: &mut M6502, addr: &mut AddressBus) -> bool {
            // If jumping to another page, it reads without overflow and takes additional tick
            //let (value, overflow) = (cpu.lookup_address.address as u8).overflowing_add(cpu.register_x);
            //if overflow {
            //    addr.address = cpu.lookup_address.address + value as u16;
            //    cpu.lookup_address.address = cpu.lookup_address.address.overflowing_add((addr.byte as u16) << 8).0;
            //    cpu.lookup_address.address = cpu.lookup_address.address.overflowing_add(cpu.register_x as u16).0;
            //    return false;
            //}
            cpu.lookup_address.address += (addr.byte as u16) << 8;
            cpu.lookup_address.address = cpu.lookup_address.address.overflowing_add(cpu.register_x as u16).0;
            cpu.lookup_address.byte = addr.byte;
            cpu.lookup_address.is_accumulator = addr.is_accumulator;
            addr.address = cpu.lookup_address.address;
            true
        }
        fn step_3(&self, cpu: &mut M6502, addr: &mut AddressBus) -> bool {
            addr.address = cpu.lookup_address.address;
            true
        }
    }

    pub struct AddressMethodAbsoluteY {}
    impl AddressMethod for AddressMethodAbsoluteY {
        fn step_0(&self, cpu: &mut M6502, addr: &mut AddressBus) -> bool {
            addr.address = cpu.program_counter;
            cpu.program_counter += 1;
            false
        }
        fn step_1(&self, cpu: &mut M6502, addr: &mut AddressBus) -> bool {
            cpu.lookup_address.address = addr.byte as u16;
            addr.address = cpu.program_counter;
            cpu.program_counter += 1;
            false
        }
        fn step_2(&self, cpu: &mut M6502, addr: &mut AddressBus) -> bool {
            // If jumping to another page, it reads without overflow and takes additional tick
            //let (value, overflow) = (cpu.lookup_address.address as u8).overflowing_add(cpu.register_y);
            //if overflow {
            //    addr.address = cpu.lookup_address.address + value as u16;
            //    cpu.lookup_address.address += (addr.byte as u16) << 8;
            //    cpu.lookup_address.address += cpu.register_y as u16;
            //    return false;
            //}
            cpu.lookup_address.address += (addr.byte as u16) << 8;
            cpu.lookup_address.address = cpu.lookup_address.address.overflowing_add(cpu.register_y as u16).0;
            cpu.lookup_address.byte = addr.byte;
            cpu.lookup_address.is_accumulator = addr.is_accumulator;
            addr.address = cpu.lookup_address.address;
            cpu.lookup_address.is_abs_y = true;
            true
        }
        fn step_3(&self, cpu: &mut M6502, addr: &mut AddressBus) -> bool {
            addr.address = cpu.lookup_address.address;
            true
        }
    }

    pub struct AddressMethodIndirect {}
    impl AddressMethod for AddressMethodIndirect {
        fn step_0(&self, cpu: &mut M6502, addr: &mut AddressBus) -> bool {
            addr.address = cpu.program_counter;
            cpu.program_counter += 1;
            cpu.lookup_address.address = 0;
            false
        }
        fn step_1(&self, cpu: &mut M6502, addr: &mut AddressBus) -> bool {
            cpu.lookup_address.address = addr.byte as u16;
            addr.address = cpu.program_counter;
            false
        }
        fn step_2(&self, cpu: &mut M6502, addr: &mut AddressBus) -> bool {
            cpu.lookup_address.address += (addr.byte as u16) << 8;
            addr.address = cpu.lookup_address.address;
            false
        }
        fn step_3(&self, cpu: &mut M6502, addr: &mut AddressBus) -> bool {
            cpu.lookup_address.address = addr.byte as u16;
            addr.address += 1;
            false
        }
        fn step_4(&self, cpu: &mut M6502, addr: &mut AddressBus) -> bool {
            cpu.lookup_address.address += (addr.byte as u16) << 8;
            cpu.lookup_address.byte = addr.byte;
            addr.address = cpu.lookup_address.address;
            true
        }
    }

    pub struct AddressMethodIndirectX {}
    impl AddressMethod for AddressMethodIndirectX {
        fn step_0(&self, cpu: &mut M6502, addr: &mut AddressBus) -> bool {
            addr.address = cpu.program_counter;
            cpu.program_counter += 1;
            cpu.lookup_address.address = 0;
            false
        }
        fn step_1(&self, cpu: &mut M6502, addr: &mut AddressBus) -> bool {
            addr.address = addr.byte.wrapping_add(cpu.register_x) as u16;
            false
        }
        fn step_2(&self, cpu: &mut M6502, addr: &mut AddressBus) -> bool {
            cpu.lookup_address.address = addr.byte as u16;
            addr.address += 1;
            false
        }
        fn step_3(&self, cpu: &mut M6502, addr: &mut AddressBus) -> bool {
            cpu.lookup_address.address += (addr.byte as u16) << 8;
            cpu.lookup_address.byte = addr.byte;
            cpu.lookup_address.is_accumulator = addr.is_accumulator;
            addr.address = cpu.lookup_address.address;
            false
        }
    }

    pub struct AddressMethodIndirectY {}
    impl AddressMethod for AddressMethodIndirectY {
        fn step_0(&self, cpu: &mut M6502, addr: &mut AddressBus) -> bool {
            addr.address = cpu.program_counter;
            cpu.program_counter += 1;
            false
        }
        fn step_1(&self, _cpu: &mut M6502, addr: &mut AddressBus) -> bool {
            addr.address = addr.byte as u16;
            false
        }
        fn step_2(&self, cpu: &mut M6502, addr: &mut AddressBus) -> bool {
            cpu.lookup_address.address = addr.byte as u16;
            addr.address += 1;
            false
        }
        fn step_3(&self, cpu: &mut M6502, addr: &mut AddressBus) -> bool {
            // If jumping to another page, it reads without overflow and takes additional tick
            let (value, overflow) = (cpu.lookup_address.address as u8).overflowing_add(cpu.register_y);
            if overflow {
                addr.address = cpu.lookup_address.address + value as u16;
                cpu.lookup_address.address += (addr.byte as u16) << 8;
                cpu.lookup_address.address += cpu.register_y as u16;
                return false;
            }
            cpu.lookup_address.address += (addr.byte as u16) << 8;
            cpu.lookup_address.address += cpu.register_y as u16;
            cpu.lookup_address.byte = addr.byte;
            cpu.lookup_address.is_accumulator = addr.is_accumulator;
            addr.address = cpu.lookup_address.address;
            true
        }
        fn step_4(&self, cpu: &mut M6502, addr: &mut AddressBus) -> bool {
            addr.address = cpu.lookup_address.address;
            true
        }
    }
}

