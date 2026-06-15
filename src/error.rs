use thiserror::Error;

#[derive(Debug, Error)]
pub enum DenspackError {
    #[error("zstd compression failed: {0}")]
    Compression(#[from] std::io::Error),

    #[error("stored blob is too short to be valid (got {got} bytes, need at least {need})")]
    BlobTooShort { got: usize, need: usize },

    #[error("unsupported fcs_size value: {0} (expected 0, 1, 2, 4, or 8)")]
    UnsupportedFcsSize(u8),

    #[error("2-byte FCS requires original_len >= 256, got {0}")]
    FcsTwoByteUnderflow(usize),

    #[error("non-zero window_size ({0}) is not supported in this reconstructor")]
    UnsupportedWindowSize(u8),

    #[error("decompressed bytes are not valid UTF-8: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),

    #[error("compressed frame header is malformed or truncated")]
    MalformedHeader,
}

pub type Result<T> = std::result::Result<T, DenspackError>;