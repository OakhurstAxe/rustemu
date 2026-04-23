
pub mod vcs {

    use std::sync::{ Arc, RwLock };

    use crate::vcs_cartridge::vcs::VcsCartridge;
    use crate::vcs_cartridge2k::vcs::VcsCartridge2k;
    use crate::vcs_cartridge4k::vcs::VcsCartridge4k;
    use crate::vcs_cartridgef8::vcs::VcsCartridgeF8;
    use emumemory::base_memory::emu_memory::BaseMemory;

    pub struct VcsCartridgeDetector {
    }

    impl VcsCartridgeDetector {

        pub fn detect_cartridge(vcs_parameters: Arc<RwLock<crate::vcs_parameters::vcs::VcsParameters>>) 
            -> Box<dyn VcsCartridge + Send + Sync> {

            let image = vcs_parameters.read().unwrap().cart_data.clone();
            let size = image.len();

            if (size <= 2048) || (size == 4096 && image[0..2048] == image[2048..4096])
            {
                if VcsCartridgeDetector::is_probably_cv(&image) {
                    panic!("No CV cartridge mapper");
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
                    panic!("No CV cartridge mapper");
                }
                else if VcsCartridgeDetector::is_probably_4ksc(&image) {
                    panic!("No 4k SC cartridge mapper");
                }
                else if VcsCartridgeDetector::is_probably_fc(&image) {
                    panic!("No FC cartridge mapper");
                }
                else if VcsCartridgeDetector::is_probably_gl(&image) {
                    panic!("No GL cartridge mapper");
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
            else if size == 8192 {
                // First check for *potential* F8
                let constexpr1: Vec<u8> = vec![ 0x8D, 0xF9, 0x1F ]; // STA $1FF9
                let constexpr2: Vec<u8> = vec![ 0x8D, 0xF9, 0xFF ]; // STA $FFF9

                let f8: bool = VcsCartridgeDetector::search_for_bytes(&image, constexpr1, 1) ||
                    VcsCartridgeDetector::search_for_bytes(&image, constexpr2, 1);

                if VcsCartridgeDetector::is_probably_sc(&image) {
                    panic!("No F8SC cartridge mapper");
                }
                else if image[0..4096] == image[4096..8192] {
                    let mut cart4k: VcsCartridge4k = VcsCartridge4k {
                        memory: [].to_vec(),
                        name: String::from("4k Cartridge"),
                        has_super_chip: false
                    };
                    cart4k.load_data(&vcs_parameters.read().unwrap().cart_data);

                    return Box::new(cart4k);                    
                }
                else if VcsCartridgeDetector::is_probably_e0(&image) {
                    panic!("No E0 cartridge mapper");
                }
                else if VcsCartridgeDetector::is_probably_3ex(&image) {
                    panic!("No 3EX cartridge mapper");
                }
                else if VcsCartridgeDetector::is_probably_3e(&image) {
                    panic!("No 3E cartridge mapper");
                }
                else if VcsCartridgeDetector::is_probably_3f(&image) {
                    panic!("No 3F cartridge mapper");
                }
                else if VcsCartridgeDetector::is_probably_ua(&image) {
                    panic!("No UA cartridge mapper");
                }
                else if VcsCartridgeDetector::is_probably_0fa0(&image) {
                    panic!("No 0FA0 cartridge mapper");
                }
                else if VcsCartridgeDetector::is_probably_fe(&image) && !f8 {
                    panic!("No FE cartridge mapper");
                }
                else if VcsCartridgeDetector::is_probably_0840(&image) {
                    panic!("No 0840 cartridge mapper");
                }
                else if VcsCartridgeDetector::is_probably_e78k(&image) {
                    panic!("No E7 cartridge mapper");
                }
                else if VcsCartridgeDetector::is_probably_wd(&image) {
                    panic!("No WD cartridge mapper");
                }
                else if VcsCartridgeDetector::is_probably_fc(&image) {
                    panic!("No FC cartridge mapper");
                }
                else if VcsCartridgeDetector::is_probably_03e0(&image) {
                    panic!("No 03E0 cartridge mapper");
                }
                else {
                    let mut cartf8: VcsCartridgeF8 = VcsCartridgeF8 {
                        memory: [].to_vec(),
                        name: String::from("F8 Cartridge"),
                        has_super_chip: false,
                        memory_offset: 0,
                    };
                    cartf8.load_data(&vcs_parameters.read().unwrap().cart_data);

                    return Box::new(cartf8);  
                }
            }

            panic!("Unknown cartridge mapper");

        }

        fn search_for_bytes(image: &Vec<u8>, signature: Vec<u8>, minhits: u32) -> bool {

            let mut count: u32 = 0;

            for i in 0..(image.len() - signature.len()) {

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
                }
            }

            count >= minhits
        
        }

        fn is_probably_sc(image: &Vec<u8>) -> bool {
            // We assume a Superchip cart repeats the first 128 bytes for the second
            // 128 bytes in the RAM area, which is the first 256 bytes of each 4K bank
            let mut position = 0;
            while position < image.len() {
                if image[position..position+128] != image[position+128..position+256] {
                    return false;
                }

                position += 4096;
            }
            true
        }

        fn _is_probably_arm(image: &Vec<u8>) -> bool {
            // ARM code contains the following 'loader' patterns in the first 1K
            // Thanks to Thomas Jentzsch of AtariAge for this advice
            let constexpr1: Vec<u8> = vec![ 0xA0, 0xC1, 0x1F, 0xE0 ]; 
            let constexpr2: Vec<u8> = vec![ 0x00, 0x80, 0x02, 0xE0 ]; 

            if VcsCartridgeDetector::search_for_bytes(image, constexpr1, 1) {
                return true;
            }
            else {
                return VcsCartridgeDetector::search_for_bytes(image, constexpr2, 1)
            }
        }

        fn is_probably_03e0(image: &Vec<u8>) -> bool {
            // 03E0 cart bankswitching for Brazilian Parker Bros ROMs, switches segment
            // 0 into bank 0 by accessing address 0x3E0 using 'LDA $3E0' or 'ORA $3E0'.
            let constexpr1: Vec<u8> = vec![ 0x0D, 0xE0, 0x03, 0x0D ]; // ORA $3E0, ORA (Popeye)
            let constexpr2: Vec<u8> = vec![ 0xAD, 0xE0, 0x03, 0xAD ]; // LDA $3E0, ORA (Montezuma's Revenge)

            if VcsCartridgeDetector::search_for_bytes(image, constexpr1, 1) ||
                VcsCartridgeDetector::search_for_bytes(image, constexpr2, 1) {
                return true;
            }

            false
        }

        fn is_probably_0840(image: &Vec<u8>) -> bool {
            // 0840 cart bankswitching is triggered by accessing addresses 0x0800
            // or 0x0840 at least twice
            let constexpr1: Vec<u8> = vec![ 0xAD, 0x00, 0x08 ]; // LDA $0800
            let constexpr2: Vec<u8> = vec![ 0xAD, 0x40, 0x08 ]; // LDA $0840
            let constexpr3: Vec<u8> = vec![ 0x2C, 0x00, 0x08 ]; // BIT $0800

            if VcsCartridgeDetector::search_for_bytes(image, constexpr1, 2) ||
                VcsCartridgeDetector::search_for_bytes(image, constexpr2, 2) ||
                VcsCartridgeDetector::search_for_bytes(image, constexpr3, 2) {
                return true;
            }

            let signature1: Vec<u8> = vec![ 0x0C, 0x00, 0x08, 0x4C ]; // NOP $0800; JMP ...
            let signature2: Vec<u8> = vec![ 0x0C, 0xFF, 0x0F, 0x4C ]; // NOP $0FFF; JMP ...

            if VcsCartridgeDetector::search_for_bytes(image, signature1, 2) ||
                VcsCartridgeDetector::search_for_bytes(image, signature2, 2) {
                return true;
            }

            false
        }

        fn is_probably_0fa0(image: &Vec<u8>) -> bool {
            // Other Brazilian (Fotomania) ROM's bankswitching switches to bank 1 by
            // accessing address 0xFC0 using 'BIT $FC0', 'BIT $FC0' or 'STA $FC0'
            // Also a game (Motocross) using 'BIT $EFC0' has been found
            let constexpr1: Vec<u8> = vec![ 0x2C, 0xC0, 0x0F ]; // BIT $FC0  (H.E.R.O., Kung-Fu Master)
            let constexpr2: Vec<u8> = vec![ 0x8D, 0xC0, 0x0F ]; // TA $FC0  (Pole Position, Subterranea)
            let constexpr3: Vec<u8> = vec![ 0xAD, 0xC0, 0x0F ]; // LDA $FC0  (Front Line, Zaxxon)
            let constexpr4: Vec<u8> = vec![ 0x2C, 0xC0, 0xEF ]; // BIT $EFC0 (Motocross)

            if VcsCartridgeDetector::search_for_bytes(image, constexpr1, 1) ||
                VcsCartridgeDetector::search_for_bytes(image, constexpr2, 1) ||
                VcsCartridgeDetector::search_for_bytes(image, constexpr3, 1) ||
                VcsCartridgeDetector::search_for_bytes(image, constexpr4, 1) {
                return true;
            }

            false
        }

        fn is_probably_3e(image: &Vec<u8>) -> bool {
            // 3E cart RAM bankswitching is triggered by storing the bank number
            // in address 3E using 'STA $3E', ROM bankswitching is triggered by
            // storing the bank number in address 3F using 'STA $3F'.
            // We expect the latter will be present at least 2 times, since there
            // are at least two banks

            let constexpr1: Vec<u8> = vec![ 0x85, 0x3E ]; // STA $3E
            let constexpr2: Vec<u8> = vec![ 0x85, 0x3F ]; // STA $3F

            VcsCartridgeDetector::search_for_bytes(image, constexpr1, 1)
                && VcsCartridgeDetector::search_for_bytes(image, constexpr2, 2)
        }

        fn is_probably_3ex(image: &Vec<u8>) -> bool {
            // 3EX cart have at least 2 occurrences of the string "3EX"
            let constexpr1: Vec<u8> = vec![ '3' as u8, 'E' as u8, 'X' as u8 ]; 
            VcsCartridgeDetector::search_for_bytes(image, constexpr1, 2)
        }

        fn is_probably_3f(image: &Vec<u8>) -> bool {
            // 3F cart bankswitching is triggered by storing the bank number
            // in address 3F using 'STA $3F'
            // We expect it will be present at least 2 times, since there are
            // at least two banks
            let constexpr1: Vec<u8> = vec![ 0x85, 0x3F ]; // STA $3F
            VcsCartridgeDetector::search_for_bytes(image, constexpr1, 2)
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

        fn is_probably_cv(image: &Vec<u8>) -> bool {
            // CV RAM access occurs at addresses $f3ff and $f400
            // These signatures are attributed to the MESS project
            let magicard_signature: Vec<u8> = vec![ 0x9D, 0xFF, 0xF3 ];  // STA $F3FF,X  MagiCard
            let videolife_signature: Vec<u8> = vec![ 0x99, 0x00, 0xF4 ];  // STA $F3FF,X  MagiCard
            
            if VcsCartridgeDetector::search_for_bytes(&image, magicard_signature, 1) {
                return true;
            }
            else {
                return VcsCartridgeDetector::search_for_bytes(&image, videolife_signature, 1);
            }
        }

        fn is_probably_e0(image: &Vec<u8>) -> bool {
            // E0 cart bankswitching is triggered by accessing addresses
            // $FE0 to $FF9 using absolute non-indexed addressing
            // To eliminate false positives (and speed up processing), we
            // search for only certain known signatures
            // Thanks to "stella@casperkitty.com" for this advice
            // These signatures are attributed to the MESS project
            let constexpr1: Vec<u8> = vec![ 0x8D, 0xE0, 0x1F ]; // STA $1FE0
            let constexpr2: Vec<u8> = vec![ 0x8D, 0xE0, 0x5F ]; // STA $5FE0
            let constexpr3: Vec<u8> = vec![ 0x8D, 0xE9, 0xFF ]; // STA $FFE9
            let constexpr4: Vec<u8> = vec![ 0x0C, 0xE0, 0x1F ]; // NOP $1FE0
            let constexpr5: Vec<u8> = vec![ 0xAD, 0xE0, 0x1F ]; // LDA $1FE0
            let constexpr6: Vec<u8> = vec![ 0xAD, 0xE9, 0xFF ]; // LDA $FFE9
            let constexpr7: Vec<u8> = vec![ 0xAD, 0xED, 0xFF ]; // LDA $FFED
            let constexpr8: Vec<u8> = vec![ 0xAD, 0xF3, 0xBF ]; // LDA $BFF3

            if VcsCartridgeDetector::search_for_bytes(image, constexpr1, 1) ||
                VcsCartridgeDetector::search_for_bytes(image, constexpr2, 1) ||
                VcsCartridgeDetector::search_for_bytes(image, constexpr3, 1) ||
                VcsCartridgeDetector::search_for_bytes(image, constexpr4, 1) ||
                VcsCartridgeDetector::search_for_bytes(image, constexpr5, 1) ||
                VcsCartridgeDetector::search_for_bytes(image, constexpr6, 1) ||
                VcsCartridgeDetector::search_for_bytes(image, constexpr7, 1) ||
                VcsCartridgeDetector::search_for_bytes(image, constexpr8, 1) {
                return true;
            }

            false
        }

        fn is_probably_e78k(image: &Vec<u8>) -> bool {
            // E78K cart bankswitching is triggered by accessing addresses
            // $FE4 to $FE6 using absolute non-indexed addressing
            // To eliminate false positives (and speed up processing), we
            // search for only certain known signatures
            let constexpr1: Vec<u8> = vec![ 0xAD, 0xE4, 0xFF ]; // LDA $FFE4
            let constexpr2: Vec<u8> = vec![ 0xAD, 0xE5, 0xFF ]; // LDA $FFE5
            let constexpr3: Vec<u8> = vec![ 0xAD, 0xE6, 0xFF ]; // LDA $FFE6

            if VcsCartridgeDetector::search_for_bytes(image, constexpr1, 1) ||
                VcsCartridgeDetector::search_for_bytes(image, constexpr2, 1) ||
                VcsCartridgeDetector::search_for_bytes(image, constexpr3, 1) {
                return true;
            }

            false
        }

        fn is_probably_fc(image: &Vec<u8>) -> bool {
            // FC bankswitching uses consecutive writes to 3 hotspots
            let constexpr1: Vec<u8> = vec![ 0x8d, 0xf8, 0x1f, 0x4a, 0x4a, 0x8d ]; // STA $1FF8, LSR, LSR, STA... Power Play Arcade Menus, 3-D Ghost Attack
            let constexpr2: Vec<u8> = vec![ 0x8d, 0xf8, 0xff, 0x8d, 0xfc, 0xff ]; // STA $FFF8, STA $FFFC        Surf's Up (4K)
            let constexpr3: Vec<u8> = vec![ 0x8c, 0xf9, 0xff, 0xad, 0xfc, 0xff ];  // STY $FFF9, LDA $FFFC        3-D Havoc
            
            if VcsCartridgeDetector::search_for_bytes(&image, constexpr1, 1) ||
                VcsCartridgeDetector::search_for_bytes(&image, constexpr2, 1) ||
                VcsCartridgeDetector::search_for_bytes(&image, constexpr3, 1) {
                    return true;
            }

            false
        }

        fn is_probably_fe(image: &Vec<u8>) -> bool {
            // FE bankswitching is very weird, but always seems to include a
            // 'JSR $xxxx'
            // These signatures are (mostly) attributed to the MESS project
            let constexpr1: Vec<u8> = vec![ 0x20, 0x00, 0xD0, 0xC6, 0xC5 ]; // JSR $D000; DEC $C5  Decathlon
            let constexpr2: Vec<u8> = vec![ 0x20, 0xC3, 0xF8, 0xA5, 0x82 ]; // JSR $F8C3; LDA $82  Robot Tank
            let constexpr3: Vec<u8> = vec![ 0xD0, 0xFB, 0x20, 0x73, 0xFE ]; // BNE $FB; JSR $FE73  Space Shuttle (NTSC/PAL)
            let constexpr4: Vec<u8> = vec![ 0xD0, 0xFB, 0x20, 0x68, 0xFE ]; // BNE $FB; JSR $FE73  Space Shuttle (SECAM)
            let constexpr5: Vec<u8> = vec![ 0x20, 0x00, 0xF0, 0x84, 0xD6 ]; // JSR $F000; $84, $D6 Thwocker

            if VcsCartridgeDetector::search_for_bytes(image, constexpr1, 1) ||
                VcsCartridgeDetector::search_for_bytes(image, constexpr2, 1) ||
                VcsCartridgeDetector::search_for_bytes(image, constexpr3, 1) ||
                VcsCartridgeDetector::search_for_bytes(image, constexpr4, 1) ||
                VcsCartridgeDetector::search_for_bytes(image, constexpr5, 1) {
                return true;
            }

            false
        }

        fn is_probably_gl(image: &Vec<u8>) -> bool {
            let constexpr: Vec<u8> = vec![ 0xad, 0xb8, 0x0c ]; // LDA $0CB8

            VcsCartridgeDetector::search_for_bytes(&image, constexpr, 1)
        }

        fn is_probably_ua(image: &Vec<u8>) -> bool {
            // UA cart bankswitching switches to bank 1 by accessing address 0x240
            // using 'STA $240' or 'LDA $240'.
            // Brazilian (Digivison) cart bankswitching switches to bank 1 by accessing address 0x2C0
            // using 'BIT $2C0', 'STA $2C0' or 'LDA $2C0'
            let constexpr1: Vec<u8> = vec![ 0x8D, 0x40, 0x02 ]; // STA $240 (Funky Fish, Pleiades)
            let constexpr2: Vec<u8> = vec![ 0xAD, 0x40, 0x02 ]; // LDA $240 (???)
            let constexpr3: Vec<u8> = vec![ 0xBD, 0x1F, 0x02 ]; // LDA $21F,X (Gingerbread Man)
            let constexpr4: Vec<u8> = vec![ 0x2C, 0xC0, 0x02 ]; // BIT $2C0 (Time Pilot)
            let constexpr5: Vec<u8> = vec![ 0x8D, 0xC0, 0x02 ]; // STA $2C0 (Fathom, Vanguard)
            let constexpr6: Vec<u8> = vec![ 0xAD, 0xC0, 0x02 ]; // LDA $2C0 (Mickey)

            if VcsCartridgeDetector::search_for_bytes(image, constexpr1, 1) ||
                VcsCartridgeDetector::search_for_bytes(image, constexpr2, 1) ||
                VcsCartridgeDetector::search_for_bytes(image, constexpr3, 1) ||
                VcsCartridgeDetector::search_for_bytes(image, constexpr4, 1) ||
                VcsCartridgeDetector::search_for_bytes(image, constexpr5, 1) ||
                VcsCartridgeDetector::search_for_bytes(image, constexpr6, 1) {
                return true;
            }

            return false
        }

        fn is_probably_wd(image: &Vec<u8>) -> bool {
            // WD cart bankswitching switches banks by accessing address 0x30..0x3f
            let constexpr1: Vec<u8> = vec![ 0xA5, 0x39, 0x4C ]; // LDA $39, JMP

            VcsCartridgeDetector::search_for_bytes(image, constexpr1, 1)
        }

    }
}
