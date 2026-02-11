use itertools::Itertools;
use regex::Regex;
use std::{
    fmt::{self, format, write},
    str::FromStr,
};

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

// TODO: half diminished etc
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SeventhType {
    Minor,
    Major,
    Dominant,
    Augmented,
    HalfDiminished,
    Diminished,
    Suspended(SuspendedType),
}

// https://en.wikipedia.org/wiki/Chord_notation#Chord_quality
// https://musictheory.pugetsound.edu/mt21c/TriadsIntroduction.html
// worth noting name base starts with the chord quality which is based on triad quality
// the quality of the chord is determined only up to the 7th. After that these are 'pure extensions' that don't change the quality of the chord.
// a 7th can be considered an extension, but it still impacts chord quality
// TODO: look into this
#[derive(Debug, PartialEq, Eq, Default, Clone, Copy)]
pub enum ChordQuality {
    #[default]
    Major, // triad quality
    Minor,      // triad quality
    Diminished, // triad quality
    Augmented,  // triad quality
    Suspended(SuspendedType),
    Seventh(SeventhType),
    Ambiguous,
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
            ChordQuality::Major | ChordQuality::Seventh(SeventhType::Dominant) => {
                TriadQuality::Major
            }
            ChordQuality::Minor | ChordQuality::Seventh(SeventhType::Minor) => TriadQuality::Minor,
            ChordQuality::Diminished => TriadQuality::Diminished,
            ChordQuality::Augmented | ChordQuality::Seventh(SeventhType::Augmented) => {
                TriadQuality::Augmented
            }
            ChordQuality::Suspended(..) | ChordQuality::Ambiguous => TriadQuality::Ambiguous,
            ChordQuality::Seventh(seventh_type) => match seventh_type {
                SeventhType::Augmented => TriadQuality::Augmented,
                SeventhType::Diminished | SeventhType::HalfDiminished => TriadQuality::Diminished,
                SeventhType::Major | SeventhType::Dominant => TriadQuality::Major,
                SeventhType::Minor => TriadQuality::Minor,
                SeventhType::Suspended(..) => TriadQuality::Ambiguous,
            },
        }
    }
}

// TODO: maybe want like secondary quality or something to hold 7s, 9s etc
// TODO: maybe there's a better way than duping this for chord and triat
impl fmt::Display for ChordQuality {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ChordQuality::Major => write!(f, "Major"),
            ChordQuality::Minor => write!(f, "Minor"),
            ChordQuality::Diminished => write!(f, "Diminished"),
            ChordQuality::Augmented => write!(f, "Augmented"),

            // TODO: 7, 9, 11 that are in stacked thirds are just X9
            // ChordQuality::Dominant => write!(f, "Dominant"),
            // ChordQuality::MinorSeventh => write!(f, "Minor Seventh"),
            // ChordQuality::AugmentedSeventh => write!(f, "Augmented Seventh"),
            // ChordQuality::SuspendedSeventh(..) => write!(f, "Suspended Seventh"), // TODO: diff 2 4 ?
            ChordQuality::Seventh(seventh_type) => match seventh_type {
                SeventhType::Augmented => write!(f, "Augmented 7th"),
                SeventhType::Diminished => write!(f, "Diminished 7th"),
                SeventhType::Major => write!(f, "Major 7th"),
                SeventhType::Minor => write!(f, "Minor 7th"),
                SeventhType::HalfDiminished => write!(f, "Half Diminished 7th"),
                SeventhType::Dominant => write!(f, "Dominant 7th"),
                SeventhType::Suspended(suspended_type) => match suspended_type {
                    SuspendedType::Sus2 => write!(f, "Dominant 7th Suspended 2nd"),
                    SuspendedType::Sus4 => write!(f, "Dominant 7th Suspended 4th"),
                },
            },

            ChordQuality::Suspended(suspended_type) => match suspended_type {
                SuspendedType::Sus2 => write!(f, "Suspended Second"),
                SuspendedType::Sus4 => write!(f, "Suspended Fourth"),
            },

            ChordQuality::Ambiguous => write!(f, "Ambiguous"),
        }
    }
}
impl fmt::Display for TriadQuality {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TriadQuality::Major => write!(f, "Major"),
            TriadQuality::Minor => write!(f, "Mior"),
            TriadQuality::Diminished => write!(f, "Diminished"),
            TriadQuality::Augmented => write!(f, "Augmened"),
            TriadQuality::Ambiguous => write!(f, "Ambiguous"),
        }
    }
}

#[derive(Debug)]
pub enum AddInterval {
    Interval(Interval),
    None,
}

// https://en.wikipedia.org/wiki/Chord_notation
#[derive(Debug)]
pub struct Chord {
    pub name: String,
    pub root: Note,
    pub notes: Vec<Note>,
    pub triad_quality: TriadQuality,
    pub chord_quality: ChordQuality,
    pub add_degree: Option<AddInterval>,
    pub intervals: Vec<theory::interval::Interval>,
}

