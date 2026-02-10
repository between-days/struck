use core::fmt;
use itertools::Itertools;

use crate::theory::note::Note;

pub const OCTAVE: [Note; 12] = [
    Note::C,
    Note::Cs,
    Note::D,
    Note::Ds,
    Note::E,
    Note::F,
    Note::Fs,
    Note::G,
    Note::Gs,
    Note::A,
    Note::As,
    Note::B,
];

// number of semitone steps
// https://en.wikipedia.org/wiki/Interval_(music)
// names refer to chromatic scale positions so we don't need to worry about scales when finding chords intervals
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Interval {
    // we only consider the ones relevant to naming for now
    MajorSecond = 2,
    MinorThird = 3,
    MajorThird = 4,
    PerfectFourth = 5,
    DiminishedFifth = 6,
    PerfectFifth = 7,
    AugmentedFifth = 8,
    DiminishedSeventh = 9,
    MinorSeventh = 10,
    Seventh = 11,
    MinorNinth = 13,
    MajorNinth = 14,
    PerfectEleventh = 17,
    Unknown = 100, // TODO: cheese for now
}

impl From<usize> for Interval {
    fn from(value: usize) -> Self {
        match value {
            3 => Interval::MinorThird,
            4 => Interval::MajorThird,
            6 => Interval::DiminishedFifth,
            7 => Interval::PerfectFifth,
            8 => Interval::AugmentedFifth,
            11 => Interval::Seventh,
            _ => Interval::Unknown,
        }
    }
}

impl fmt::Display for Interval {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Interval::MajorSecond => write!(f, "Major 2nd"),
            Interval::MinorThird => write!(f, "Minor 2nd"),
            Interval::MajorThird => write!(f, "Major 3rd"),
            Interval::PerfectFourth => write!(f, "Perfect 5th"),
            Interval::DiminishedFifth => write!(f, "Diminished 5th"),
            Interval::PerfectFifth => write!(f, "Perfect 5th"),
            Interval::AugmentedFifth => write!(f, "Augmented 5th"),
            Interval::MinorSeventh => write!(f, "Minor 7th"),
            Interval::Seventh => write!(f, "5th"),
            Interval::DiminishedSeventh => write!(f, "Diminished 7th"),
            Interval::MinorNinth => write!(f, "Minor 9th"),
            Interval::MajorNinth => write!(f, "Minor 9th"),
            Interval::PerfectEleventh => write!(f, "Perfect 11th"),
            Interval::Unknown => write!(f, "Unknown"),
        }
    }
}

// get this many semitones above the note
pub fn get_interval(note: &Note, interval: Interval) -> &Note {
    // get where the root note is in octave
    let root_index = match OCTAVE.iter().position(|x| x == note) {
        Some(res) => res,
        None => 0, // TODO: fix this
    };

    // need to loop back around by 12 so
    let interval_index = (root_index + interval as usize) % 12;

    return match OCTAVE.get(interval_index) {
        Some(res) => res,
        None => &Note::A, // TODO: fix this
    };
}

// find what interval a note is from root
// count how many semitones we need to get to the note, looping around
pub fn find_interval(root: &Note, note: &Note) -> Interval {
    // we could use the integer values of the note enum, but feels more extensible to use the ordering in the octave array in this module
    // we can find the integer position of the root, integer position of the note
    let root_pos = OCTAVE
        .into_iter()
        .find_position(|e| *e == *root)
        .expect("NOTE NOT PRESENT IN OCTAVE")
        .0;

    let mut note_pos = OCTAVE
        .into_iter()
        .find_position(|e| *e == *note)
        .expect("NOTE NOT PRESENT IN OCTAVE")
        .0;

    // TODO: look into this cheese
    // this is just for the lap around so a 9th is not a 2nd etc
    if note_pos < root_pos {
        note_pos = note_pos + 12;
    }

    let semitones = note_pos - root_pos;

    return Interval::from(semitones);
}

#[cfg(test)]
mod tests {
    use super::*;

    // a basic case that doesn't need loop around
    #[test]
    fn test_get_interval_normal_hop() {
        let root = Note::C;
        let interval = Interval::MajorThird;

        let ret = get_interval(&root, interval);

        assert_eq!(*ret, Note::E);
    }

    // test the circular nature of the intervals
    #[test]
    fn test_get_interval_lap_around() {
        let root = Note::G;
        let interval = Interval::PerfectFifth;

        let ret = get_interval(&root, interval);

        assert_eq!(*ret, Note::D);
    }

    #[test]
    fn test_get_interval_lap_around_check_minor7th() {
        let root = Note::G;
        let interval = Interval::MinorSeventh;

        let ret = get_interval(&root, interval);

        assert_eq!(*ret, Note::F);
    }
}
