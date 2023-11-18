mod sound;
mod note;

use ggez::{Context, GameError, GameResult, graphics};
use ggez::graphics::{Text, DrawParam};
use ggez::event::{self, EventHandler, KeyCode, KeyMods};
use crate::sound::AudioManager;
use crate::note::Note;

#[derive(Debug)]
enum Scale {
    PentatonicMinor,
    PentatonicMajor,
    Chromatic,
    Hexatonic,
    Major,
    Minor,
    Hirajoshi,
    Phrygian,
    Yo
}

#[derive(Debug)]
pub enum Shawzin {
    Mimica,
    Nelumbo,
    Corbu,
    Tiamat,
    AristeiPrime,
    Narmer,
    KiraS,
    VoidSSong,
    Courtly,
    Lonesome
}

struct MyGame {
    left_arrow_pressed: bool,
    right_arrow_pressed: bool,
    down_arrow_pressed: bool,
    space_pressed: bool,
    scale: Scale,
    audio_manager_1: AudioManager,
    audio_manager_2: AudioManager,
    audio_manager_3: AudioManager,
    audio_manager_1l: AudioManager,
    audio_manager_2l: AudioManager,
    audio_manager_3l: AudioManager,
    audio_manager_1d: AudioManager,
    audio_manager_2d: AudioManager,
    audio_manager_3d: AudioManager,
    audio_manager_1r: AudioManager,
    audio_manager_2r: AudioManager,
    audio_manager_3r: AudioManager,
    shawzin: Shawzin,
}

