
pub mod nopcodes {

    use crate::n6502::emu_cpu::N6502;
    use crate::n6502::emu_cpu::AddressBus;
    use crate::n6502::emu_cpu::{CARRY_FLAG, ZERO_FLAG, INTERRUPT_FLAG, DECIMAL_MODE, BREAK_COMMAND, IGNORED, OVERFLOW_FLAG, NEGATIVE_FLAG};

    const RESET_FIRST_READ: u16 = 0xFFFC;
    const RESET_SECOND_READ: u16 = 0xFFFD;

    pub struct OpCodesUtils {}
    impl OpCodesUtils {
        pub fn get_opcodes() -> Vec<Box<dyn CpuOperation>> {
            let mut op_code_lookup: Vec<Box<dyn CpuOperation>> = Vec::with_capacity(0x200);
            for _i in 0..0x200 {
                op_code_lookup.push(Box::new(CpuOpPanic {}));
            }
            op_code_lookup[0x01] = Box::new(CpuOpOra {});
            op_code_lookup[0x02] = Box::new(CpuOpBrk {});
            op_code_lookup[0x05] = Box::new(CpuOpOra {});
            op_code_lookup[0x06] = Box::new(CpuOpAsl {});
            op_code_lookup[0x08] = Box::new(CpuOpPhp {});
            op_code_lookup[0x09] = Box::new(CpuOpOra {});
            op_code_lookup[0x0a] = Box::new(CpuOpAsl {});

            op_code_lookup[0x10] = Box::new(CpuOpBpl {});
            op_code_lookup[0x11] = Box::new(CpuOpOra {});
            op_code_lookup[0x15] = Box::new(CpuOpOra {});
            op_code_lookup[0x18] = Box::new(CpuOpClc {});
            op_code_lookup[0x19] = Box::new(CpuOpOra {});

            op_code_lookup[0x20] = Box::new(CpuOpJsr {});
            op_code_lookup[0x24] = Box::new(CpuOpBit {});
            op_code_lookup[0x25] = Box::new(CpuOpAnd {});
            op_code_lookup[0x26] = Box::new(CpuOpRol {});
            op_code_lookup[0x29] = Box::new(CpuOpAnd {});
            op_code_lookup[0x2a] = Box::new(CpuOpRol {});
            op_code_lookup[0x2c] = Box::new(CpuOpBit {});

            op_code_lookup[0x30] = Box::new(CpuOpBmi {});
            op_code_lookup[0x35] = Box::new(CpuOpAnd {});
            op_code_lookup[0x38] = Box::new(CpuOpSec {});
            op_code_lookup[0x3d] = Box::new(CpuOpAnd {});

            op_code_lookup[0x45] = Box::new(CpuOpEor {});
            op_code_lookup[0x46] = Box::new(CpuOpLsr {});
            op_code_lookup[0x48] = Box::new(CpuOpPha {});
            op_code_lookup[0x49] = Box::new(CpuOpEor {});
            op_code_lookup[0x4a] = Box::new(CpuOpLsr {});
            op_code_lookup[0x4c] = Box::new(CpuOpJmp {});
            op_code_lookup[0x4e] = Box::new(CpuOpLsr {});

            op_code_lookup[0x50] = Box::new(CpuOpBvc {});
            op_code_lookup[0x56] = Box::new(CpuOpLsr {});

            op_code_lookup[0x60] = Box::new(CpuOpRts {});
            op_code_lookup[0x65] = Box::new(CpuOpAdc {});
            op_code_lookup[0x66] = Box::new(CpuOpRor {});
            op_code_lookup[0x68] = Box::new(CpuOpPla {});
            op_code_lookup[0x69] = Box::new(CpuOpAdc {});
            op_code_lookup[0x6a] = Box::new(CpuOpRor {});

            op_code_lookup[0x70] = Box::new(CpuOpBvs {});
            op_code_lookup[0x75] = Box::new(CpuOpAdc {});
            op_code_lookup[0x78] = Box::new(CpuOpSei {});
            op_code_lookup[0x79] = Box::new(CpuOpAdc {});
            op_code_lookup[0x7d] = Box::new(CpuOpAdc {});

            op_code_lookup[0x84] = Box::new(CpuOpSty {});
            op_code_lookup[0x85] = Box::new(CpuOpSta {});
            op_code_lookup[0x86] = Box::new(CpuOpStx {});
            op_code_lookup[0x88] = Box::new(CpuOpDey {});
            op_code_lookup[0x8a] = Box::new(CpuOpTxa {});
            op_code_lookup[0x8c] = Box::new(CpuOpSty {});
            op_code_lookup[0x8d] = Box::new(CpuOpSta {});

            op_code_lookup[0x90] = Box::new(CpuOpBcc {});
            op_code_lookup[0x91] = Box::new(CpuOpSta {});
            op_code_lookup[0x94] = Box::new(CpuOpSty {});
            op_code_lookup[0x95] = Box::new(CpuOpSta {});
            op_code_lookup[0x96] = Box::new(CpuOpStx {});
            op_code_lookup[0x98] = Box::new(CpuOpTya {});
            op_code_lookup[0x99] = Box::new(CpuOpSta {});
            op_code_lookup[0x9a] = Box::new(CpuOpTxs {});

            op_code_lookup[0xa0] = Box::new(CpuOpLdy {});
            op_code_lookup[0xa1] = Box::new(CpuOpLda {});
            op_code_lookup[0xa2] = Box::new(CpuOpLdx {});
            op_code_lookup[0xa4] = Box::new(CpuOpLdy {});
            op_code_lookup[0xa5] = Box::new(CpuOpLda {});
            op_code_lookup[0xa6] = Box::new(CpuOpLdx {});
            op_code_lookup[0xa8] = Box::new(CpuOpTay {});
            op_code_lookup[0xa9] = Box::new(CpuOpLda {});
            op_code_lookup[0xaa] = Box::new(CpuOpTax {});
            op_code_lookup[0xad] = Box::new(CpuOpLda {});
            op_code_lookup[0xae] = Box::new(CpuOpLdx {});

            op_code_lookup[0xb0] = Box::new(CpuOpBcs {});
            op_code_lookup[0xb1] = Box::new(CpuOpLda {});
            op_code_lookup[0xb4] = Box::new(CpuOpLdy {});
            op_code_lookup[0xb5] = Box::new(CpuOpLda {});
            op_code_lookup[0xb6] = Box::new(CpuOpLdx {});
            op_code_lookup[0xb9] = Box::new(CpuOpLda {});
            op_code_lookup[0xba] = Box::new(CpuOpTsx {});
            op_code_lookup[0xbc] = Box::new(CpuOpLdy {});
            op_code_lookup[0xbd] = Box::new(CpuOpLda {});
            op_code_lookup[0xbe] = Box::new(CpuOpLdx {});

            op_code_lookup[0xc0] = Box::new(CpuOpCpy {});
            op_code_lookup[0xc4] = Box::new(CpuOpCpy {});
            op_code_lookup[0xc5] = Box::new(CpuOpCmp {});
            op_code_lookup[0xc6] = Box::new(CpuOpDec {});
            op_code_lookup[0xc8] = Box::new(CpuOpIny {});
            op_code_lookup[0xc9] = Box::new(CpuOpCmp {});
            op_code_lookup[0xca] = Box::new(CpuOpDex {});

            op_code_lookup[0xd0] = Box::new(CpuOpBne {});
            op_code_lookup[0xd1] = Box::new(CpuOpCmp {});
            op_code_lookup[0xd5] = Box::new(CpuOpCmp {});
            op_code_lookup[0xd6] = Box::new(CpuOpDec {});
            op_code_lookup[0xd8] = Box::new(CpuOpCld {});
            op_code_lookup[0xd9] = Box::new(CpuOpCmp {});
            op_code_lookup[0xdd] = Box::new(CpuOpCmp {});

            op_code_lookup[0xe0] = Box::new(CpuOpCpx {});
            op_code_lookup[0xe4] = Box::new(CpuOpCpx {});
            op_code_lookup[0xe5] = Box::new(CpuOpSbc {});
            op_code_lookup[0xe6] = Box::new(CpuOpInc {});
            op_code_lookup[0xe8] = Box::new(CpuOpInx {});
            op_code_lookup[0xe9] = Box::new(CpuOpSbc {});
            op_code_lookup[0xea] = Box::new(CpuOpNop {});
            op_code_lookup[0xec] = Box::new(CpuOpCpx {});

            op_code_lookup[0xf0] = Box::new(CpuOpBeq {});
            op_code_lookup[0xf6] = Box::new(CpuOpInc {});
            op_code_lookup[0xf8] = Box::new(CpuOpSed {});
            op_code_lookup[0xf9] = Box::new(CpuOpSbc {});
            op_code_lookup[0xff] = Box::new(CpuOpIsc {});

            op_code_lookup[0x100] = Box::new(CpuOpReset {});
            op_code_lookup
        }

