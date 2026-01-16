
pub mod vcs {
        
    use std::rc::Rc;
    use std::cell::RefCell;

    use emumemory::base_memory::emu_memory::BaseMemory;
    use crate::{vcs_cartridge::vcs::VcsCartridge, vcs_cartridge2k::vcs::VcsCartridge2k, vcs_parameters::vcs::VcsParameters, vcs_riot::vcs::VcsRiot};
    use crate::vcs_tia::vcs::VcsTia;

    pub struct VcsMemory {
        vcs_tia: Rc<RefCell<VcsTia>>,
        vcs_riot: Rc<RefCell<VcsRiot>>,
        vcs_cartridge: VcsCartridge2k
    }

    impl VcsMemory {
        pub fn new(vcs_parameters: &VcsParameters, tia:Rc<RefCell<VcsTia>>, riot: Rc<RefCell<VcsRiot>>) -> VcsMemory {
            Self {
                vcs_tia: tia,
                vcs_riot: riot,
                vcs_cartridge: <VcsCartridge2k as VcsCartridge>::get_cartridge(&vcs_parameters)
            }
        }
    }

    impl emumemory::memory_mapper::emu_memory::MemoryMapper for VcsMemory {
        
        fn cpu_read(&self, mut location: u16) -> u8 {
            let result: u8;

            location = location & 0x1FFF;
            
            if location & 0x1080 == 0 {
                location &= 0x0F;
                location += 0x30;
                result = self.vcs_tia.borrow().read(location);
            }
            else if location & 0x1280 == 0x0080 {
                location &= 0x7F;
                result = self.vcs_riot.borrow_mut().read(location)
            }
            else if location & 0x1280 == 0x0280 {
                location &= 0x1F;
                if location == 0x06 || location == 0x07 {
                    location -= 2;
                }
                result = self.vcs_riot.borrow_mut().read(location);
            }
            else if location >= 0x1000 && location < 0x2000 {
                location -= 0x1000;
                let a_13_set = (0x2000 & location) > 0;
                result = self.vcs_cartridge.read_a13(location, a_13_set)
            }
            else {
                panic!()
            }

            result
        }

        fn cpu_write(&mut self, mut location: u16, byte: u8) {

            if location & 0x1080 == 0 {
                location &= 0xFF;
                if location >= 0x40
                {
                    location -= 0x40;
                }
                self.vcs_tia.borrow_mut().write(location, byte);
            }
            // Working RAM A12=0, A9=0, A7=1 0 **0* 1*** ****
            else if location & 0x1280 == 0x0080 {
                location &= 0x7F;
                self.vcs_riot.borrow_mut().write(location, byte);
            }            
            // PIA I/O Mirrors - A12=0, A9=1, A7=1  0 **1* 1*** ****
            else if location & 0x1280 == 0x0280 {
                // copies of PIA
                location &= 0x7F;
                
                // Timer locations duplicated in each block
                if location == 0x06 || location == 0x07 {
                    location -= 2;
                }
                self.vcs_riot.borrow_mut().write(location, byte);
            }
            // Cartridge ROM
            else if location >= 0x1000 && location < 0x2000 {
                location -= 0x1000;
                self.vcs_cartridge.write(location, byte);
            }              
            else {
                panic!();
            }

        }

    }

}