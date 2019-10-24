use sdl2::audio::{AudioCVT, AudioCallback, AudioSpecDesired, AudioSpecWAV};
use std::thread;
use std::time::Duration;

struct SimpleCallback {
    buffer: Vec<u8>,
    position: usize,
}

impl AudioCallback for SimpleCallback {
    type Channel = i16;

    // This function is called whenever the audio subsystem wants more data to play
    fn callback(&mut self, out: &mut [i16]) {
        for value in out.iter_mut() {
            *value = if self.position < self.buffer.len() {
                let sample = i16::from_le_bytes([
                    self.buffer[self.position],
                    self.buffer[self.position + 1],
                ]);
                self.position += 2;
                sample
            } else {
                0
            }
        }
    }
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let audio_subsystem = sdl_context.audio().unwrap();

    let desired_audio_spec = AudioSpecDesired {
        freq: Some(44_100),
        // Mono
        channels: Some(1),
        // Doesn't matter here, use the default value
        samples: None,
    };

    let mut audio_device = audio_subsystem
        .open_playback(None, &desired_audio_spec, |spec| {
            let wav = AudioSpecWAV::load_wav("beep.wav").unwrap();
            let converter = AudioCVT::new(
                wav.format,
                wav.channels,
                wav.freq,
                spec.format,
                spec.channels,
                spec.freq,
            )
            .unwrap();
            let data = converter.convert(wav.buffer().to_vec());

            SimpleCallback {
                buffer: data,
                position: 0,
            }
        })
        .unwrap();

    // This starts the playback.
    audio_device.resume();

    thread::sleep(Duration::from_millis(1_000));
    {
        // The AudioDeviceLockGuard returned by the lock() method gives us safe and exclusive
        // access to the callback structure. This allows us to modify the position or the buffer.
        let mut lock = audio_device.lock();
        // lock dereferences to SimpleCallback so we can access SimpleCallback's attributes
        // directly
        lock.position = 0;
    }
    thread::sleep(Duration::from_millis(1_000));
}
