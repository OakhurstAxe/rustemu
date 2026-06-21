
pub mod vcs {

    pub struct VcsParameters {
        pub console_type: crate::vcs_console_type::vcs::ConsoleType,
    }

    impl VcsParameters {

        pub fn new() -> VcsParameters {
            Self {
                console_type: crate::vcs_console_type::vcs::ConsoleType::NTSC,
            }
        }

    }

}