use std::fs::File;
use std::io::BufReader;
use rodio::{OutputStream, Sink, Source};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use crate::{Note, Shawzin};

pub struct AudioManager {
    #[allow(unused)]
    stream: OutputStream,
    sink: Arc<Mutex<Sink>>,
}

impl AudioManager {
    pub fn new() -> Self {
        let (stream, stream_handle) = OutputStream::try_default().expect("Failed to open audio output stream");
        let sink = Sink::try_new(&stream_handle).expect("Failed to create audio sink");

        AudioManager {
            stream,
            sink: Arc::new(Mutex::new(sink)),
        }
    }

    pub fn play_sound_for_note(&self, note: Note, shawzin: &Shawzin) {
        if note == Note::B3 {
            self.play_sound(246.94, 500);
            return;
        }
        let file_path = format!("sounds/{}.ogg", note.to_file_name(shawzin));
        if let Ok(file) = File::open(file_path) {
            let source = rodio::Decoder::new(BufReader::new(file)).expect("Failed to decode audio");
            self.sink.lock().unwrap().append(source);
        } else {
            eprintln!("Failed to open sound file for {:?}", note);
        }
    }

    pub fn play_sound(&self, frequency: f32, duration_millis: u64) {
        let source = sine_wave(frequency, duration_millis);
        self.sink.lock().unwrap().append(source);
    }


    // Add a method to stop the current sound
    pub fn stop_sound(&self) {
        self.sink.lock().unwrap().stop();
    }
}

fn sine_wave(frequency: f32, duration_millis: u64) -> impl Source<Item = f32> + Send + 'static {
    let amplitude = 0.3; // Adjust the amplitude as needed
    rodio::source::SineWave::new(frequency)
        .take_duration(Duration::from_millis(duration_millis))
        .amplify(amplitude)
        .convert_samples::<f32>()
}
