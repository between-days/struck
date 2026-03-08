use crate::theory::{
    error::{NoteParseError, PitchClassParseError},
    pitch_class::{self, PitchClass},
};
use std::{fmt, str::FromStr};
#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub struct Note {
    pub pitch_class: PitchClass,
    pub pitch_octave: u8,
    // literal frequency for synth to use on a sine
    // pub frequency: f64,
}

// impl Eq for Note
// TODO: might be able to do something here about choosing between Db and C#, all depends on the context of the -
// position of the PitchClass in the chord, job for a while later though
// it might make sense to change the PitchClasses above from C, Cs, D etc and change them to just octave positions like -
// 0, 1, 2, 3 or some kind of pitch class type
// and have the printout decide the PitchClass name based on the chord context
impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let pitch_class_str = match self.pitch_class {
            PitchClass::C => "C",
            PitchClass::Cs => "C#",
            PitchClass::D => "D",
            PitchClass::Ds => "D#",
            PitchClass::E => "E",
            PitchClass::F => "F",
            PitchClass::Fs => "F#",
            PitchClass::G => "G",
            PitchClass::Gs => "G#",
            PitchClass::A => "A",
            PitchClass::As => "A#",
            PitchClass::B => "B",
        };

        // TODO: work out formatting later
        // write!(f, "{}{}", pitch_class_str, self.pitch_octave)
        write!(f, "{}", pitch_class_str)
    }
}

impl FromStr for Note {
    type Err = NoteParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pitch_class = PitchClass::from_str(s)?;

        // default octave 4 for now
        Ok(Note {
            pitch_class: pitch_class,
            pitch_octave: 4,
        })
    }
}

// TODO: where should this live?
// TODO: maybe should refactor to take pitch classes and assume notes as this is also used for getting scale notes which come from just pitch_classes atm
// entering G B D will go to G4 B4 D4 which is typically incorrect, that input usually means G4 B4 D5 which is a G major chord
// for cli input, we should assume the notes are in ascending order
// this is also used for playing scales
pub fn assume_note_ordering(mut notes: Vec<Note>) -> Vec<Note> {
    let mut index = 1;

    // go over each note in the list, creating a new list in assumed order i.e get that D5 mentioned in above comment
    while index < notes.len() {
        // if note is lower than one before, increase octave
        if notes.get(index) < notes.get(index - 1) {
            let n = notes[index];

            let nn = Note {
                pitch_class: n.pitch_class,
                pitch_octave: n.pitch_octave + 1,
            };

            notes[index] = nn;
        }

        index = index + 1;
    }

    return notes;
}

// impl Note {
//     // pub fn new(pitch_class: pitch_class::PitchClass, pitch_octave: u8) -> Note {
//     //     // TODO: ban above certain octave
//     //     return Note {
//     //         pitch_class: pitch_class,
//     //         pitch_octave: pitch_octave,
//     //         // frequency: 2332.0,
//     //     };
//     // }

//     // pub fn parse(str: &str) -> Result<Note, NoteParseError> {
//     //     let pitch_class = PitchClass::parse(str);
//     //     // TODO: for now default to 4th octave
//     //     let octave = 4;

//     //     match (pitch_class) {
//     //         (Err(e_p)) => {
//     //             return Err(NoteParseError::InvalidPitchClassStringValue(
//     //                 "invalid pitch class".to_string(),
//     //             ));
//     //         }
//     //         (Ok(p_c)) => {
//     //             return Ok(Note::new(p_c, octave));
//     //         }
//     //     }
//     // }
// }
