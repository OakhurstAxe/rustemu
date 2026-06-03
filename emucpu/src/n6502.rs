

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
    }

    impl AddressBus {
        fn new(address: u16, write: bool, byte: u8) -> AddressBus {
            Self {
                address: address,
                write: write,
                byte: byte,
                is_accumulator: false,
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
        _debug: u8,
    }

    impl N6502 {
        fn new () -> N6502 {
            Self {
                program_counter: 0,
                stack_pointer_page: 0,
                stack_pointer: 0,
                accumulator: 0,
                register_x: 0,
                register_y: 0,
                status_register: 0,
                _debug: 0,
            }
        }
    }

    #[derive(PartialEq)]
    enum RunnerStep {
        ReadOpCode,
        AddressStep,
        OpCodeStep,
    }

    pub struct Runner {
        cpu: N6502,
        op_code: u16,
        op_step: u8,
        address_step: u8,
        runner_step: RunnerStep,
        is_reset_set: bool,
        op_code_lookup: Vec<Box<dyn CpuOperation>>,
        address_method_lookup: Vec<Box<dyn AddressMethod>>,
        address_lookup: AddressBus,
    }

    impl Runner {
        pub fn new () -> Runner {
            Self {
                cpu: N6502::new(),
                op_code: 0x100,
                op_step: 0,
                address_step: 0,
                runner_step: RunnerStep::ReadOpCode,
                is_reset_set: true,
                op_code_lookup: OpCodesUtils::get_opcodes(),
                address_method_lookup: AddressOpCodes::get_address_methods(),
                address_lookup: AddressBus::new(0, false, 0),
            }
        }

        pub fn execute_tick(&mut self, addr: &mut AddressBus) {
            
            // Geting new op code
            if self.op_step == 0 && self.runner_step == RunnerStep::ReadOpCode {
                addr.is_accumulator = false;
                if self.is_reset_set {
                    self.op_code = 0x100;
                    self.runner_step = RunnerStep::OpCodeStep;
                    self.is_reset_set = false;
                } else {
                    self.runner_step = RunnerStep::AddressStep;
                    self.op_code = addr.byte as u16;
                    self.cpu.program_counter += 1;
                }
            }

            // Calling address/op tick
            if self.runner_step == RunnerStep::AddressStep {
                if self.address_method_lookup[self.op_code as usize].execute(&mut self.cpu, addr, self.address_step, &mut self.address_lookup) {
                    self.address_step = 0;
                    self.runner_step = RunnerStep::OpCodeStep;
                    addr.address = self.address_lookup.address;
                    addr.write = false;
                } else {
                    self.address_step += 1;
                }
            } else if self.runner_step == RunnerStep::OpCodeStep {
                addr.address = self.address_lookup.address;
                if self.op_code_lookup[self.op_code as usize].execute(&mut self.cpu, addr, self.op_step) {
                    self.op_step = 0;
                    self.runner_step = RunnerStep::ReadOpCode;
                } else {
                    self.op_step += 1;
                }
            }
            
            // read next opcode
            if self.runner_step == RunnerStep::ReadOpCode {
                addr.address = self.cpu.program_counter;
                addr.write = false;
            }
        }

    }

}
