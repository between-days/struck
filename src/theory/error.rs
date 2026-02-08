#[derive(Debug)]
pub enum NoteParseError {
    InvalidNoteStringValue(String),
}

#[derive(Debug)]
pub enum ChordParseError {
    InvalidChordName(String),
    // TODO: maybe NoteParseError(NoteParseError),
}
