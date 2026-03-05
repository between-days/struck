// this is kind of strange file, it's a joiner to relate notes together so naturally it's needs concepts of pitch class, octave etc

use core::fmt;
use itertools::Itertools;

use crate::theory::{note::Note, pitch_class};

pub const OCTAVE: [pitch_class::PitchClass; 12] = [
    pitch_class::PitchClass::C,
    pitch_class::PitchClass::Cs,
    pitch_class::PitchClass::D,
    pitch_class::PitchClass::Ds,
    pitch_class::PitchClass::E,
    pitch_class::PitchClass::F,
    pitch_class::PitchClass::Fs,
    pitch_class::PitchClass::G,
    pitch_class::PitchClass::Gs,
    pitch_class::PitchClass::A,
    pitch_class::PitchClass::As,
    pitch_class::PitchClass::B,
];

// number of semitone steps
// https://en.wikipedia.org/wiki/Interval_(music)
// names refer to chromatic scale positions so we don't need to worry about scales when finding chords intervals
#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd)]
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
    DiminishedNinth = 12,
    MinorNinth = 13,
    MajorNinth = 14,
    PerfectEleventh = 17,
    Unknown = 100, // TODO: cheese for now
}

impl From<i8> for Interval {
    fn from(value: i8) -> Self {
        match value {
            2 => Interval::MajorSecond,
            3 => Interval::MinorThird,
            4 => Interval::MajorThird,
            5 => Interval::PerfectFourth,
            6 => Interval::DiminishedFifth,
            7 => Interval::PerfectFifth,
            8 => Interval::AugmentedFifth,
            9 => Interval::DiminishedSeventh,
            10 => Interval::MinorSeventh,
            11 => Interval::Seventh,
            12 => Interval::DiminishedNinth,
            13 => Interval::MinorNinth,
            14 => Interval::MajorNinth,
            17 => Interval::PerfectEleventh,
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
            Interval::DiminishedNinth => write!(f, "Diminished 9th"),
            Interval::MinorNinth => write!(f, "Minor 9th"),
            Interval::MajorNinth => write!(f, "Minor 9th"),
            Interval::PerfectEleventh => write!(f, "Perfect 11th"),
            Interval::Unknown => write!(f, "Unknown"),
        }
    }
}

impl Note {
    // find what interval a note is from root
    // count how many semitones we need to get to the note, looping around
    // TODO: as mentioned in chord.rs, we have a conflict between 2nds and 9ths, this is currently handled in chord.rs by checking whether the last interval is more than the current in the loop
    // and we use that to get implied octaves up for 9, 11
    // this seems a little jank but it can wait
    // TODO: currently don't bother with compound intervals above the 11th
    pub fn find_interval_semis(&self, note: &Note) -> i8 {
        // we could use the integer values of the note enum, but feels more extensible to use the ordering in the octave array in this module
        // we can find the integer position of the root, integer position of the note
        let root_pos = OCTAVE
            .into_iter()
            .find_position(|e| *e == self.pitch_class)
            .expect("NOTE NOT PRESENT IN OCTAVE")
            .0 as i8;

        let note_pos = OCTAVE
            .into_iter()
            .find_position(|e| *e == note.pitch_class)
            .expect("NOTE NOT PRESENT IN OCTAVE")
            .0 as i8;

        // this is the difference within an octave
        let diff = note_pos - root_pos;

        // now factor in octave circularity
        let diff_full = diff + 12 * (note.pitch_octave as i8 - self.pitch_octave as i8) as i8;

        return diff_full;
    }

    // TODO: don't allow compound intervals above 11th
    pub fn find_interval(&self, note: &Note) -> Interval {
        Interval::from(self.find_interval_semis(note))
    }

    // get this many semitones above the root note
    pub fn get_interval(&self, interval: Interval) -> Note {
        // get where the root note is in octave
        let root_index = match OCTAVE.iter().position(|x| *x == self.pitch_class) {
            Some(res) => res,
            None => 0, // TODO: fix this
        };

        let a = root_index + interval as usize;

        // taking out the multiples of 12 leaves us with the correct pitch class index in the octave array
        // counting the 12s gives us the octave shift
        // this will be the new pitch class index
        let remainder_12 = a % 12;
        let number_of_12s = a / 12;

        let np = OCTAVE.get(remainder_12).expect("NOT IN ARRAY");

        return Note::new(*np, self.pitch_octave + number_of_12s as u8);
    }

