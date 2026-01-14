

use emucpu::base_cpu::emu_cpu::BaseCpu;
use emucpu::m6502::emu_cpu::M6502;
use emumemory::base_memory::emu_memory::BaseMemory;

#[test]
fn test_opcode_0x01() {
    let mut cpu = M6502::new(Box::new(M6502TestMemory::new()));

    cpu.reset();
    cpu.op_0x01();

    assert_eq!(cpu.get_overflow_ticks(), 6);
    assert_eq!(cpu.get_accumulator(), 1);
    assert_eq!(cpu.get_status_register(), 1);
}

pub struct M6502TestMemory {
    ram: emumemory::memory_ram::emu_memory::MemoryRam
}

impl M6502TestMemory {
    pub fn new() -> Self {
        let mut ram = emumemory::memory_ram::emu_memory::MemoryRam::new(String::from("RAM Test"));
        ram.load_data(&[1, 0, 0, 4, 5, 6, 7, 8, 9, 10]);

        Self {
            ram: ram
        }
    }
}

impl emumemory::memory_mapper::emu_memory::MemoryMapper for M6502TestMemory {
        fn cpu_read(&self, location: u16) -> u8 {

            let result: u8;

            if location == 0xfffc {
                result = 0;
            }
            else if location == 0xfffd {
                result = 0;
            }
            else {
                result = self.ram.read(location);
            }

            result
        }

        fn cpu_write(&mut self, location: u16, byte: u8) {
            self.ram.write(location, byte);
        }
}


