use itertools::Itertools;
use regex::Regex;
use std::{fmt, str::FromStr};

use crate::theory::{
    self,
    error::ChordParseError,
    interval::{find_interval, get_interval, Interval},
    note::Note,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SuspendedType {
    Sus2,
    Sus4,
}
// https://en.wikipedia.org/wiki/Chord_notation#Chord_quality
// https://musictheory.pugetsound.edu/mt21c/TriadsIntroduction.html
// worth noting name base starts with the chord quality which is based on traid quality
#[derive(Debug, PartialEq, Eq, Default, Clone, Copy)]
pub enum ChordQuality {
    #[default]
    Major, // triad quality
    Minor,      // triad quality
    Diminished, // triad quality
    Augmented,  // triad quality
    Suspended(SuspendedType),
    Dominant,
    Ambiguous,
}
impl FromStr for ChordQuality {
    type Err = ChordParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "m" => Ok(ChordQuality::Minor),
            "dim" => Ok(ChordQuality::Diminished),
            "aug" => Ok(ChordQuality::Augmented),
            "sus2" => Ok(ChordQuality::Suspended(SuspendedType::Sus2)),
            "sus4" => Ok(ChordQuality::Suspended(SuspendedType::Sus4)),
            _ => Err(ChordParseError::InvalidChordName(
                "invalid chord name".to_string(),
            )),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Default)]
pub enum TriadQuality {
    #[default]
    Major,
    Minor,
    Diminished,
    Augmented,
    Ambiguous,
}

impl From<ChordQuality> for TriadQuality {
    fn from(chord_quality: ChordQuality) -> Self {
        match chord_quality {
            ChordQuality::Major | ChordQuality::Dominant => TriadQuality::Major,
            ChordQuality::Minor => TriadQuality::Minor,
            ChordQuality::Diminished => TriadQuality::Diminished,
            ChordQuality::Augmented => TriadQuality::Augmented,
            ChordQuality::Suspended(..) | ChordQuality::Ambiguous => TriadQuality::Ambiguous,
        }
    }
}

// TODO: maybe want like secondary quality or something to hold 7s, 9s etc
// TODO: maybe there's a better way than duping this for chord and triat
impl fmt::Display for ChordQuality {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ChordQuality::Major | ChordQuality::Dominant => write!(f, ""),
            ChordQuality::Minor => write!(f, "m"),
            ChordQuality::Diminished => write!(f, "dim"),
            ChordQuality::Augmented => write!(f, "aug"),

            ChordQuality::Suspended(suspended_type) => match suspended_type {
                SuspendedType::Sus2 => write!(f, "sus2"),
                SuspendedType::Sus4 => write!(f, "sus4"),
            },

            ChordQuality::Ambiguous => write!(f, "!-ambiguous-!"),
        }
    }
}
impl fmt::Display for TriadQuality {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TriadQuality::Major => write!(f, ""),
            TriadQuality::Minor => write!(f, "m"),
            TriadQuality::Diminished => write!(f, "dim"),
            TriadQuality::Augmented => write!(f, "aug"),
            TriadQuality::Ambiguous => write!(f, "ambiguous"),
        }
    }
}

// https://en.wikipedia.org/wiki/Chord_notation
#[derive(Debug)]
pub struct Chord {
    pub name: String,
    pub root: Note,
    pub notes: Vec<Note>,
    pub triad_quality: TriadQuality,
    pub chord_quality: ChordQuality,
    pub intervals: Vec<theory::interval::Interval>,
}

impl fmt::Display for Chord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.root, self.triad_quality)
    }
}

impl Chord {
    pub fn list_notes_in_chord(&self) {
        println!("Notes in chord: {}", self.notes.iter().format(", "));
    }

    pub fn builder() -> ChordBuilder {
        ChordBuilder::default()
    }

    // TODO: look into from_x cases
}

impl From<ChordQuality> for Vec<Interval> {
    fn from(chord_quality: ChordQuality) -> Self {
        match chord_quality {
            ChordQuality::Minor => vec![Interval::MinorThird, Interval::PerfectFifth],
            ChordQuality::Major => vec![Interval::MajorThird, Interval::PerfectFifth],
            ChordQuality::Diminished => vec![Interval::MinorThird, Interval::DiminishedFifth],
            ChordQuality::Augmented => vec![Interval::MajorThird, Interval::AugmentedFifth],
            ChordQuality::Suspended(suspended_type) => match suspended_type {
                SuspendedType::Sus2 => vec![Interval::MajorSecond, Interval::PerfectFifth],
                SuspendedType::Sus4 => vec![Interval::PerfectFourth, Interval::PerfectFifth],
            },
            _ => vec![],
        }
    }
}

