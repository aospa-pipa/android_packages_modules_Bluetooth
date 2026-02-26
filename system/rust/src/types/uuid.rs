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

//! Support for Bluetooth-style UUIDs

use std::fmt;

const BLUETOOTH_BASE_UUID: u128 = 0x00000000_0000_1000_8000_00805F9B34FB;

/// Universal Unique Identifier
///
/// [Specification](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host/service-discovery-protocol--sdp--specification.html#UUID-5f7f2ee0-5e87-0c77-417f-fd7aac61f6f5)
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Uuid(u128);

impl Uuid {
    /// Convert a Bluetooth 16-bit UUID to a full UUID
    pub const fn uuid16(value: u16) -> Self {
        Self(((value as u128) << 96) | BLUETOOTH_BASE_UUID)
    }

    /// Convert a Bluetooth 32-bit UUID to a full UUID
    pub const fn uuid32(value: u32) -> Self {
        Self(((value as u128) << 96) | BLUETOOTH_BASE_UUID)
    }

    /// Wrap a full UUID into this type
    pub const fn uuid128(value: u128) -> Self {
        Self(value)
    }

    /// Produce a UUID from big endian bytes
    pub const fn from_be_bytes(value: [u8; 16]) -> Self {
        Self(u128::from_be_bytes(value))
    }

    /// Convert a UUID to big endian bytes
    pub const fn to_be_bytes(&self) -> [u8; 16] {
        self.0.to_be_bytes()
    }
}

impl fmt::Display for Uuid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let [b0, b1, b2, b3, b4, b5, b6, b7, b8, b9, b10, b11, b12, b13, b14, b15] =
            self.0.to_be_bytes();
        write!(
            f,
            "Uuid128(0x{:08X}_{:04X}_{:04X}_{:04X}_{:012X})",
            u32::from_be_bytes([b0, b1, b2, b3]),
            u16::from_be_bytes([b4, b5]),
            u16::from_be_bytes([b6, b7]),
            u16::from_be_bytes([b8, b9]),
            u64::from_be_bytes([0, 0, b10, b11, b12, b13, b14, b15]),
        )
    }
}
