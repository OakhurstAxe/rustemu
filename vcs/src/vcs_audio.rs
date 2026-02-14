
pub mod vcs {

    use std::sync::{ Arc, RwLock };

    use crate::vcs_tia::vcs::VcsTia;
    use crate::vcs_audio_channel::vcs::{DATA_SAMPLE_RATE_HZ, VcsAudioChannel};

    pub struct VcsAudio {
        vcs_tia: Arc<RwLock<VcsTia>>,
        channels: Vec<VcsAudioChannel>,
        frames_per_second: u32,
    }

    impl VcsAudio {

        pub fn new(tia: Arc<RwLock<VcsTia>>, frames_per_second: u32) -> VcsAudio {

            let mut achannels: Vec<VcsAudioChannel> = Vec::with_capacity(2);
            achannels.push(VcsAudioChannel::new(frames_per_second));
            achannels.push(VcsAudioChannel::new(frames_per_second));

            Self {
                vcs_tia: tia,
                channels: achannels,
                frames_per_second: frames_per_second,
            }
        }

        pub fn execute_tick(&mut self) {
            let mut register1: u8 = self.vcs_tia.read().unwrap().get_audio_v0();
            let mut register2: u8 = self.vcs_tia.read().unwrap().get_audio_f0();
            let mut register3: u8 = self.vcs_tia.read().unwrap().get_audio_c0();
            self.channels[0].set_channel_settings(register1, register2, register3);

            register1 = self.vcs_tia.read().unwrap().get_audio_v1();
            register2 = self.vcs_tia.read().unwrap().get_audio_f1();
            register3 = self.vcs_tia.read().unwrap().get_audio_c1();
            self.channels[1].set_channel_settings(register1, register2, register3);
        }

        pub fn get_audio_buffer(&mut self, channel: usize) -> Vec<u16> {
            let buffer = self.channels[channel].callback(DATA_SAMPLE_RATE_HZ / self.frames_per_second as usize).clone();

            buffer
        }

    }
}