        pub fn get_ticks() -> Vec<u8> {
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
            op_code_ticks[0xff] = 7;
            op_code_ticks
        }

        fn push_stack(cpu: &mut N6502, addr: &mut AddressBus, byte: u8) {
            if cpu.stack_pointer == 0 {
                eprintln!("Stack overflow");
            }
            addr.address = cpu.stack_pointer + cpu.stack_pointer_page;
            addr.byte = byte;
            addr.write = true;
            cpu.stack_pointer -= 1;
        }

        fn pop_stack(cpu: &mut N6502, addr: &mut AddressBus) {
            if cpu.stack_pointer > 255 {
                eprintln!("Stack underflow");
            }
            cpu.stack_pointer += 1;
            addr.address = cpu.stack_pointer + cpu.stack_pointer_page;
        }

        fn get_status_flag(cpu: &mut N6502, flag: u8) -> u8 {
            cpu.status_register & flag
        }

        fn set_status_flag(cpu: &mut N6502, flag: u8, set_clear: bool) {
            if set_clear {
                cpu.status_register |= flag;
            }
            else {
                cpu.status_register &= !flag;
            }
        }

        fn set_negative(cpu: &mut N6502, byte: u8) {
            OpCodesUtils::set_status_flag(cpu, NEGATIVE_FLAG, (byte & 0x80) != 0);
        }

        fn set_zero(cpu: &mut N6502, byte: u8) {
            OpCodesUtils::set_status_flag(cpu, ZERO_FLAG, byte == 0);
        }

        fn set_negative_zero(cpu: &mut N6502, byte: u8) {
            OpCodesUtils::set_negative(cpu, byte);
            OpCodesUtils::set_zero(cpu, byte);
        }
    }

