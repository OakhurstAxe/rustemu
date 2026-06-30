

pub mod emu_cpu{

    use crate::n6502_address::naddress::{*};
    use crate::n6502_opcodes::nopcodes::{*};

    pub const CARRY_FLAG: u8 = 1;
    pub const ZERO_FLAG: u8 = 2;
    pub const INTERRUPT_FLAG: u8 = 4;
    pub const DECIMAL_MODE: u8 = 8;
    pub const BREAK_COMMAND: u8 = 16;
    pub const IGNORED: u8 = 32;
    pub const OVERFLOW_FLAG: u8 =64;
    pub const NEGATIVE_FLAG: u8 = 128;

    #[derive(Default)]
    pub struct AddressBus {
        pub address: u16,
        pub write: bool,
        pub byte: u8,
        pub is_accumulator: bool,
        pub is_abs_y: bool,
    }

    impl AddressBus {
        fn new(address: u16, write: bool, byte: u8) -> AddressBus {
            Self {
                address,
                write,
                byte,
                is_accumulator: false,
                is_abs_y: false,
            }
        }
    }

    pub struct N6502 {
        pub program_counter: u16,
        pub stack_pointer_page: u16,
        pub stack_pointer: u16,
        pub accumulator: u8,
        pub register_x: u8,
        pub register_y: u8,
        pub status_register: u8,
        pub lookup_address: AddressBus,
        pub in_interrupt: bool,
        _debug: u8,
    }

    impl N6502 {
        fn new () -> N6502 {
            Self {
                program_counter: 0,
                stack_pointer_page: 0x100,
                stack_pointer: 0,
                accumulator: 0,
                register_x: 0,
                register_y: 0,
                status_register: 0,
                lookup_address: AddressBus::new(0, false, 0),
                in_interrupt: false,
                _debug: 0,
            }
        }
    }

    #[derive(PartialEq)]
    pub enum M6502Version {
        AtariVcs,
        Nes
    }

    #[derive(PartialEq)]
    enum M6502RunnerStep {
        ReadPc,
        AddressStep,
        AddressStepLoadByte,
        OpCodeStep,
        OpCodeWrite,
    }

    pub struct M6502Runner {
        cpu: N6502,
        op_code: u16,
        op_step: u8,
        address_step: u8,
        runner_step: M6502RunnerStep,
        is_reset_set: bool,
        op_code_lookup: Vec<Box<dyn CpuOperation>>,
        op_code_ticks: Vec<u8>,
        address_method_lookup: Vec<Box<dyn AddressMethod>>,
        tick_count: u8,
        is_nmi_set: bool,
        _debug: u8,
    }

    impl M6502Runner {
        pub fn new (version: M6502Version) -> M6502Runner {

            let mut cpu = N6502::new();
            if version == M6502Version::AtariVcs {
                cpu.stack_pointer_page = 0x00;
            }

            Self {
                cpu,
                op_code: 0x100,
                op_step: 0,
                address_step: 0,
                runner_step: M6502RunnerStep::ReadPc,
                is_reset_set: true,
                op_code_lookup: OpCodesUtils::get_opcodes(),
                op_code_ticks: OpCodesUtils::get_ticks(),
                address_method_lookup: AddressOpCodes::get_address_methods(),
                tick_count: 0,
                is_nmi_set: false,
                _debug: 0,
            }
        }

        pub fn set_nmi(&mut self) {
            self.is_nmi_set = true;
        }

        pub fn execute_tick(&mut self, addr: &mut AddressBus) {
            
            if self.runner_step == M6502RunnerStep::AddressStepLoadByte {
                self.cpu.lookup_address.byte = addr.byte;
                self.runner_step = M6502RunnerStep::OpCodeStep;
            }

            // Geting new op code
            if self.runner_step == M6502RunnerStep::ReadPc {
                addr.is_accumulator = false;
                self.cpu.lookup_address.is_abs_y = false;
                if self.is_reset_set {
                    self.op_code = 0x100;
                    self.runner_step = M6502RunnerStep::OpCodeStep;
                    self.is_reset_set = false;
                } else if self.is_nmi_set {
                    self.op_code = 0x101;
                    self.runner_step = M6502RunnerStep::OpCodeStep;
                    self.is_nmi_set = false;
                } else {
                    self.runner_step = M6502RunnerStep::AddressStep;
                    if self.op_code_ticks[self.op_code as usize] != self.tick_count 
                     && self.op_code != 0xd0 && self.op_code != 0x10 
                     && self.op_code != 0xb0 && self.op_code != 0x30
                     && self.op_code != 0xf0 && self.op_code != 0x90 
                     && self.op_code != 0x50 {
                        //println!("PC: {:x}, opcode: {:x} tickcount: {} expected: {}", self.cpu.program_counter, self.op_code, self.tick_count, self.op_code_ticks[self.op_code as usize]);
                    }
                    self.op_code = addr.byte as u16;
                    self.cpu.program_counter += 1;
                    self.tick_count = 0;
                }
            }

            self.tick_count += 1;

            // Calling address/op tick
            if self.runner_step == M6502RunnerStep::AddressStep {
                if self.address_method_lookup[self.op_code as usize].execute(&mut self.cpu, addr, self.address_step) {
                    self.address_step = 0;
                    self.runner_step = M6502RunnerStep::AddressStepLoadByte;
                } else {
                    self.address_step += 1;
                }
            } 

            if self.runner_step == M6502RunnerStep::AddressStepLoadByte && self.op_code_lookup[self.op_code as usize].needs_addr_byte(addr) == false {
                self.cpu.lookup_address.byte = addr.byte;
                self.runner_step = M6502RunnerStep::OpCodeStep;
            }

            if self.runner_step == M6502RunnerStep::OpCodeStep {
                if self.op_code_lookup[self.op_code as usize].execute(&mut self.cpu, addr, self.op_step) {
                    self.op_step = 0;
                    self.runner_step = M6502RunnerStep::OpCodeWrite;
                } else {
                    self.op_step += 1;
                }
            } 

            if self.runner_step == M6502RunnerStep::OpCodeWrite && addr.write == false {
                self.runner_step = M6502RunnerStep::ReadPc;
                addr.address = self.cpu.program_counter;
            }

        }

    }

}
