//! Builds the Frame Content Size (FCS) bytes for zstd frame reconstruction.
//!
//! The FCS field encodes the original (uncompressed) length in little-endian.
//! Its size is determined by the `fcs_size` we stripped from the header during
//! compression and stored in our compact blob.
//!
//! Rules match the zstd spec exactly:
//!   fcs_size 0 -> no bytes (length unknown)
//!   fcs_size 1 -> 1 byte  u8  (original_len fits in u8)
//!   fcs_size 2 -> 2 bytes u16 little-endian, value is (original_len - 256)
//!   fcs_size 4 -> 4 bytes u32 little-endian
//!   fcs_size 8 -> 8 bytes u64 little-endian

use crate::error::{DenspackError, Result};

/// Encode `original_len` into the FCS byte sequence for the given `fcs_size`.
pub fn build_fcs_bytes(original_len: usize, fcs_size: u8) -> Result<Vec<u8>> {
    match fcs_size {
        0 => Ok(vec![]),
        1 => Ok(vec![original_len as u8]),
        2 => {
            if original_len < 256 {
                return Err(DenspackError::FcsTwoByteUnderflow(original_len));
            }
            let encoded = (original_len - 256) as u16;
            Ok(encoded.to_le_bytes().to_vec())
        }
        4 => {
            let encoded = original_len as u32;
            Ok(encoded.to_le_bytes().to_vec())
        }
        8 => {
            let encoded = original_len as u64;
            Ok(encoded.to_le_bytes().to_vec())
        }
        other => Err(DenspackError::UnsupportedFcsSize(other)),
    }
}