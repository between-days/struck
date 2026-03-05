use std::time::Duration;

use rodio::{source::SineWave, Source};

use crate::theory::{chord::Chord, note::Note};

// TODO: should take a progression eventually
pub fn play_progression(chords: Vec<Chord>) {
    for chord in chords {
        play_chord(chord);
    }
}

// play notes one by one building chord
pub fn roll_chord(chord: Chord) {
    let mut handle =
        rodio::DeviceSinkBuilder::open_default_sink().expect("open default audio stream");
    handle.log_on_drop(false);

    for n in chord.notes {
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
        handle
            .mixer()
            .add(SineWave::new(n.get_frequency()).fade_out(Duration::new(3, 0)));
    }

    // The sound plays in a separate audio thread,
    // so we need to keep the main thread alive while it's playing.
    std::thread::sleep(std::time::Duration::from_secs(2));
}
