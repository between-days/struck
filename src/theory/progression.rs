use std::ops::Not;

use crate::{
    parser::chord_parser,
    theory::{
        chord::Chord,
        error::ProgressionParseError,
        note::{assume_note_ordering, Note},
        pitch_class::{self, PitchClass},
        scale::{Key, Mode, Scale, ScaleDegree},
    },
};

// impl Key {

// }

pub struct ProgressionItem {
    pub scale_degree: ScaleDegree,
    pub chord: Chord,
}

pub struct Progression {
    // chords: Vec<Chord>
    pub progression_items: Vec<ProgressionItem>,
    pub key: Option<Key>,
}

impl Progression {
    // TODO: wonder about referencing the scale degrees, the chord quality etc
    // should this take I, II, III or something containing the desired quality as well such that we can technically go off key but -
    // chords will still hold there reference to the key scale degree
    pub fn from_key_and_scale_degrees(key: Key, scale_degrees: Vec<ScaleDegree>) -> Progression {
        let scale = Scale::from_key(key);

        // TODO: put this in scale.rs
        let mut notes: Vec<Note> = scale
            .pitch_classes
            .iter()
            .map(|pc| Note {
                pitch_class: *pc,
                pitch_octave: 4,
            })
            .collect();

        notes = assume_note_ordering(notes);

        // get default qualities for each scale degree and build a chord to sit in a progression item
        let progression_items = scale_degrees
            .iter()
            .map(|scale_degree| {
                // TODO: the notes vs pitch class -> decide the octave is getting sticky, focus on getting something clean and reusable for that
                let root = notes.get(*scale_degree as usize).expect("TODO:");
                // let chord_quality key.  ( *scale_degree);

                let chord_quality = key.mode.get_quality_at_scale_degree(*scale_degree);

                let chord = Chord::from_root_and_quality(root.clone(), chord_quality);

                ProgressionItem {
                    scale_degree: *scale_degree,
                    chord,
                }
            })
            .collect();

        return Progression {
            progression_items: progression_items,
            key: Some(key),
        };
    }

    // pub fn from_string_list(chords_str: &str) -> Result<Progression, ProgressionParseError> {
    //     let chords: Vec<Chord> = chords_str
    //         .trim()
    //         .split_terminator(" ")
    //         .map(|cs| chord_parser::identify_from_name(cs.to_string()))
    //         .collect();

    //     // TODO: we don't know the key yet
    //     let key = Key {
    //         tonic: pitch_class::PitchClass::A,
    //         mode: Mode::Aeolian,
    //     };

    //     Ok(Progression { chords, key })
    // }
}
