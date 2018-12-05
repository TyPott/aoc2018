use std::convert::From;
use std::num;

#[derive(Debug)]
pub enum ParseRecordError {
    DateFormatError(chrono::ParseError),
    ParseIntError(num::ParseIntError),
    RecordFormatError,
}

impl From<num::ParseIntError> for ParseRecordError {
    fn from(e: num::ParseIntError) -> Self {
        ParseRecordError::ParseIntError(e)
    }
}

impl From<chrono::ParseError> for ParseRecordError {
    fn from(e: chrono::ParseError) -> Self {
        ParseRecordError::DateFormatError(e)
    }
}
