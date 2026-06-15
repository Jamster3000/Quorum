//! Core compress / decompress logic.
//!
//! ## What we strip from the full zstd frame
//!
//! A normal zstd frame looks like:
//!   [4 magic][1 descriptor][0-1 window][0-4 dict_id][0-8 FCS][3 block_hdr][payload][4 checksum?]
//!
//! Magic, dict_id, and window_size are identical for every message compressed with
//! the same dictionary, so we store them once externally.
//!
//! What we keep per-message (the "stored blob"):
//!   [1 descriptor][1 fcs_size][2 original_len big-endian][3 block_hdr][payload...]
//!
//! That's 7 bytes of overhead instead of the full header, saving 6–7 bytes per message.

use crate::error::{DenspackError, Result};
use crate::fcs::build_fcs_bytes;
use crate::header::{FrameConstants, FrameHeader};
use zstd::bulk::{Compressor, Decompressor};
use zstd::dict::{DecoderDictionary, EncoderDictionary};

/// Compresses `text` using the provided zstd dictionary and returns a minimal
/// stored blob with shared frame constants stripped out.
///
/// Three optional optimisations are applied using spare reserved bits in the
/// descriptor byte (bits that zstd always sets to zero, so we own them freely):
///
/// - **Bit 2** — block header trailing zeros: if bytes 1 and 2 of the zstd
///   block header are both `0x00`, only byte 0 is stored. Reconstructed as
///   `[b, 0x00, 0x00]` on decompress.
///
/// - **Bit 3** — short length: if the original string is under 256 bytes,
///   `orig_len` is stored as a single `u8` instead of a `u16`, saving 1 byte.
///
/// - **Bit 4** — reserved, unused. Available for future optimisations.
///
/// # Stored blob layout
///
/// ```text
/// byte 0        : descriptor (real zstd descriptor | our flag bits 2, 3)
/// byte 1        : orig_len as u8  (if bit 3 set)
///   OR
/// bytes 1–2     : orig_len as u16 big-endian  (if bit 3 not set)
/// byte 2 or 3   : block header byte 0 only  (if bit 2 set)
///   OR
/// bytes 2–4 or 3–5 : full 3-byte block header  (if bit 2 not set)
/// remaining     : compressed payload (zstd entropy-coded, do not touch)
/// ```
///
/// # What is stripped and stored externally
///
/// The following are identical across every message compressed with the same
/// dictionary, so they are stored once externally (e.g. a database constants
/// row) rather than per message:
///
/// - `magic`       — always `28 b5 2f fd` for zstd frames
/// - `dict_id`     — changes only when the dictionary is retrained
/// - `window_size` — always `0` for single-segment frames
pub fn compress(text: &str, enc_dict: &EncoderDictionary<'static>) -> Result<Vec<u8>> {
    let mut compressor = Compressor::with_prepared_dictionary(enc_dict)
        .map_err(DenspackError::Compression)?;
    let full = compressor
        .compress(text.as_bytes())
        .map_err(DenspackError::Compression)?;

    let hdr = FrameHeader::parse(&full)?;
    let original_len = text.len();

    let block_hdr = full
        .get(hdr.header_size..hdr.header_size + 3)
        .ok_or(DenspackError::MalformedHeader)?;

    let payload = &full[hdr.header_size + 3..];

    // Bit 2: block header trailing zeros stripped
    let (stored_block_hdr, blk_flag) = if block_hdr[1] == 0 && block_hdr[2] == 0 {
        (&block_hdr[..1], true)
    } else {
        (&block_hdr[..], false)
    };

    // Bit 3: orig_len fits in 1 byte (< 256)
    let len_flag = original_len < 256;

    let mut descriptor_stored = hdr.descriptor;
    if blk_flag { descriptor_stored |= 0x04; }
    if len_flag { descriptor_stored |= 0x08; }

    let mut stored = Vec::with_capacity(6 + payload.len());
    stored.push(descriptor_stored);
    if len_flag {
        stored.push(original_len as u8);
    } else {
        stored.extend_from_slice(&(original_len as u16).to_be_bytes());
    }
    stored.extend_from_slice(stored_block_hdr);
    stored.extend_from_slice(payload);

    Ok(stored)
}

/// Decompresses a stored blob produced by [`compress`] back into the original string.
///
/// Reads the flag bits from byte 0 of the blob to determine the layout, then
/// reconstructs a valid zstd frame by re-inserting the externally stored
/// constants (`magic`, `dict_id`) and rebuilding the stripped fields before
/// handing off to the zstd decompressor.
///
/// # Arguments
///
/// - `stored`    — the blob returned by [`compress`]
/// - `constants` — the shared frame constants extracted via [`probe_constants`]
///                 when the dictionary was first built. Must match the dictionary
///                 used to compress — if the dictionary is retrained, the constants
///                 change and old blobs become unreadable with new constants.
/// - `dec_dict`  — the prepared decoder dictionary built from the same raw bytes
///                 used during compression
///
/// # Errors
///
/// Returns [`DenspackError::BlobTooShort`] if `stored` is under 4 bytes.
/// Returns [`DenspackError::UnsupportedWindowSize`] if `constants.window_size != 0`.
/// Returns [`DenspackError::Compression`] if zstd fails to decompress.
/// Returns [`DenspackError::Utf8`] if the decompressed bytes are not valid UTF-8.
pub fn decompress(
    stored: &[u8],
    constants: &FrameConstants,
    dec_dict: &DecoderDictionary<'static>,
) -> Result<String> {
    if stored.len() < 4 {
        return Err(DenspackError::BlobTooShort {
            got: stored.len(),
            need: 4,
        });
    }

    let descriptor_stored = stored[0];
    let blk_flag = (descriptor_stored & 0x04) != 0;
    let len_flag = (descriptor_stored & 0x08) != 0;
    let descriptor = descriptor_stored & !0x04 & !0x08;

    let fcs_flag = (descriptor >> 6) & 0x03;
    let single_segment = (descriptor >> 5) & 0x01;
    let fcs_size = match fcs_flag {
        0 => if single_segment == 1 { 1 } else { 0 },
        1 => 2,
        2 => 4,
        3 => 8,
        _ => unreachable!(),
    };

    let (original_len, block_start) = if len_flag {
        (stored[1] as usize, 2)
    } else {
        (u16::from_be_bytes([stored[1], stored[2]]) as usize, 3)
    };

    let (block_hdr, payload) = if blk_flag {
        let b = stored[block_start];
        (&[b, 0x00, 0x00] as &[u8], &stored[block_start + 1..])
    } else {
        (&stored[block_start..block_start + 3], &stored[block_start + 3..])
    };

    let fcs_bytes = build_fcs_bytes(original_len, fcs_size)?;

    if constants.window_size != 0 {
        return Err(DenspackError::UnsupportedWindowSize(constants.window_size));
    }

    let mut full = Vec::new();
    full.extend_from_slice(&constants.magic);
    full.push(descriptor);
    full.extend_from_slice(&constants.dict_id);
    full.extend_from_slice(&fcs_bytes);
    full.extend_from_slice(block_hdr);
    full.extend_from_slice(payload);

    let mut decompressor = Decompressor::with_prepared_dictionary(dec_dict)
        .map_err(DenspackError::Compression)?;
    let decompressed = decompressor
        .decompress(&full, original_len + 64)
        .map_err(DenspackError::Compression)?;

    Ok(String::from_utf8(decompressed)?)
}
