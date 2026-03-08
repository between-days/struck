use thiserror::Error;

#[derive(Debug, Error)]
pub enum ScaleDegreeParseError {
    #[error("invalid scale degree string value: {0}")]
    InvalidStringValue(String),
}

#[derive(Debug, Error)]
pub enum PitchClassParseError {
    #[error("invalid pitch class string value: {0}")]
    InvalidStringValue(String),
}

#[derive(Debug, Error)]
pub enum NoteParseError {
    #[error("invalid note string value: {0}")]
    InvalidNoteStringValue(String),
    #[error("invalid octave string value: {0}")]
    InvalidOctaveStringValue(String),
    #[error("note error caused by: {0}")]
    PitchClassParseError(#[from] PitchClassParseError),
}

#[derive(Debug, Error)]
pub enum ChordNameInvalid {
    #[error("regex failed to find root note in string")]
    CanNotDetectRootNote,

    #[error("invalid chord quality string: {0}")]
    InvalidQualityString(String),

    #[error("invalid chord name string: {0}")]
    UnknownOrigin(String),
}

#[derive(Debug, Error)]
pub enum ChordParseError {
    // InvalidChordName(String),
    // TODO: maybe NoteParseError(NoteParseError),
    #[error("invalid chord name: {0}")]
    ChordNameInvalid(ChordNameInvalid),
    #[error("chord error caused by: {0}")]
    NoteParseError(#[from] NoteParseError),
    #[error("unknown error origin: {0}")]
    Unknown(String),
}

pub enum ProgressionParseError {
    Progression(String),
}
