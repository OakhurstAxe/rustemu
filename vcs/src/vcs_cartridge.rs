
pub mod vcs {

    use std::sync::{ Arc, RwLock };

    use emumemory::base_memory::emu_memory::BaseMemory;
    use crate::vcs_cartridge2k::vcs::VcsCartridge2k;
    use crate::vcs_cartridge4k::vcs::VcsCartridge4k;
    
    pub trait VcsCartridge: BaseMemory {
        
        fn read_a13(&self, location: u16, a13set: bool) -> u8;

        fn read_offset(&self, location: u16, memory_offset: u16) -> u8;
    }

    pub struct VcsCartridgeDetector {
    }

    impl VcsCartridgeDetector {

        pub fn detect_cartridge(vcs_parameters: Arc<RwLock<crate::vcs_parameters::vcs::VcsParameters>>) 
            -> Box<dyn VcsCartridge + Send> {

            let image = vcs_parameters.read().unwrap().cart_data.clone();
            let size = image.len();

            if (size <= 2048) || (size == 4096 && image[0..2048] == image[2048..4096])
            {
                if VcsCartridgeDetector::is_probably_cv(&image) {
                    // Bankswitch::Type::_CV
                }
                else {
                    // Bankswitch::Type::_2K
                    let mut cart2k: VcsCartridge2k = VcsCartridge2k {
                        memory: [].to_vec(),
                        name: String::from("2k Cartridge"),
                        has_super_chip: false
                    };
                    cart2k.load_data(&vcs_parameters.read().unwrap().cart_data);

                    return Box::new(cart2k);
                }
            }
            else if size == 4096
            {
                if VcsCartridgeDetector::is_probably_cv(&image) {
//                    type = Bankswitch::Type::_CV;
                    let x = 10;
                }
                else if VcsCartridgeDetector::is_probably_4ksc(&image) {
//                    type = Bankswitch::Type::_4KSC;
                    let x = 10;
                }
                else if VcsCartridgeDetector::is_probably_fc(&image) {
//                    type = Bankswitch::Type::_FC;
                    let x = 10;
                }
                else if VcsCartridgeDetector::is_probably_gl(&image) {
//                    type = Bankswitch::Type::_GL;
                    let x = 10;
                }
                else {
                    let mut cart4k: VcsCartridge4k = VcsCartridge4k {
                        memory: [].to_vec(),
                        name: String::from("4k Cartridge"),
                        has_super_chip: false
                    };
                    cart4k.load_data(&vcs_parameters.read().unwrap().cart_data);

                    return Box::new(cart4k);
                }
            }

            let mut cart2k: crate::vcs_cartridge2k::vcs::VcsCartridge2k;

            cart2k = crate::vcs_cartridge2k::vcs::VcsCartridge2k {
                memory: [].to_vec(),
                name: String::from("2k Cartridge"),
                has_super_chip: false
            };

            cart2k.load_data(&vcs_parameters.read().unwrap().cart_data);

            Box::new(cart2k)
        }

        fn is_probably_cv(image: &Vec<u8>) -> bool {
            // CV RAM access occurs at addresses $f3ff and $f400
            // These signatures are attributed to the MESS project
            let magicard_signature: Vec<u8> = vec![ 0x9D, 0xFF, 0xF3 ];  // STA $F3FF,X  MagiCard
            let videolife_signature: Vec<u8> = vec![ 0x99, 0x00, 0xF4 ];  // STA $F3FF,X  MagiCard
            
            if VcsCartridgeDetector::search_for_bytes(&image, magicard_signature, 3) {
                return true;
            }
            else {
                return VcsCartridgeDetector::search_for_bytes(&image, videolife_signature, 3);
            }
        }

        fn is_probably_gl(image: &Vec<u8>) -> bool {
            let constexpr: Vec<u8> = vec![ 0xad, 0xb8, 0x0c ]; // LDA $0CB8

            VcsCartridgeDetector::search_for_bytes(&image, constexpr, 3)
        }

        fn is_probably_4ksc(image: &Vec<u8>) -> bool{
            // We check if the first 256 bytes are identical *and* if there's
            // an "SC" signature for one of our larger SC types at 1FFA.
            let first: u8 = image[0];

            for i in 1..256 {
                if image[i] != first {
                    return false;
                }
            }

            let size = image.len();
            image[size-6] == b'S' && image[size-5] == b'C'
        }

        fn is_probably_fc(image: &Vec<u8>) -> bool {
            // FC bankswitching uses consecutive writes to 3 hotspots
            let constexpr1: Vec<u8> = vec![ 0x8d, 0xf8, 0x1f, 0x4a, 0x4a, 0x8d ]; // STA $1FF8, LSR, LSR, STA... Power Play Arcade Menus, 3-D Ghost Attack
            let constexpr2: Vec<u8> = vec![ 0x8d, 0xf8, 0xff, 0x8d, 0xfc, 0xff ]; // STA $FFF8, STA $FFFC        Surf's Up (4K)
            let constexpr3: Vec<u8> = vec![ 0x8c, 0xf9, 0xff, 0xad, 0xfc, 0xff ];  // STY $FFF9, LDA $FFFC        3-D Havoc
            
            if VcsCartridgeDetector::search_for_bytes(&image, constexpr1, 6) ||
                VcsCartridgeDetector::search_for_bytes(&image, constexpr2, 6) ||
                VcsCartridgeDetector::search_for_bytes(&image, constexpr3, 6) {
                    return true;
            }

            false
        }

        fn search_for_bytes(image: &Vec<u8>, signature: Vec<u8>, minhits: u32) -> bool {

            let mut count: u32 = 0;

            for mut i in 0..(image.len() - signature.len()) {

                let j: u32 = 0;

                for j in 0..signature.len() {

                    if image[i + j] != signature[j] {
                        break;
                    }
                }

                if j == signature.len() as u32 {

                    count += 1;

                    if count == minhits {
                        break;
                    }
                    
                    i += signature.len();  // skip past this signature 'window' entirely
                }
            }

            count == minhits
        
        }

    }

}