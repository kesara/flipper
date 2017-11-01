pub mod waves {
    extern crate sdl2;

    use sdl2::audio::{AudioCallback, AudioSpecDesired};

    pub struct SquareWave {
        pub phase_inc: f32,
        pub phase: f32,
        pub volume: f32,
    }

    impl AudioCallback for SquareWave {
        type Channel = f32;

        fn callback(&mut self, out: &mut [f32]) {
            // generate a square wave
            for x in out.iter_mut() {
                *x = if self.phase < 0.5 {
                    self.volume
                } else {
                    -self.volume
                };
                self.phase = (self.phase + self.phase_inc) % 1.0;
            }
        }
    }

    pub fn get_audio_spec(freq: i32) -> AudioSpecDesired {
        return AudioSpecDesired {
            freq: Some(freq),
            channels: Some(1), // mono
            samples: None, // default sample size
        };
    }
}