#[derive(Default)]
pub struct ChordBuilder {
    name: String,
    root: Note,
    notes: Vec<Note>,
    intervals: Vec<Interval>,
    triad_quality: TriadQuality,
    chord_quality: ChordQuality,
}

impl ChordBuilder {
    pub fn new() -> ChordBuilder {
        ChordBuilder {
            name: String::from("empty"),
            root: Note::default(),
            notes: Vec::new(),
            triad_quality: TriadQuality::default(),
            chord_quality: ChordQuality::default(),
            intervals: Vec::new(),
        }
    }

    pub fn root(mut self, root: Note) -> ChordBuilder {
        self.root = root;
        self
    }

    pub fn notes(mut self, notes: Vec<Note>) -> ChordBuilder {
        self.notes = notes;
        self
    }

    pub fn triad_quality(mut self, triad_quality: TriadQuality) -> ChordBuilder {
        self.triad_quality = triad_quality;
        self
    }

    pub fn chord_quality(mut self, chord_quality: ChordQuality) -> ChordBuilder {
        self.chord_quality = chord_quality;
        self
    }

    pub fn intervals(mut self, intervals: Vec<Interval>) -> ChordBuilder {
        self.intervals = intervals;
        self
    }

    pub fn name(mut self, name: String) -> ChordBuilder {
        self.name = name;
        self
    }

    pub fn build(self) -> Chord {
        Chord {
            name: self.name,
            root: self.root,
            notes: self.notes,
            intervals: self.intervals,
            triad_quality: self.triad_quality,
            chord_quality: self.chord_quality,
        }
    }
}

// TODO: validate that notes contains root in first position
// TODO: validate that notes are unique or filter them to make them so
// TODO: validate that there are at least 2 notes - i only know of power chords with 2 for now though

// account for _all_ intervals present in the chord. I think this needs to be exhaustive or we'll run into issues with dangling notes later
fn find_all_intervals_from_root_and_notes(root: &Note, notes: Vec<Note>) -> Vec<Interval> {
    // go through each note finding what interval it is
    let intervals = notes
        .iter()
        .skip(1)
        .map(|n| find_interval(root, &n))
        .collect();

    return intervals;
}

// take list of notes, a root, work out whether it could be major, minor, dim, sus, aug
// once we have the start, we can check later if there's a 7th or other add
// for now it just picks from major, minor, diminished, aug...
pub fn derive_chord_quality_from_intervals(intervals: &Vec<Interval>) -> ChordQuality {
    // TODO: ignore power chords for now

    // https://musictheory.pugetsound.edu/mt21c/TriadsIntroduction.html
    // from above we know there are 4 qualities of triads - augmented, major, minor and diminished
    // if it's got a minor third, it's either minor or diminished - diminished just meaning the diminished 5th
    // if it's got a major third, it's either major or augmented - augmented just meaning the raised 5th
    // gist being we can 'modify' the fifth into augmented or diminished -> augmented or diminished chord

    let has_minor_third = intervals.contains(&Interval::MinorThird);
    let has_major_third = intervals.contains(&Interval::MajorThird);
    let has_diminished_fifth = intervals.contains(&Interval::DiminishedFifth);
    let has_perfect_fifth = intervals.contains(&Interval::PerfectFifth);
    let has_augmented_fifth = intervals.contains(&Interval::AugmentedFifth);

    match (has_minor_third, has_major_third) {
        (true, true) => return ChordQuality::Ambiguous,
        (false, false) => {
            // if it's not got minor or major 3rd it's either ambiguous or a suspended chord
            if !has_perfect_fifth {
                return ChordQuality::Ambiguous;
            };

            let has_second = intervals.contains(&Interval::MajorSecond);
            let has_fourth = intervals.contains(&Interval::MajorSecond);

            if has_second && has_fourth {
                return ChordQuality::Ambiguous;
            };

            if has_second {
                return ChordQuality::Suspended(SuspendedType::Sus2);
            }

            if has_fourth {
                return ChordQuality::Suspended(SuspendedType::Sus4);
            }

            return ChordQuality::Ambiguous;
        }
        (true, false) => {
            if has_perfect_fifth {
                return ChordQuality::Minor;
            }
            if has_diminished_fifth && !has_augmented_fifth {
                return ChordQuality::Diminished;
            }

            return ChordQuality::Ambiguous;
        }
        (false, true) => {
            if has_perfect_fifth {
                return ChordQuality::Major;
            }
            if has_augmented_fifth && !has_diminished_fifth {
                return ChordQuality::Augmented;
            }

            // TODO: might not need this and copy above, might just be able to fall through to Ambiguous
            return ChordQuality::Ambiguous;
        }
    }

    // return ChordQuality::Ambiguous;
}

