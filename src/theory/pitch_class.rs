use std::{fmt, str::FromStr};

use crate::theory::error::PitchClassParseError;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub enum PitchClass {
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
// position of the PitchClass in the chord, job for a while later though
// it might make sense to change the PitchClasses above from C, Cs, D etc and change them to just octave positions like -
// 0, 1, 2, 3 or some kind of pitch class type
// and have the printout decide the PitchClass name based on the chord context
impl fmt::Display for PitchClass {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PitchClass::C => write!(f, "C"),
            PitchClass::Cs => write!(f, "C#"),
            PitchClass::D => write!(f, "D"),
            PitchClass::Ds => write!(f, "D#"),
            PitchClass::E => write!(f, "E"),
            PitchClass::F => write!(f, "F"),
            PitchClass::Fs => write!(f, "F#"),
            PitchClass::G => write!(f, "G"),
            PitchClass::Gs => write!(f, "G#"),
            PitchClass::A => write!(f, "A"),
            PitchClass::As => write!(f, "A#"),
            PitchClass::B => write!(f, "B"),
        }
    }
}

impl FromStr for PitchClass {
    type Err = PitchClassParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "C" => Ok(PitchClass::C),
            "C#" => Ok(PitchClass::Cs),
            "Db" => Ok(PitchClass::Cs), // TODO: worry about flats and sharp matches later
            "D" => Ok(PitchClass::D),
            "D#" => Ok(PitchClass::Ds),
            "E" => Ok(PitchClass::E),
            "F" => Ok(PitchClass::F),
            "F#" => Ok(PitchClass::Fs),
            "G" => Ok(PitchClass::G),
            "G#" => Ok(PitchClass::Gs),
            "A" => Ok(PitchClass::A),
            "A#" => Ok(PitchClass::As),
            "B" => Ok(PitchClass::B),
            _ => Err(PitchClassParseError::InvalidPitchClassStringValue(
                s.to_string(),
            )),
        }
    }
}
impl PitchClass {
    pub fn parse(str: &str) -> Result<PitchClass, PitchClassParseError> {
        return PitchClass::from_str(str);
    }
}
