use itertools::Itertools;
use regex::Regex;
use std::{
    fmt::{self, format, write},
    str::FromStr,
};

use crate::theory::{
    self,
    chord::{
        derive_chord_quality_from_intervals, find_all_intervals_from_root_and_notes,
        get_add_interval_from_add, get_notes_from_root_and_intervals, Chord, ChordBuilder,
        ChordQuality, SeventhType, SuspendedType, TriadQuality,
    },
    error::ChordParseError,
    interval::{find_interval, get_interval, Interval},
    note::Note,
};

pub fn parse_chord_quality(s: &str) -> Result<ChordQuality, ChordParseError> {
    match s {
        "m" => Ok(ChordQuality::Minor),
        "dim" => Ok(ChordQuality::Diminished),
        "aug" => Ok(ChordQuality::Augmented),
        "sus2" => Ok(ChordQuality::Suspended(SuspendedType::Sus2)),
        "sus4" => Ok(ChordQuality::Suspended(SuspendedType::Sus4)),
        "aug7" => Ok(ChordQuality::Seventh(SeventhType::Augmented)),
        "m7" => Ok(ChordQuality::Seventh(SeventhType::Minor)),
        "7" => Ok(ChordQuality::Seventh(SeventhType::Dominant)),

        //(DominantType::Seventh)),
        // "9" => Ok(ChordQuality::Dominant(DominantType::Ninth)),
        _ => Err(ChordParseError::InvalidChordName(
            "invalid chord name".to_string(),
        )),
    }
}
// }

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
        ChordQuality::Seventh(seventh_type) => match seventh_type {
            SeventhType::Augmented => format!("{}aug7", root),
            SeventhType::Major => format!("{}maj7", root),
            SeventhType::HalfDiminished => format!("{}○7", root),
            SeventhType::Minor => format!("{}m7", root),
            SeventhType::Diminished => format!("{}dim7", root),
            SeventhType::Dominant => format!("{}7", root),
            SeventhType::Suspended(suspended_type) => match suspended_type {
                SuspendedType::Sus2 => format!("{}7sus2", root),
                SuspendedType::Sus4 => format!("{}7sus4", root),
            },
        },

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