    pub trait CpuOperation: Sync + Send {
        fn execute(&self, cpu: &mut N6502, addr: &mut AddressBus, step: u8) -> bool {
            let mut result = false;
            match step {
                0 => result = self.step_0(cpu, addr),
                1 => result = self.step_1(cpu, addr),
                2 => result = self.step_2(cpu, addr),
                3 => result = self.step_3(cpu, addr),
                4 => result = self.step_4(cpu, addr),
                5 => result = self.step_5(cpu, addr),
                _ => {}
            }
            result            
        }
        fn needs_addr_byte(&self, addr: &mut AddressBus) -> bool { 
            if addr.is_accumulator {
                return false;
            }
            true 
        }
        fn step_0(&self, _cpu: &mut N6502, _addr: &mut AddressBus) -> bool { true }
        fn step_1(&self, _cpu: &mut N6502, _addr: &mut AddressBus) -> bool { true }
        fn step_2(&self, _cpu: &mut N6502, _addr: &mut AddressBus) -> bool { true }
        fn step_3(&self, _cpu: &mut N6502, _addr: &mut AddressBus) -> bool { true }
        fn step_4(&self, _cpu: &mut N6502, _addr: &mut AddressBus) -> bool { true }
        fn step_5(&self, _cpu: &mut N6502, _addr: &mut AddressBus) -> bool { true }
    }

    struct CpuOpPanic {}
    impl CpuOperation for CpuOpPanic {
        fn step_0(&self, _cpu: &mut N6502, _addr: &mut AddressBus) -> bool {
            true
        }
    }

    // System Opcodes
    struct CpuOpNop {}
    impl CpuOperation for CpuOpNop {
        fn needs_addr_byte(&self, _addr: &mut AddressBus) -> bool { false }
        fn step_0(&self, _cpu: &mut N6502, _addr: &mut AddressBus) -> bool {
            false
        }
        fn step_1(&self, _cpu: &mut N6502, _addr: &mut AddressBus) -> bool {
            true
        }
    }

    struct CpuOpBrk {}
    impl CpuOperation for CpuOpBrk {
        fn needs_addr_byte(&self, _addr: &mut AddressBus) -> bool { false }
        fn step_0(&self, cpu: &mut N6502, addr: &mut AddressBus) -> bool {
            OpCodesUtils::set_status_flag(cpu, INTERRUPT_FLAG, true);
            cpu.program_counter += 1;            
            OpCodesUtils::push_stack(cpu, addr, (cpu.program_counter & 0xff00 >> 8) as u8);
            false
        }
        fn step_1(&self, cpu: &mut N6502, addr: &mut AddressBus) -> bool {
            OpCodesUtils::push_stack(cpu, addr, (cpu.program_counter & 0xff) as u8);
            false
        }
        fn step_2(&self, cpu: &mut N6502, addr: &mut AddressBus) -> bool {
            OpCodesUtils::push_stack(cpu, addr, cpu.status_register);
            false
        }
        fn step_3(&self, cpu: &mut N6502, addr: &mut AddressBus) -> bool {
            addr.address = 0xfffe;
            cpu.program_counter = 0;
            false
        }
        fn step_4(&self, cpu: &mut N6502, addr: &mut AddressBus) -> bool {
            addr.address = 0xffff;
            cpu.program_counter = addr.byte as u16;
            false
        }
        fn step_5(&self, cpu: &mut N6502, addr: &mut AddressBus) -> bool {
            cpu.program_counter += (addr.byte as u16) << 8;
            true
        }
    }

    // Load Store Operations
    struct CpuOpLda {}
    impl CpuOperation for CpuOpLda {
        fn step_0(&self, cpu: &mut N6502, _addr: &mut AddressBus) -> bool {
            cpu.accumulator = cpu.lookup_address.byte;
            OpCodesUtils::set_negative_zero(cpu, cpu.accumulator);
            true
        }
    }

    struct CpuOpLdx {}
    impl CpuOperation for CpuOpLdx {
        fn step_0(&self, cpu: &mut N6502, _addr: &mut AddressBus) -> bool {
            cpu.register_x = cpu.lookup_address.byte;
            OpCodesUtils::set_negative_zero(cpu, cpu.register_x);
            true
        }
    }

    struct CpuOpLdy {}
    impl CpuOperation for CpuOpLdy {
        fn step_0(&self, cpu: &mut N6502, _addr: &mut AddressBus) -> bool {
            cpu.register_y = cpu.lookup_address.byte;
            OpCodesUtils::set_negative_zero(cpu, cpu.register_y);
            true
        }
    }

    struct CpuOpSta {}
    impl CpuOperation for CpuOpSta {
        fn needs_addr_byte(&self, _addr: &mut AddressBus) -> bool { false }
        fn step_0(&self, cpu: &mut N6502, addr: &mut AddressBus) -> bool {
            addr.address = cpu.lookup_address.address;
            addr.byte = cpu.accumulator;
            addr.write = true;
            true
        }
    }

    struct CpuOpStx {}
    impl CpuOperation for CpuOpStx {
        fn needs_addr_byte(&self, _addr: &mut AddressBus) -> bool { false }
        fn step_0(&self, cpu: &mut N6502, addr: &mut AddressBus) -> bool {
            addr.address = cpu.lookup_address.address;
            addr.byte = cpu.register_x;
            addr.write = true;
            true
        }
    }

    struct CpuOpSty {}
    impl CpuOperation for CpuOpSty {
        fn needs_addr_byte(&self, _addr: &mut AddressBus) -> bool { false }
        fn step_0(&self, cpu: &mut N6502, addr: &mut AddressBus) -> bool {
            addr.address = cpu.lookup_address.address;
            addr.byte = cpu.register_y;
            addr.write = true;
            true
        }
    }

    // Register Transfers
    struct CpuOpTax {}
    impl CpuOperation for CpuOpTax {
        fn step_0(&self, cpu: &mut N6502, _addr: &mut AddressBus) -> bool {
            cpu.register_x = cpu.accumulator;
            OpCodesUtils::set_negative_zero(cpu, cpu.register_x);
            true
        }
    }