impl fmt::Display for Chord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Information on chord {}\nRoot: {}\nChord quality: {}\nTriad quality: {:?}\nIntervals: {:?}\nNotes: {:?}",
            self.name,
            self.root,
            self.chord_quality,
            self.triad_quality,
            // TODO: fix print formatting
            self.intervals.iter().format(", "),
            self.notes.iter().format(", ")
        )
    }
}

impl From<ChordQuality> for Vec<Interval> {
    fn from(chord_quality: ChordQuality) -> Self {
        match chord_quality {
            ChordQuality::Minor => vec![Interval::MinorThird, Interval::PerfectFifth],
            ChordQuality::Major => vec![Interval::MajorThird, Interval::PerfectFifth],
            ChordQuality::Diminished => vec![Interval::MinorThird, Interval::DiminishedFifth],
            ChordQuality::Augmented => vec![Interval::MajorThird, Interval::AugmentedFifth],

            ChordQuality::Seventh(seventh_type) => match seventh_type {
                SeventhType::Dominant => vec![
                    Interval::MajorThird,
                    Interval::PerfectFifth,
                    Interval::DiminishedSeventh,
                ],
                SeventhType::Augmented => vec![
                    Interval::MajorThird,
                    Interval::AugmentedFifth,
                    Interval::MinorSeventh,
                ],
                SeventhType::Diminished => vec![
                    Interval::MinorThird,
                    Interval::DiminishedFifth,
                    Interval::DiminishedSeventh,
                ],

                SeventhType::HalfDiminished => vec![
                    Interval::MinorThird,
                    Interval::DiminishedFifth,
                    Interval::MinorSeventh,
                ],

                SeventhType::Minor => vec![
                    Interval::MinorThird,
                    Interval::PerfectFifth,
                    Interval::MinorSeventh,
                ],
                SeventhType::Major => vec![
                    Interval::MajorThird,
                    Interval::PerfectFifth,
                    Interval::MinorSeventh,
                ],

                SeventhType::Suspended(suspended_type) => match suspended_type {
                    SuspendedType::Sus2 => vec![
                        Interval::MajorSecond,
                        Interval::PerfectFifth,
                        Interval::MinorSeventh,
                    ],
                    SuspendedType::Sus4 => vec![
                        Interval::MajorSecond,
                        Interval::PerfectFifth,
                        Interval::MinorSeventh,
                    ],
                },
            },

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
    add_degree: Option<AddInterval>,
    triad_quality: TriadQuality,
    chord_quality: ChordQuality,
}

impl ChordBuilder {
    pub fn new() -> ChordBuilder {
        ChordBuilder {
            name: String::from("empty"),
            root: Note::default(),
            notes: Vec::new(),
            add_degree: None,
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
            add_degree: self.add_degree,
        }
    }
}

// TODO: validate that notes contains root in first position
// TODO: validate that notes are unique or filter them to make them so
// TODO: validate that there are at least 2 notes - i only know of power chords with 2 for now though

// account for _all_ intervals present in the chord. I think this needs to be exhaustive or we'll run into issues with dangling notes later
// we have an issue of deciding between a 9th and a 2nd as we don't have the ability to check the octave of the note.
// TODO: clean this up
// but for now we'll rely on the order of the notes given to infer the octave, as in if the semitones before are greater than the one we're on, it's an octave shift.
// e.g. if the 2nd interval is preceeded by any fifth or 7th -> it's not a 2nd, it's a ninth
pub fn find_all_intervals_from_root_and_notes(root: &Note, notes: Vec<Note>) -> Vec<Interval> {
    // go through each note finding what interval it is
    let mut intervals: Vec<Interval> = notes
        .iter()
        .skip(1)
        .map(|n| find_interval(root, &n))
        .collect();

    // cheese to make sure 2nd, 4th is correctly reassigned to 9, 11
    // find the index where the intervals are going down i.e. 5th to a 2nd
    // tells us we need octave shift for rest
    if intervals.len() >= 2 {
        let mut shift_index = 0;

        for (i, e) in intervals.iter().skip(1).enumerate() {
            if e < intervals.get(i + 1 - 1).expect("TODO: if less than 2") {
                shift_index = i + 1;
            }
        }

        if shift_index > 0 {
            for i in shift_index..intervals.len() {
                intervals[i] = Interval::from(intervals[i] as usize + 12)
            }
        }
    }

    intervals.dedup();
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
    // for 7th chords, the 5th can be omitted

    let has_second = intervals.contains(&Interval::MajorSecond);
    let has_fourth = intervals.contains(&Interval::PerfectFourth);
    let has_minor_third = intervals.contains(&Interval::MinorThird);
    let has_major_third = intervals.contains(&Interval::MajorThird);
    let has_diminished_fifth = intervals.contains(&Interval::DiminishedFifth);
    let has_perfect_fifth = intervals.contains(&Interval::PerfectFifth);
    let has_augmented_fifth = intervals.contains(&Interval::AugmentedFifth);
    let has_minor_seventh = intervals.contains(&Interval::MinorSeventh);

    // TODO: clean up this match maze
    match (has_minor_third, has_major_third) {
        (true, true) => return ChordQuality::Ambiguous,
        (false, false) => {
            // if no minor or major 3rd it's either suspended, an omited 5th 7, or ambiguous
            if !has_perfect_fifth {
                return ChordQuality::Ambiguous;
            };

            if has_second && has_fourth {
                return ChordQuality::Ambiguous;
            };

            if has_second {
                if has_minor_seventh {
                    return ChordQuality::Seventh(SeventhType::Suspended(SuspendedType::Sus2));
                }

                return ChordQuality::Suspended(SuspendedType::Sus2);
            }

            if has_fourth {
                if has_minor_seventh {
                    return ChordQuality::Seventh(SeventhType::Suspended(SuspendedType::Sus4));
                }

                return ChordQuality::Suspended(SuspendedType::Sus4);
            }

            return ChordQuality::Ambiguous;
        }
        (true, false) => {
            if has_perfect_fifth {
                if has_minor_seventh {
                    return ChordQuality::Seventh(SeventhType::Minor);
                }

                return ChordQuality::Minor;
            } else if has_diminished_fifth && !has_augmented_fifth {
                if has_minor_seventh {
                    return ChordQuality::Seventh(SeventhType::HalfDiminished);
                }

                return ChordQuality::Diminished;
            }

            if has_minor_seventh {
                return ChordQuality::Seventh(SeventhType::Minor);
            }

            return ChordQuality::Ambiguous;
        }
        (false, true) => {
            if has_perfect_fifth {
                if has_minor_seventh {
                    return ChordQuality::Seventh(SeventhType::Dominant);
                }

                return ChordQuality::Major;
            } else if has_augmented_fifth && !has_diminished_fifth {
                if has_minor_seventh {
                    ChordQuality::Seventh(SeventhType::Augmented);
                }

                return ChordQuality::Augmented;
            }

            if has_minor_seventh {
                return ChordQuality::Seventh(SeventhType::Dominant);
            }

            return ChordQuality::Ambiguous;
        }
    }

    // return ChordQuality::Ambiguous;
}

// TODO: look into whether we need triad quality, look into generating scale as context for intervals
pub fn get_add_interval_from_add(add_str: &str) -> Interval {
    match add_str {
        // TODO: need more of these
        "7" => Interval::MinorSeventh,
        "9" => Interval::MajorNinth,
        "11" => Interval::PerfectEleventh,
        _ => Interval::Unknown, // TODO: look into this
    }
}

pub fn get_notes_from_root_and_intervals(root: &Note, intervals: &Vec<Interval>) -> Vec<Note> {
    std::iter::once(root)
        .chain(intervals.iter().map(|i| get_interval(&root, i.clone())))
        .cloned()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    //
    // find_all_intervals_from_root_and_notes
    //

    #[test]
    fn test_find_all_intervals_from_root_and_notes_gm11() {
        // Gm11
        let root = Note::G;
        let notes = vec![root, Note::As, Note::D, Note::F, Note::A, Note::C];

        let ret = find_all_intervals_from_root_and_notes(&root, notes);

        assert_eq!(
            ret,
            vec![
                Interval::MinorThird,
                Interval::PerfectFifth,
                Interval::MinorSeventh,
                Interval::MajorNinth,
                Interval::PerfectEleventh
            ]
        );
    }

    #[test]
    fn test_find_all_intervals_from_root_and_notes_gm11_missing_5th() {
        // Gdim11
        let root = Note::G;
        let notes = vec![root, Note::As, Note::F, Note::A, Note::C];

        let ret = find_all_intervals_from_root_and_notes(&root, notes);

        assert_eq!(
            ret,
            vec![
                Interval::MinorThird,
                Interval::MinorSeventh,
                Interval::MajorNinth,
                Interval::PerfectEleventh
            ]
        );
    }

    //
    // get_notes_from_root_and_intervals
    //
    #[test]
    fn test_get_notes_from_root_and_intervals() {
        // Gdim11
        let root = Note::G;
        let intervals = vec![
            Interval::MinorThird,
            Interval::DiminishedFifth,
            Interval::DiminishedSeventh,
            Interval::MinorNinth,
            Interval::PerfectEleventh,
        ];

        let ret = get_notes_from_root_and_intervals(&root, &intervals);

        assert_eq!(
            ret,
            vec![Note::G, Note::As, Note::Cs, Note::E, Note::Gs, Note::C]
        );
    }

    //
    // derive_chord_quality_from_intervals
    //

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
    fn test_derive_chord_quality_from_intervals_omitted_5th_7th() {
        let intervals = vec![Interval::MajorThird, Interval::MinorSeventh];

        let ret = derive_chord_quality_from_intervals(&intervals);

        assert_eq!(ret, ChordQuality::Seventh(SeventhType::Dominant));
    }
}