// TODO: need better naming than identify_x
// maybe pub fn from_name ?
// TODO: clean up pulling from name so that no part of string is left unaccounted for
// that way can reject unrecognized features
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

    // TODO: refactor cleaner
    let chord_quality_re = Regex::new(r"(dim|m|aug|sus2|sus4)").unwrap();

    let mut chord_quality = match chord_quality_re.find(&chord_name) {
        Some(chord_quality_match) => {
            let str = chord_quality_match.as_str();
            match parse_chord_quality(str) {
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

    let mut intervals: Vec<Interval> = Vec::from(chord_quality);

    // sharps before for priority match
    // now we have base qualities aug, sus etc from above
    // we try to enrich with 7th quality
    // the regex below will catch all 7, 9, 11s => catches all 7 variations
    // TODO: ^ for string start but watch Xm and Xaug7
    let extension_quality_re =
        Regex::new(r"(aug|dim|C#|C|D#|D|E|F#|F|G#|G|A#|A|B|m)(7|9|11)").unwrap();
    // TODO: loop over all to catch things like G7dim9
    chord_quality = match extension_quality_re.captures(&chord_name) {
        Some(extension_captures) => {
            // TODO: clean up, feels weird to be putting notes here
            // if we just hang on chord quality here we'll miss the things like G7dim9, Gdim9
            match &extension_captures[2] {
                "7" => match chord_quality {
                    // fully diminished needs diminished 7th
                    ChordQuality::Diminished => {
                        intervals.push(Interval::DiminishedSeventh);
                    }

                    _ => intervals.push(Interval::MinorSeventh),
                },
                // TODO: it might be that 9s/11s can take a modifier like G7aug9, look into this
                // for now A gdim9 is treated like a Gdim7add9
                "9" => {
                    match chord_quality {
                        ChordQuality::Diminished => {
                            intervals.push(Interval::DiminishedSeventh);
                        }

                        _ => intervals.push(Interval::MinorSeventh),
                    }

                    intervals.push(Interval::MajorNinth);
                }
                "11" => {
                    match chord_quality {
                        ChordQuality::Diminished => {
                            intervals.push(Interval::DiminishedSeventh);
                            intervals.push(Interval::MinorNinth);
                        }

                        _ => {
                            intervals.push(Interval::MinorSeventh);
                            intervals.push(Interval::MajorNinth);
                        }
                    }

                    intervals.push(Interval::PerfectEleventh);
                }
                _ => {}
            };

            // make sure intervals are unique
            intervals.dedup();

            // if there's an extension, the chord quality is affected
            // TODO: might be cleaner to just recalc quality on intervals here instead
            match chord_quality {
                ChordQuality::Suspended(suspended_type) => {
                    ChordQuality::Seventh(SeventhType::Suspended(suspended_type))
                }
                ChordQuality::Minor => ChordQuality::Seventh(SeventhType::Minor),
                ChordQuality::Major => ChordQuality::Seventh(SeventhType::Major),
                ChordQuality::Diminished => ChordQuality::Seventh(SeventhType::Diminished),
                ChordQuality::Augmented => ChordQuality::Seventh(SeventhType::Augmented),

                // TODO: rest of these after
                // ChordQuality::Diminished=> ChordQuality::Diminished,
                _ => chord_quality,
            }
        }
        None => chord_quality,
    };

    let triad_quality = TriadQuality::from(chord_quality);

    // TODO: maybe this should come before chord quality because adds might be just adding a -
    // minor 7th for example which makes it a dominant if it's a major triad quality
    // TODO: allow more adds
    // matches certain numbers found after add
    // rust regex doesn't have look before
    let add_re = Regex::new(r"(add)(7|9|11)").unwrap();
    let add_degree = match add_re.captures(&chord_name) {
        Some(add_captures) => match get_add_interval_from_add(&add_captures[2]) {
            Interval::Unknown => None,
            interval => Some(interval),
        },
        None => None,
    };

    match add_degree {
        Some(interval) => {
            // with another interval we might be changing the chord quality
            // an example of this is typing Gadd7 (G major triad added 7th(minor)) => G7 dominant chord
            // if it's 'normal' 7 we'll have the 7th from above
            if !intervals.contains(&interval) {
                intervals.push(interval);
                chord_quality = derive_chord_quality_from_intervals(&intervals);
            }
        }
        None => {}
    }

    let notes = get_notes_from_root_and_intervals(&root, &intervals);

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

    //
    // identify_from_root_and_notes
    //

    #[test]
    fn test_identify_from_root_and_notes_complex_gm11() {
        let root = Note::G;
        let notes = vec![Note::G, Note::As, Note::D, Note::F, Note::A, Note::C];

        let ret = identify_from_root_and_notes(&root, &notes);

        assert_eq!(ret.triad_quality, TriadQuality::Minor);
        assert_eq!(ret.chord_quality, ChordQuality::Seventh(SeventhType::Minor));
        assert_eq!(
            ret.intervals,
            vec![
                Interval::MinorThird,
                Interval::PerfectFifth,
                Interval::MinorSeventh,
                Interval::MajorNinth,
                Interval::PerfectEleventh
            ]
        )
    }

    //
    // identify_chord_from_name
    //

    #[test]
    fn test_identify_from_name_gsus2() {
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

    #[test]
    fn test_identify_from_name_gm() {
        let ret = identify_from_name("Gm".to_string()).expect("hmm");
        assert_eq!(ret.name, "Gm");
        assert_eq!(ret.chord_quality, ChordQuality::Minor);
        assert_eq!(ret.notes, vec![Note::G, Note::As, Note::D]);
        assert_eq!(ret.triad_quality, TriadQuality::Minor);
        assert_eq!(ret.root, Note::G);
        assert_eq!(
            ret.intervals,
            vec![Interval::MinorThird, Interval::PerfectFifth]
        )
    }

    #[test]
    fn test_identify_from_name_gm7() {
        let ret = identify_from_name("Gm7".to_string()).expect("hmm");
        assert_eq!(ret.name, "Gm7");
        assert_eq!(ret.root, Note::G);
        assert_eq!(ret.chord_quality, ChordQuality::Seventh(SeventhType::Minor));
        assert_eq!(ret.triad_quality, TriadQuality::Minor);
        assert_eq!(
            ret.intervals,
            vec![
                Interval::MinorThird,
                Interval::PerfectFifth,
                Interval::MinorSeventh
            ],
        );
        assert_eq!(ret.notes, vec![Note::G, Note::As, Note::D, Note::F]);
    }

    #[test]
    fn test_identify_from_name_gaug7() {
        let ret = identify_from_name("Gaug7".to_string()).expect("hmm");
        assert_eq!(ret.name, "Gaug7");
        assert_eq!(ret.root, Note::G);
        assert_eq!(
            ret.chord_quality,
            ChordQuality::Seventh(SeventhType::Augmented)
        );
        assert_eq!(ret.triad_quality, TriadQuality::Augmented);
        assert_eq!(
            ret.intervals,
            vec![
                Interval::MajorThird,
                Interval::AugmentedFifth,
                Interval::MinorSeventh
            ],
        );
        assert_eq!(ret.notes, vec![Note::G, Note::B, Note::Ds, Note::F]);
    }

    #[test]
    fn test_identify_from_name_gdim7() {
        let ret = identify_from_name("Gdim7".to_string()).expect("hmm");
        assert_eq!(ret.name, "Gdim7");
        assert_eq!(ret.root, Note::G);
        assert_eq!(
            ret.chord_quality,
            ChordQuality::Seventh(SeventhType::Diminished)
        );
        assert_eq!(ret.triad_quality, TriadQuality::Diminished);
        assert_eq!(
            ret.intervals,
            vec![
                Interval::MinorThird,
                Interval::DiminishedFifth,
                Interval::DiminishedSeventh
            ],
        );
        assert_eq!(ret.notes, vec![Note::G, Note::As, Note::Cs, Note::E]);
    }

    #[test]
    fn test_identify_from_name_gadd7_coalesce_to_g7() {
        let ret = identify_from_name("Gadd7".to_string()).expect("hmm");
        assert_eq!(ret.name, "Gadd7");
        assert_eq!(ret.root, Note::G);
        assert_eq!(
            ret.chord_quality,
            ChordQuality::Seventh(SeventhType::Dominant)
        );
        assert_eq!(ret.triad_quality, TriadQuality::Major);
        assert_eq!(
            ret.intervals,
            vec![
                Interval::MajorThird,
                Interval::PerfectFifth,
                Interval::MinorSeventh
            ],
        );
        assert_eq!(ret.notes, vec![Note::G, Note::B, Note::D, Note::F]);
    }

    #[test]
    fn test_identify_complex_g7sus2() {
        let ret = identify_from_name("G7sus2".to_string()).expect("hmm");
        assert_eq!(ret.name, "G7sus2");
        assert_eq!(ret.root, Note::G);
        assert_eq!(
            ret.chord_quality,
            ChordQuality::Seventh(SeventhType::Suspended(SuspendedType::Sus2))
        );
        assert_eq!(ret.triad_quality, TriadQuality::Ambiguous);
        assert_eq!(
            ret.intervals,
            vec![
                Interval::MajorSecond,
                Interval::PerfectFifth,
                Interval::MinorSeventh,
            ],
        );
        assert_eq!(ret.notes, vec![Note::G, Note::A, Note::D, Note::F]);
    }

    #[test]
    fn test_identify_complex_g7sus2add11() {
        let ret = identify_from_name("G7sus2add11".to_string()).expect("hmm");
        assert_eq!(ret.name, "G7sus2add11");
        assert_eq!(ret.root, Note::G);
        assert_eq!(
            ret.chord_quality,
            ChordQuality::Seventh(SeventhType::Suspended(SuspendedType::Sus2))
        );
        assert_eq!(ret.triad_quality, TriadQuality::Ambiguous);
        assert_eq!(
            ret.intervals,
            vec![
                Interval::MajorSecond,
                Interval::PerfectFifth,
                Interval::MinorSeventh,
                Interval::PerfectEleventh
            ],
        );

        // TODO: have code work odering for notes relative ot root, this is jank ordering
        assert_eq!(ret.notes, vec![Note::G, Note::A, Note::D, Note::F, Note::C]);
    }

    // it's worth noting that the naming works like: take the last interval before the number and diminish, or augment the 'chain'
    // so a gdim9 means diminished 7th, a gdim11 means a diminished 7th and 9th.
    #[test]
    fn test_identify_higher_extension_with_diminished_gdim11() {
        let ret = identify_from_name("Gdim11".to_string()).expect("hmm");
        assert_eq!(ret.name, "Gdim11");
        assert_eq!(ret.root, Note::G);
        assert_eq!(
            ret.chord_quality,
            ChordQuality::Seventh(SeventhType::Diminished)
        );
        assert_eq!(ret.triad_quality, TriadQuality::Diminished);
        assert_eq!(
            ret.intervals,
            vec![
                Interval::MinorThird,
                Interval::DiminishedFifth,
                Interval::DiminishedSeventh,
                Interval::MinorNinth,
                Interval::PerfectEleventh
            ],
        );

        assert_eq!(
            ret.notes,
            vec![Note::G, Note::As, Note::Cs, Note::E, Note::Gs, Note::C]
        );
    }

    #[test]
    fn test_identify_higher_extension_with_augmented_gaug11() {
        let ret = identify_from_name("Gaug11".to_string()).expect("hmm");
        assert_eq!(ret.name, "Gaug11");
        assert_eq!(ret.root, Note::G);
        assert_eq!(
            ret.chord_quality,
            ChordQuality::Seventh(SeventhType::Augmented)
        );
        assert_eq!(ret.triad_quality, TriadQuality::Augmented);
        assert_eq!(
            ret.intervals,
            vec![
                Interval::MajorThird,
                Interval::AugmentedFifth,
                Interval::MinorSeventh,
                Interval::MajorNinth,
                Interval::PerfectEleventh
            ],
        );

        assert_eq!(
            ret.notes,
            vec![Note::G, Note::B, Note::Ds, Note::F, Note::A, Note::C]
        );
    }
}
