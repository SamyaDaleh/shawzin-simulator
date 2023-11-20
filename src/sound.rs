use std::fs::File;
use std::io::BufReader;
use rodio::{OutputStream, Sink};
use std::sync::{Arc, Mutex};
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
        let file_path = format!("sounds/{}.ogg", note.to_file_name(shawzin));
        if let Ok(file) = File::open(file_path) {
            let source = rodio::Decoder::new(BufReader::new(file)).expect("Failed to decode audio");
            self.sink.lock().unwrap().append(source);
        } else {
            eprintln!("Failed to open sound file for {:?}", note);
        }
    }
    pub fn stop_sound(&self) {
        self.sink.lock().unwrap().stop();
    }
}
