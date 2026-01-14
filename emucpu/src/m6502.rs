
pub mod emu_cpu {

    use std::ptr::fn_addr_eq;

    use emumemory::memory_mapper::emu_memory::MemoryMapper;
    use crate::base_cpu::emu_cpu::BaseCpu;

    const CARRY_FLAG: u8 = 1;
    const ZERO_FLAG: u8 = 2;
    const INTERRUPT_FLAG: u8 = 4;
    const DECIMAL_MODE: u8 = 8;
    const BREAK_COMMAND: u8 = 16;
    const IGNORED: u8 = 32;
    const OVERFLOW_FLAG: u8 =64;
    const NEGATIVE_FLAG: u8 = 128;

    #[derive(Copy, Clone)]
    struct OperationStruct {
        operation: fn(&mut M6502, memory: &mut dyn MemoryMapper, fn(&mut M6502, memory: &dyn MemoryMapper) -> u16),
        address_method: fn(&mut M6502, memory: &dyn MemoryMapper) -> u16,
        ticks: u8
    }

    pub struct M6502 {
        //memory: Arc<Mutex< dyn MemoryMapper>>,

        overflow_ticks: i32,
        program_counter: u16,
        stack_pointer_page: u16,
        stack_pointer: u16,
        accumulator: u8,
        register_x: u8,
        register_y: u8,
        status_register: u8,

        op_code_lookup: [OperationStruct; 0x100],
        operation: OperationStruct,
        instruction: u8
    }

    impl BaseCpu for M6502 {

        fn execute_tick(&mut self, memory: &mut dyn MemoryMapper) {
            if self.overflow_ticks  > 1 {
                self.overflow_ticks -= 1;
                return;
            }
            self.overflow_ticks -= 1;
            
            self.call_op_method(self.operation.operation, memory, self.operation.address_method);

            self.get_next_operation(memory);
        }   

        fn reset(&mut self, memory: &dyn MemoryMapper) {
            //operation_ = opCodeLookup_[0xea]; //OpNOP       
            self.stack_pointer_page = 0x100;
            self.stack_pointer = 255;
            let pcl: u8 = memory.cpu_read(0xfffc);
            let pch: u8 = memory.cpu_read(0xfffd);
            self.program_counter = ((pch as u16) << 8) + pcl as u16;
            self.accumulator = 0;
            self.register_x = 0;
            self.register_y = 0;
            self.status_register = 0x40;
        } 
    }

    impl M6502 {

        pub fn new() -> Self {

            let op_code_lookup: [OperationStruct; 0x100] = M6502::get_op_codes();
            let operation: OperationStruct = op_code_lookup[0xea];

            Self {
                    //memory: memory_mapper,

                    overflow_ticks: 0,
                    program_counter: 0,
                    stack_pointer_page: 0x100,
                    stack_pointer: 0,
                    accumulator: 0,
                    register_x: 0,
                    register_y: 0,
                    status_register: 0x40,
                    op_code_lookup: op_code_lookup,
                    operation: operation,
                    instruction: 0
            }
        }

