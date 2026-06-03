


pub mod naddress{

    use crate::n6502::emu_cpu::N6502;
    use crate::n6502::emu_cpu::AddressBus;

    pub struct AddressOpCodes {}
    impl AddressOpCodes {
        pub fn get_address_methods() -> Vec<Box<dyn AddressMethod>> {
            let mut op_code_lookup: Vec<Box<dyn AddressMethod>> = Vec::with_capacity(0x200);
            for _i in 0..256 {
                op_code_lookup.push(Box::new(AddressMethodAbsolute {}));
            }
            op_code_lookup[0x0a] = Box::new(AddressMethodAccumulator {});

            op_code_lookup[0x10] = Box::new(AddressMethodImmediate {});
            op_code_lookup[0x19] = Box::new(AddressMethodAbsoluteY {});

            op_code_lookup[0x20] = Box::new(AddressMethodAbsolute {});
            op_code_lookup[0x25] = Box::new(AddressMethodZero {});
            op_code_lookup[0x29] = Box::new(AddressMethodImmediate {});

            op_code_lookup[0x30] = Box::new(AddressMethodImmediate {});
            op_code_lookup[0x38] = Box::new(AddressMethodNull {});

            op_code_lookup[0x49] = Box::new(AddressMethodImmediate {});
            op_code_lookup[0x4a] = Box::new(AddressMethodAccumulator {});
            op_code_lookup[0x4c] = Box::new(AddressMethodAbsolute {});

            op_code_lookup[0x60] = Box::new(AddressMethodNull {});
            op_code_lookup[0x65] = Box::new(AddressMethodZero {});

            op_code_lookup[0x78] = Box::new(AddressMethodNull {});

            op_code_lookup[0x84] = Box::new(AddressMethodZero {});
            op_code_lookup[0x85] = Box::new(AddressMethodZero {});
            op_code_lookup[0x88] = Box::new(AddressMethodNull {});

            op_code_lookup[0x90] = Box::new(AddressMethodImmediate {});
            op_code_lookup[0x95] = Box::new(AddressMethodZeroX {});
            op_code_lookup[0x99] = Box::new(AddressMethodAbsoluteY {});
            op_code_lookup[0x9a] = Box::new(AddressMethodNull {});

            op_code_lookup[0xa0] = Box::new(AddressMethodImmediate {});
            op_code_lookup[0xa2] = Box::new(AddressMethodImmediate {});
            op_code_lookup[0xa4] = Box::new(AddressMethodZero {});
            op_code_lookup[0xa5] = Box::new(AddressMethodZero {});
            op_code_lookup[0xa9] = Box::new(AddressMethodImmediate {});
            op_code_lookup[0xad] = Box::new(AddressMethodAbsolute {});

            op_code_lookup[0xb0] = Box::new(AddressMethodImmediate {});
            op_code_lookup[0xb1] = Box::new(AddressMethodIndirectY {});
            op_code_lookup[0xb9] = Box::new(AddressMethodAbsoluteY {});

            op_code_lookup[0xc0] = Box::new(AddressMethodImmediate {});
            op_code_lookup[0xc8] = Box::new(AddressMethodNull {});
            op_code_lookup[0xc9] = Box::new(AddressMethodImmediate {});
            op_code_lookup[0xca] = Box::new(AddressMethodNull {});

            op_code_lookup[0xd0] = Box::new(AddressMethodImmediate {});
            op_code_lookup[0xd8] = Box::new(AddressMethodNull {});
            op_code_lookup[0xd9] = Box::new(AddressMethodAbsoluteY {});

            op_code_lookup[0xe6] = Box::new(AddressMethodZero {});
            op_code_lookup[0xe9] = Box::new(AddressMethodImmediate {});

            op_code_lookup[0xf0] = Box::new(AddressMethodImmediate {});
            op_code_lookup
        }
    }

    pub trait AddressMethod: Sync + Send {
        fn execute(&self, cpu: &mut N6502, addr: &mut AddressBus, step: u8, lookup: &mut AddressBus) -> bool {
            let mut result = false;
            match step {
                0 => result = self.step_0(cpu, addr, lookup),
                1 => result = self.step_1(cpu, addr, lookup),
                2 => result = self.step_2(cpu, addr, lookup),
                3 => result = self.step_3(cpu, addr, lookup),
                _ => {}
            }
            result            
        }
        fn step_0(&self, _cpu: &mut N6502, _addr: &mut AddressBus, _lookup: &mut AddressBus) -> bool { true }
        fn step_1(&self, _cpu: &mut N6502, _addr: &mut AddressBus, _lookup: &mut AddressBus) -> bool { true }
        fn step_2(&self, _cpu: &mut N6502, _addr: &mut AddressBus, _lookup: &mut AddressBus) -> bool { true }
        fn step_3(&self, _cpu: &mut N6502, _addr: &mut AddressBus, _lookup: &mut AddressBus) -> bool { true }
    }

