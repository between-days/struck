use clearscreen;
use dialoguer::{theme::ColorfulTheme, Input, Select};

use crate::{
    parser::{self, chord_parser::identify_from_root_and_notes},
    theory::{
        self,
        chord::{Chord, ChordQuality},
        error::{ChordParseError, NoteParseError},
        note::Note,
        player::{self},
    },
};

pub fn handle_menu() {
    let items = vec![
        "Information on a known chord",
        "Create chord from notes",
        "Play some chords",
        "Quit",
    ];

    // Loop the menu until the user decides to quit
    loop {
        clearscreen::clear().expect("failed to clear screen");
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choose your activity")
            .items(&items)
            .default(0)
            .interact_opt()
            .expect("Failed to handle input");

        match selection {
            Some(index) => match index {
                0 => {
                    handle_chord_loop();
                }
                1 => {
                    handle_notes_loop();
                }
                2 => {
                    handle_progression_loop();
                }
                3 => {
                    println!("Goodbye!");
                    break;
                }
                _ => unreachable!(),
            },
            None => {
                println!("Goodbye!");
                break;
            }
        }

        println!();
    }
}

fn handle_progression_loop() {
    // find each chord string
    // get each chord from that
    // play all of them
    // in the future, it should be then taking those chords and building a progression object,
    // which has it's own features with things like key, pattern e.g. i Iv idim etc

    println!("Enter chords seperated by space e.g. A#m B7 Cdim (space or exit to exit)");

    loop {
        let input: String = Input::new().with_prompt("> ").interact_text().expect(""); // TODO: probably won't panic

        if input.trim().is_empty() || input.trim().eq("exit") {
            break;
        }

        // don't forget to trim here, otherwise there'll be chord names " "
        let chord_strings = input.trim().split_terminator(" ");

        let chords: Vec<Chord> = chord_strings
            .map(|cs| parser::chord_parser::identify_from_name(cs.trim().to_string()).expect("msg"))
            .collect();

        player::play_progression(chords);
    }
}

fn handle_notes_loop() {
    println!("Enter notes seperated by space e.g. A# B C");

    loop {
        let input: String = Input::new()
            .with_prompt("> ")
            .with_initial_text(" ")
            .interact_text()
            .expect(""); // TODO: probably won't panic

        if input.trim().is_empty() || input.trim().eq("exit") {
            break;
        }

        match identify_chord_from_notes(input) {
            Ok(()) => (),
            Err(e) => println!("caught error: {:?}", e),
        }
    }
}

// keep taking chord names until exit
fn handle_chord_loop() {
    println!("Enter chord name, space input or exit to go back");

    loop {
        let chord_name: String = Input::new()
            .with_prompt("Enter chord name ")
            .with_initial_text(" ")
            .interact_text()
            .expect(""); // TODO: probably won't panic

        if chord_name.trim().is_empty() || chord_name.trim().eq("exit") {
            break;
        }

        match identify_notes_from_chord_name(chord_name) {
            Ok(()) => (),
            Err(e) => println!("caught error: {:?}", e),
        }
    }
}

fn identify_notes_from_chord_name(chord_name: String) -> Result<(), ChordParseError> {
    let chord = match parser::chord_parser::identify_from_name(chord_name) {
        Ok(res) => res,
        Err(_) => {
            return Err(ChordParseError::InvalidChordName(
                "error identifying from name".to_string(),
            ))
        }
    };

    println!("{}", chord);
    println!("Rolling chord...");
    player::roll_chord(chord);
    Ok(())
}

fn identify_chord_from_notes(notes_raw: String) -> Result<(), NoteParseError> {
    let notes: Vec<theory::note::Note> = notes_raw
        .split_whitespace()
        .map(|n| Note::parse(n).unwrap())
        .collect();

    let mut possible_chords = vec![];

    // for each of the notes treated as the root, get what chords it could be considered
    notes.iter().for_each(|root: &Note| {
        let chord = identify_from_root_and_notes(root, &notes);

        if chord.chord_quality != ChordQuality::Ambiguous {
            possible_chords.push(chord);
        }
    });

    if possible_chords.len() == 0 {
        println!("No possible chords found!")
    } else {
        print!("Could be: ");
        possible_chords.iter().for_each(|c| println!("{}", c.name));
    }

    return Ok(());
}
