
use emumemory::base_memory::emu_memory::BaseMemory;
use emumemory::memory_ram::emu_memory::MemoryRam;
use emumemory::memory_rom::emu_memory::MemoryRom;

#[test]
fn test_ram_load_read() {
    let mut memory = MemoryRam::new(
        String::from("RAM Test")
    );

    memory.load_data(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    assert_eq!(memory.read(0), 1);
    assert_eq!(memory.read(4), 5);
}

#[test]
fn test_ram_write() {
    let mut memory = MemoryRam::new(
        String::from("RAM Test")
    );

    memory.load_data(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    assert_eq!(memory.read(0), 1);
    assert_eq!(memory.read(4), 5);
    memory.write(0, 51);
    memory.write(4, 72);
    assert_eq!(memory.read(0), 51);
    assert_eq!(memory.read(4), 72);
}

#[test]
fn test_ram_name() {
    let memory = MemoryRam::new(
        String::from("RAM Test")
    );

    assert_eq!(memory.get_name(), String::from("RAM Test"));
}

