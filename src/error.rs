// error.rs

#[derive(thiserror::Error, Debug)]
pub enum CodecError {
    #[error("Unsupported codec value: {0}")]
    UnsupportedValue(u8),
}