        fn get_op_codes() -> [OperationStruct; 0x100] {
            let panic_operation = OperationStruct { operation: M6502::op_panic, address_method: M6502::null_address, ticks: 2 };
            let mut op_code_lookup: [OperationStruct; 0x100] = [panic_operation; 0x100];

            op_code_lookup[0x00] = OperationStruct { operation: M6502::op_brk, address_method: M6502::null_address, ticks: 7 };
            op_code_lookup[0x01] = OperationStruct { operation: M6502::op_ora, address_method: M6502::indirect_x_address, ticks: 6 };
            op_code_lookup[0x02] = OperationStruct { operation: M6502::op_brk, address_method: M6502::null_address, ticks: 7 };
            op_code_lookup[0x03] = OperationStruct { operation: M6502::op_slo, address_method: M6502::indirect_x_address, ticks: 8 };
            op_code_lookup[0x04] = OperationStruct { operation: M6502::op_nop, address_method: M6502::zero_address, ticks: 3 };
            op_code_lookup[0x05] = OperationStruct { operation: M6502::op_ora, address_method: M6502::zero_address, ticks: 3 };
            op_code_lookup[0x06] = OperationStruct { operation: M6502::op_asl, address_method: M6502::zero_address, ticks: 5 };
            op_code_lookup[0x07] = OperationStruct { operation: M6502::op_slo, address_method: M6502::zero_address, ticks: 5 };
            op_code_lookup[0x08] = OperationStruct { operation: M6502::op_php, address_method: M6502::null_address, ticks: 3 };
            op_code_lookup[0x09] = OperationStruct { operation: M6502::op_ora, address_method: M6502::immediate_address, ticks: 2 };
            op_code_lookup[0x0a] = OperationStruct { operation: M6502::op_asl, address_method: M6502::accumulator_address, ticks: 2 };
            op_code_lookup[0x0b] = OperationStruct { operation: M6502::op_brk, address_method: M6502::null_address, ticks: 7 };
            op_code_lookup[0x0c] = OperationStruct { operation: M6502::op_nop, address_method: M6502::absolute_address, ticks: 4 };
            op_code_lookup[0x0d] = OperationStruct { operation: M6502::op_ora, address_method: M6502::absolute_address, ticks: 4 };
            op_code_lookup[0x0e] = OperationStruct { operation: M6502::op_asl, address_method: M6502::absolute_address, ticks: 6 };
            op_code_lookup[0x0f] = OperationStruct { operation: M6502::op_slo, address_method: M6502::absolute_address, ticks: 6 };

            op_code_lookup[0x10] = OperationStruct { operation: M6502::op_bpl, address_method: M6502::immediate_address, ticks: 2 };
            op_code_lookup[0x11] = OperationStruct { operation: M6502::op_ora, address_method: M6502::indirect_y_address, ticks: 5 };
            op_code_lookup[0x12] = OperationStruct { operation: M6502::op_brk, address_method: M6502::null_address, ticks: 7 };
            op_code_lookup[0x13] = OperationStruct { operation: M6502::op_slo, address_method: M6502::indirect_y_address_no_overflow, ticks: 8 };
            op_code_lookup[0x14] = OperationStruct { operation: M6502::op_nop, address_method: M6502::zero_x_address, ticks: 4 };
            op_code_lookup[0x15] = OperationStruct { operation: M6502::op_ora, address_method: M6502::zero_x_address, ticks: 4 };
            op_code_lookup[0x16] = OperationStruct { operation: M6502::op_asl, address_method: M6502::zero_x_address, ticks: 6 };
            op_code_lookup[0x17] = OperationStruct { operation: M6502::op_slo, address_method: M6502::zero_x_address, ticks: 6 };
            op_code_lookup[0x18] = OperationStruct { operation: M6502::op_clc, address_method: M6502::null_address, ticks: 2 };
            op_code_lookup[0x19] = OperationStruct { operation: M6502::op_ora, address_method: M6502::absolute_y_address, ticks: 4 };
            op_code_lookup[0x1a] = OperationStruct { operation: M6502::op_nop, address_method: M6502::null_address, ticks: 2 };
            op_code_lookup[0x1b] = OperationStruct { operation: M6502::op_slo, address_method: M6502::absolute_y_address_no_overflow, ticks: 7 };
            op_code_lookup[0x1c] = OperationStruct { operation: M6502::op_nop, address_method: M6502::absolute_x_address, ticks: 4 };
            op_code_lookup[0x1d] = OperationStruct { operation: M6502::op_ora, address_method: M6502::absolute_x_address, ticks: 4 };
            op_code_lookup[0x1e] = OperationStruct { operation: M6502::op_asl, address_method: M6502::absolute_x_address_no_overflow, ticks: 7 };
            op_code_lookup[0x1f] = OperationStruct { operation: M6502::op_slo, address_method: M6502::absolute_x_address_no_overflow, ticks: 7 };

            op_code_lookup[0x20] = OperationStruct { operation: M6502::op_jsr, address_method: M6502::absolute_address, ticks: 6 };
            op_code_lookup[0x24] = OperationStruct { operation: M6502::op_bit, address_method: M6502::zero_address, ticks: 3 };
            op_code_lookup[0x25] = OperationStruct { operation: M6502::op_and, address_method: M6502::zero_address, ticks: 3 };
            op_code_lookup[0x29] = OperationStruct { operation: M6502::op_and, address_method: M6502::immediate_address, ticks: 2 };
            op_code_lookup[0x2a] = OperationStruct { operation: M6502::op_rol, address_method: M6502::accumulator_address, ticks: 2 };

            op_code_lookup[0x30] = OperationStruct { operation: M6502::op_bmi, address_method: M6502::immediate_address, ticks: 2 };
            op_code_lookup[0x38] = OperationStruct { operation: M6502::op_sec, address_method: M6502::null_address, ticks: 2 };

            op_code_lookup[0x40] = OperationStruct { operation: M6502::op_rti, address_method: M6502::null_address, ticks: 6 };
            op_code_lookup[0x45] = OperationStruct { operation: M6502::op_eor, address_method: M6502::zero_address, ticks: 3 };
            op_code_lookup[0x49] = OperationStruct { operation: M6502::op_eor, address_method: M6502::immediate_address, ticks: 2 };
            op_code_lookup[0x4a] = OperationStruct { operation: M6502::op_lsr, address_method: M6502::accumulator_address, ticks: 2 };
            op_code_lookup[0x4c] = OperationStruct { operation: M6502::op_jmp, address_method: M6502::absolute_address, ticks: 3 };

            op_code_lookup[0x50] = OperationStruct { operation: M6502::op_bvc, address_method: M6502::immediate_address, ticks: 2 };
            op_code_lookup[0x5f] = OperationStruct { operation: M6502::op_brk, address_method: M6502::null_address, ticks: 7 };

            op_code_lookup[0x60] = OperationStruct { operation: M6502::op_rts, address_method: M6502::null_address, ticks: 6 };
            op_code_lookup[0x65] = OperationStruct { operation: M6502::op_adc, address_method: M6502::zero_address, ticks: 3 };
            op_code_lookup[0x69] = OperationStruct { operation: M6502::op_adc, address_method: M6502::immediate_address, ticks: 2 };

            op_code_lookup[0x70] = OperationStruct { operation: M6502::op_bvs, address_method: M6502::immediate_address, ticks: 2 };
            op_code_lookup[0x75] = OperationStruct { operation: M6502::op_adc, address_method: M6502::zero_address, ticks: 4 };
            op_code_lookup[0x78] = OperationStruct { operation: M6502::op_sei, address_method: M6502::null_address, ticks: 2 };
            op_code_lookup[0x79] = OperationStruct { operation: M6502::op_adc, address_method: M6502::absolute_y_address, ticks: 4 };

            op_code_lookup[0x80] = OperationStruct { operation: M6502::op_nop, address_method: M6502::immediate_address, ticks: 2 };
            op_code_lookup[0x84] = OperationStruct { operation: M6502::op_sty, address_method: M6502::zero_address, ticks: 3 };
            op_code_lookup[0x85] = OperationStruct { operation: M6502::op_sta, address_method: M6502::zero_address, ticks: 3 };
            op_code_lookup[0x86] = OperationStruct { operation: M6502::op_stx, address_method: M6502::zero_address, ticks: 3 };
            op_code_lookup[0x88] = OperationStruct { operation: M6502::op_dey, address_method: M6502::null_address, ticks: 2 };
            op_code_lookup[0x8a] = OperationStruct { operation: M6502::op_txa, address_method: M6502::null_address, ticks: 2 };
            op_code_lookup[0x8d] = OperationStruct { operation: M6502::op_sta, address_method: M6502::absolute_address, ticks: 4 };

            op_code_lookup[0x90] = OperationStruct { operation: M6502::op_bcc, address_method: M6502::immediate_address, ticks: 2 };
            op_code_lookup[0x94] = OperationStruct { operation: M6502::op_sty, address_method: M6502::zero_x_address, ticks: 4 };
            op_code_lookup[0x95] = OperationStruct { operation: M6502::op_sta, address_method: M6502::zero_x_address, ticks: 4 };
            op_code_lookup[0x98] = OperationStruct { operation: M6502::op_tya, address_method: M6502::null_address, ticks: 2 };
            op_code_lookup[0x99] = OperationStruct { operation: M6502::op_sta, address_method: M6502::absolute_y_address_no_overflow, ticks: 5 };
            op_code_lookup[0x9a] = OperationStruct { operation: M6502::op_txs, address_method: M6502::null_address, ticks: 2 };
            op_code_lookup[0x9b] = OperationStruct { operation: M6502::op_brk, address_method: M6502::null_address, ticks: 7 };

            op_code_lookup[0xa0] = OperationStruct { operation: M6502::op_ldy, address_method: M6502::immediate_address, ticks: 2 };
            op_code_lookup[0xa2] = OperationStruct { operation: M6502::op_ldx, address_method: M6502::immediate_address, ticks: 2 };
            op_code_lookup[0xa3] = OperationStruct { operation: M6502::op_brk, address_method: M6502::null_address, ticks: 7 };
            op_code_lookup[0xa4] = OperationStruct { operation: M6502::op_ldy, address_method: M6502::zero_address, ticks: 3 };
            op_code_lookup[0xa5] = OperationStruct { operation: M6502::op_lda, address_method: M6502::zero_address, ticks: 3 };
            op_code_lookup[0xa6] = OperationStruct { operation: M6502::op_ldx, address_method: M6502::zero_address, ticks: 3 };
            op_code_lookup[0xa8] = OperationStruct { operation: M6502::op_tay, address_method: M6502::null_address, ticks: 2 };
            op_code_lookup[0xa9] = OperationStruct { operation: M6502::op_lda, address_method: M6502::immediate_address, ticks: 2 };
            op_code_lookup[0xaa] = OperationStruct { operation: M6502::op_tax, address_method: M6502::null_address, ticks: 2 };
            op_code_lookup[0xad] = OperationStruct { operation: M6502::op_lda, address_method: M6502::absolute_address, ticks: 4 };

            op_code_lookup[0xb0] = OperationStruct { operation: M6502::op_bcs, address_method: M6502::immediate_address, ticks: 2 };
            op_code_lookup[0xb1] = OperationStruct { operation: M6502::op_lda, address_method: M6502::indirect_y_address, ticks: 5 };
            op_code_lookup[0xb5] = OperationStruct { operation: M6502::op_lda, address_method: M6502::zero_x_address, ticks: 4 };
            op_code_lookup[0xb9] = OperationStruct { operation: M6502::op_lda, address_method: M6502::absolute_y_address, ticks: 4 };
            op_code_lookup[0xba] = OperationStruct { operation: M6502::op_tsx, address_method: M6502::null_address, ticks: 2 };
            op_code_lookup[0xbd] = OperationStruct { operation: M6502::op_lda, address_method: M6502::absolute_x_address, ticks: 4 };

            op_code_lookup[0xc8] = OperationStruct { operation: M6502::op_iny, address_method: M6502::null_address, ticks: 2 };
            op_code_lookup[0xc9] = OperationStruct { operation: M6502::op_cmp, address_method: M6502::immediate_address, ticks: 2 };
            op_code_lookup[0xca] = OperationStruct { operation: M6502::op_dex, address_method: M6502::null_address, ticks: 2 };

            op_code_lookup[0xd0] = OperationStruct { operation: M6502::op_bne, address_method: M6502::immediate_address, ticks: 2 };
            op_code_lookup[0xd6] = OperationStruct { operation: M6502::op_dec, address_method: M6502::zero_x_address, ticks: 6 };
            op_code_lookup[0xd8] = OperationStruct { operation: M6502::op_cld, address_method: M6502::null_address, ticks: 2 };

            op_code_lookup[0xe0] = OperationStruct { operation: M6502::op_cpx, address_method: M6502::immediate_address, ticks: 2 };
            op_code_lookup[0xe5] = OperationStruct { operation: M6502::op_sbc, address_method: M6502::zero_address, ticks: 3 };
            op_code_lookup[0xe6] = OperationStruct { operation: M6502::op_inc, address_method: M6502::zero_address, ticks: 5 };
            op_code_lookup[0xe8] = OperationStruct { operation: M6502::op_inx, address_method: M6502::null_address, ticks: 2 };
            op_code_lookup[0xe9] = OperationStruct { operation: M6502::op_sbc, address_method: M6502::immediate_address, ticks: 2 };
            op_code_lookup[0xea] = OperationStruct { operation: M6502::op_nop, address_method: M6502::null_address, ticks: 2 };

            op_code_lookup[0xf0] = OperationStruct { operation: M6502::op_beq, address_method: M6502::immediate_address, ticks: 2 };
            op_code_lookup[0xf6] = OperationStruct { operation: M6502::op_inc, address_method: M6502::zero_x_address, ticks: 6 };
            op_code_lookup[0xf8] = OperationStruct { operation: M6502::op_sed, address_method: M6502::null_address, ticks: 2 };
            op_code_lookup[0xfc] = OperationStruct { operation: M6502::op_nop, address_method: M6502::absolute_x_address, ticks: 4 };
            op_code_lookup[0xff] = OperationStruct { operation: M6502::op_isc, address_method: M6502::absolute_x_address_no_overflow, ticks: 7 };

            op_code_lookup
        }

