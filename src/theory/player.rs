// TODO: consider making this a trait, an impl of the player using rodio, decouple

use std::time::Duration;

use rodio::{
    source::{BltFilter, SawtoothWave, SineWave, TriangleWave},
    Source,
};

use crate::theory::{
    chord::Chord,
    note::Note,
    progression::{self, Progression},
};

// play notes one by one building chord
pub fn play_note(note: &Note) {
    //}, milliseconds: u32) {
    let mut handle =
        rodio::DeviceSinkBuilder::open_default_sink().expect("open default audio stream");
    handle.log_on_drop(false);

    let freq = note.get_frequency();
    let cutoff = freq * 1.2;

    let source = SawtoothWave::new(freq)
        .fade_out(Duration::new(0, 500000000))
        .low_pass_with_q(cutoff as u32, 5.0);

    // let filtered = BltFilter::new(
    //     source,
    //     BltFilterT::LowPass,
    //     1000.0, // cutoff frequency (Hz)
    //     0.7     // resonance (Q)
    // );

    handle.mixer().add(source);

    // The sound plays in a separate audio thread,
    // so we need to keep the main thread alive while it's playing.
    std::thread::sleep(std::time::Duration::from_millis(500));
}

// TODO: should take a progression eventually
pub fn play_progression(progression: Progression) {
    for item in progression.progression_items {
        play_chord(item.chord);
    }
}

// TODO: should take a progression eventually
pub fn play_chords(chords: Vec<Chord>) {
    for chord in chords {
        play_chord(chord);
    }
}

// play notes one by one building chord
pub fn roll_chord(chord: &Chord) {
    let mut handle =
        rodio::DeviceSinkBuilder::open_default_sink().expect("open default audio stream");
    handle.log_on_drop(false);

    for n in &chord.notes {
        handle.mixer().add(SineWave::new(n.get_frequency()));
        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    // The sound plays in a separate audio thread,
    // so we need to keep the main thread alive while it's playing.
    std::thread::sleep(std::time::Duration::from_secs(2));
}

// play notes all at once
pub fn play_chord(chord: Chord) {
    let mut handle =
        rodio::DeviceSinkBuilder::open_default_sink().expect("open default audio stream");
    handle.log_on_drop(false);

    // play all at once with a fade over the 2 seconds
    for n in chord.notes {
        let freq = n.get_frequency();
        let cutoff = freq * 1.2;

        // let source = SawtoothWave::new(freq)
        let source = SineWave::new(n.get_frequency())
            .fade_out(Duration::new(2, 0))
            ;
            // .low_pass_with_q(cutoff as u32, 5.0);

        handle.mixer().add(source);
        // .add(SineWave::new(n.get_frequency()).fade_out(Duration::new(3, 0)));
    }

    // The sound plays in a separate audio thread,
    // so we need to keep the main thread alive while it's playing.
    std::thread::sleep(std::time::Duration::from_secs(2));
}
