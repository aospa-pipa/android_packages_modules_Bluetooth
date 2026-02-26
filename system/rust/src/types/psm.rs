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

//! Protocol / Service Multiplexer (PSM)

/// Protocol / Service Multiplexer (PSM)
///
/// Defined where it is [initially used in the L2CAP spec](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host/logical-link-control-and-adaptation-protocol-specification.html#UUID-0e926515-4735-ebb7-6aea-ad27343cf8be)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Psm(u16);

impl Psm {
    /// Create a new PSM from a 16-bit value
    pub const fn new(value: u16) -> Self {
        Self(value)
    }

    /// Return the 16-bit value of the PSM
    pub const fn value(&self) -> u16 {
        self.0
    }
}
