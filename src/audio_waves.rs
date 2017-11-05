pub mod waves {
    extern crate rand;
    extern crate sdl2;

    use rand::{thread_rng, Rng};
    use sdl2::audio::{AudioCallback, AudioSpecDesired};

    pub struct SquareWave {
        pub phase_inc: f32,
        pub phase: f32,
        pub volume: f32,
    }

    pub struct WhiteNoise {
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

    impl AudioCallback for WhiteNoise {
        type Channel = f32;

        fn callback(&mut self, out: &mut [f32]) {
            // generate white noice
            let mut rng = thread_rng();
            for x in out.iter_mut() {
                let delta = rng.next_f32();
                *x = if self.volume < delta {
                    self.volume / delta * rng.next_f32()
                } else {
                    self.volume - delta
                }
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