    struct CpuOpTay {}
    impl CpuOperation for CpuOpTay {
        fn step_0(&self, cpu: &mut N6502, _addr: &mut AddressBus) -> bool {
            cpu.register_y = cpu.accumulator;
            OpCodesUtils::set_negative_zero(cpu, cpu.register_y);
            true
        }
    }

    struct CpuOpTxa {}
    impl CpuOperation for CpuOpTxa {
        fn step_0(&self, cpu: &mut N6502, _addr: &mut AddressBus) -> bool {
            cpu.accumulator = cpu.register_x;
            OpCodesUtils::set_negative_zero(cpu, cpu.accumulator);
            true
        }
    }

    struct CpuOpTya {}
    impl CpuOperation for CpuOpTya {
        fn step_0(&self, cpu: &mut N6502, _addr: &mut AddressBus) -> bool {
            cpu.accumulator = cpu.register_y;
            OpCodesUtils::set_negative_zero(cpu, cpu.accumulator);
            true
        }
    }

    // Stack Operations
    struct CpuOpTsx {}
    impl CpuOperation for CpuOpTsx {
        fn step_0(&self, cpu: &mut N6502, _addr: &mut AddressBus) -> bool {
            cpu.register_x = cpu.stack_pointer as u8;
            OpCodesUtils::set_negative_zero(cpu, cpu.register_x);
            true
        }
    }

    struct CpuOpTxs {}
    impl CpuOperation for CpuOpTxs {
        fn step_0(&self, cpu: &mut N6502, _addr: &mut AddressBus) -> bool {
            cpu.stack_pointer = cpu.register_x as u16;
            true
        }
    }

    struct CpuOpPha {}
    impl CpuOperation for CpuOpPha {
        fn step_0(&self, cpu: &mut N6502, addr: &mut AddressBus) -> bool {
            OpCodesUtils::push_stack(cpu, addr, cpu.accumulator);
            true
        }
    }
    
    struct CpuOpPhp {}
    impl CpuOperation for CpuOpPhp {
        fn step_0(&self, cpu: &mut N6502, addr: &mut AddressBus) -> bool {
            if !cpu.in_interrupt {
                OpCodesUtils::set_status_flag(cpu, BREAK_COMMAND, true);
            }
            OpCodesUtils::set_status_flag(cpu, IGNORED, true);
            OpCodesUtils::push_stack(cpu, addr, cpu.status_register);
            if !cpu.in_interrupt {
                OpCodesUtils::set_status_flag(cpu, BREAK_COMMAND, false);
            }
            OpCodesUtils::set_status_flag(cpu, IGNORED, false);
            true
        }
    }

    struct CpuOpPla {}
    impl CpuOperation for CpuOpPla {
        fn step_0(&self, cpu: &mut N6502, addr: &mut AddressBus) -> bool {
            OpCodesUtils::pop_stack(cpu, addr);
            false
        }
        fn step_1(&self, cpu: &mut N6502, addr: &mut AddressBus) -> bool {
            cpu.status_register = addr.byte;
            false
        }
    }

    // Logical Operations
    struct CpuOpAnd {}
    impl CpuOperation for CpuOpAnd {
        fn step_0(&self, cpu: &mut N6502, _addr: &mut AddressBus) -> bool {
            let byte: u8 = cpu.lookup_address.byte;
            cpu.accumulator &= byte;
            OpCodesUtils::set_negative_zero(cpu, cpu.accumulator);
            true
        }
    }

    struct CpuOpEor {}
    impl CpuOperation for CpuOpEor {
        fn step_0(&self, cpu: &mut N6502, _addr: &mut AddressBus) -> bool {
            cpu.accumulator ^= cpu.lookup_address.byte;
            OpCodesUtils::set_negative_zero(cpu, cpu.accumulator);
            true
        }
    }

    struct CpuOpOra {}
    impl CpuOperation for CpuOpOra {
        fn step_0(&self, cpu: &mut N6502, _addr: &mut AddressBus) -> bool {
            cpu.accumulator |= cpu.lookup_address.byte;
            OpCodesUtils::set_negative_zero(cpu, cpu.accumulator);
            true
        }
    }

    struct CpuOpBit {}
    impl CpuOperation for CpuOpBit {
        fn step_0(&self, cpu: &mut N6502, _addr: &mut AddressBus) -> bool {
            let byte = cpu.lookup_address.byte;
            OpCodesUtils::set_status_flag(cpu, OVERFLOW_FLAG, (byte & 0x40) != 0);
            OpCodesUtils::set_negative(cpu, byte);
            OpCodesUtils::set_zero(cpu, byte & cpu.accumulator);
            true
        }
    }

    struct CpuOpAdc {}
    impl CpuOperation for CpuOpAdc {
        fn step_0(&self, cpu: &mut N6502, _addr: &mut AddressBus) -> bool {
            let byte: u8 = cpu.lookup_address.byte;
            let mut value: u8;

            let mut carry: u8 = 0;
            if OpCodesUtils::get_status_flag(cpu, CARRY_FLAG) != 0 {
                carry = 1;
            }

            let(value1, overflow1) = cpu.accumulator.overflowing_add(byte);
            let(value2, overflow2) = value1.overflowing_add(carry);
            let mut overflow3 = false;
            let mut overflow4 = false;
            value = value2;

            if OpCodesUtils::get_status_flag(cpu, DECIMAL_MODE) != 0 {
                if (value & 0x0f) > 0x09 {
                    let (value4, overflow5) = value.overflowing_add(0x06);
                    value = value4;
                    overflow3 = overflow5;
                }
                if (value & 0xf0) > 0x90 {
                    let (value5, overflow6) = value.overflowing_add(0x60);
                    value = value5;
                    overflow4 = overflow6;
                }
            }
            OpCodesUtils::set_status_flag(cpu, OVERFLOW_FLAG, ((cpu.accumulator ^ value) & (byte ^ value) & 0x080) == 0x80);
            OpCodesUtils::set_status_flag(cpu, CARRY_FLAG, overflow1 || overflow2 || overflow3 || overflow4);
            OpCodesUtils::set_negative_zero(cpu, value);
            cpu.accumulator = value;
            true
        }
    }