impl MyGame {
    fn get_note_from_input(&self, scale: &Scale, combined_key: &str) -> Option<Note> {
        match scale {
            Scale::PentatonicMinor => match combined_key {
                "1_" => Some(Note::C4),
                "2_" => Some(Note::DSharp4),
                "3_" => Some(Note::F4),
                "1<" => Some(Note::G4),
                "2<" => Some(Note::ASharp4),
                "3<" => Some(Note::C5),
                "1v" => Some(Note::DSharp5),
                "2v" => Some(Note::F5),
                "3v" => Some(Note::G5),
                "1>" => Some(Note::ASharp5),
                "2>" => Some(Note::C6),
                "3>" => Some(Note::DSharp6),
                _ => None,
            },
            Scale::PentatonicMajor => match combined_key {
                "1_" => Some(Note::C4),
                "2_" => Some(Note::D4),
                "3_" => Some(Note::E4),
                "1<" => Some(Note::G4),
                "2<" => Some(Note::A4),
                "3<" => Some(Note::C5),
                "1v" => Some(Note::D5),
                "2v" => Some(Note::E5),
                "3v" => Some(Note::G5),
                "1>" => Some(Note::A5),
                "2>" => Some(Note::C6),
                "3>" => Some(Note::D6),
                _ => None,
            },
            Scale::Chromatic => match combined_key {
                "1_" => Some(Note::C4),
                "2_" => Some(Note::CSharp4),
                "3_" => Some(Note::D4),
                "1<" => Some(Note::DSharp4),
                "2<" => Some(Note::E4),
                "3<" => Some(Note::F4),
                "1v" => Some(Note::FSharp4),
                "2v" => Some(Note::G4),
                "3v" => Some(Note::GSharp4),
                "1>" => Some(Note::A4),
                "2>" => Some(Note::ASharp4),
                "3>" => Some(Note::B4),
                _ => None,
            },
            Scale::Hexatonic => match combined_key {
                "1_" => Some(Note::C4),
                "2_" => Some(Note::DSharp4),
                "3_" => Some(Note::F4),
                "1<" => Some(Note::FSharp4),
                "2<" => Some(Note::G4),
                "3<" => Some(Note::ASharp4),
                "1v" => Some(Note::C5),
                "2v" => Some(Note::DSharp5),
                "3v" => Some(Note::F5),
                "1>" => Some(Note::FSharp5),
                "2>" => Some(Note::G5),
                "3>" => Some(Note::ASharp5),
                _ => None,
            },
            Scale::Major => match combined_key {
                "1_" => Some(Note::C4),
                "2_" => Some(Note::D4),
                "3_" => Some(Note::E4),
                "1<" => Some(Note::F4),
                "2<" => Some(Note::G4),
                "3<" => Some(Note::A4),
                "1v" => Some(Note::B4),
                "2v" => Some(Note::C5),
                "3v" => Some(Note::D5),
                "1>" => Some(Note::E5),
                "2>" => Some(Note::F5),
                "3>" => Some(Note::G5),
                _ => None,
            },
            Scale::Minor => match combined_key {
                "1_" => Some(Note::C4),
                "2_" => Some(Note::D4),
                "3_" => Some(Note::DSharp4),
                "1<" => Some(Note::F4),
                "2<" => Some(Note::G4),
                "3<" => Some(Note::GSharp4),
                "1v" => Some(Note::ASharp4),
                "2v" => Some(Note::C5),
                "3v" => Some(Note::D5),
                "1>" => Some(Note::DSharp5),
                "2>" => Some(Note::F5),
                "3>" => Some(Note::G5),
                _ => None,
            },
            Scale::Hirajoshi => match combined_key {
                "1_" => Some(Note::C4),
                "2_" => Some(Note::CSharp4),
                "3_" => Some(Note::F4),
                "1<" => Some(Note::FSharp4),
                "2<" => Some(Note::ASharp4),
                "3<" => Some(Note::C5),
                "1v" => Some(Note::CSharp5),
                "2v" => Some(Note::F5),
                "3v" => Some(Note::FSharp5),
                "1>" => Some(Note::A5),
                "2>" => Some(Note::C6),
                "3>" => Some(Note::CSharp6),
                _ => None,
            },
            Scale::Phrygian => match combined_key {
                "1_" => Some(Note::C4),
                "2_" => Some(Note::CSharp4),
                "3_" => Some(Note::E4),
                "1<" => Some(Note::F4),
                "2<" => Some(Note::G4),
                "3<" => Some(Note::GSharp4),
                "1v" => Some(Note::ASharp4),
                "2v" => Some(Note::C5),
                "3v" => Some(Note::CSharp5),
                "1>" => Some(Note::E5),
                "2>" => Some(Note::F5),
                "3>" => Some(Note::G5),
                _ => None,
            },
            Scale::Yo => match combined_key {
                "1_" => Some(Note::CSharp4),
                "2_" => Some(Note::DSharp4),
                "3_" => Some(Note::FSharp4),
                "1<" => Some(Note::GSharp4),
                "2<" => Some(Note::ASharp4),
                "3<" => Some(Note::CSharp5),
                "1v" => Some(Note::DSharp5),
                "2v" => Some(Note::FSharp5),
                "3v" => Some(Note::GSharp5),
                "1>" => Some(Note::ASharp5),
                "2>" => Some(Note::CSharp6),
                "3>" => Some(Note::DSharp6),
                _ => None,
            },
        }
    }

