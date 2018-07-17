use sdl2;
use sdl2::audio::{AudioCallback, AudioDevice, AudioSpecDesired};

pub struct Audio {
    device: AudioDevice<SquareWave>,
}

impl Audio {
    pub fn new(sdl_context: &sdl2::Sdl) -> Self {
        let audio_system = sdl_context.audio().unwrap();

        let desired_spec = AudioSpecDesired {
            freq: Some(44100),
            channels: Some(1),
            samples: None,
        };

        let device = audio_system
            .open_playback(None, &desired_spec, |spec| {
                println!("{:?}", spec);

                SquareWave {
                    phase_inc: 240.0 / spec.freq as f32,
                    phase: 0.0,
                    volume: 0.5,
                }
            })
            .unwrap();

        Audio { device }
    }

    pub fn start_beep(&self) {
        self.device.resume();
    }

    pub fn stop_beep(&self) {
        self.device.pause();
    }
}

struct SquareWave {
    phase_inc: f32,
    phase: f32,
    volume: f32,
}

impl AudioCallback for SquareWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        out.iter_mut().for_each(|v| {
            let next_val = if self.phase < 0.5 { 1.0 } else { -1.0 };
            *v = self.volume * next_val;
            self.phase = (self.phase + self.phase_inc) % 1.0;
        });
    }
}