    struct CpuOpSbc {}
    impl CpuOperation for CpuOpSbc {
        fn step_0(&self, cpu: &mut N6502, _addr: &mut AddressBus) -> bool {
            let byte: u8 = !cpu.lookup_address.byte;
            let mut value: u8;

            let mut carry: u8 = 0;
            if OpCodesUtils::get_status_flag(cpu, CARRY_FLAG) != 0 {
                carry = 1;
            }

            let(value1, overflow1) = cpu.accumulator.overflowing_add(byte);
            let(value2, overflow2) = value1.overflowing_add(carry);
            value = value2;
            let mut overflow3 = false;
            let mut overflow4 = false;

            if OpCodesUtils::get_status_flag(cpu, DECIMAL_MODE) != 0 {
                if (value & 0x0f) > 0x09 {
                    let (value4, overflow5) = value.overflowing_sub(0x06);
                    value = value4;
                    overflow3 = overflow5;
                }
                if (value & 0xf0) > 0x90 {
                    let (value5, overflow6) = value.overflowing_sub(0x60);
                    value = value5;
                    overflow4 = overflow6;
                }
            }
            OpCodesUtils::set_status_flag(cpu, OVERFLOW_FLAG, ((cpu.accumulator ^ value) & (byte ^ value) & 0x80) == 0x80);
            OpCodesUtils::set_status_flag(cpu, CARRY_FLAG, overflow1 || overflow2 || overflow3 || overflow4);
            OpCodesUtils::set_negative_zero(cpu, value);
            cpu.accumulator = value;
            true
        }
    }

    struct CpuOpCmp {}
    impl CpuOperation for CpuOpCmp {
        fn step_0(&self, cpu: &mut N6502, _addr: &mut AddressBus) -> bool {
            OpCodesUtils::set_status_flag(cpu, CARRY_FLAG, cpu.accumulator >= cpu.lookup_address.byte);
            OpCodesUtils::set_status_flag(cpu,ZERO_FLAG, cpu.accumulator == cpu.lookup_address.byte);
            let negzerocheck= cpu.accumulator.overflowing_sub(cpu.lookup_address.byte).0;
            OpCodesUtils::set_status_flag(cpu,NEGATIVE_FLAG, negzerocheck & 0x80 != 0);
            true
        }
    }

    struct CpuOpCpx {}
    impl CpuOperation for CpuOpCpx {
        fn step_0(&self, cpu: &mut N6502, _addr: &mut AddressBus) -> bool {
            OpCodesUtils::set_status_flag(cpu, CARRY_FLAG, cpu.register_x >= cpu.lookup_address.byte);
            OpCodesUtils::set_status_flag(cpu, ZERO_FLAG, cpu.register_x == cpu.lookup_address.byte);
            let negzerocheck= cpu.register_x.overflowing_sub(cpu.lookup_address.byte).0;
            OpCodesUtils::set_status_flag(cpu, NEGATIVE_FLAG, negzerocheck & 0x80 != 0);
            true
        }
    }

    struct CpuOpCpy {}
    impl CpuOperation for CpuOpCpy {
        fn step_0(&self, cpu: &mut N6502, _addr: &mut AddressBus) -> bool {
            OpCodesUtils::set_status_flag(cpu, CARRY_FLAG, cpu.register_y >= cpu.lookup_address.byte);
            OpCodesUtils::set_status_flag(cpu, ZERO_FLAG, cpu.register_y == cpu.lookup_address.byte);
            let negzerocheck= cpu.register_y.overflowing_sub(cpu.lookup_address.byte).0;
            OpCodesUtils::set_status_flag(cpu, NEGATIVE_FLAG, negzerocheck & 0x80 != 0);
            true
        }
    }

    // Increment and Decrement Operations
    struct CpuOpInc {}
    impl CpuOperation for CpuOpInc {
        fn step_0(&self, cpu: &mut N6502, addr: &mut AddressBus) -> bool {
            addr.byte = cpu.lookup_address.byte.overflowing_add(1).0;
            addr.address = cpu.lookup_address.address;
            addr.write = true;
            OpCodesUtils::set_negative_zero(cpu, addr.byte);
            false
        }
        fn step_1(&self, _cpu: &mut N6502, _addr: &mut AddressBus) -> bool {
            false
        }
        fn step_2(&self, _cpu: &mut N6502, _addr: &mut AddressBus) -> bool {
            true
        }
    }

    struct CpuOpInx {}
    impl CpuOperation for CpuOpInx {
        fn step_0(&self, cpu: &mut N6502, _addr: &mut AddressBus) -> bool {
            cpu.register_x = cpu.register_x.overflowing_add(1).0;
            OpCodesUtils::set_negative_zero(cpu, cpu.register_x);
            true
        }
    }

