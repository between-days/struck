#[derive(Debug)]
pub enum PitchClassParseError {
    InvalidPitchClassStringValue(String),
}

#[derive(Debug)]
pub enum NoteParseError {
    InvalidPitchClassStringValue(String),
    InvalidOctaveStringValue(String),
}

#[derive(Debug)]
pub enum ChordParseError {
    InvalidChordName(String),
    // TODO: maybe NoteParseError(NoteParseError),
}