    fn play_shawzin_sound(&self, keycode: KeyCode) {
        let base_sound = match keycode {
            KeyCode::Key1 => "1",
            KeyCode::Key2 => "2",
            KeyCode::Key3 => "3",
            _ => return,
        };

        if self.left_arrow_pressed {
            let modifier = "<";

            let mut combined_key: String = "".to_owned();

            combined_key.push(base_sound.parse().unwrap());
            combined_key.push(modifier.parse().unwrap());

            if let Some(mut note) = self.get_note_from_input(&self.scale, &combined_key) {
                if self.space_pressed {
                    note = note.transpose_down_semitone();
                }
                match combined_key.as_str() {
                    "1<" => {
                        self.audio_manager_1l.stop_sound();
                        self.audio_manager_1l.play_sound_for_note(note, &self.shawzin);
                    }
                    "2<" => {
                        self.audio_manager_2l.stop_sound();
                        self.audio_manager_2l.play_sound_for_note(note, &self.shawzin);
                    }
                    "3<" => {
                        self.audio_manager_3l.stop_sound();
                        self.audio_manager_3l.play_sound_for_note(note, &self.shawzin);
                    }
                    _ => {}
                }
            }
        }
        if self.right_arrow_pressed {
            let modifier =">";

            let mut combined_key: String = "".to_owned();

            combined_key.push(base_sound.parse().unwrap());
            combined_key.push(modifier.parse().unwrap());

            if let Some(mut note) = self.get_note_from_input(&self.scale, &combined_key) {
                if self.space_pressed {
                    note = note.transpose_down_semitone();
                }
                match combined_key.as_str() {
                    "1>" => {
                        self.audio_manager_1r.stop_sound();
                        self.audio_manager_1r.play_sound_for_note(note, &self.shawzin);
                    }
                    "2>" => {
                        self.audio_manager_2r.stop_sound();
                        self.audio_manager_2r.play_sound_for_note(note, &self.shawzin);
                    }
                    "3>" => {
                        self.audio_manager_3r.stop_sound();
                        self.audio_manager_3r.play_sound_for_note(note, &self.shawzin);
                    }
                    _ => {}
                }
            }
        }
        if self.down_arrow_pressed {
            let modifier ="v";

            let mut combined_key: String = "".to_owned();

            combined_key.push(base_sound.parse().unwrap());
            combined_key.push(modifier.parse().unwrap());

            if let Some(mut note) = self.get_note_from_input(&self.scale, &combined_key) {
                if self.space_pressed {
                    note = note.transpose_down_semitone();
                }
                match combined_key.as_str() {
                    "1v" => {
                        self.audio_manager_1d.stop_sound();
                        self.audio_manager_1d.play_sound_for_note(note, &self.shawzin);
                    }
                    "2v" => {
                        self.audio_manager_2d.stop_sound();
                        self.audio_manager_2d.play_sound_for_note(note, &self.shawzin);
                    }
                    "3v" => {
                        self.audio_manager_3d.stop_sound();
                        self.audio_manager_3d.play_sound_for_note(note, &self.shawzin);
                    }
                    _ => {}
                }
            }
        }

        if !self.left_arrow_pressed && !self.right_arrow_pressed && !self.down_arrow_pressed {
            let modifier ="_";

            let mut combined_key: String = "".to_owned();

            combined_key.push(base_sound.parse().unwrap());
            combined_key.push(modifier.parse().unwrap());

            if let Some(mut note) = self.get_note_from_input(&self.scale, &combined_key) {
                if self.space_pressed {
                    note = note.transpose_down_semitone();
                }
                match combined_key.as_str() {
                    "1_" => {
                        self.audio_manager_1.stop_sound();
                        self.audio_manager_1.play_sound_for_note(note, &self.shawzin);
                    }
                    "2_" => {
                        self.audio_manager_2.stop_sound();
                        self.audio_manager_2.play_sound_for_note(note, &self.shawzin);
                    }
                    "3_" => {
                        self.audio_manager_3.stop_sound();
                        self.audio_manager_3.play_sound_for_note(note, &self.shawzin);
                    }
                    _ => {}
                }
            }
        };
    }
}