    struct CpuOpIny {}
    impl CpuOperation for CpuOpIny {
        fn step_0(&self, cpu: &mut N6502, _addr: &mut AddressBus) -> bool {
            cpu.register_y = cpu.register_y.overflowing_add(1).0;
            OpCodesUtils::set_negative_zero(cpu, cpu.register_y);
            true
        }
    }

    struct CpuOpDec {}
    impl CpuOperation for CpuOpDec {
        fn step_0(&self, cpu: &mut N6502, addr: &mut AddressBus) -> bool {
            addr.byte = cpu.lookup_address.byte.overflowing_sub(1).0;
            addr.address = cpu.lookup_address.address;
            addr.write = true;
            OpCodesUtils::set_negative_zero(cpu, addr.byte);
            false
        }
        fn step_1(&self, _cpu: &mut N6502, _addr: &mut AddressBus) -> bool {
            false
        }
    }

    struct CpuOpDex {}
    impl CpuOperation for CpuOpDex {
        fn step_0(&self, cpu: &mut N6502, _addr: &mut AddressBus) -> bool {
            cpu.register_x = cpu.register_x.overflowing_sub(1).0;
            OpCodesUtils::set_negative_zero(cpu, cpu.register_x);
            true
        }
    }

    struct CpuOpDey {}
    impl CpuOperation for CpuOpDey {
        fn needs_addr_byte(&self, _addr: &mut AddressBus) -> bool { false }
        fn step_0(&self, cpu: &mut N6502, _addr: &mut AddressBus) -> bool {
            cpu.register_y = cpu.register_y.overflowing_sub(1).0;
            OpCodesUtils::set_negative_zero(cpu, cpu.register_y);
            false
        }
    }

    // Shift Operations
    struct CpuOpAsl {}
    impl CpuOperation for CpuOpAsl {
        fn needs_addr_byte(&self, _addr: &mut AddressBus) -> bool { true }
        fn step_0(&self, cpu: &mut N6502, addr: &mut AddressBus) -> bool {
            let mut byte: u8 = cpu.lookup_address.byte;
 
            if cpu.lookup_address.is_accumulator {
                byte = cpu.accumulator;
            }

            OpCodesUtils::set_status_flag(cpu,CARRY_FLAG, (byte & 0x80) != 0);
            byte <<= 1;
            OpCodesUtils::set_negative_zero(cpu, byte);

            if cpu.lookup_address.is_accumulator {
                cpu.accumulator = byte;
            } else {
                addr.byte = byte;
                addr.address = cpu.lookup_address.address;
                addr.write = true;
            }
            true
        }
    }

    struct CpuOpLsr {}
    impl CpuOperation for CpuOpLsr {
        fn needs_addr_byte(&self, _addr: &mut AddressBus) -> bool { true }
        fn step_0(&self, cpu: &mut N6502, addr: &mut AddressBus) -> bool {
            let mut byte: u8 = cpu.lookup_address.byte;
            if cpu.lookup_address.is_accumulator {
                byte = cpu.accumulator;
            }

            OpCodesUtils::set_status_flag(cpu,CARRY_FLAG, (byte & 0x01) != 0);
            byte >>= 1;
            OpCodesUtils::set_negative_zero(cpu, byte);

            if cpu.lookup_address.is_accumulator {
                cpu.accumulator = byte;
            } else {
                addr.byte = byte;
                addr.address = cpu.lookup_address.address;
                addr.write = true;
            }
            true
        }
    }

    struct CpuOpRol {}
    impl CpuOperation for CpuOpRol {
        fn step_0(&self, cpu: &mut N6502, addr: &mut AddressBus) -> bool {
            let mut byte: u8 = cpu.lookup_address.byte;

            if cpu.lookup_address.is_accumulator {
                byte = cpu.accumulator;
            }

            let mut carry: u8 = 0;
            if OpCodesUtils::get_status_flag(cpu, CARRY_FLAG) != 0 {
                carry = 0x01;
            }

            OpCodesUtils::set_status_flag(cpu, CARRY_FLAG, (byte & 0x80) != 0);
            byte = (byte << 1) | carry;
            OpCodesUtils::set_negative_zero(cpu, byte);

            if cpu.lookup_address.is_accumulator {
                cpu.accumulator = byte;
            } else {
                addr.byte = byte;
                addr.address = cpu.lookup_address.address;
                addr.write = true;
            }
            false
        }
        fn step_1(&self, cpu: &mut N6502, addr: &mut AddressBus) -> bool {
            false
        }
    }

    struct CpuOpRor {}
    impl CpuOperation for CpuOpRor {
        fn step_0(&self, cpu: &mut N6502, addr: &mut AddressBus) -> bool {
            let mut byte: u8 = cpu.lookup_address.byte;

            let mut carry: u8 = 0;
            if OpCodesUtils::get_status_flag(cpu, CARRY_FLAG) != 0 {
                carry = 0x80;
            }

            OpCodesUtils::set_status_flag(cpu,CARRY_FLAG, (byte & 0x01) != 0);
            byte = (byte >> 1) | carry;
            OpCodesUtils::set_negative_zero(cpu, byte);

            if cpu.lookup_address.is_accumulator {
                cpu.accumulator = byte;
            } else {
                addr.byte = byte;
                addr.address = cpu.lookup_address.address;
                addr.write = true;
            }
            true
        }
    }

    // Jumps and Call Operations
    struct CpuOpJmp {}
    impl CpuOperation for CpuOpJmp {
        fn needs_addr_byte(&self, _addr: &mut AddressBus) -> bool { false }
        fn step_0(&self, cpu: &mut N6502, _addr: &mut AddressBus) -> bool {
            cpu.program_counter = cpu.lookup_address.address;
            true
        }
    }

