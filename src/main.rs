use sdl2::audio::{AudioCallback, AudioSpecDesired};
use std::thread;
use std::time::Duration;

struct SimpleCallback;

impl AudioCallback for SimpleCallback {
    type Channel = i16;

    // This function is called whenever the audio subsystem wants more data to play
    fn callback(&mut self, out: &mut [i16]) {
        for value in out.iter_mut() {
            *value = 0;
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

    let audio_device = audio_subsystem
        .open_playback(None, &desired_audio_spec, |_spec| SimpleCallback {})
        .unwrap();

    // This starts the playback.
    audio_device.resume();

    thread::sleep(Duration::from_millis(1_000));
}
