use dialoguer::{theme::ColorfulTheme, Input, Select};

use crate::theory::{
    self,
    chord::{identify_from_root_and_notes, ChordQuality},
    error::{ChordParseError, NoteParseError},
    note::Note,
};

pub fn handle_menu() {
    let items = vec![
        "Information on a known chord",
        "Create chord from notes",
        "Quit",
    ];

    // Loop the menu until the user decides to quit
    loop {
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choose your activity")
            .items(&items)
            .default(0)
            .interact_opt()
            .expect("Failed to handle input");

        match selection {
            Some(index) => {
                match index {
                    0 => {
                        let chord_name: String = Input::new()
                            .with_prompt("Enter chord name ")
                            .interact_text()
                            .expect(""); // TODO: probably won't panic

                        match identify_notes_from_chord_name(chord_name) {
                            Ok(()) => (),
                            Err(e) => println!("caught error: {:?}", e),
                        }
                    }
                    1 => {
                        let notes_raw: String = Input::new()
                            .with_prompt("Enter notes seperated by space e.g. A# B C ")
                            .interact_text()
                            .expect(""); // TODO: probably won't panic

                        match identify_chord_from_notes(notes_raw) {
                            Ok(()) => (),
                            Err(e) => println!("caught error: {:?}", e),
                        }
                    }
                    2 => {
                        println!("Goodbye!");
                        break;
                    }
                    _ => unreachable!(),
                }
            }
            None => {
                println!("Goodbye!");
                break;
            }
        }

        println!();
    }
}

fn identify_notes_from_chord_name(chord_name: String) -> Result<(), ChordParseError> {
    let chord = match theory::chord::identify_from_name(chord_name) {
        Ok(res) => res,
        Err(_) => {
            return Err(ChordParseError::InvalidChordName(
                "error identifying from name".to_string(),
            ))
        }
    };

    chord.list_notes_in_chord();

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
        println!("Could be: ");
        possible_chords.iter().for_each(|c| println!("{}", c.name));
    }

    return Ok(());
}