        fn call_op_method(&mut self, op: fn(&mut M6502, memory: &mut dyn MemoryMapper, fn(&mut M6502, memory: &dyn MemoryMapper) -> u16), memory: &mut dyn MemoryMapper, address_method: fn(&mut M6502, memory: &dyn MemoryMapper) -> u16) {
            op(self, memory, address_method);            
        }

        pub fn get_next_operation(&mut self, memory: &dyn MemoryMapper) {
            self.instruction = memory.cpu_read(self.program_counter);
            self.operation = self.op_code_lookup[memory.cpu_read(self.program_counter) as usize];
            self.program_counter += 1;

            self.overflow_ticks += self.operation.ticks as i32;
            self.set_overflow_for_operation(memory);
            self.set_overflow_for_address_access(memory, self.operation.address_method);
        }

        fn set_overflow_for_operation(&mut self, memory: &dyn MemoryMapper)
        {
            // If branch operaion takes a branch it causes extra tick
            if (fn_addr_eq(self.operation.operation, M6502::op_bcc as fn(&mut M6502, memory: &mut dyn MemoryMapper, fn(&mut M6502, memory: &dyn MemoryMapper) -> u16)) && self.get_status_flag(CARRY_FLAG) > 0) 
                || (fn_addr_eq(self.operation.operation, M6502::op_bcs as fn(&mut M6502, memory: &mut dyn MemoryMapper, fn(&mut M6502, memory: &dyn MemoryMapper) -> u16)) && self.get_status_flag(CARRY_FLAG) > 0) 
                || (fn_addr_eq(self.operation.operation, M6502::op_beq as fn(&mut M6502, memory: &mut dyn MemoryMapper, fn(&mut M6502, memory: &dyn MemoryMapper) -> u16)) && self.get_status_flag(ZERO_FLAG) > 0) 
                || (fn_addr_eq(self.operation.operation, M6502::op_bmi as fn(&mut M6502, memory: &mut dyn MemoryMapper, fn(&mut M6502, memory: &dyn MemoryMapper) -> u16)) && self.get_status_flag(NEGATIVE_FLAG) > 0) 
                || (fn_addr_eq(self.operation.operation, M6502::op_bne as fn(&mut M6502, memory: &mut dyn MemoryMapper, fn(&mut M6502, memory: &dyn MemoryMapper) -> u16)) && self.get_status_flag(ZERO_FLAG) > 0) 
                || (fn_addr_eq(self.operation.operation, M6502::op_bpl as fn(&mut M6502, memory: &mut dyn MemoryMapper, fn(&mut M6502, memory: &dyn MemoryMapper) -> u16)) && self.get_status_flag(NEGATIVE_FLAG) > 0) 
                || (fn_addr_eq(self.operation.operation, M6502::op_bvc as fn(&mut M6502, memory: &mut dyn MemoryMapper, fn(&mut M6502, memory: &dyn MemoryMapper) -> u16)) && self.get_status_flag(OVERFLOW_FLAG) > 0) 
                || (fn_addr_eq(self.operation.operation, M6502::op_bvs as fn(&mut M6502, memory: &mut dyn MemoryMapper, fn(&mut M6502, memory: &dyn MemoryMapper) -> u16)) && self.get_status_flag(OVERFLOW_FLAG) > 0) {
                self.overflow_ticks += 1;
                let address_met = self.operation.address_method;
                let relative_address: u16 = address_met(self, memory);
                if ((self.program_counter & 0x00FF) + relative_address) > 0x00FF {
                    self.overflow_ticks += 1;
                }
                self.program_counter -= 1;
            }
        }
        
