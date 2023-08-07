use std::convert::TryFrom;
use serde::{Serialize, Deserialize};
use strum_macros::Display;
use crate::error::CodecError;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Display)]
pub enum Codec {
    C8,
    // C16
}

impl TryFrom<u8> for Codec {
    type Error = CodecError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x08 => Ok(Codec::C8),
            // 0x10 => Ok(Codec::C16),
            _ => Err(CodecError::UnsupportedValue(value)),
        }
    }
}