    pub struct AddressMethodNull {}
    impl AddressMethod for AddressMethodNull {
        fn step_0(&self, _cpu: &mut N6502, _addr: &mut AddressBus, lookup: &mut AddressBus) -> bool {
            lookup.address = 0;
            true
        }
    }

    pub struct AddressMethodZero {}
    impl AddressMethod for AddressMethodZero {
        fn step_0(&self, cpu: &mut N6502, addr: &mut AddressBus, _lookup: &mut AddressBus) -> bool {
            addr.address = cpu.program_counter;
            addr.write = false;
            cpu.program_counter += 1;
            false
        }
        fn step_1(&self, _cpu: &mut N6502, addr: &mut AddressBus, lookup: &mut AddressBus) -> bool {
            lookup.address = addr.byte as u16;
            true
        }
    }

    pub struct AddressMethodAccumulator {}
    impl AddressMethod for AddressMethodAccumulator {
        fn step_0(&self, cpu: &mut N6502, addr: &mut AddressBus, lookup: &mut AddressBus) -> bool {
            lookup.address = cpu.accumulator as u16;
            true
        }
    }

    pub struct AddressMethodImmediate {}
    impl AddressMethod for AddressMethodImmediate {
        fn step_0(&self, cpu: &mut N6502, addr: &mut AddressBus, lookup: &mut AddressBus) -> bool {
            lookup.address = cpu.program_counter;
            cpu.program_counter += 1;
            true
        }
    }

    pub struct AddressMethodZeroX {}
    impl AddressMethod for AddressMethodZeroX {
        fn step_0(&self, cpu: &mut N6502, addr: &mut AddressBus, _lookup: &mut AddressBus) -> bool {
            addr.address = cpu.program_counter;
            addr.write = false;
            cpu.program_counter += 1;
            false
        }
        fn step_1(&self, cpu: &mut N6502, addr: &mut AddressBus, lookup: &mut AddressBus) -> bool {
            lookup.address = addr.byte as u16 + cpu.register_x as u16;
            if addr.byte as u16 & 0x00ff + (cpu.register_x as u16) > 0x00ff {
                return false;
            }
            true
        }
        fn step_2(&self, _cpu: &mut N6502, _addr: &mut AddressBus, _lookup: &mut AddressBus) -> bool {
            true
        }
    }

    pub struct AddressMethodAbsolute {}
    impl AddressMethod for AddressMethodAbsolute {
        fn step_0(&self, cpu: &mut N6502, addr: &mut AddressBus, lookup: &mut AddressBus) -> bool {
            addr.address = cpu.program_counter;
            cpu.program_counter += 1;
            lookup.address = 0;
            false
        }
        fn step_1(&self, cpu: &mut N6502, addr: &mut AddressBus, lookup: &mut AddressBus) -> bool {
            addr.address = cpu.program_counter;
            cpu.program_counter += 1;
            lookup.address |= addr.byte as u16;
            false
        }
        fn step_2(&self, _cpu: &mut N6502, addr: &mut AddressBus, lookup: &mut AddressBus) -> bool {
            lookup.address |= (addr.byte as u16) << 8;
            true
        }
    }

    pub struct AddressMethodAbsoluteY {}
    impl AddressMethod for AddressMethodAbsoluteY {
        fn step_0(&self, cpu: &mut N6502, addr: &mut AddressBus, lookup: &mut AddressBus) -> bool {
            addr.address = cpu.program_counter;
            addr.write = false;
            cpu.program_counter += 1;
            lookup.address = 0;
            false
        }
        fn step_1(&self, cpu: &mut N6502, addr: &mut AddressBus, lookup: &mut AddressBus) -> bool {
            addr.address = cpu.program_counter;
            addr.write = false;
            cpu.program_counter += 1;
            lookup.address = addr.byte as u16;
            false
        }
        fn step_2(&self, cpu: &mut N6502, addr: &mut AddressBus, lookup: &mut AddressBus) -> bool {
            lookup.address += (addr.byte as u16) << 8;
            lookup.address += cpu.register_y as u16;
            true
        }
    }

    pub struct AddressMethodIndirectY {}
    impl AddressMethod for AddressMethodIndirectY {
        fn step_0(&self, cpu: &mut N6502, addr: &mut AddressBus, lookup: &mut AddressBus) -> bool {
            addr.address = cpu.program_counter;
            addr.write = false;
            cpu.program_counter += 1;
            lookup.address = 0;
            false
        }
        fn step_1(&self, cpu: &mut N6502, addr: &mut AddressBus, lookup: &mut AddressBus) -> bool {
            addr.address = addr.byte as u16;
            addr.write = false;
            false
        }
        fn step_2(&self, cpu: &mut N6502, addr: &mut AddressBus, lookup: &mut AddressBus) -> bool {
            lookup.address = addr.byte as u16;
            addr.address += 1;
            addr.write = false;
            false
        }
        fn step_3(&self, cpu: &mut N6502, addr: &mut AddressBus, lookup: &mut AddressBus) -> bool {
            lookup.address += (addr.byte as u16) << 8;
            lookup.address += cpu.register_y as u16;
            true
        }
    }
}