    struct CpuOpJsr {}
    impl CpuOperation for CpuOpJsr {
        fn needs_addr_byte(&self, _addr: &mut AddressBus) -> bool { false }
        fn step_0(&self, cpu: &mut N6502, addr: &mut AddressBus) -> bool {
            cpu.program_counter += 1;
            OpCodesUtils::push_stack(cpu, addr, ((cpu.program_counter & 0xff00) >> 8) as u8);
            false
        }
        fn step_1(&self, cpu: &mut N6502, addr: &mut AddressBus) -> bool {
            OpCodesUtils::push_stack(cpu, addr, (cpu.program_counter & 0x00ff) as u8);
            false
        }
        fn step_2(&self, cpu: &mut N6502, _addr: &mut AddressBus) -> bool {
            cpu.program_counter = cpu.lookup_address.address;
            false
        }
    }

    struct CpuOpRts {}
    impl CpuOperation for CpuOpRts {
        fn needs_addr_byte(&self, _addr: &mut AddressBus) -> bool { false }
        fn step_0(&self, cpu: &mut N6502, addr: &mut AddressBus) -> bool {
            OpCodesUtils::pop_stack(cpu, addr);
            false
        }
        fn step_1(&self, cpu: &mut N6502, addr: &mut AddressBus) -> bool {
            cpu.program_counter = addr.byte as u16;
            OpCodesUtils::pop_stack(cpu, addr);
            false
        }
        fn step_2(&self, cpu: &mut N6502, addr: &mut AddressBus) -> bool {
            cpu.program_counter += ((addr.byte as u16) << 8) - 1;
            false
        }
        fn step_3(&self, _cpu: &mut N6502, _addr: &mut AddressBus) -> bool {
            false
        }
        fn step_4(&self, _cpu: &mut N6502, _addr: &mut AddressBus) -> bool {
            false
        }
    }

    // Branch Operations
    struct CpuOpBcc {}
    impl CpuOperation for CpuOpBcc {
        fn needs_addr_byte(&self, _addr: &mut AddressBus) -> bool { false }
        fn step_0(&self, _cpu: &mut N6502, _addr: &mut AddressBus) -> bool {
            false
        }
        fn step_1(&self, cpu: &mut N6502, addr: &mut AddressBus) -> bool {
            if OpCodesUtils::get_status_flag(cpu, CARRY_FLAG) == 0 {
                let relative_address: i8 = addr.byte as i8;
                cpu.program_counter = (cpu.program_counter as i16 + relative_address as i16) as u16;
                return false;
            }
            true
        }
    }

    struct CpuOpBcs {}
    impl CpuOperation for CpuOpBcs {
        fn needs_addr_byte(&self, _addr: &mut AddressBus) -> bool { false }
        fn step_0(&self, _cpu: &mut N6502, _addr: &mut AddressBus) -> bool {
            false
        }
        fn step_1(&self, cpu: &mut N6502, addr: &mut AddressBus) -> bool {
            if OpCodesUtils::get_status_flag(cpu, CARRY_FLAG) != 0 {
                let relative_address: i8 = addr.byte as i8;
                cpu.program_counter = (cpu.program_counter as i16 + relative_address as i16) as u16;
                return false;
            }
            true
        }
    }

    struct CpuOpBne {}
    impl CpuOperation for CpuOpBne {
        fn needs_addr_byte(&self, _addr: &mut AddressBus) -> bool { false }
        fn step_0(&self, _cpu: &mut N6502, _addr: &mut AddressBus) -> bool {
            false
        }
        fn step_1(&self, cpu: &mut N6502, addr: &mut AddressBus) -> bool {
            if OpCodesUtils::get_status_flag(cpu, ZERO_FLAG) == 0 {
                let relative_address: i8 = addr.byte as i8;
                cpu.program_counter = (cpu.program_counter as i16 + relative_address as i16) as u16;
                return false;
            }
            true
        }
    }

    struct CpuOpBeq {}
    impl CpuOperation for CpuOpBeq {
        fn needs_addr_byte(&self, _addr: &mut AddressBus) -> bool { false }
        fn step_0(&self, _cpu: &mut N6502, _addr: &mut AddressBus) -> bool {
            false
        }
        fn step_1(&self, cpu: &mut N6502, addr: &mut AddressBus) -> bool {
            if OpCodesUtils::get_status_flag(cpu, ZERO_FLAG) != 0 {
                let relative_address: i8 = addr.byte as i8;
                cpu.program_counter = (cpu.program_counter as i16 + relative_address as i16) as u16;
                return false;
            }
            true
        }
    }

    struct CpuOpBpl {}
    impl CpuOperation for CpuOpBpl {
        fn needs_addr_byte(&self, _addr: &mut AddressBus) -> bool { false }
        fn step_0(&self, _cpu: &mut N6502, _addr: &mut AddressBus) -> bool {
            false
        }
        fn step_1(&self, cpu: &mut N6502, addr: &mut AddressBus) -> bool {
            if OpCodesUtils::get_status_flag(cpu, NEGATIVE_FLAG) == 0 {
                let relative_address: i8 = addr.byte as i8;
                cpu.program_counter = (cpu.program_counter as i16 + relative_address as i16) as u16;
                return false;
            }
            true
        }
    }

