//! Denspack is a library for compressing and decompressing messages using zstd with a shared dictionary.
//!
//! It is designed to minimise the number of bytes a compressed string takes up. This does exactly that through a few different techniques.
//!
//! To run the tests and see it working, run `cargo test -- --nocapture`

mod codec;
mod error;
mod fcs;
mod header;

pub use codec::{compress, decompress};
pub use error::{DenspackError, Result};
pub use header::{FrameConstants, FrameHeader};

pub use zstd::dict::{DecoderDictionary, EncoderDictionary};

/// Train a zstd dictionary from a collection of sample strings or byte slices.
///
/// `capacity` is the max dictionary size in bytes; 4096 is a good default for
/// short message workloads.
///
/// Returns the raw dictionary bytes. Persist these — you'll need them every
/// time you compress or decompress.
pub fn train_dictionary(samples: &[impl AsRef<str>], capacity: usize) -> Result<Vec<u8>> {
    let byte_samples: Vec<&[u8]> = samples.iter().map(|s| s.as_ref().as_bytes()).collect();
    zstd::dict::from_samples(&byte_samples, capacity).map_err(DenspackError::Compression)
}

/// Compress a probe message with the given dictionary bytes and extract the
/// [`FrameConstants`] that are shared across all frames.
///
/// Call this once after training a new dictionary and persist the result.
/// The probe uses level 19 to match the Python reference implementation.
pub fn probe_constants(dict_bytes: &[u8], sample: &str) -> Result<FrameConstants> {
    let enc_dict = EncoderDictionary::copy(dict_bytes, 19);
    let mut compressor = zstd::bulk::Compressor::with_prepared_dictionary(&enc_dict)
        .map_err(DenspackError::Compression)?;
    let full = compressor
        .compress(sample.as_bytes())
        .map_err(DenspackError::Compression)?;
    FrameConstants::extract(&full)
}