        fn set_overflow_for_address_access(&mut self, memory: &dyn MemoryMapper, address_method: fn(&mut M6502, memory: &dyn MemoryMapper) -> u16) {
            let mut carry: u16 = 0;
            
            // overflow on address lookup ONLY if low byte carrys to 
            // high byte by adding X or Y register
            if fn_addr_eq(address_method, M6502::absolute_x_address as for<'a, 'b> fn(&'a mut M6502, &'b (dyn MemoryMapper + 'b)) -> u16) {
                let loadl: u8 = memory.cpu_read(self.program_counter);
                carry = loadl as u16 + self.register_x as u16;
            }
            else if fn_addr_eq(address_method, M6502::absolute_y_address as for<'a, 'b> fn(&'a mut M6502, &'b (dyn MemoryMapper + 'b)) -> u16) {
                let loadl: u8 = memory.cpu_read(self.program_counter);
                carry = loadl as u16 + self.register_y as u16;
            }
            else if fn_addr_eq(address_method, M6502::indirect_y_address as for<'a, 'b> fn(&'a mut M6502, &'b (dyn MemoryMapper + 'b)) -> u16) {
                let indirect: u16 = memory.cpu_read(self.program_counter) as u16;
                let loadl: u8 = memory.cpu_read(indirect & 0xff);
                carry = loadl as u16 + self.register_y as u16;
            }
            
            // Carry goes into high byte, so requires extra clock cycle
            if carry > 0x00FF {
                self.overflow_ticks += 1;
            }
        }

        fn push_stack(&mut self, byte: u8, memory: &mut dyn MemoryMapper) {
            if self.stack_pointer == 0 {
                panic!("Stack overflow");
            }
            memory.cpu_write(self.stack_pointer + self.stack_pointer_page, byte);
            self.stack_pointer -= 1;
        }

        fn pop_stack(&mut self, memory: &dyn MemoryMapper) -> u8 {
            if self.stack_pointer > 255 {
                panic!("Stack underflow");
            }
            self.stack_pointer += 1;
            memory.cpu_read(self.stack_pointer + self.stack_pointer_page)
        }

        fn null_address(&mut self, _memory: &dyn MemoryMapper) -> u16 {
            0
        }

        fn accumulator_address(&mut self, _memory: &dyn MemoryMapper) -> u16 {
            self.accumulator as u16
        }

        fn immediate_address(&mut self, _memory: &dyn MemoryMapper) -> u16 {
            let address: u16 = self.program_counter;
            self.program_counter += 1;
            address
        }
        
        fn zero_address(&mut self, memory: &dyn MemoryMapper) -> u16 {
            let address: u16 = memory.cpu_read(self.program_counter) as u16 & 0xFF;
            self.program_counter += 1;
            address
        }
        
        fn zero_x_address(&mut self, memory: &dyn MemoryMapper) -> u16 {
            let address: u16 = memory.cpu_read(self.program_counter) as u16 + self.register_x as u16 & 0xFF;
            self.program_counter += 1;
            address
        }

        fn zero_y_address(&mut self, memory: &dyn MemoryMapper) -> u16 {
            let address: u16 = memory.cpu_read(self.program_counter) as u16 + self.register_y as u16 & 0xFF;
            self.program_counter += 1;
            address
        }

        fn absolute_address(&mut self, memory: &dyn MemoryMapper) -> u16 {
            let loadl: u8 = memory.cpu_read(self.program_counter);
            self.program_counter += 1;
            let loadh: u8 = memory.cpu_read(self.program_counter);
            self.program_counter += 1;
            let address: u16 = ((loadh as u16) << 8) + (loadl as u16);
            address
        }

        fn absolute_x_address(&mut self, memory: &dyn MemoryMapper) -> u16 {
            let loadl: u8 = memory.cpu_read(self.program_counter);
            self.program_counter += 1;
            let loadh: u8 = memory.cpu_read(self.program_counter);
            self.program_counter += 1;
            let address: u16 = ((loadh as u16) << 8) + loadl as u16 + self.register_x as u16;
            address
        }

        fn absolute_x_address_no_overflow(&mut self, memory: &dyn MemoryMapper) -> u16 {
            self.absolute_x_address(memory)
        }

        fn absolute_y_address(&mut self, memory: &dyn MemoryMapper) -> u16 {
            let loadl: u8 = memory.cpu_read(self.program_counter);
            self.program_counter += 1;
            let loadh: u8 = memory.cpu_read(self.program_counter);
            self.program_counter += 1;
            let address: u16 = ((loadh as u16) << 8) + loadl as u16 + self.register_y as u16;
            address
        }

        fn absolute_y_address_no_overflow(&mut self, memory: &dyn MemoryMapper) -> u16 {
            self.absolute_y_address(memory)
        }

        fn indirect_x_address(&mut self, memory: &dyn MemoryMapper) -> u16 {
            let mut indirect: u16 = memory.cpu_read(self.program_counter) as u16 + self.register_x as u16;
            self.program_counter += 1;
            let loadl: u8 = memory.cpu_read(indirect & 0xff);
            indirect += 1;
            let loadh: u8 = memory.cpu_read(indirect & 0xff);
            ((loadh as u16) << 8) + loadl as u16
        }

        fn indirect_y_address(&mut self, memory: &dyn MemoryMapper) -> u16 {
            let mut indirect: u16 = memory.cpu_read(self.program_counter) as u16;
            self.program_counter += 1;
            let loadl: u8 = memory.cpu_read(indirect & 0xff);
            indirect += 1;
            let loadh: u8 = memory.cpu_read(indirect & 0xff);
            let address: u16 = (loadh as u16).wrapping_shl(8) + loadl as u16 + self.register_y as u16;
            address 
        }

        fn indirect_y_address_no_overflow(&mut self, memory: &dyn MemoryMapper) -> u16 {
            self.indirect_y_address(memory)
        }

        pub fn get_overflow_ticks(&self) -> i32 {
            self.overflow_ticks
        }

        pub fn get_accumulator(&self) -> u8 {
            self.accumulator
        }

        pub fn get_register_x(&self) -> u8 {
            self.register_x
        }

        pub fn get_register_y(&self) -> u8 {
            self.register_y
        }

        pub fn get_status_register(&self) -> u8 {
            self.status_register
        }

        fn get_status_flag(&self, flag: u8) -> u8 {
            self.status_register & flag
        }

        fn set_status_flag(&mut self, flag: u8, set_clear: bool) {
            if set_clear {
                self.status_register |= flag;
            }
            else {
                self.status_register &= !flag;
            }
        }

        fn set_negative(&mut self, byte: u8) {
            self.set_status_flag(NEGATIVE_FLAG, (byte & 0x80) > 0);
        }

        fn set_zero(&mut self, byte: u8) {
            self.set_status_flag(ZERO_FLAG, byte == 0);
        }

        fn set_negative_zero(&mut self, byte: u8) {
            self.set_negative(byte);
            self.set_zero(byte);
        }

        // Load Store operations
        fn op_lda(&mut self, memory: &mut dyn MemoryMapper, address_method: fn(&mut M6502, memory: &dyn MemoryMapper) -> u16) {
            let address: u16 = address_method(self, memory);
            self.accumulator = memory.cpu_read(address);
            self.set_negative_zero(self.accumulator);
        }

        fn op_ldx(&mut self, memory: &mut dyn MemoryMapper, address_method: fn(&mut M6502, memory: &dyn MemoryMapper) -> u16) {
            self.register_x = memory.cpu_read(address_method(self, memory));
            self.set_negative_zero(self.register_x);
        }

        fn op_ldy(&mut self, memory: &mut dyn MemoryMapper, address_method: fn(&mut M6502, memory: &dyn MemoryMapper) -> u16){
            self.register_y = memory.cpu_read(address_method(self, memory));
            self.set_negative_zero(self.register_y);
        }

        fn op_sta(&mut self, memory: &mut dyn MemoryMapper, address_method: fn(&mut M6502, memory: &dyn MemoryMapper) -> u16){
            let address: u16 = address_method(self, memory);
            memory.cpu_write(address, self.accumulator);
        }
        
        fn op_stx(&mut self, memory: &mut dyn MemoryMapper, address_method: fn(&mut M6502, memory: &dyn MemoryMapper) -> u16){
            let address: u16 = address_method(self, memory);
            memory.cpu_write(address, self.register_x);
        }

        fn op_sty(&mut self, memory: &mut dyn MemoryMapper, address_method: fn(&mut M6502, memory: &dyn MemoryMapper) -> u16){
            let address: u16 = address_method(self, memory);
            memory.cpu_write(address, self.register_y);
        }

        // Register transfers
        fn op_tax(&mut self, _memory: &mut dyn MemoryMapper, _address_method: fn(&mut M6502, memory: &dyn MemoryMapper) -> u16) {
            self.register_x = self.accumulator;
            self.set_negative_zero(self.register_x);
        }

        fn op_tay(&mut self, _memory: &mut dyn MemoryMapper, _address_method: fn(&mut M6502, memory: &dyn MemoryMapper) -> u16) {
            self.register_y = self.accumulator;
            self.set_negative_zero(self.register_y);
        }

        fn op_txa(&mut self, _memory: &mut dyn MemoryMapper, _address_method: fn(&mut M6502, memory: &dyn MemoryMapper) -> u16) {
            self.accumulator = self.register_x;
            self.set_negative_zero(self.accumulator);
        }

        fn op_tya(&mut self, _memory: &mut dyn MemoryMapper, _address_method: fn(&mut M6502, memory: &dyn MemoryMapper) -> u16) {
            self.accumulator = self.register_y;
            self.set_negative_zero(self.accumulator);
        }

        // Stack operaions
        fn op_tsx(&mut self, _memory: &mut dyn MemoryMapper, _address_method: fn(&mut M6502, memory: &dyn MemoryMapper) -> u16) {
            self.register_x = self.stack_pointer as u8;
            self.set_negative_zero(self.register_x);
        }

        fn op_txs(&mut self, _memory: &mut dyn MemoryMapper, _address_method: fn(&mut M6502, memory: &dyn MemoryMapper) -> u16) {
            self.stack_pointer = self.register_x as u16;
        }

        fn op_pha(&mut self, memory: &mut dyn MemoryMapper, _address_method: fn(&mut M6502, memory: &dyn MemoryMapper) -> u16) {
            self.push_stack(self.accumulator, memory);
        }

        fn op_php(&mut self, memory: &mut dyn MemoryMapper, _address_method: fn(&mut M6502, memory: &dyn MemoryMapper) -> u16) {
            self.set_status_flag(BREAK_COMMAND, true);
            self.set_status_flag(IGNORED, true);
            self.push_stack(self.status_register, memory);
            self.set_status_flag(BREAK_COMMAND, false);
            self.set_status_flag(IGNORED, false);
        }
        fn op_pla(&mut self, memory: &mut dyn MemoryMapper, _address_method: fn(&mut M6502, memory: &dyn MemoryMapper) -> u16) {
            self.accumulator = self.pop_stack(memory);
            self.set_negative_zero(self.accumulator);
        }

        fn op_plp(&mut self, memory: &mut dyn MemoryMapper, _address_method: fn(&mut M6502, memory: &dyn MemoryMapper) -> u16) {
            self.status_register = self.pop_stack(memory);
        }

        // Logical operations
        fn op_and(&mut self, memory: &mut dyn MemoryMapper, address_method: fn(&mut M6502, memory: &dyn MemoryMapper) -> u16){
            let byte: u8 = memory.cpu_read(address_method(self, memory));
            self.accumulator &= byte;
            self.set_negative_zero(self.accumulator);
        }

        fn op_eor(&mut self, memory: &mut dyn MemoryMapper, address_method: fn(&mut M6502, memory: &dyn MemoryMapper) -> u16){
            let byte: u8 = memory.cpu_read(address_method(self, memory));
            self.accumulator ^= byte;
            self.set_negative_zero(self.accumulator);
        }

        fn op_ora(&mut self, memory: &mut dyn MemoryMapper, address_method: fn(&mut M6502, memory: &dyn MemoryMapper) -> u16){
            let byte: u8 = memory.cpu_read(address_method(self, memory));
            self.accumulator |= byte;
            self.set_negative_zero(self.accumulator);
        }

        fn op_bit(&mut self, memory: &mut dyn MemoryMapper, address_method: fn(&mut M6502, memory: &dyn MemoryMapper) -> u16) {
            let byte: u8 = memory.cpu_read(address_method(self, memory));
            self.set_status_flag(OVERFLOW_FLAG, byte & 0x40 > 0);
            self.set_negative(byte);
            self.set_zero(byte & self.accumulator);
        }

        fn op_adc(&mut self, memory: &mut dyn MemoryMapper, address_method: fn(&mut M6502, memory: &dyn MemoryMapper) -> u16) {
            let mut byte: u8 = memory.cpu_read(address_method(self, memory));
            let mut value: u8;

            if self.get_status_flag(DECIMAL_MODE) > 0 {
                byte = (((byte & 0xF0) >> 4) * 10) + (byte & 0x0F) + self.get_status_flag(CARRY_FLAG);
                let temp_accumulator: u8 = (((self.accumulator & 0xF0) >> 4) * 10) + (self.accumulator & 0x0F);
                value = temp_accumulator + byte + self.get_status_flag(CARRY_FLAG);
                self.set_status_flag(CARRY_FLAG, value > 99);
                if value > 99 {
                    value -= 100;
                }
                value = ((value / 10) << 4) + (value % 10);
            }
            else
            {
                value = self.accumulator + byte + self.get_status_flag(CARRY_FLAG);
                self.set_status_flag(CARRY_FLAG, value > 255);
            }
            // set overflow if highest bit of accumulator and byte are same 
            self.set_status_flag(OVERFLOW_FLAG, ((self.accumulator ^ value) & (byte ^ value) & 0x80) != 0);
            self.accumulator = value as u8;
            self.set_negative_zero(self.accumulator);
        }

        // Same as ADC, except switch input byte to 1s-Compliment
        fn op_sbc(&mut self, memory: &mut dyn MemoryMapper, address_method: fn(&mut M6502, memory: &dyn MemoryMapper) -> u16) {
            let mut byte: u8 = memory.cpu_read(address_method(self, memory));
            let mut value: u8 = 0;

            if self.get_status_flag(DECIMAL_MODE) > 0 {
                byte = (((byte & 0xF0) >> 4) * 10) + (byte & 0x0F);
                let temp_accumulator: u8 = (((self.accumulator & 0xF0) >> 4) * 10) + (self.accumulator & 0x0F);
                let (result, mut _overflowed) = temp_accumulator.overflowing_sub(byte);
                (value, _overflowed) = result.overflowing_sub(self.get_status_flag(CARRY_FLAG));
                if (value as i8) < 0 {
                    (value, _overflowed) = value.overflowing_add(100);
                }
                value = ((value / 10) << 4) + (value % 10);
            }
            else
            {
                byte = !byte;
                let (tmp_value, _overflow) = byte.overflowing_add(self.get_status_flag(CARRY_FLAG));
                let (value, _overflow) = self.accumulator.overflowing_add(tmp_value);
                self.set_status_flag(CARRY_FLAG, value > 255);
            }
            self.set_status_flag(OVERFLOW_FLAG, ((self.accumulator ^ value) & (byte ^ value) & 0x80) != 0);
            self.accumulator = value;
            self.set_negative_zero(self.accumulator);
        }

        fn op_cmp(&mut self, memory: &mut dyn MemoryMapper, address_method: fn(&mut M6502, memory: &dyn MemoryMapper) -> u16){
            let byte: u8 = memory.cpu_read(address_method(self, memory));
            self.set_status_flag(CARRY_FLAG, self.accumulator >= byte);
            let(result, _overflow) = self.accumulator.overflowing_sub(byte);
            self.set_negative_zero(result);
        }

        fn op_cpx(&mut self, memory: &mut dyn MemoryMapper, address_method: fn(&mut M6502, memory: &dyn MemoryMapper) -> u16){
            let byte: u8 = memory.cpu_read(address_method(self, memory));
            self.set_status_flag(CARRY_FLAG, self.register_x >= byte);
            self.set_negative_zero(self.register_x - byte);
        }

        fn op_cpy(&mut self, memory: &mut dyn MemoryMapper, address_method: fn(&mut M6502, memory: &dyn MemoryMapper) -> u16){
            let byte: u8 = memory.cpu_read(address_method(self, memory));
            self.set_status_flag(CARRY_FLAG, self.register_y >= byte);
            self.set_negative_zero(self.register_y - byte);
        }

        // Increment and decrement operations
        fn op_inc(&mut self, memory: &mut dyn MemoryMapper, _address_method: fn(&mut M6502, memory: &dyn MemoryMapper) -> u16) {
            let address: u16 = _address_method(self, memory);
            let byte: u8 = memory.cpu_read(address);
            let(byte, _overflow) = byte.overflowing_add(1);
            memory.cpu_write(address, byte);
            self.set_negative_zero(byte);
        }

        fn op_inx(&mut self, _memory: &mut dyn MemoryMapper, _address_method: fn(&mut M6502, memory: &dyn MemoryMapper) -> u16) {
            let (value, _overflow) = self.register_x.overflowing_add(1);
            self.register_x = value;
            self.set_negative_zero(self.register_x);
        }

        fn op_iny(&mut self, _memory: &mut dyn MemoryMapper, _address_method: fn(&mut M6502, memory: &dyn MemoryMapper) -> u16) {
            self.register_y += 1;
            self.set_negative_zero(self.register_y);
        }

        fn op_dec(&mut self, memory: &mut dyn MemoryMapper, _address_method: fn(&mut M6502, memory: &dyn MemoryMapper) -> u16) {
            let address: u16 = _address_method(self, memory);
            let byte: u8 = memory.cpu_read(address);
            let (byte, _overflow) = byte.overflowing_sub(1);
            memory.cpu_write(address, byte);
            self.set_negative_zero(byte);
        }

        fn op_dex(&mut self, _memory: &mut dyn MemoryMapper, _address_method: fn(&mut M6502, memory: &dyn MemoryMapper) -> u16) {
            let (result, _overflow) = self.register_x.overflowing_sub(1);
            self.register_x = result;
            self.set_negative_zero(self.register_x);
        }

        fn op_dey(&mut self, _memory: &mut dyn MemoryMapper, _address_method: fn(&mut M6502, memory: &dyn MemoryMapper) -> u16) {
            self.register_y -= 1;
            self.set_negative_zero(self.register_y);
        }

        // Shift operations
        fn op_asl(&mut self, memory: &mut dyn MemoryMapper, address_method: fn(&mut M6502, memory: &dyn MemoryMapper) -> u16){
            let mut byte: u8;
            let mut address: u16 = 0;

            if fn_addr_eq(address_method, M6502::accumulator_address as for<'a, 'b> fn(&'a mut M6502, &'b (dyn MemoryMapper + 'b)) -> u16) {
                byte = self.accumulator;
            }
            else {
                address = address_method(self, memory);
                byte = memory.cpu_read(address);
            }
            self.set_status_flag(CARRY_FLAG, byte & 0x80 > 0);
            byte = byte << 1;
            self.set_negative_zero(byte);
            if fn_addr_eq(address_method, M6502::accumulator_address as for<'a, 'b> fn(&'a mut M6502, &'b (dyn MemoryMapper + 'b)) -> u16) {
                self.accumulator = byte;
            }
            else {
                memory.cpu_write(address, byte);
            }
        }    

        fn op_lsr(&mut self, memory: &mut dyn MemoryMapper, address_method: fn(&mut M6502, memory: &dyn MemoryMapper) -> u16){
            let mut byte: u8 = 0;
            let mut address: u16 = 0;

            if fn_addr_eq(address_method, M6502::accumulator_address as for<'a, 'b> fn(&'a mut M6502, &'b (dyn MemoryMapper + 'b)) -> u16) {
                byte = address_method(self, memory) as u8;
            }
            else {
                address = address_method(self, memory);
                byte = memory.cpu_read(address);
            }
            self.set_status_flag(CARRY_FLAG, byte & 0x01 > 0);
            byte = byte >> 1;
            self.set_negative_zero(byte);
            if fn_addr_eq(address_method, M6502::accumulator_address as for<'a, 'b> fn(&'a mut M6502, &'b (dyn MemoryMapper + 'b)) -> u16) {
                self.accumulator = byte;
            }
            else {
                memory.cpu_write(address, byte);
            }
        }

        fn op_rol(&mut self, memory: &mut dyn MemoryMapper, address_method: fn(&mut M6502, memory: &dyn MemoryMapper) -> u16){
            let mut byte: u8;
            let mut address: u16 = 0;
            if fn_addr_eq(address_method, M6502::accumulator_address as for<'a, 'b> fn(&'a mut M6502, &'b (dyn MemoryMapper + 'b)) -> u16) {
                byte = address_method(self, memory) as u8;
            }
            else {
                address = address_method(self, memory);
                byte = memory.cpu_read(address);
            }
            let temp: u8 = (byte << 1) + self.get_status_flag(CARRY_FLAG);
            self.set_status_flag(CARRY_FLAG, (byte & 0x80) > 0);
            byte = temp;
            self.set_negative_zero(byte);
            if fn_addr_eq(address_method, M6502::accumulator_address as for<'a, 'b> fn(&'a mut M6502, &'b (dyn MemoryMapper + 'b)) -> u16) {
                self.accumulator  = byte;
            }
            else {
                memory.cpu_write(address, byte);
            }
        }

        fn op_ror(&mut self, memory: &mut dyn MemoryMapper, address_method: fn(&mut M6502, memory: &dyn MemoryMapper) -> u16){
            let mut byte: u8;
            let mut address: u16 = 0;
            if fn_addr_eq(address_method, M6502::accumulator_address as for<'a, 'b> fn(&'a mut M6502, &'b (dyn MemoryMapper + 'b)) -> u16) {
                byte = address_method(self, memory) as u8;
            }
            else {
                address = address_method(self, memory);
                byte = memory.cpu_read(address);
            }
            let temp: u8 = (byte >> 1) + self.get_status_flag(CARRY_FLAG);
            self.set_status_flag(CARRY_FLAG, byte & 0x01 > 0);
            byte = temp;
            self.set_negative_zero(byte);
            if fn_addr_eq(address_method, M6502::accumulator_address as for<'a, 'b> fn(&'a mut M6502, &'b (dyn MemoryMapper + 'b)) -> u16) {
                self.accumulator  = byte;
            }
            else
            {
                memory.cpu_write(address, byte);
            }
        }

        // Jumps and Call operations
        fn op_jmp(&mut self, memory: &mut dyn MemoryMapper, _address_method: fn(&mut M6502, memory: &dyn MemoryMapper) -> u16) {
            let address: u16 = _address_method(self, memory);
            self.program_counter = address;
        }

        fn op_jsr(&mut self, memory: &mut dyn MemoryMapper, address_method: fn(&mut M6502, memory: &dyn MemoryMapper) -> u16) {
            let jump_address: u16 = address_method(self, memory);
            self.program_counter -= 1;
            self.push_stack(((self.program_counter & 0xff00) >> 8) as u8, memory);
            self.push_stack((self.program_counter & 0x00ff) as u8, memory);
            self.program_counter = jump_address;
        }

        fn op_rts(&mut self, memory: &mut dyn MemoryMapper, _address_method: fn(&mut M6502, memory: &dyn MemoryMapper) -> u16) {
            let loadl: u8 = self.pop_stack(memory);
            let loadh: u8 = self.pop_stack(memory);
            let address: u16 = ((loadh as u16) << 8) + loadl as u16;
            self.program_counter = address;
            self.program_counter += 1;
        }

        // Branch operations
        fn op_bcc(&mut self, memory: &mut dyn MemoryMapper, address_method: fn(&mut M6502, memory: &dyn MemoryMapper) -> u16){
            let relative_address: i8 = memory.cpu_read(address_method(self, memory)) as i8;
            if self.get_status_flag(CARRY_FLAG) == 0 {
                if relative_address > 0 {
                    self.program_counter += relative_address as u16;
                }
                else {
                    self.program_counter -= relative_address.abs() as u16;
                }
            }
        }

        fn op_bcs(&mut self, memory: &mut dyn MemoryMapper, address_method: fn(&mut M6502, memory: &dyn MemoryMapper) -> u16){
            let relative_address: i8 = memory.cpu_read(address_method(self, memory)) as i8;
            if self.get_status_flag(CARRY_FLAG) > 0 {
                if relative_address > 0 {
                    self.program_counter += relative_address as u16;
                }
                else {
                    self.program_counter -= relative_address.abs() as u16;
                }
            }
        }

        fn op_beq(&mut self, memory: &mut dyn MemoryMapper, address_method: fn(&mut M6502, memory: &dyn MemoryMapper) -> u16){
            let relative_address: i8 = memory.cpu_read(address_method(self, memory)) as i8;
            if self.get_status_flag(ZERO_FLAG) > 0 {
                if relative_address > 0 {
                    self.program_counter += relative_address as u16;
                }
                else {
                    self.program_counter -= relative_address.abs() as u16;
                }
            }
        }
        
        fn op_bmi(&mut self, memory: &mut dyn MemoryMapper, address_method: fn(&mut M6502, memory: &dyn MemoryMapper) -> u16){
            let relative_address: i8 = memory.cpu_read(address_method(self, memory)) as i8;
            if self.get_status_flag(NEGATIVE_FLAG) > 0 {
                if relative_address > 0 {
                    self.program_counter += relative_address as u16;
                }
                else {
                    self.program_counter -= relative_address.abs() as u16;
                }
            }
        }  

        fn op_bne(&mut self, memory: &mut dyn MemoryMapper, address_method: fn(&mut M6502, memory: &dyn MemoryMapper) -> u16){
            let relative_address: i8 = memory.cpu_read(address_method(self, memory)) as i8;
            if self.get_status_flag(ZERO_FLAG) == 0 {
                if relative_address > 0 {
                    self.program_counter += relative_address as u16;
                }
                else {
                    self.program_counter -= relative_address.abs() as u16;
                }
            }
        }
        
        fn op_bpl(&mut self, memory: &mut dyn MemoryMapper, address_method: fn(&mut M6502, memory: &dyn MemoryMapper) -> u16){
            let relative_address: i8 = memory.cpu_read(address_method(self, memory)) as i8;
            if self.get_status_flag(NEGATIVE_FLAG) == 0 {
                if relative_address > 0 {
                    self.program_counter += relative_address as u16;
                }
                else {
                    self.program_counter -= relative_address.abs() as u16;
                }
            }
        }

        fn op_bvc(&mut self, memory: &mut dyn MemoryMapper, address_method: fn(&mut M6502, memory: &dyn MemoryMapper) -> u16){
            let relative_address: i8 = memory.cpu_read(address_method(self, memory)) as i8;
            if self.get_status_flag(OVERFLOW_FLAG) == 0 {
                if relative_address > 0 {
                    self.program_counter += relative_address as u16;
                }
                else {
                    self.program_counter -= relative_address.abs() as u16;
                }
            }
        }

        fn op_bvs(&mut self, memory: &mut dyn MemoryMapper, address_method: fn(&mut M6502, memory: &dyn MemoryMapper) -> u16){
            let relative_address: i8 = memory.cpu_read(address_method(self, memory)) as i8;
            if self.get_status_flag(OVERFLOW_FLAG) > 0 {
                if relative_address > 0 {
                    self.program_counter += relative_address as u16;
                }
                else {
                    self.program_counter -= relative_address.abs() as u16;
                }
            }
        }

        // Status Flag operations
        fn op_clc(&mut self, _memory: &mut dyn MemoryMapper, _address_method: fn(&mut M6502, memory: &dyn MemoryMapper) -> u16) {
            self.set_status_flag(CARRY_FLAG, false);
        }

        fn op_cld(&mut self, _memory: &mut dyn MemoryMapper, _address_method : fn(&mut M6502, memory: &dyn MemoryMapper) -> u16) {
            self.set_status_flag(DECIMAL_MODE, false);
        }

        fn op_cli(&mut self, _memory: &mut dyn MemoryMapper, _address_method : fn(&mut M6502, memory: &dyn MemoryMapper) -> u16) {
            self.set_status_flag(INTERRUPT_FLAG, false);
        }

        fn op_clv(&mut self, _memory: &mut dyn MemoryMapper, _address_method : fn(&mut M6502, memory: &dyn MemoryMapper) -> u16) {
            self.set_status_flag(OVERFLOW_FLAG, false);
        }

        fn op_sec(&mut self, _memory: &mut dyn MemoryMapper, _address_method: fn(&mut M6502, memory: &dyn MemoryMapper) -> u16) {
            self.set_status_flag(CARRY_FLAG, true);
        }

        fn op_sed(&mut self, _memory: &mut dyn MemoryMapper, _address_method: fn(&mut M6502, memory: &dyn MemoryMapper) -> u16) {
            self.set_status_flag(DECIMAL_MODE, true);
        }

        fn op_sei(&mut self, _memory: &mut dyn MemoryMapper, _address_method: fn(&mut M6502, memory: &dyn MemoryMapper) -> u16) {
            self.set_status_flag(INTERRUPT_FLAG, true);
        }

        // System operations
        fn op_brk(&mut self, memory: &mut dyn MemoryMapper, _address_method: fn(&mut M6502, memory: &dyn MemoryMapper) -> u16) {
            self.set_status_flag(INTERRUPT_FLAG, true);
            self.program_counter += 1;
            self.push_stack(((self.program_counter & 0xff00) >> 8) as u8, memory);
            self.push_stack((self.program_counter & 0xff) as u8, memory);
            self.set_status_flag(BREAK_COMMAND, true);
            self.push_stack(self.status_register, memory);
            self.set_status_flag(BREAK_COMMAND, false);
            let loadl: u8 = memory.cpu_read(0xfffe);
            let loadh: u8 = memory.cpu_read(0xffff);
            let load: u16 = ((loadh as u16) << 8) + loadl as u16;
            self.program_counter = load;
        }

        fn op_nop(&mut self, _memory: &mut dyn MemoryMapper, _address_method: fn(&mut M6502, memory: &dyn MemoryMapper) -> u16) {
        }

        fn op_rti(&mut self, memory: &mut dyn MemoryMapper, _address_method: fn(&mut M6502, memory: &dyn MemoryMapper) -> u16) {
            self.status_register = self.pop_stack(memory);
            self.set_status_flag(BREAK_COMMAND, false);
            let loadl: u8 = self.pop_stack(memory);
            let loadh: u8 = self.pop_stack(memory);
            let load: u16 = ((loadh as u16) << 8) + loadl as u16;
            self.program_counter = load;
        }

        fn op_isc(&mut self, memory: &mut dyn MemoryMapper, address_method: fn(&mut M6502, memory: &dyn MemoryMapper) -> u16) {
            let address: u16 = address_method(self, memory);
            let mut byte: u8 = memory.cpu_read(address);
            byte += 1;
            memory.cpu_write(address, byte);
            
            byte = !byte;
            let value: u16 = (self.accumulator + byte + self.get_status_flag(CARRY_FLAG)) as u16;
            self.set_status_flag(CARRY_FLAG, value > 255);
            self.set_status_flag(OVERFLOW_FLAG, (self.accumulator as u16 ^ value) & (byte as u16 ^ value) & 0x80 != 0);
            self.accumulator = value as u8;
            self.set_negative_zero(self.accumulator);
        }

        fn op_slo(&mut self, memory: &mut dyn MemoryMapper, address_method: fn(&mut M6502, memory: &dyn MemoryMapper) -> u16) {
            let address: u16 = memory.cpu_read(address_method(self, memory)) as u16;
            let mut byte: u8 = memory.cpu_read(address);
            self.set_status_flag(CARRY_FLAG, (byte & 0x80) > 0);
            byte = byte << 1;
            memory.cpu_write(address, byte);

            self.accumulator |= byte;
            self.set_negative_zero(self.accumulator);
        }     

        fn op_panic(&mut self, _memory: &mut dyn MemoryMapper, _address_method: fn(&mut M6502, memory: &dyn MemoryMapper) -> u16) {
            panic!("operation not implemented yet 0x{:x}", self.instruction);
        }

    }

}