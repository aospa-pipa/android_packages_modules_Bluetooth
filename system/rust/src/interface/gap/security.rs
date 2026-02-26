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

//! Types and interfaces for security configuration

/// Low-Energy Security
///
/// Provides a security minimum requirement for a service, request, or channel. These requirements
/// are designed to be merged together as part of connection management to reach a level of
/// security that satisfies all simultaneous users of a connection.
///
/// Mode 3 is explicitly omitted because it is not used for connection security, only broadcast
/// security.
///
/// [Specification](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host/generic-access-profile.html#UUID-ddcb7f31-abfd-dfeb-4901-a4b0597ec718)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LeSecurity {
    /// [`Secrecy`] is used for either insecure connections or connections requiring secrecy.
    ///
    /// It corresponds to Mode 1 in the spec.
    ///
    /// Most users should select this mode.
    Secrecy(LeMode1Level),
    /// [`Integrity`] is used for connections where eavesdropping is not an issue, but data
    /// integrity may be.
    ///
    /// It corresponds to Mode 2 in the spec.
    Integrity(LeMode2Level),
}

/// Low-Energy Security Levels, Mode 1
///
/// Mode 1 security levels define increasingly secure ways of providing data secrecy.
///
/// [Specification](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host/generic-access-profile.html#UUID-27ccc8b9-1647-e539-319b-a2e9063d8a8a)
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum LeMode1Level {
    /// No Security - all data may be forged or observed.
    ///
    /// Corresponds to Level 1.
    NoSecurity = 1,
    /// Unauthenticated pairing with encryption - a purely passive eavesdropper will not be able to
    /// forge or observe data unless they were present during pairing. An active attacker will be
    /// able to both forge and observe data, even if they were not present during pairing.
    /// Data is authenticated, but the pairing is not.
    ///
    /// Corresponds to Level 2.
    UnauthPair = 2,
    /// Authenticated pairing with encryption - an attacker who was not present when the pairing
    /// was established will not be able to forge or observe data.
    ///
    /// Corresponds to Level 3.
    AuthPair = 3,
    /// Require Authenticated [LE Secure Connections](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host/security-manager-specification.html#UUID-81649dc8-2b08-0ce9-d080-f327f1632f52).
    /// This is currently believed to provide both integrity and secrecy, even in the presence of
    /// an active attacker during pairing.
    ///
    /// Corresponds to Level 4.
    Secure = 4,
}

/// Low-Energy Security Levels, Mode 2
///
/// Mode 2 security levels define levels of connection-based data authentication.
///
/// There is no ability to specify signing-only security with Secure Connections.
/// Use Mode 1 Level 4 if that is required.
///
/// [Specification](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host/generic-access-profile.html#UUID-43c1a747-5e81-0b07-d3fb-a44a0624ef53)
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum LeMode2Level {
    /// Unauthenticated pairing with data signing.
    ///
    /// It is unclear when this would be appropriate for use - data forging requires an active
    /// attacker, and with unauthenticated pairing, an attacker can repair and sign whatever they
    /// want.
    ///
    /// Corresponds to Level 1.
    #[deprecated = "Provides no security beyond what `NoSecurity` does"]
    UnauthPair = 1,
    /// Authenticated pairing with data signing - an attacker who was not present when the pairing
    /// was established will not be able to forge data.
    AuthPair = 2,
}

impl LeSecurity {
    /// Merges two security requirements to provide a minimal security requirement which satisfies
    /// both.
    ///
    /// Follows [Mixed security modes requirements](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host/generic-access-profile.html#UUID-3ecbd464-4f57-3c03-6adf-e335a26d5ee7)
    pub fn join(self, other: Self) -> Self {
        use std::cmp::max;
        match (self, other) {
            (Self::Secrecy(left_level), Self::Secrecy(right_level)) => {
                Self::Secrecy(max(left_level, right_level))
            }
            (Self::Integrity(left_level), Self::Integrity(right_level)) => {
                Self::Integrity(max(left_level, right_level))
            }
            (Self::Integrity(_), Self::Secrecy(_)) => other.join(self),
            (Self::Secrecy(secrecy_level), Self::Integrity(LeMode2Level::AuthPair)) => {
                Self::Secrecy(max(secrecy_level, LeMode1Level::AuthPair))
            }
            (Self::Secrecy(secrecy_level), Self::Integrity(LeMode2Level::UnauthPair)) => {
                Self::Secrecy(max(secrecy_level, LeMode1Level::UnauthPair))
            }
        }
    }
}
