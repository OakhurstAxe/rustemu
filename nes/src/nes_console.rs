

pub mod nes {

    use std::sync::RwLock;

    use emucpu::n6502::emu_cpu::M6502Runner;
    use emucpu::prelude::*;
    use emumemory::prelude::*;

    use crate::nes_cartridge::nes::NesCartridge;
    use crate::{nes_cartridge_000::nes::NesCartridge000, nes_memory::nes::NesMemory};
    use crate::nes_ppu::nes::NesPpu;
    use crate::nes_inesfile::nes::INesFile;
    use crate::nes_apu::nes::NesApu;

    pub const TICKS_PER_FRAME: u16 =  59736;

    pub struct NesAudioEvent {
        pub channel_mix: Vec<u16>,
    }

    pub struct NesConsole {
        inframe: RwLock<bool>,
        cpu_runner: M6502Runner,
        addr: AddressBus,
        apu: NesApu,
        ppu: NesPpu,
        cartridge: NesCartridge000,
        cpu_work_ram: MemoryRam,
        left_controller: u8,
        _right_controller: u8,
        _debug: u8,
        pub frame: u32,
    }

    unsafe impl Send for NesConsole {}

    impl NesConsole {

        pub fn new (rom_file: String) -> NesConsole {
            let mut ines_file: INesFile = INesFile::new();
            ines_file.load_file(rom_file);
            let mut cartridge: NesCartridge000 = NesCartridge000::new();
            cartridge.load_prog_rom(ines_file.get_prog_rom_data());
            cartridge.load_char_rom(ines_file.get_char_rom_data());

            let ppu: NesPpu = NesPpu::new();
            let apu: NesApu = NesApu::new();
            //let memory = NesMemory::new (Arc::clone(&cartridge), ppu, Arc::clone(&apu));
            //let mut cpu: M6502<NesMemory> = M6502::new(memory);
            //cpu.reset();
            //cpu.disable_dec();

            let mut temp_instance = Self {
                inframe: RwLock::new(false),
                cpu_runner: M6502Runner::new(M6502Version::Nes),
                addr: AddressBus { address: 0 , write: false, byte: 0, is_accumulator: false, is_abs_y: false },
                apu,
                ppu,
                cartridge,
                cpu_work_ram: MemoryRam::new(String::from("CPU Work RAM"), 0x0800),                left_controller: 0,
                _right_controller: 0,
                _debug: 0,
                frame: 0,
            };

            temp_instance.start_up();   
            //temp_instance.cpu.set_nmi();
        
            temp_instance
        }

        fn start_up(&mut self)
        {
            //self.cpu.reset();
            //self.cpu.memory.ppu.reset();
        }

        fn get_audio(&mut self) -> Vec<u8> {
            self.apu.get_audio_buffer()
        }

        fn ram_execute_tick(&mut self) {
        
            if (0x0000..0x2000).contains(&self.addr.address) {
                let location = self.addr.address % 0x800;  // mirroring
                if location == 0x600 {
                    println!("set up brk");
                }
                if self.addr.write {
                    self.cpu_work_ram.write(location, self.addr.byte);
                    self.addr.write = false;
                } else {
                    self.addr.byte = self.cpu_work_ram.read(location);
                }
            }   

        }

        pub fn run_frame(&mut self) -> (Option<Vec<u8>>, Option<Vec<u8>>) {

            self.frame += 1;
            let mut ticks: i32 = 0;

            if *self.inframe.read().unwrap() {
                return (None, None);
            }
            *self.inframe.write().unwrap() = true;

            while ticks < TICKS_PER_FRAME as i32 {

                if (0x4020..0x6000).contains(&self.addr.address) {
                    eprintln!("unknown address");
                }
                self.cartridge.execute_tick(&mut self.addr);
                self.ram_execute_tick();

                if (ticks % 2) == 0 {
                    self.apu.execute_tick(&mut self.addr);
                }
                if self.apu.is_irq_set() {
                    //self.cpu_runner.set_irq();
                    self.apu.reset_irq();
                }

                if (ticks % 3) == 0 && self.apu.ppu_dma_write == 0{
                    self.cpu_runner.execute_tick(&mut self.addr);
                }

                self.ppu.execute_tick(&mut self.addr, &self.cartridge);

                if self.ppu.is_nmi_set() {
                    self.cpu_runner.set_nmi();
                    self.ppu.reset_nmi();
                }
                self.read_gamepad();
                ticks += 1;
            }
            
            let video = self.ppu.get_screen();
            let audio = self.get_audio();

            *self.inframe.write().unwrap() = false;
            
            (Some(video), Some(audio))
        }
            
        pub fn left_controler_a(&mut self, value: bool) {
            self.left_controller &= 0xfe;

            if value {
                self.left_controller |= 0x01;
            }
        }

        pub fn left_controler_b(&mut self, value: bool) {
            self.left_controller &= 0xfd;

            if value {
                self.left_controller |= 0x02;
            }
        }

        pub fn left_controler_select(&mut self, value: bool) {
            self.left_controller &= 0xfb;

            if value {
                self.left_controller |= 0x04;
            }
        }

        pub fn left_controler_start(&mut self, value: bool) {
            self.left_controller &= 0xf7;

            if value {
                self.left_controller |= 0x08;
            }
        }

        pub fn left_controler_up_down(&mut self, value: i32) {
            self.left_controller &= 0xcf;

            if value < 0 {
                self.left_controller |= 0x10;
            }

            if value > 0 {
                self.left_controller |= 0x20;
            }
        }

        pub fn left_controler_left_right(&mut self, value: i32) {
            self.left_controller &= 0x3f;

            if value < 0 {
                self.left_controller |= 0x40;
            }

            if value > 0 {
                self.left_controller |= 0x80;
            }
        }
            
        fn read_gamepad(&mut self) {

            if self.apu.is_write_flag_set(0x4016) {
                self.apu.set_left_controller(self.left_controller);
            }

            //if self.apu.write().unwrap().is_write_flag_set(0x4017) {
               // self.apu.write().unwrap().set_right_controller(self.right_controller);
            //}
        } 

    }
}
