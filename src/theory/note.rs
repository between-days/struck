use std::{fmt, str::FromStr};

use crate::theory::error::NoteParseError;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub enum Note {
    #[default]
    C,
    Cs,
    D,
    Ds,
    E,
    F,
    Fs,
    G,
    Gs,
    A,
    As,
    B,
}

// TODO: might be able to do something here about choosing between Db and C#, all depends on the context of the -
// position of the note in the chord, job for a while later though
// it might make sense to change the notes above from C, Cs, D etc and change them to just octave positions like -
// 0, 1, 2, 3 or some kind of pitch class type
// and have the printout decide the note name based on the chord context
impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Note::C => write!(f, "C"),
            Note::Cs => write!(f, "C#"),
            Note::D => write!(f, "D"),
            Note::Ds => write!(f, "D#"),
            Note::E => write!(f, "E"),
            Note::F => write!(f, "F"),
            Note::Fs => write!(f, "F#"),
            Note::G => write!(f, "G"),
            Note::Gs => write!(f, "G#"),
            Note::A => write!(f, "A"),
            Note::As => write!(f, "A#"),
            Note::B => write!(f, "B"),
        }
    }
}

impl FromStr for Note {
    type Err = NoteParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "C" => Ok(Note::C),
            "C#" => Ok(Note::Cs),
            "Db" => Ok(Note::Cs), // TODO: worry about flats and sharp matches later
            "D" => Ok(Note::D),
            "D#" => Ok(Note::Ds),
            "E" => Ok(Note::E),
            "F" => Ok(Note::F),
            "F#" => Ok(Note::Fs),
            "G" => Ok(Note::G),
            "G#" => Ok(Note::Gs),
            "A" => Ok(Note::A),
            "A#" => Ok(Note::As),
            "B" => Ok(Note::B),
            _ => Err(NoteParseError::InvalidNoteStringValue(s.to_string())),
        }
    }
}
impl Note {
    pub fn parse(str: &str) -> Result<Note, NoteParseError> {
        return Note::from_str(str);
    }
}
