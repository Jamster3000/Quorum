//! Parses the zstandard frame header to extract layout information.
//!
//! A zstd frame starts with:
//!   [4 bytes magic][1 byte descriptor][0-1 bytes window][0/1/2/4 bytes dict_id][0/1/2/4/8 bytes FCS]
//!
//! The descriptor byte encodes which optional fields are present and how large they are.
//! it's parse it here to know exactly where the block data begins, and what to strip/restore.

use crate::error::{DenspackError, Result};

#[derive(Debug, Clone)]
pub struct FrameHeader {
    /// The raw descriptor byte (byte index 4 in the frame).
    pub descriptor: u8,
    /// Number of bytes used by the dict_id field (0, 1, 2, or 4).
    pub dict_id_size: usize,
    /// The raw FCS flag bits (0–3), stored so we can reproduce the header exactly.
    pub fcs_flag: u8,
    /// Number of bytes used by the Frame Content Size field (0, 1, 2, 4, or 8).
    pub fcs_size: usize,
    /// 0 if single-segment (no window descriptor byte), 1 if multi-segment.
    pub window_size: u8,
    /// Total byte length of the header (magic + descriptor + window + dict_id + FCS).
    pub header_size: usize,
}

impl FrameHeader {
    /// Parse the header layout from a fully-compressed zstd frame.
    ///
    /// `full` must be the complete output of a zstd compressor — at minimum the
    /// magic (4 bytes) + descriptor (1 byte).
    pub fn parse(full: &[u8]) -> Result<Self> {
        if full.len() < 5 {
            return Err(DenspackError::MalformedHeader);
        }

        let descriptor = full[4];

        // Low 2 bits of the descriptor encode dict_id field size.
        let dict_id_flag = descriptor & 0x03;
        // Bit 5 is the single-segment flag — when set, no window descriptor byte.
        let single_segment_flag = (descriptor >> 5) & 0x01;
        // Top 2 bits are the FCS flag — encodes how many bytes the content-size field uses.
        let fcs_flag = (descriptor >> 6) & 0x03;

        let dict_id_size = match dict_id_flag {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 4,
            _ => unreachable!(), // 2-bit value, always 0–3
        };

        let fcs_size = match fcs_flag {
            0 => {
                if single_segment_flag == 1 {
                    1
                } else {
                    0
                }
            }
            1 => 2,
            2 => 4,
            3 => 8,
            _ => unreachable!(),
        };

        let window_size: u8 = if single_segment_flag == 1 { 0 } else { 1 };

        // magic(4) + descriptor(1) + window(0 or 1) + dict_id(variable) + fcs(variable)
        let header_size = 4 + 1 + window_size as usize + dict_id_size + fcs_size;

        if full.len() < header_size {
            return Err(DenspackError::MalformedHeader);
        }

        Ok(Self {
            descriptor,
            dict_id_size,
            fcs_flag,
            fcs_size,
            window_size,
            header_size,
        })
    }
}

/// The shared constants that are identical across all frames compressed with the
/// same dictionary. These are extracted once from a probe compression and then
/// stored externally (e.g. in a database) rather than repeating them per message.
#[derive(Debug, Clone)]
pub struct FrameConstants {
    /// Always `[0x28, 0xB5, 0x2F, 0xFD]` for zstd — kept here for completeness.
    pub magic: [u8; 4],
    /// The dictionary ID bytes embedded in each frame. Length varies (0–4 bytes).
    pub dict_id: Vec<u8>,
    /// 0 = single-segment (no window descriptor), 1 = multi-segment.
    pub window_size: u8,
}

impl FrameConstants {
    /// Extract the shared constants from any fully-compressed zstd frame.
    ///
    /// In practice you compress one sample message and call this once. The
    /// returned `FrameConstants` should be stored alongside your dictionary
    /// so it's available at decompression time.
    pub fn extract(full: &[u8]) -> Result<Self> {
        if full.len() < 4 {
            return Err(DenspackError::MalformedHeader);
        }

        let hdr = FrameHeader::parse(full)?;

        // dict_id lives immediately after: magic(4) + descriptor(1) + window(0/1)
        let dict_id_offset = 5 + hdr.window_size as usize;
        let dict_id = full[dict_id_offset..dict_id_offset + hdr.dict_id_size].to_vec();

        let magic: [u8; 4] = full[0..4].try_into().map_err(|_| DenspackError::MalformedHeader)?;

        Ok(Self {
            magic,
            dict_id,
            window_size: hdr.window_size,
        })
    }
}