impl EventHandler<GameError> for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::Color::BLACK);

        let scale_text = Text::new(format!("Current Scale (press tab to change): {:?}", self.scale));
        let dest_point_1 = [10.0, 10.0]; // Position where the text will be drawn (x, y)
        let shawzin_text = Text::new(format!("Current Shawzin (press A to change): {:?}", self.shawzin));
        let dest_point_2 = [10.0, 25.0]; // Position where the text will be drawn (x, y)

        graphics::draw(ctx, &scale_text, DrawParam::default().dest(dest_point_1))?;
        graphics::draw(ctx, &shawzin_text, DrawParam::default().dest(dest_point_2))?;

        let keys = vec!["1", "2", "3"];
        let modifiers = vec![">", "v", "<", "_"];

        let mut ascii_art = String::new();

        for &modifier in &modifiers {
            ascii_art.push_str("\n   |  |  | \n");
            let mut line = format!("{}: ", modifier);
            for &key in &keys {
                let combo = format!("{}{}", key, modifier);
                if let Some(note) = self.get_note_from_input(&self.scale, &combo) {
                    let note_str = format!("{:?}", note).replace("Sharp", "#");
                    line.push_str(&format!("{} ", note_str));
                }
            }
            ascii_art.push_str(&line);
        }

        ascii_art.push_str("\n   |  |  |");
        ascii_art.push_str("\n   1  2  3");

        let scale_ascii_art = Text::new(ascii_art);
        let dest_ascii_art = [10.0, 50.0];

        graphics::draw(ctx, &scale_ascii_art, DrawParam::default().dest(dest_ascii_art))?;

        graphics::present(ctx)
    }

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods, _repeat: bool) {
        match keycode {
            KeyCode::Left => self.left_arrow_pressed = true,
            KeyCode::Right => self.right_arrow_pressed = true,
            KeyCode::Down => self.down_arrow_pressed = true,
            KeyCode::Space => self.space_pressed = true,
            KeyCode::Tab => {
                self.scale = match &self.scale {
                    Scale::PentatonicMinor => Scale::PentatonicMajor,
                    Scale::PentatonicMajor => Scale::Chromatic,
                    Scale::Chromatic => Scale::Hexatonic,
                    Scale::Hexatonic => Scale::Major,
                    Scale::Major => Scale::Minor,
                    Scale::Minor => Scale::Hirajoshi,
                    Scale::Hirajoshi => Scale::Phrygian,
                    Scale::Phrygian => Scale::Yo,
                    Scale::Yo => Scale::PentatonicMinor,
                };
            },
            KeyCode::A => {
                self.shawzin = match &self.shawzin {
                    Shawzin::Mimica => Shawzin::Nelumbo,
                    Shawzin::Nelumbo => Shawzin::Corbu,
                    Shawzin::Corbu => Shawzin::Tiamat,
                    Shawzin::Tiamat => Shawzin::AristeiPrime,
                    Shawzin::AristeiPrime => Shawzin::Narmer,
                    Shawzin::Narmer => Shawzin::KiraS,
                    Shawzin::KiraS => Shawzin::VoidSSong,
                    Shawzin::VoidSSong => Shawzin::Courtly,
                    Shawzin::Courtly => Shawzin::Lonesome,
                    Shawzin::Lonesome => Shawzin::Mimica,
                }
            }
            KeyCode::Key1 | KeyCode::Key2 | KeyCode::Key3 => self.play_shawzin_sound(keycode),
            _ => (),
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods) {
        match keycode {
            KeyCode::Left => self.left_arrow_pressed = false,
            KeyCode::Right => self.right_arrow_pressed = false,
            KeyCode::Down => self.down_arrow_pressed = false,
            KeyCode::Space => self.space_pressed = false,
            _ => (),
        }
    }
}

fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("shawzin_simulator", "NegankoPrime");
    let (ctx, event_loop) = cb.build()?;
    let my_game = MyGame {
        left_arrow_pressed: false,
        right_arrow_pressed: false,
        down_arrow_pressed: false,
        space_pressed: false,
        scale: Scale::Major,
        audio_manager_1: AudioManager::new(),
        audio_manager_2: AudioManager::new(),
        audio_manager_3: AudioManager::new(),
        audio_manager_1l: AudioManager::new(),
        audio_manager_2l: AudioManager::new(),
        audio_manager_3l: AudioManager::new(),
        audio_manager_1d: AudioManager::new(),
        audio_manager_2d: AudioManager::new(),
        audio_manager_3d: AudioManager::new(),
        audio_manager_1r: AudioManager::new(),
        audio_manager_2r: AudioManager::new(),
        audio_manager_3r: AudioManager::new(),
        shawzin: Shawzin::Mimica,
    };

    event::run(ctx, event_loop, my_game)
}
