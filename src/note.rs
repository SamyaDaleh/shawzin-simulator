use std::collections::HashMap;
use crate::Shawzin;

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
#[derive(Copy)]
pub enum Note {
    C4, CSharp4, D4, DSharp4, E4, F4, FSharp4, G4, GSharp4, A4, ASharp4, B4,
    C5, CSharp5, D5, DSharp5, E5, F5, FSharp5, G5, GSharp5, A5, ASharp5, B5,
    C6, CSharp6, D6, DSharp6,
    B3,
}

impl Note {
    pub(crate) fn to_file_name(&self, shawzin: &Shawzin) -> &'static str {
        match shawzin {
            Shawzin::Courtly => {
                match self {
                    Note::B3 => "DuviriShawzinOctZeroB",
                    Note::C4 => "DuviriShawzinOctOneC",
                    Note::CSharp4 => "DuviriShawzinOctOneCSharp",
                    Note::D4 => "DuviriShawzinOctOneD",
                    Note::DSharp4 => "DuviriShawzinOctOneDSharp",
                    Note::E4 => "DuviriShawzinOctOneE",
                    Note::F4 => "DuviriShawzinOctOneF",
                    Note::FSharp4 => "DuviriShawzinOctOneFSharp",
                    Note::G4 => "DuviriShawzinOctOneG",
                    Note::GSharp4 => "DuviriShawzinOctOneGSharp",
                    Note::A4 => "DuviriShawzinOctOneA",
                    Note::ASharp4 => "DuviriShawzinOctOneASharp",
                    Note::B4 => "DuviriShawzinOctOneB",
                    Note::C5 => "DuviriShawzinOctTwoC",
                    Note::CSharp5 => "DuviriShawzinOctTwoCSharp",
                    Note::D5 => "DuviriShawzinOctTwoD",
                    Note::DSharp5 => "DuviriShawzinOctTwoDSharp",
                    Note::E5 => "DuviriShawzinOctTwoE",
                    Note::F5 => "DuviriShawzinOctTwoF",
                    Note::FSharp5 => "DuviriShawzinOctTwoFSharp",
                    Note::G5 => "DuviriShawzinOctTwoG",
                    Note::GSharp5 => "DuviriShawzinOctTwoGSharp",
                    Note::A5 => "DuviriShawzinOctTwoA",
                    Note::ASharp5 => "DuviriShawzinOctTwoASharp",
                    Note::B5 => "DuviriShawzinOctTwoB",
                    Note::C6 => "DuviriShawzinOctThreeC",
                    Note::CSharp6 => "DuviriShawzinOctThreeCSharp",
                    Note::D6 => "DuviriShawzinOctThreeD",
                    Note::DSharp6 => "DuviriShawzinOctThreeDSharp",
                }
            }
            Shawzin::Mimica => {
                match self {
                    Note::B3 => "ShawzinOctZeroB",
                    Note::C4 => "ShawzinOctOneC",
                    Note::CSharp4 => "ShawzinOctOneCSharp",
                    Note::D4 => "ShawzinOctOneD",
                    Note::DSharp4 => "ShawzinOctOneDSharp",
                    Note::E4 => "ShawzinOctOneE",
                    Note::F4 => "ShawzinOctOneF",
                    Note::FSharp4 => "ShawzinOctOneFSharp",
                    Note::G4 => "ShawzinOctOneG",
                    Note::GSharp4 => "ShawzinOctOneGSharp",
                    Note::A4 => "ShawzinOctOneA",
                    Note::ASharp4 => "ShawzinOctOneASharp",
                    Note::B4 => "ShawzinOctOneB",
                    Note::C5 => "ShawzinOctTwoC",
                    Note::CSharp5 => "ShawzinOctTwoCSharp",
                    Note::D5 => "ShawzinOctTwoD",
                    Note::DSharp5 => "ShawzinOctTwoDSharp",
                    Note::E5 => "ShawzinOctTwoE",
                    Note::F5 => "ShawzinOctTwoF",
                    Note::FSharp5 => "ShawzinOctTwoFSharp",
                    Note::G5 => "ShawzinOctTwoG",
                    Note::GSharp5 => "ShawzinOctTwoGSharp",
                    Note::A5 => "ShawzinOctTwoA",
                    Note::ASharp5 => "ShawzinOctTwoASharp",
                    Note::B5 => "ShawzinOctTwoB",
                    Note::C6 => "ShawzinOctThreeC",
                    Note::CSharp6 => "ShawzinOctThreeCSharp",
                    Note::D6 => "ShawzinOctThreeD",
                    Note::DSharp6 => "ShawzinOctThreeDSharp",
                }
            }
            Shawzin::Nelumbo => {
                match self {
                    Note::B3 => "LotusShawzinOctZeroB",
                    Note::C4 => "LotusShawzinOctOneC",
                    Note::CSharp4 => "LotusShawzinOctOneCSharp",
                    Note::D4 => "LotusShawzinOctOneD",
                    Note::DSharp4 => "LotusShawzinOctOneDSharp",
                    Note::E4 => "LotusShawzinOctOneE",
                    Note::F4 => "LotusShawzinOctOneF",
                    Note::FSharp4 => "LotusShawzinOctOneFSharp",
                    Note::G4 => "LotusShawzinOctOneG",
                    Note::GSharp4 => "LotusShawzinOctOneGSharp",
                    Note::A4 => "LotusShawzinOctOneA",
                    Note::ASharp4 => "LotusShawzinOctOneASharp",
                    Note::B4 => "LotusShawzinOctOneB",
                    Note::C5 => "LotusShawzinOctTwoC",
                    Note::CSharp5 => "LotusShawzinOctTwoCSharp",
                    Note::D5 => "LotusShawzinOctTwoD",
                    Note::DSharp5 => "LotusShawzinOctTwoDSharp",
                    Note::E5 => "LotusShawzinOctTwoE",
                    Note::F5 => "LotusShawzinOctTwoF",
                    Note::FSharp5 => "LotusShawzinOctTwoFSharp",
                    Note::G5 => "LotusShawzinOctTwoG",
                    Note::GSharp5 => "LotusShawzinOctTwoGSharp",
                    Note::A5 => "LotusShawzinOctTwoA",
                    Note::ASharp5 => "LotusShawzinOctTwoASharp",
                    Note::B5 => "LotusShawzinOctTwoB",
                    Note::C6 => "LotusShawzinOctThreeC",
                    Note::CSharp6 => "LotusShawzinOctThreeCSharp",
                    Note::D6 => "LotusShawzinOctThreeD",
                    Note::DSharp6 => "LotusShawzinOctThreeDSharp",
                }
            }
            Shawzin::Corbu => {
                match self {
                    Note::B3 => "GrineerShawzinOctZeroB",
                    Note::C4 => "GrineerShawzinOctOneC",
                    Note::CSharp4 => "GrineerShawzinOctOneCSharp",
                    Note::D4 => "GrineerShawzinOctOneD",
                    Note::DSharp4 => "GrineerShawzinOctOneDSharp",
                    Note::E4 => "GrineerShawzinOctOneE",
                    Note::F4 => "GrineerShawzinOctOneF",
                    Note::FSharp4 => "GrineerShawzinOctOneFSharp",
                    Note::G4 => "GrineerShawzinOctOneG",
                    Note::GSharp4 => "GrineerShawzinOctOneGSharp",
                    Note::A4 => "GrineerShawzinOctOneA",
                    Note::ASharp4 => "GrineerShawzinOctOneASharp",
                    Note::B4 => "GrineerShawzinOctOneB",
                    Note::C5 => "GrineerShawzinOctTwoC",
                    Note::CSharp5 => "GrineerShawzinOctTwoCSharp",
                    Note::D5 => "GrineerShawzinOctTwoD",
                    Note::DSharp5 => "GrineerShawzinOctTwoDSharp",
                    Note::E5 => "GrineerShawzinOctTwoE",
                    Note::F5 => "GrineerShawzinOctTwoF",
                    Note::FSharp5 => "GrineerShawzinOctTwoFSharp",
                    Note::G5 => "GrineerShawzinOctTwoG",
                    Note::GSharp5 => "GrineerShawzinOctTwoGSharp",
                    Note::A5 => "GrineerShawzinOctTwoA",
                    Note::ASharp5 => "GrineerShawzinOctTwoASharp",
                    Note::B5 => "GrineerShawzinOctTwoB",
                    Note::C6 => "GrineerShawzinOctThreeC",
                    Note::CSharp6 => "GrineerShawzinOctThreeCSharp",
                    Note::D6 => "GrineerShawzinOctThreeD",
                    Note::DSharp6 => "GrineerShawzinOctThreeDSharp",
                }
            }
            Shawzin::Tiamat => {
                match self {
                    Note::B3 => "SentientShawzinOctZeroB",
                    Note::C4 => "SentientShawzinOctOneC",
                    Note::CSharp4 => "SentientShawzinOctOneCSharp",
                    Note::D4 => "SentientShawzinOctOneD",
                    Note::DSharp4 => "SentientShawzinOctOneDSharp",
                    Note::E4 => "SentientShawzinOctOneE",
                    Note::F4 => "SentientShawzinOctOneF",
                    Note::FSharp4 => "SentientShawzinOctOneFSharp",
                    Note::G4 => "SentientShawzinOctOneG",
                    Note::GSharp4 => "SentientShawzinOctOneGSharp",
                    Note::A4 => "SentientShawzinOctOneA",
                    Note::ASharp4 => "SentientShawzinOctOneASharp",
                    Note::B4 => "SentientShawzinOctOneB",
                    Note::C5 => "SentientShawzinOctTwoC",
                    Note::CSharp5 => "SentientShawzinOctTwoCSharp",
                    Note::D5 => "SentientShawzinOctTwoD",
                    Note::DSharp5 => "SentientShawzinOctTwoDSharp",
                    Note::E5 => "SentientShawzinOctTwoE",
                    Note::F5 => "SentientShawzinOctTwoF",
                    Note::FSharp5 => "SentientShawzinOctTwoFSharp",
                    Note::G5 => "SentientShawzinOctTwoG",
                    Note::GSharp5 => "SentientShawzinOctTwoGSharp",
                    Note::A5 => "SentientShawzinOctTwoA",
                    Note::ASharp5 => "SentientShawzinOctTwoASharp",
                    Note::B5 => "SentientShawzinOctTwoB",
                    Note::C6 => "SentientShawzinOctThreeC",
                    Note::CSharp6 => "SentientShawzinOctThreeCSharp",
                    Note::D6 => "SentientShawzinOctThreeD",
                    Note::DSharp6 => "SentientShawzinOctThreeDSharp",
                }
            }
            Shawzin::AristeiPrime => {
                match self {
                    Note::B3 => "PrimeShawzinOctZeroB",
                    Note::C4 => "PrimeShawzinOctOneC",
                    Note::CSharp4 => "PrimeShawzinOctOneCSharp",
                    Note::D4 => "PrimeShawzinOctOneD",
                    Note::DSharp4 => "PrimeShawzinOctOneDSharp",
                    Note::E4 => "PrimeShawzinOctOneE",
                    Note::F4 => "PrimeShawzinOctOneF",
                    Note::FSharp4 => "PrimeShawzinOctOneFSharp",
                    Note::G4 => "PrimeShawzinOctOneG",
                    Note::GSharp4 => "PrimeShawzinOctOneGSharp",
                    Note::A4 => "PrimeShawzinOctOneA",
                    Note::ASharp4 => "PrimeShawzinOctOneASharp",
                    Note::B4 => "PrimeShawzinOctOneB",
                    Note::C5 => "PrimeShawzinOctTwoC",
                    Note::CSharp5 => "PrimeShawzinOctTwoCSharp",
                    Note::D5 => "PrimeShawzinOctTwoD",
                    Note::DSharp5 => "PrimeShawzinOctTwoDSharp",
                    Note::E5 => "PrimeShawzinOctTwoE",
                    Note::F5 => "PrimeShawzinOctTwoF",
                    Note::FSharp5 => "PrimeShawzinOctTwoFSharp",
                    Note::G5 => "PrimeShawzinOctTwoG",
                    Note::GSharp5 => "PrimeShawzinOctTwoGSharp",
                    Note::A5 => "PrimeShawzinOctTwoA",
                    Note::ASharp5 => "PrimeShawzinOctTwoASharp",
                    Note::B5 => "PrimeShawzinOctTwoB",
                    Note::C6 => "PrimeShawzinOctThreeC",
                    Note::CSharp6 => "PrimeShawzinOctThreeCSharp",
                    Note::D6 => "PrimeShawzinOctThreeD",
                    Note::DSharp6 => "PrimeShawzinOctThreeDSharp",
                }
            }
            Shawzin::Narmer => {
                match self {
                    Note::B3 => "NarmerShawzinOctZeroB",
                    Note::C4 => "NarmerShawzinOctOneC",
                    Note::CSharp4 => "NarmerShawzinOctOneCSharp",
                    Note::D4 => "NarmerShawzinOctOneD",
                    Note::DSharp4 => "NarmerShawzinOctOneDSharp",
                    Note::E4 => "NarmerShawzinOctOneE",
                    Note::F4 => "NarmerShawzinOctOneF",
                    Note::FSharp4 => "NarmerShawzinOctOneFSharp",
                    Note::G4 => "NarmerShawzinOctOneG",
                    Note::GSharp4 => "NarmerShawzinOctOneGSharp",
                    Note::A4 => "NarmerShawzinOctOneA",
                    Note::ASharp4 => "NarmerShawzinOctOneASharp",
                    Note::B4 => "NarmerShawzinOctOneB",
                    Note::C5 => "NarmerShawzinOctTwoC",
                    Note::CSharp5 => "NarmerShawzinOctTwoCSharp",
                    Note::D5 => "NarmerShawzinOctTwoD",
                    Note::DSharp5 => "NarmerShawzinOctTwoDSharp",
                    Note::E5 => "NarmerShawzinOctTwoE",
                    Note::F5 => "NarmerShawzinOctTwoF",
                    Note::FSharp5 => "NarmerShawzinOctTwoFSharp",
                    Note::G5 => "NarmerShawzinOctTwoG",
                    Note::GSharp5 => "NarmerShawzinOctTwoGSharp",
                    Note::A5 => "NarmerShawzinOctTwoA",
                    Note::ASharp5 => "NarmerShawzinOctTwoASharp",
                    Note::B5 => "NarmerShawzinOctTwoB",
                    Note::C6 => "NarmerShawzinOctThreeC",
                    Note::CSharp6 => "NarmerShawzinOctThreeCSharp",
                    Note::D6 => "NarmerShawzinOctThreeD",
                    Note::DSharp6 => "NarmerShawzinOctThreeDSharp",
                }
            }
            Shawzin::KiraS => {
                match self {
                    Note::B3 => "ZarimanShawzinOctZeroB",
                    Note::C4 => "ZarimanShawzinOctOneC",
                    Note::CSharp4 => "ZarimanShawzinOctOneCSharp",
                    Note::D4 => "ZarimanShawzinOctOneD",
                    Note::DSharp4 => "ZarimanShawzinOctOneDSharp",
                    Note::E4 => "ZarimanShawzinOctOneE",
                    Note::F4 => "ZarimanShawzinOctOneF",
                    Note::FSharp4 => "ZarimanShawzinOctOneFSharp",
                    Note::G4 => "ZarimanShawzinOctOneG",
                    Note::GSharp4 => "ZarimanShawzinOctOneGSharp",
                    Note::A4 => "ZarimanShawzinOctOneA",
                    Note::ASharp4 => "ZarimanShawzinOctOneASharp",
                    Note::B4 => "ZarimanShawzinOctOneB",
                    Note::C5 => "ZarimanShawzinOctTwoC",
                    Note::CSharp5 => "ZarimanShawzinOctTwoCSharp",
                    Note::D5 => "ZarimanShawzinOctTwoD",
                    Note::DSharp5 => "ZarimanShawzinOctTwoDSharp",
                    Note::E5 => "ZarimanShawzinOctTwoE",
                    Note::F5 => "ZarimanShawzinOctTwoF",
                    Note::FSharp5 => "ZarimanShawzinOctTwoFSharp",
                    Note::G5 => "ZarimanShawzinOctTwoG",
                    Note::GSharp5 => "ZarimanShawzinOctTwoGSharp",
                    Note::A5 => "ZarimanShawzinOctTwoA",
                    Note::ASharp5 => "ZarimanShawzinOctTwoASharp",
                    Note::B5 => "ZarimanShawzinOctTwoB",
                    Note::C6 => "ZarimanShawzinOctThreeC",
                    Note::CSharp6 => "ZarimanShawzinOctThreeCSharp",
                    Note::D6 => "ZarimanShawzinOctThreeD",
                    Note::DSharp6 => "ZarimanShawzinOctThreeDSharp",
                }
            }
            Shawzin::VoidSSong => {
                match self {
                    Note::B3 => "ZarimanVoidShawzinOctZeroB",
                    Note::C4 => "ZarimanVoidShawzinOctOneC",
                    Note::CSharp4 => "ZarimanVoidShawzinOctOneCSharp",
                    Note::D4 => "ZarimanVoidShawzinOctOneD",
                    Note::DSharp4 => "ZarimanVoidShawzinOctOneDSharp",
                    Note::E4 => "ZarimanVoidShawzinOctOneE",
                    Note::F4 => "ZarimanVoidShawzinOctOneF",
                    Note::FSharp4 => "ZarimanVoidShawzinOctOneFSharp",
                    Note::G4 => "ZarimanVoidShawzinOctOneG",
                    Note::GSharp4 => "ZarimanVoidShawzinOctOneGSharp",
                    Note::A4 => "ZarimanVoidShawzinOctOneA",
                    Note::ASharp4 => "ZarimanVoidShawzinOctOneASharp",
                    Note::B4 => "ZarimanVoidShawzinOctOneB",
                    Note::C5 => "ZarimanVoidShawzinOctTwoC",
                    Note::CSharp5 => "ZarimanVoidShawzinOctTwoCSharp",
                    Note::D5 => "ZarimanVoidShawzinOctTwoD",
                    Note::DSharp5 => "ZarimanVoidShawzinOctTwoDSharp",
                    Note::E5 => "ZarimanVoidShawzinOctTwoE",
                    Note::F5 => "ZarimanVoidShawzinOctTwoF",
                    Note::FSharp5 => "ZarimanVoidShawzinOctTwoFSharp",
                    Note::G5 => "ZarimanVoidShawzinOctTwoG",
                    Note::GSharp5 => "ZarimanVoidShawzinOctTwoGSharp",
                    Note::A5 => "ZarimanVoidShawzinOctTwoA",
                    Note::ASharp5 => "ZarimanVoidShawzinOctTwoASharp",
                    Note::B5 => "ZarimanVoidShawzinOctTwoB",
                    Note::C6 => "ZarimanVoidShawzinOctThreeC",
                    Note::CSharp6 => "ZarimanVoidShawzinOctThreeCSharp",
                    Note::D6 => "ZarimanVoidShawzinOctThreeD",
                    Note::DSharp6 => "ZarimanVoidShawzinOctThreeDSharp",
                }
            }
            Shawzin::Lonesome => {
                match self {
                    Note::B3 => "DuviriErsatzShawzinOctZeroB",
                    Note::C4 => "DuviriErsatzShawzinOctOneC",
                    Note::CSharp4 => "DuviriErsatzShawzinOctOneCSharp",
                    Note::D4 => "DuviriErsatzShawzinOctOneD",
                    Note::DSharp4 => "DuviriErsatzShawzinOctOneDSharp",
                    Note::E4 => "DuviriErsatzShawzinOctOneE",
                    Note::F4 => "DuviriErsatzShawzinOctOneF",
                    Note::FSharp4 => "DuviriErsatzShawzinOctOneFSharp",
                    Note::G4 => "DuviriErsatzShawzinOctOneG",
                    Note::GSharp4 => "DuviriErsatzShawzinOctOneGSharp",
                    Note::A4 => "DuviriErsatzShawzinOctOneA",
                    Note::ASharp4 => "DuviriErsatzShawzinOctOneASharp",
                    Note::B4 => "DuviriErsatzShawzinOctOneB",
                    Note::C5 => "DuviriErsatzShawzinOctTwoC",
                    Note::CSharp5 => "DuviriErsatzShawzinOctTwoCSharp",
                    Note::D5 => "DuviriErsatzShawzinOctTwoD",
                    Note::DSharp5 => "DuviriErsatzShawzinOctTwoDSharp",
                    Note::E5 => "DuviriErsatzShawzinOctTwoE",
                    Note::F5 => "DuviriErsatzShawzinOctTwoF",
                    Note::FSharp5 => "DuviriErsatzShawzinOctTwoFSharp",
                    Note::G5 => "DuviriErsatzShawzinOctTwoG",
                    Note::GSharp5 => "DuviriErsatzShawzinOctTwoGSharp",
                    Note::A5 => "DuviriErsatzShawzinOctTwoA",
                    Note::ASharp5 => "DuviriErsatzShawzinOctTwoASharp",
                    Note::B5 => "DuviriErsatzShawzinOctTwoB",
                    Note::C6 => "DuviriErsatzShawzinOctThreeC",
                    Note::CSharp6 => "DuviriErsatzShawzinOctThreeCSharp",
                    Note::D6 => "DuviriErsatzShawzinOctThreeD",
                    Note::DSharp6 => "DuviriErsatzShawzinOctThreeDSharp",
                }
            }
        }
    }

    pub(crate) fn transpose_down_semitone(self) -> Note {
        match self {
            Note::C4 => Note::B3,
            Note::CSharp4 => Note::C4,
            Note::D4 => Note::CSharp4,
            Note::DSharp4 => Note::D4,
            Note::E4 => Note::DSharp4,
            Note::F4 => Note::E4,
            Note::FSharp4 => Note::F4,
            Note::G4 => Note::FSharp4,
            Note::GSharp4 => Note::G4,
            Note::A4 => Note::GSharp4,
            Note::ASharp4 => Note::A4,
            Note::C5 => Note::B4,
            Note::CSharp5 => Note::C5,
            Note::D5 => Note::CSharp5,
            Note::DSharp5 => Note::D5,
            Note::E5 => Note::DSharp5,
            Note::F5 => Note::E5,
            Note::FSharp5 => Note::F5,
            Note::G5 => Note::FSharp5,
            Note::GSharp5 => Note::G5,
            Note::A5 => Note::GSharp5,
            Note::ASharp5 => Note::A5,
            Note::C6 => Note::B5,
            Note::CSharp6 => Note::C6,
            Note::D6 => Note::CSharp6,
            Note::DSharp6 => Note::D6,
            _ => self,
        }
    }
}