    // TODO: feels wrong here
    // get the frequency in hertz for the note
    // frequency for notes is calculated with the following logic:
    // each octave is 12 times the last, each consecutive note is 2 root 12 factor difference, a4 is a clean 440hz in standard equal temperament
    pub fn get_frequency(&self) -> f32 {
        // f = 440 * 2^(n/12) where n is number of semitones from A4
        let a4 = Note::new(super::pitch_class::PitchClass::A, 4);

        let semis = a4.find_interval_semis(&self);

        let b: f32 = 2.0;
        return 440.0 * b.powf(semis as f32 / 12.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    //
    // get_frequency_for_note
    //

    #[test]
    fn test_get_c4() {
        let c4 = Note::new(crate::theory::pitch_class::PitchClass::C, 4);

        let ret = c4.get_frequency();

        assert_eq!(ret, 261.62555)
    }

    //
    // find_interval_semis
    //

    #[test]
    fn test_find_interval_semis_a4_c4() {
        let root = Note::new(pitch_class::PitchClass::A, 4);
        let note = Note::new(pitch_class::PitchClass::C, 4);

        let ret = root.find_interval_semis(&note);

        assert_eq!(ret, -9);
    }

    #[test]
    fn test_find_interval_semis_a4_c9() {
        let root = Note::new(pitch_class::PitchClass::A, 4);
        let note = Note::new(pitch_class::PitchClass::C, 9);

        let ret = root.find_interval_semis(&note);

        assert_eq!(ret, 51);
    }

    //
    // find_interval
    //

    #[test]
    fn test_find_interval_9th() {
        let root = Note::new(pitch_class::PitchClass::A, 4);
        let note = Note::new(pitch_class::PitchClass::B, 5);

        let ret = root.find_interval(&note);

        assert_eq!(ret, Interval::MajorNinth);
    }

    //
    // get_interval
    //

    // a basic case that doesn't need loop around
    #[test]
    fn test_get_interval_normal_hop() {
        let root = Note::new(pitch_class::PitchClass::A, 4);
        let interval = Interval::MajorThird;

        let ret = root.get_interval(interval);

        assert_eq!(ret, Note::new(pitch_class::PitchClass::Cs, 5));
    }

    // test the circular nature of the intervals
    #[test]
    fn test_get_interval_lap_around() {
        let root = Note::new(pitch_class::PitchClass::G, 4);
        let interval = Interval::PerfectFifth;

        let ret = root.get_interval(interval);

        assert_eq!(ret, Note::new(pitch_class::PitchClass::D, 5));
    }

    #[test]
    fn test_get_interval_lap_around_check_minor7th() {
        let root = Note::new(pitch_class::PitchClass::G, 4);
        let interval = Interval::MinorSeventh;

        let ret = root.get_interval(interval);

        assert_eq!(ret, Note::new(pitch_class::PitchClass::F, 5));
    }

    //
    // find_interval
    //

    #[test]
    fn test_find_interval_no_wrap() {
        let root = Note::new(pitch_class::PitchClass::C, 4);
        let note = Note::new(pitch_class::PitchClass::E, 4);

        let ret = root.find_interval(&note);

        assert_eq!(ret, Interval::MajorThird);
    }

    #[test]
    fn test_find_interval_lap_around_gminor7th() {
        let root = Note::new(pitch_class::PitchClass::G, 4);
        let note = Note::new(pitch_class::PitchClass::F, 5);

        let ret = root.find_interval(&note);

        assert_eq!(ret, Interval::MinorSeventh);
    }

    #[test]
    fn test_find_interval_lap_around_g9th() {
        let root = Note::new(pitch_class::PitchClass::G, 4);
        let note = Note::new(pitch_class::PitchClass::A, 4);

        let ret = root.find_interval(&note);

        assert_eq!(ret, Interval::MajorSecond);
    }
}
