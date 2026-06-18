
pub mod vcs {

    use crate::vcs_tia::vcs::TiaAudio;
    use crate::vcs_audio_channel::vcs::{DATA_SAMPLE_RATE_HZ, VcsAudioChannel};

    pub struct VcsAudio {
        channels: Vec<VcsAudioChannel>,
        frames_per_second: u32,
    }

    impl VcsAudio {

        pub fn new(frames_per_second: u32) -> VcsAudio {

            let channels: Vec<VcsAudioChannel> = vec![
            VcsAudioChannel::new(frames_per_second),
            VcsAudioChannel::new(frames_per_second)];

            Self {
                channels,
                frames_per_second,
            }
        }

        pub fn samples_per_frame(&self) -> usize {
            DATA_SAMPLE_RATE_HZ / self.frames_per_second as usize
        }

        pub fn execute_frame(&mut self, audio_regs: TiaAudio) {
            self.channels[0].set_channel_settings(audio_regs.v0, audio_regs.f0, audio_regs.c0);
            self.channels[1].set_channel_settings(audio_regs.v1, audio_regs.f1, audio_regs.c1);
        }

        pub fn get_audio_buffer(&mut self, channel: usize) -> Vec<f32> {
            let samples_per_frame = self.samples_per_frame();
            self.channels[channel].get_buffer(samples_per_frame).clone()
        }

    }
}