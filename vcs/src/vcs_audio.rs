
pub mod vcs {

    use std::sync::{ Arc, RwLock };

    use crate::vcs_tia::vcs::VcsTia;
    use crate::vcs_audio_channel::vcs::{SAMPLES_PER_FRAME, VcsAudioChannel};

    pub struct VcsAudio {
        vcs_tia: Arc<RwLock<VcsTia>>,
        channels: Vec<Arc<RwLock<VcsAudioChannel>>>,
    }

    impl VcsAudio {

        pub fn new(tia: Arc<RwLock<VcsTia>>) -> VcsAudio {

            let mut achannels: Vec<Arc<RwLock<VcsAudioChannel>>> = Vec::with_capacity(2);
            achannels.push(Arc::new(RwLock::new(VcsAudioChannel::new())));
            achannels.push(Arc::new(RwLock::new(VcsAudioChannel::new())));

            Self {
                vcs_tia: tia,
                channels: achannels,
            }
        }

        pub fn execute_tick(&mut self) {
            let mut register1: u8 = self.vcs_tia.read().unwrap().get_audio_v0();
            let mut register2: u8 = self.vcs_tia.read().unwrap().get_audio_f0();
            let mut register3: u8 = self.vcs_tia.read().unwrap().get_audio_c0();
            self.channels[0].write().unwrap().set_channel_settings(register1, register2, register3);

            register1 = self.vcs_tia.read().unwrap().get_audio_v1();
            register2 = self.vcs_tia.read().unwrap().get_audio_f1();
            register3 = self.vcs_tia.read().unwrap().get_audio_c1();
            self.channels[1].write().unwrap().set_channel_settings(register1, register2, register3);
        }

        pub fn get_audio_buffer(&mut self, channel: usize) -> Vec<u16> {
            let buffer: Vec<u16> = self.channels[channel].write().unwrap().callback(SAMPLES_PER_FRAME);

            buffer
        }

    }
}