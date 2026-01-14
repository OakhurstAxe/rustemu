
pub mod vcs {

    pub struct VcsParameters {
        pub console_type: crate::vcs_console_type::vcs::ConsoleType,
        pub cart_data: Vec<u8>,
        pub mapper: String,
        pub has_super_chip: bool
    }

    impl VcsParameters {

        pub fn new(rom: Vec<u8>) -> VcsParameters {
            Self {
                console_type: crate::vcs_console_type::vcs::ConsoleType::NTSC,
                cart_data: rom,
                mapper: String::from("2K"),
                has_super_chip: false
            }
        }

    }

}