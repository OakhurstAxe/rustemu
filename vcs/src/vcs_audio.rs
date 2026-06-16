
pub mod vcs {

    use crate::vcs_tia::vcs::TiaAudio;
    use crate::vcs_audio_channel::vcs::{DATA_SAMPLE_RATE_HZ, VcsAudioChannel};

    pub struct VcsAudio {
        channels: Vec<VcsAudioChannel>,
        frames_per_second: u32,
    }

    impl VcsAudio {

        pub fn new(frames_per_second: u32) -> VcsAudio {

            let mut achannels: Vec<VcsAudioChannel> = Vec::with_capacity(2);
            achannels.push(VcsAudioChannel::new(frames_per_second));
            achannels.push(VcsAudioChannel::new(frames_per_second));

            Self {
                channels: achannels,
                frames_per_second: frames_per_second,
            }
        }

        pub fn execute_frame(&mut self, audio_regs: TiaAudio) {
            let mut register1: u8 = audio_regs.v0;
            let mut register2: u8 = audio_regs.f0;
            let mut register3: u8 = audio_regs.c0;
            self.channels[0].set_channel_settings(register1, register2, register3);

            register1 = audio_regs.v1;
            register2 = audio_regs.f1;
            register3 = audio_regs.c1;
            self.channels[1].set_channel_settings(register1, register2, register3);
        }

        pub fn get_audio_buffer(&mut self, channel: usize) -> Vec<f32> {
            let buffer = self.channels[channel].callback(DATA_SAMPLE_RATE_HZ / self.frames_per_second as usize).clone();

            buffer
        }

    }
}