    struct CpuOpBmi {}
    impl CpuOperation for CpuOpBmi {
        fn needs_addr_byte(&self, _addr: &mut AddressBus) -> bool { false }
        fn step_0(&self, _cpu: &mut N6502, _addr: &mut AddressBus) -> bool {
            false
        }
        fn step_1(&self, cpu: &mut N6502, addr: &mut AddressBus) -> bool {
            if OpCodesUtils::get_status_flag(cpu, NEGATIVE_FLAG) != 0 {
                let relative_address: i8 = addr.byte as i8;
                cpu.program_counter = (cpu.program_counter as i16 + relative_address as i16) as u16;
                return false;
            }
            true
        }
    }

    struct CpuOpBvc {}
    impl CpuOperation for CpuOpBvc {
        fn needs_addr_byte(&self, _addr: &mut AddressBus) -> bool { false }
        fn step_0(&self, _cpu: &mut N6502, _addr: &mut AddressBus) -> bool {
            false
        }
        fn step_1(&self, cpu: &mut N6502, addr: &mut AddressBus) -> bool {
            if OpCodesUtils::get_status_flag(cpu, OVERFLOW_FLAG) == 0 {
                let relative_address: i8 = addr.byte as i8;
                cpu.program_counter = (cpu.program_counter as i16 + relative_address as i16) as u16;
                return false;
            }
            true
        }
    }

    struct CpuOpBvs {}
    impl CpuOperation for CpuOpBvs {
        fn needs_addr_byte(&self, _addr: &mut AddressBus) -> bool { false }
        fn step_0(&self,_cpu: &mut N6502, _addr: &mut AddressBus) -> bool {
            false
        }
        fn step_1(&self, cpu: &mut N6502, addr: &mut AddressBus) -> bool {
            if OpCodesUtils::get_status_flag(cpu, OVERFLOW_FLAG) != 0 {
                let relative_address: i8 = addr.byte as i8;
                cpu.program_counter = (cpu.program_counter as i16 + relative_address as i16) as u16;
                return false;
            }
            true
        }
    }

    // Status Flag Operations
    struct CpuOpClc {}
    impl CpuOperation for CpuOpClc {
        fn step_0(&self, cpu: &mut N6502, _addr: &mut AddressBus) -> bool {
            OpCodesUtils::set_status_flag(cpu, CARRY_FLAG, false);
            true
        }
    }

    struct CpuOpSec {}
    impl CpuOperation for CpuOpSec {
        fn step_0(&self, cpu: &mut N6502, _addr: &mut AddressBus) -> bool {
            OpCodesUtils::set_status_flag(cpu, CARRY_FLAG, true);
            true
        }
    }

    struct CpuOpCld {}
    impl CpuOperation for CpuOpCld {
        fn step_0(&self, cpu: &mut N6502, _addr: &mut AddressBus) -> bool {
            OpCodesUtils::set_status_flag(cpu, DECIMAL_MODE, false);
            true
        }
    }

    struct CpuOpSed {}
    impl CpuOperation for CpuOpSed {
        fn needs_addr_byte(&self, _addr: &mut AddressBus) -> bool { false }
        fn step_0(&self, cpu: &mut N6502, _addr: &mut AddressBus) -> bool {
            OpCodesUtils::set_status_flag(cpu, DECIMAL_MODE, true);
            true
        }
    }

    struct CpuOpSei {}
    impl CpuOperation for CpuOpSei {
        fn needs_addr_byte(&self, _addr: &mut AddressBus) -> bool { false }
        fn step_0(&self, cpu: &mut N6502, _addr: &mut AddressBus) -> bool {
            OpCodesUtils::set_status_flag(cpu, INTERRUPT_FLAG, true);
            true
        }
    }

    // Unofficial Opcodes
    struct CpuOpIsc {}
    impl CpuOperation for CpuOpIsc {
        fn step_0(&self, cpu: &mut N6502, addr: &mut AddressBus) -> bool {
            cpu.lookup_address.byte = cpu.lookup_address.byte.overflowing_add(1).0;
            addr.byte = cpu.lookup_address.byte;
            addr.address = cpu.lookup_address.address;
            addr.write = true;
            false
        }
        fn step_1(&self, cpu: &mut N6502, _addr: &mut AddressBus) -> bool {
            cpu.lookup_address.byte = !cpu.lookup_address.byte;
            let mut carry = 0;
            if OpCodesUtils::get_status_flag(cpu, CARRY_FLAG) > 0 {
                carry = 1;
            }
            let value = cpu.accumulator as u16 + cpu.lookup_address.byte as u16 + carry;
            OpCodesUtils::set_status_flag(cpu, CARRY_FLAG, value > 255);
            OpCodesUtils::set_status_flag(cpu, OVERFLOW_FLAG, (cpu.accumulator as u16 ^ value) & (cpu.lookup_address.byte as u16 ^ value) & 0x80 != 0);
            cpu.accumulator = value as u8;
            OpCodesUtils::set_negative_zero(cpu, cpu.accumulator);
            true
        }
    }

    // Reset and Interrupts
    struct CpuOpReset {}
    impl CpuOperation for CpuOpReset {
        fn step_0(&self, cpu: &mut N6502, addr: &mut AddressBus) -> bool {
            cpu.stack_pointer = 0xff;
            cpu.accumulator = 0;
            cpu.register_x = 0;
            cpu.register_y = 0;
            cpu.status_register = 0;
            addr.address = RESET_FIRST_READ;
            false
        }
        fn step_1(&self, cpu: &mut N6502, addr: &mut AddressBus) -> bool {
            cpu.program_counter = addr.byte as u16;
            addr.address = RESET_SECOND_READ;
            false
        } 
        fn step_2(&self, cpu: &mut N6502, addr: &mut AddressBus) -> bool {
            cpu.program_counter += (addr.byte as u16) << 8;
            true
        }
    }
}
