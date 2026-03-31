/*
 * Copyright (C) 2026 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

//! All LE Audio related type definition.

use thiserror::Error;

// --- Error Handling ---

/// Errors related to LE Audio data parsing and processing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum Error {
    /// Data is too short or malformed.
    #[error("Truncated data")]
    TruncatedData,
}

/// Type alias for LE Audio results.
pub type Result<T> = std::result::Result<T, Error>;

// --- LTV (Length-Type-Value) Definition and helpers ---

/// Trait for LTV.
pub trait Ltv: Sized {
    /// The unique type identifier for this LTV.
    const TYPE: u8;

    /// Decodes the value from raw bytes. Returns None if data is malformed.
    fn decode(data: &[u8]) -> Option<Self>;
}

/// Represents a borrowed LTV entry pointing into a raw buffer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LtvEntry<'a> {
    /// The type of the LTV entry.
    pub r#type: u8,
    /// The value of the LTV entry (borrowed).
    pub value: &'a [u8],
}

impl<'a> LtvEntry<'a> {
    /// Returns the value as a u8 if possible (requires length 1).
    pub fn as_u8(&self) -> Option<u8> {
        if self.value.len() == 1 {
            Some(self.value[0])
        } else {
            None
        }
    }

    /// Returns the value as a u16 (little-endian) if possible (requires length 2).
    pub fn as_le_u16(&self) -> Option<u16> {
        self.value.try_into().ok().map(u16::from_le_bytes)
    }

    /// Returns the value as a u32 (interpreting 3 bytes as little-endian u24) if possible
    /// (requires length 3).
    pub fn as_le_u24(&self) -> Option<u32> {
        if self.value.len() == 3 {
            Some(
                u32::from(self.value[0])
                    | (u32::from(self.value[1]) << 8)
                    | (u32::from(self.value[2]) << 16),
            )
        } else {
            None
        }
    }

    /// Returns the value as a u32 (little-endian) if possible (requires length 4).
    pub fn as_le_u32(&self) -> Option<u32> {
        self.value.try_into().ok().map(u32::from_le_bytes)
    }

    /// Returns the value as a string (UTF-8).
    pub fn as_string(&self) -> String {
        String::from_utf8_lossy(self.value).into_owned()
    }
}

/// Extension trait for types that can be searched for specific LTV entries.
///
/// This trait provides convenient methods to find or decode specific LTV entries
/// from a collection (like a slice of [`LtvEntry`]), without having to manually
/// iterate or handle raw byte offsets.
///
/// # Examples
///
/// ```
/// let ltvs: &[LtvEntry] = ...;
///
/// // Get a strongly-typed value
/// if let Some(freq) = ltvs.get::<SamplingFrequency>() {
///     println!("Sampling Frequency: {}", freq);
/// }
///
/// Or find a raw entry by its type ID
/// if let Some(entry) = ltvs.find(0x01) {
///     println!("Raw value length: {}", entry.value.len());
/// }
/// ```
pub trait LtvIterExt {
    /// Finds the first LTV entry of the specified type.
    fn find(&self, r#type: u8) -> Option<&LtvEntry<'_>>;

    /// Finds and decodes the first LTV entry of the specified type.
    fn get<T: Ltv>(&self) -> Option<T>;
}

impl<'a> LtvIterExt for [LtvEntry<'a>] {
    fn find(&self, r#type: u8) -> Option<&LtvEntry<'a>> {
        self.iter().find(|entry| entry.r#type == r#type)
    }

    fn get<T: Ltv>(&self) -> Option<T> {
        self.find(T::TYPE).and_then(|entry| T::decode(entry.value))
    }
}

/// Decodes all LTV entries from a raw buffer into a structured container.
///
/// This function iterates over raw LTV entries and allows a closure to fold them
/// into a provided initial configuration object.
///
/// # Examples
///
/// ```
/// // Assuming Metadata implements Default
/// let raw_data = [0x02, 0x01, 0x03, 0x05, 0x03, b'T', b'e', b's', b't'];
/// let metadata = decode_from_ltv_entries(&raw_data, Metadata::default(), |mut acc, entry| {
///     // apply entry to acc...
///     Ok(acc)
/// }).unwrap();
/// ```
pub fn decode_from_ltv_entries<T, F>(data: &[u8], config: T, mut extend: F) -> Result<T>
where
    F: FnMut(T, LtvEntry<'_>) -> Result<T>,
{
    LtvIterator::new(data).try_fold(config, |acc, entry| extend(acc, entry?))
}

// Iterator over LTV entries in a byte slice.
struct LtvIterator<'a> {
    data: &'a [u8],
}

impl<'a> LtvIterator<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data }
    }
}

impl<'a> Iterator for LtvIterator<'a> {
    type Item = Result<LtvEntry<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        if let [len_byte, rest @ ..] = self.data {
            let len = *len_byte as usize;
            if len == 0 {
                return None;
            }
            if rest.len() < len {
                return Some(Err(Error::TruncatedData));
            }
            let (entry_data, next_data) = rest.split_at(len);
            let entry = LtvEntry { r#type: entry_data[0], value: &entry_data[1..] };
            self.data = next_data;
            return Some(Ok(entry));
        }
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use googletest::prelude::*;

    #[derive(Debug, PartialEq, Eq, Clone)]
    struct TestLtv(u8);
    impl Ltv for TestLtv {
        const TYPE: u8 = 0x01;
        fn decode(data: &[u8]) -> Option<Self> {
            data.first().map(|&v| Self(v))
        }
    }

    #[googletest::test]
    fn as_u8_returns_value_for_single_byte_input() {
        // Verify that a single-byte LTV value can be correctly retrieved as a u8.
        let entry = LtvEntry { r#type: 0x01, value: &[0x42] };
        expect_that!(entry.as_u8(), some(eq(0x42)));
    }

    #[googletest::test]
    fn as_u8_returns_none_for_multi_byte_input() {
        // Verify that as_u8 returns None when the value length is not exactly 1.
        let entry = LtvEntry { r#type: 0x01, value: &[0x42, 0x43] };
        expect_that!(entry.as_u8(), none());
    }

    #[googletest::test]
    fn as_le_u16_returns_value_for_two_byte_input() {
        // Verify that a two-byte LTV value is correctly interpreted as a little-endian u16.
        let entry = LtvEntry { r#type: 0x01, value: &[0x01, 0x02] };
        expect_that!(entry.as_le_u16(), some(eq(0x0201)));
    }

    #[googletest::test]
    fn as_le_u16_returns_none_for_single_byte_input() {
        // Verify that as_le_u16 returns None when the value length is shorter than 2.
        let entry = LtvEntry { r#type: 0x01, value: &[0x01] };
        expect_that!(entry.as_le_u16(), none());
    }

    #[googletest::test]
    fn as_le_u24_returns_value_for_three_byte_input() {
        // Verify that a three-byte LTV value is correctly interpreted as a little-endian u24 (u32).
        let entry = LtvEntry { r#type: 0x01, value: &[0x01, 0x02, 0x03] };
        expect_that!(entry.as_le_u24(), some(eq(0x030201)));
    }

    #[googletest::test]
    fn as_le_u32_returns_value_for_four_byte_input() {
        // Verify that a four-byte LTV value is correctly interpreted as a little-endian u32.
        let entry = LtvEntry { r#type: 0x01, value: &[0x01, 0x02, 0x03, 0x04] };
        expect_that!(entry.as_le_u32(), some(eq(0x04030201)));
    }

    #[googletest::test]
    fn iterator_stops_at_zero_length_byte() {
        // Verify that the LTV iterator stops immediately when encountering a zero length byte,
        // which signifies the start of padding in Advertising Data.
        let data = [0x02, 0x01, 0x08, 0x00, 0x02, 0x02, 0x09];
        let mut iter = LtvIterator::new(&data);

        expect_that!(iter.next(), some(ok(anything())));
        expect_that!(iter.next(), none());
    }

    #[googletest::test]
    fn iterator_returns_error_on_truncated_data() {
        // Verify that the iterator detects and reports a TruncatedData error when an entry's
        // length byte exceeds the remaining data in the buffer.
        let data = [0x05, 0x01, 0x01];
        let mut iter = LtvIterator::new(&data);
        expect_that!(iter.next(), some(err(matches_pattern!(Error::TruncatedData))));
    }

    #[googletest::test]
    fn searchable_find_returns_entry_when_type_exists() {
        // Verify that searchable.find correctly locates an entry by its type ID.
        let entries =
            [LtvEntry { r#type: 0x01, value: &[0x10] }, LtvEntry { r#type: 0x02, value: &[0x20] }];
        expect_that!(LtvIterExt::find(&entries[..], 0x01), some(anything()));
    }

    #[googletest::test]
    fn searchable_get_decodes_ltv_into_struct() {
        // Verify that searchable.get decodes the found LTV entry into the requested
        // typed structure.
        let entries = [LtvEntry { r#type: 0x01, value: &[0x42] }];
        let val = LtvIterExt::get::<TestLtv>(&entries[..]);
        expect_that!(val, some(eq(&TestLtv(0x42))));
    }
}