// take a note as a root, take some notes, work out what chord it could be
pub fn identify_from_root_and_notes(root: &Note, notes: &Vec<Note>) -> Chord {
    let chord_builder = ChordBuilder::new();

    let intervals = find_all_intervals_from_root_and_notes(root, notes.clone());

    // identify chord quality, gives us a foundation for naming
    let chord_quality = derive_chord_quality_from_intervals(&intervals);

    // TODO: maybe move this to function later
    let chord_name = match chord_quality {
        ChordQuality::Ambiguous => "Ambiguous".to_string(),
        ChordQuality::Minor => format!("{}m", root),
        ChordQuality::Major => format!("{}", root),
        ChordQuality::Diminished => format!("{}dim", root),
        ChordQuality::Augmented => format!("{}aug", root),
        ChordQuality::Dominant => format!("{}", root),
        ChordQuality::Suspended(SuspendedType::Sus2) => format!("{}sus2", root),
        ChordQuality::Suspended(SuspendedType::Sus4) => format!("{}sus4", root),
    };

    chord_builder
        .root(*root)
        .name(chord_name)
        // .notes(notes) TODO::notes on
        .intervals(intervals)
        .chord_quality(chord_quality)
        .triad_quality(TriadQuality::from(chord_quality))
        .build()
}

// TODO: need better naming than indentify_x
// maybe pub fn from_name ?
pub fn identify_from_name(chord_name: String) -> Result<Chord, ChordParseError> {
    // TODO: seems like diologuer has options for adding validators so try split validation and move there

    // sharps before normals so we don't pick up only note
    let root_re = Regex::new(r"(A#|A|B|C#|C|D#|D|E|F#|F|G#|G)").unwrap();

    let root = match root_re.find(&chord_name) {
        Some(mat) => match Note::from_str(mat.as_str()) {
            Ok(n) => n,
            Err(e) => {
                return Err(ChordParseError::InvalidChordName(
                    "couldn't identify root note in string".to_string(),
                ))
            }
        },
        None => {
            return Err(ChordParseError::InvalidChordName(
                "couldn't identify root note in string".to_string(),
            ))
        }
    };

    let chord_quality_re = Regex::new(r"(m|dim|aug|sus2|sus4)").unwrap();

    let chord_quality = match chord_quality_re.find(&chord_name) {
        Some(chord_quality_match) => {
            let str = chord_quality_match.as_str();
            match ChordQuality::from_str(str) {
                Ok(c) => c,
                Err(_) => {
                    return Err(ChordParseError::InvalidChordName(
                        "couldn't identify root note in string".to_string(),
                    ))
                }
            }
        }
        None => {
            // if there's no chord quality subscript showing up in the name, it would have to be a major triad
            // => this chord has a major triad quality we don't know about 7s, 9s etc yet so this would have to be a major triad
            ChordQuality::Major
        }
    };

    let triad_quality = TriadQuality::from(chord_quality);
    let intervals: Vec<Interval> = Vec::from(chord_quality);
    let notes: Vec<Note> = std::iter::once(&root)
        .chain(intervals.iter().map(|i| get_interval(&root, i.clone())))
        .cloned()
        .collect();

    Ok(ChordBuilder::new()
        .name(chord_name)
        .root(root)
        .intervals(intervals)
        .notes(notes)
        .chord_quality(chord_quality)
        .triad_quality(triad_quality)
        .build())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_derive_chord_quality_from_intervals_major_triad_pure() {
        let intervals = vec![Interval::MajorThird, Interval::PerfectFifth];

        let ret = derive_chord_quality_from_intervals(&intervals);

        assert_eq!(ret, ChordQuality::Major);
    }

    #[test]
    fn test_derive_chord_quality_from_intervals_major_triad_all_fifths() {
        let intervals = vec![
            Interval::MajorThird,
            Interval::PerfectFifth,
            Interval::DiminishedFifth,
            Interval::AugmentedFifth,
        ];

        let ret = derive_chord_quality_from_intervals(&intervals);

        assert_eq!(ret, ChordQuality::Major);
    }

    #[test]
    fn test_identify_from_name() {
        let ret = identify_from_name("Gsus2".to_string()).expect("hmm");
        assert_eq!(ret.name, "Gsus2");
        assert_eq!(
            ret.chord_quality,
            ChordQuality::Suspended(SuspendedType::Sus2)
        );
        assert_eq!(ret.notes, vec![Note::G, Note::A, Note::D]);
        assert_eq!(ret.triad_quality, TriadQuality::Ambiguous);
        assert_eq!(ret.root, Note::G);
        assert_eq!(
            ret.intervals,
            vec![Interval::MajorSecond, Interval::PerfectFifth]
        )
    }
}
