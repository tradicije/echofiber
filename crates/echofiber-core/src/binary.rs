//! Checked, read-only primitives for parsing untrusted binary input.
//!
//! `BinaryReader` never allocates or modifies its input. Every operation checks
//! its range before accessing bytes and reports an absolute byte offset.

use std::fmt;

/// An error while reading a binary input.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BinaryReadError {
    /// A requested read extends beyond the reader's remaining bytes.
    UnexpectedEnd {
        /// Absolute byte offset where the read began.
        offset: usize,
        /// Number of bytes the operation required.
        requested: usize,
        /// Number of bytes available from `offset`.
        remaining: usize,
    },
    /// Adding a read length to its offset overflowed `usize`.
    OffsetOverflow {
        /// Absolute byte offset where the read began.
        offset: usize,
        /// Number of bytes requested.
        requested: usize,
    },
    /// A seek target falls outside the current reader.
    InvalidSeek {
        /// Requested position relative to this reader's start.
        requested: usize,
        /// Length of this reader's byte range.
        length: usize,
    },
}

impl fmt::Display for BinaryReadError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnexpectedEnd { offset, requested, remaining } => write!(
                formatter,
                "read of {requested} byte(s) at offset {offset} exceeds {remaining} remaining byte(s)"
            ),
            Self::OffsetOverflow { offset, requested } => write!(
                formatter,
                "read of {requested} byte(s) at offset {offset} overflows the address space"
            ),
            Self::InvalidSeek { requested, length } => {
                write!(formatter, "position {requested} is outside a {length}-byte reader")
            }
        }
    }
}

impl std::error::Error for BinaryReadError {}

/// A cursor over a fixed, immutable byte range.
#[derive(Clone, Debug)]
pub struct BinaryReader<'input> {
    bytes: &'input [u8],
    position: usize,
    base_offset: usize,
}

impl<'input> BinaryReader<'input> {
    /// Creates a reader over an entire input file.
    #[must_use]
    pub const fn new(bytes: &'input [u8]) -> Self {
        Self { bytes, position: 0, base_offset: 0 }
    }

    /// Returns the absolute offset of the next byte to read.
    #[must_use]
    pub const fn offset(&self) -> usize {
        self.base_offset + self.position
    }

    /// Returns the reader position relative to its own start.
    #[must_use]
    pub const fn relative_position(&self) -> usize {
        self.position
    }