#[derive(Debug)]
pub enum Scale {
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

pub struct ScaleMapping {
    notes: HashMap<&'static str, Note>,
}

impl ScaleMapping {
    pub(crate) fn new(scale: &Scale) -> Self {
        let notes = match scale {
            Scale::PentatonicMinor => {
                vec![
                    ("1_", Note::C4), ("2_", Note::DSharp4), ("3_", Note::F4),
                    ("1<", Note::G4), ("2<", Note::ASharp4), ("3<", Note::C5),
                    ("1v", Note::DSharp5), ("2v", Note::F5), ("3v", Note::G5),
                    ("1>", Note::ASharp5), ("2>", Note::C6), ("3>", Note::DSharp6),
                ]
            },
            Scale::PentatonicMajor => {
                vec![
                    ("1_", Note::C4), ("2_", Note::D4), ("3_", Note::E4),
                    ("1<", Note::G4), ("2<", Note::A4), ("3<", Note::C5),
                    ("1v", Note::D5), ("2v", Note::E5), ("3v", Note::G5),
                    ("1>", Note::A5), ("2>", Note::C6), ("3>", Note::D6),
                ]
            },
            Scale::Chromatic => {
                vec![
                    ("1_", Note::C4), ("2_", Note::CSharp4), ("3_", Note::D4),
                    ("1<", Note::DSharp4), ("2<", Note::E4), ("3<", Note::F4),
                    ("1v", Note::FSharp4), ("2v", Note::G4), ("3v", Note::GSharp4),
                    ("1>", Note::A4), ("2>", Note::ASharp4), ("3>", Note::B4),
                ]
            },
            Scale::Hexatonic => {
                vec![
                    ("1_", Note::C4), ("2_", Note::DSharp4), ("3_", Note::F4),
                    ("1<", Note::FSharp4), ("2<", Note::G4), ("3<", Note::ASharp4),
                    ("1v", Note::C5), ("2v", Note::DSharp5), ("3v", Note::F5),
                    ("1>", Note::FSharp5), ("2>", Note::G5), ("3>", Note::ASharp5),
                ]
            },
            Scale::Major => {
                vec![
                    ("1_", Note::C4), ("2_", Note::D4), ("3_", Note::E4),
                    ("1<", Note::F4), ("2<", Note::G4), ("3<", Note::A4),
                    ("1v", Note::B4), ("2v", Note::C5), ("3v", Note::D5),
                    ("1>", Note::E5), ("2>", Note::F5), ("3>", Note::G5),
                ]
            },
            Scale::Minor => {
                vec![
                    ("1_", Note::C4), ("2_", Note::D4), ("3_", Note::DSharp4),
                    ("1<", Note::F4), ("2<", Note::G4), ("3<", Note::GSharp4),
                    ("1v", Note::ASharp4), ("2v", Note::C5), ("3v", Note::D5),
                    ("1>", Note::DSharp5), ("2>", Note::F5), ("3>", Note::G5),
                ]
            },
            Scale::Hirajoshi => {
                vec![
                    ("1_", Note::C4), ("2_", Note::CSharp4), ("3_", Note::F4),
                    ("1<", Note::FSharp4), ("2<", Note::ASharp4), ("3<", Note::C5),
                    ("1v", Note::CSharp5), ("2v", Note::F5), ("3v", Note::FSharp5),
                    ("1>", Note::A5), ("2>", Note::C6), ("3>", Note::CSharp6),
                ]
            },
            Scale::Phrygian => {
                vec![
                    ("1_", Note::C4), ("2_", Note::CSharp4), ("3_", Note::E4),
                    ("1<", Note::F4), ("2<", Note::G4), ("3<", Note::GSharp4),
                    ("1v", Note::ASharp4), ("2v", Note::C5), ("3v", Note::CSharp5),
                    ("1>", Note::E5), ("2>", Note::F5), ("3>", Note::G5),
                ]
            },
            Scale::Yo => {
                vec![
                    ("1_", Note::CSharp4), ("2_", Note::DSharp4), ("3_", Note::FSharp4),
                    ("1<", Note::GSharp4), ("2<", Note::ASharp4), ("3<", Note::CSharp5),
                    ("1v", Note::DSharp5), ("2v", Note::FSharp5), ("3v", Note::GSharp5),
                    ("1>", Note::ASharp5), ("2>", Note::CSharp6), ("3>", Note::DSharp6),
                ]
            },
        }.into_iter().collect();

        ScaleMapping { notes }
    }

    pub(crate) fn get_note(&self, combined_key: &str) -> Option<Note> {
        self.notes.get(combined_key).copied()
    }
}
