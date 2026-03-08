// TODO: docker dive style cli ui would be cool

use std::slice;

use rand::prelude::*;
use rand::Rng;

use crate::theory::interval::Interval;
use crate::theory::progression::Progression;
use crate::theory::scale::ScaleDegree;
use crate::util::A4;
use crate::util::B4;
use crate::util::D4;
use crate::util::E4;
use crate::util::F4;
use crate::util::G4;
use crate::util::GS4;
use crate::{
    cli::handle_menu,
    theory::{
        note::{assume_note_ordering, Note},
        player,
        scale::{Key, Scale},
    },
};
mod cli;
mod parser;
mod theory;
mod util;

// const CHORD_FORMAT: &str = "[Root note] [quality (blank for major)]";

fn main() {
    // handle_menu();
    // play_scale_test();
    // requiem_play_test();

    for i in 0..2 {
        test_prog();
    }
}

fn test_prog() {
    let key = Key {
        tonic: theory::pitch_class::PitchClass::B,
        mode: theory::scale::Mode::Mixolydian,
    };
    let scale_degs = vec![
        ScaleDegree::I,
        ScaleDegree::V,
        ScaleDegree::VI,
        ScaleDegree::IV,
    ];

    let prog = Progression::from_key_and_scale_degrees(key, scale_degs);

    for item in &prog.progression_items {
        println!("{}", item.chord.name);
    }

    player::play_progression(prog);
}

fn requiem_play_test() {
    let key = Key {
        tonic: theory::pitch_class::PitchClass::G,
        mode: theory::scale::Mode::Aeolian,
    };

    let root = Note {
        pitch_class: key.tonic,
        pitch_octave: 4,
    };

    let scale = Scale::from_key(key);

    // get assumed ordering for octaves
    let mut notes: Vec<Note> = scale
        .pitch_classes
        .iter()
        .map(|pc| Note {
            pitch_class: *pc,
            pitch_octave: 4,
        })
        .collect();

    notes = assume_note_ordering(notes);

    let intervals = vec![
        Interval::MinorThird,
        Interval::MajorSecond,
        Interval::Root,
        Interval::PerfectFifth,
    ];

    println!("is: {:?}", intervals);

    // let melody: Vec<Note> = intervals
    //     .iter()
    //     .map(|i| {
    //         println!("i: {:?}", *i as usize);
    //         return root.get_interval(*i);
    //         // return *notes.get(*i as usize ).expect("msg")}
    //     }
    //     )
    //     .collect();

    let melody = vec![G4, G4, G4, D4, E4, E4, D4, D4, B4, B4, A4, A4, G4];

    // for _ in 0..2 {
    for n in &melody {
        player::play_note(&n);
    }
    // }
}

fn play_scale_test() {
    let key = Key {
        tonic: theory::pitch_class::PitchClass::A,
        mode: theory::scale::Mode::Phyrgian,
    };

    let scale = Scale::from_key(key);

    // get assumed ordering for octaves

    let mut notes: Vec<Note> = scale
        .pitch_classes
        .iter()
        .map(|pc| Note {
            pitch_class: *pc,
            pitch_octave: 4,
        })
        .collect();
    // notes.push(Note {
    //     pitch_class: key.tonic,
    //     pitch_octave: 4,
    // });

    notes = assume_note_ordering(notes);

    print!("Notes: ");
    for n in &notes {
        print!(", {}", n);
    }

    println!("");

    let mut melody: Vec<&Note> = vec![];

    let mut rng = rand::rng();
    for i in 0..7 {
        let num = rng.random_range(0..(notes.len() - 1));
        // player::play_note(notes.get(num).expect("msg"));
        melody.push(notes.get(num).expect("msg"));
    }

    // melody.push(notes.get(0).expect("msg"));

    for _ in 0..2 {
        for n in &melody {
            player::play_note(&n);
        }
    }

    // player::play_note(notes.get(0).expect("msg"));

    // for n in &notes {
    //     player::play_note(&n);
    // }

    // for n in notes.iter().rev() {
    //     player::play_note(&n);
    // }

    // let last = scale.pitch_classes.last().expect("msg");

    // let o = last.
}
