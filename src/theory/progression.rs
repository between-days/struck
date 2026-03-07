use crate::{
    parser::chord_parser,
    theory::{
        chord::Chord,
        error::ProgressionParseError,
        pitch_class::{self, PitchClass},
    },
};

// each scale will have positions like iv, v, vi
// those are the chords build from the progression where the chord root is the scale degree in question
pub enum ScaleDegree {
    I,
    ii,
    iii,
    IV,
    V,
    vi,
    viidim,
}

// just western for now
// essentially all scales are the major scale, but the mode changes what the tonic is
// so ionian is the name of the typical major scale, and it's tonic is at position 0 in the major scale of a given note
// dorian has it's tonic at position 1 and so on down the list
pub enum Mode {
    Ionian = 0,
    Dorian = 1,
    Phyrgian = 2,
    Lydian = 3,
    Mixolydian = 4,
    Aeolian = 5,
    Locrian = 6,

    // TODO: fix later
    Unknown = 100,
}

pub struct Key {
    tonic: PitchClass,
    mode: Mode,
}

// impl Key {

// }

pub struct Progression {
    chords: Vec<Chord>,
    key: Key,
}

// impl Progression {
//     pub fn from_string_list(chords_str: &str) -> Result<Progression, ProgressionParseError> {
//         let chords: Vec<Chord> = chords_str
//             .trim()
//             .split_terminator(" ")
//             .map(|cs| chord_parser::identify_from_name(cs.to_string()))
//             .collect();

//         // TODO: we don't know the key yet
//         let key = Key {
//             tonic: pitch_class::PitchClass::A,
//             mode: Mode::Aeolian,
//         };

//         Ok(Progression { chords, key })
//     }
// }