    /// Returns the total length of this reader's bounded byte range.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.bytes.len()
    }

    /// Returns whether this reader contains no bytes.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    /// Returns the number of unread bytes.
    #[must_use]
    pub const fn remaining(&self) -> usize {
        self.bytes.len() - self.position
    }

    /// Moves the cursor to a position relative to this reader's start.
    ///
    /// Seeking exactly to the end is valid.
    ///
    /// # Errors
    ///
    /// Returns [`BinaryReadError::InvalidSeek`] if `position` is after the end.
    pub fn seek(&mut self, position: usize) -> Result<(), BinaryReadError> {
        if position > self.bytes.len() {
            return Err(BinaryReadError::InvalidSeek {
                requested: position,
                length: self.bytes.len(),
            });
        }
        self.position = position;
        Ok(())
    }

    /// Advances the cursor by `length` bytes.
    ///
    /// # Errors
    ///
    /// Returns an error if the skipped range extends beyond the reader.
    pub fn skip(&mut self, length: usize) -> Result<(), BinaryReadError> {
        self.read_bytes(length).map(|_| ())
    }

    /// Reads a bounded byte slice without allocating.
    ///
    /// # Errors
    ///
    /// Returns an error if the requested range extends beyond the reader.
    pub fn read_bytes(&mut self, length: usize) -> Result<&'input [u8], BinaryReadError> {
        let offset = self.offset();
        let end = self
            .position
            .checked_add(length)
            .ok_or(BinaryReadError::OffsetOverflow { offset, requested: length })?;
        if end > self.bytes.len() {
            return Err(BinaryReadError::UnexpectedEnd {
                offset,
                requested: length,
                remaining: self.remaining(),
            });
        }
        let result = &self.bytes[self.position..end];
        self.position = end;
        Ok(result)
    }

    /// Creates a nested reader over the next `length` bytes and advances this reader.
    ///
    /// Errors from the nested reader retain their absolute offsets in the input.
    ///
    /// # Errors
    ///
    /// Returns an error if the requested range extends beyond the reader.
    pub fn read_sub_reader(&mut self, length: usize) -> Result<Self, BinaryReadError> {
        let base_offset = self.offset();
        let bytes = self.read_bytes(length)?;
        Ok(Self { bytes, position: 0, base_offset })
    }

    /// Reads an unsigned 8-bit integer.
    ///
    /// # Errors
    ///
    /// Returns an error if one byte is unavailable.
    pub fn read_u8(&mut self) -> Result<u8, BinaryReadError> {
        Ok(self.read_array::<1>()?[0])
    }

    /// Reads a little-endian unsigned 16-bit integer.
    ///
    /// # Errors
    ///
    /// Returns an error if two bytes are unavailable.
    pub fn read_u16_le(&mut self) -> Result<u16, BinaryReadError> {
        Ok(u16::from_le_bytes(self.read_array()?))
    }

    /// Reads a big-endian unsigned 16-bit integer.
    ///
    /// # Errors
    ///
    /// Returns an error if two bytes are unavailable.
    pub fn read_u16_be(&mut self) -> Result<u16, BinaryReadError> {
        Ok(u16::from_be_bytes(self.read_array()?))
    }

    /// Reads a little-endian signed 16-bit integer.
    ///
    /// # Errors
    ///
    /// Returns an error if two bytes are unavailable.
    pub fn read_i16_le(&mut self) -> Result<i16, BinaryReadError> {
        Ok(i16::from_le_bytes(self.read_array()?))
    }

    /// Reads a little-endian unsigned 32-bit integer.
    ///
    /// # Errors
    ///
    /// Returns an error if four bytes are unavailable.
    pub fn read_u32_le(&mut self) -> Result<u32, BinaryReadError> {
        Ok(u32::from_le_bytes(self.read_array()?))
    }

    /// Reads a big-endian unsigned 32-bit integer.
    ///
    /// # Errors
    ///
    /// Returns an error if four bytes are unavailable.
    pub fn read_u32_be(&mut self) -> Result<u32, BinaryReadError> {
        Ok(u32::from_be_bytes(self.read_array()?))
    }

    /// Reads a little-endian signed 32-bit integer.
    ///
    /// # Errors
    ///
    /// Returns an error if four bytes are unavailable.
    pub fn read_i32_le(&mut self) -> Result<i32, BinaryReadError> {
        Ok(i32::from_le_bytes(self.read_array()?))
    }

    /// Reads a little-endian unsigned 64-bit integer.
    ///
    /// # Errors
    ///
    /// Returns an error if eight bytes are unavailable.
    pub fn read_u64_le(&mut self) -> Result<u64, BinaryReadError> {
        Ok(u64::from_le_bytes(self.read_array()?))
    }

    /// Reads a little-endian IEEE 754 single-precision float.
    ///
    /// # Errors
    ///
    /// Returns an error if four bytes are unavailable.
    pub fn read_f32_le(&mut self) -> Result<f32, BinaryReadError> {
        Ok(f32::from_le_bytes(self.read_array()?))
    }

    /// Reads a little-endian IEEE 754 double-precision float.
    ///
    /// # Errors
    ///
    /// Returns an error if eight bytes are unavailable.
    pub fn read_f64_le(&mut self) -> Result<f64, BinaryReadError> {
        Ok(f64::from_le_bytes(self.read_array()?))
    }

    fn read_array<const LENGTH: usize>(&mut self) -> Result<[u8; LENGTH], BinaryReadError> {
        let bytes = self.read_bytes(LENGTH)?;
        let mut array = [0; LENGTH];
        array.copy_from_slice(bytes);
        Ok(array)
    }
}

#[cfg(test)]
mod tests {
    use super::{BinaryReadError, BinaryReader};

    #[test]
    fn reads_little_and_big_endian_values() {
        let mut reader = BinaryReader::new(&[0x34, 0x12, 0x01, 0x02, 0x03, 0x04]);

        assert_eq!(reader.read_u16_le(), Ok(0x1234));
        assert_eq!(reader.read_u32_be(), Ok(0x0102_0304));
        assert_eq!(reader.offset(), 6);
        assert_eq!(reader.remaining(), 0);
    }

    #[test]
    fn rejects_a_read_past_the_end_without_moving_the_cursor() {
        let mut reader = BinaryReader::new(&[0xAA, 0xBB, 0xCC]);
        reader.skip(2).expect("two bytes should be available");

        assert_eq!(
            reader.read_u32_le(),
            Err(BinaryReadError::UnexpectedEnd { offset: 2, requested: 4, remaining: 1 })
        );
        assert_eq!(reader.offset(), 2);
    }

    #[test]
    fn nested_reader_preserves_absolute_error_offsets() {
        let mut reader = BinaryReader::new(&[0x00, 0xAA, 0xBB]);
        reader.skip(1).expect("one byte should be available");
        let mut block = reader.read_sub_reader(2).expect("block should be available");

        assert_eq!(
            block.read_u32_le(),
            Err(BinaryReadError::UnexpectedEnd { offset: 1, requested: 4, remaining: 2 })
        );
        assert_eq!(reader.offset(), 3);
    }

    #[test]
    fn rejects_an_invalid_seek_without_moving_the_cursor() {
        let mut reader = BinaryReader::new(&[0x00, 0x01]);
        reader.skip(1).expect("one byte should be available");

        assert_eq!(reader.seek(3), Err(BinaryReadError::InvalidSeek { requested: 3, length: 2 }));
        assert_eq!(reader.relative_position(), 1);
    }

    #[test]
    fn reads_floating_point_values() {
        let mut reader = BinaryReader::new(&[0x00, 0x00, 0x60, 0x40]);

        assert_eq!(reader.read_f32_le(), Ok(3.5));
    }
}
