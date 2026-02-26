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

//! Generic Access Profile, or GAP, as defined in [Volume 3 Part C of the Bluetooth Core
//! Specification](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host/generic-access-profile.html)

use crate::interface::gatt;
use crate::types::{Address, Psm};
use std::future::Future;

pub mod security;
pub use security::LeSecurity;

/// Client for the Generic Access Profile - allows access to methods which do not need another
/// resource (e.g. a link) to operate on.
pub trait Gap {
    /// Error type for operations on the interface.
    type Error: std::error::Error;

    /// Connection type returned by this GAP implementation.
    type Connection: Connection;

    /// Establishes a connection to a bluetooth device.
    ///
    /// The input address refers to the public identity address of the remote device.
    ///
    /// Implements the [Direct Connection Establishment procedure](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host/generic-access-profile.html#UUID-d2794b9e-859a-b990-ae4d-ae7ea50b36b9)
    /// or [Auto Connection Establishment procedure](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host/generic-access-profile.html#UUID-4c36c39d-507e-e922-5f63-ce2613dfb82f)
    /// based on the provided [`ConnectionKind`].
    ///
    /// If the future is dropped or completed, the implementation will no longer be accepting
    /// connections for the requested address.
    ///
    /// If the connection already exists then the future resolves immediately returning the connection object.
    fn connect(
        &self,
        address: Address,
        mode: ConnectionMode,
    ) -> impl Future<Output = Result<Self::Connection, Self::Error>> + '_;
}

/// The connection establishment mode.
#[derive(Copy, Clone, Debug)]
pub enum ConnectionMode {
    /// A directed connection to a specific device.
    ///
    /// Implements the [Direct Connection Establishment procedure](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host/generic-access-profile.html#UUID-d2794b9e-859a-b990-ae4d-ae7ea50b36b9)
    LeDirected,
    /// An undirected connection to a device.
    ///
    /// Implements the [Auto Connection Establishment procedure](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host/generic-access-profile.html#UUID-4c36c39d-507e-e922-5f63-ce2613dfb82f)
    LeUndirected,
}

/// A low-energy link to a Bluetooth device
///
/// This does not grant exclusive access to the device; other components may receive a handle to
/// the same link.
pub trait Connection {
    /// Channel type produced by this link
    type Channel: Channel;

    /// Gatt client available on this link
    type GattClient: gatt::Client;

    /// Error type used for this link
    type Error: std::error::Error;

    /// Establish a channel over a link
    ///
    /// [Specification](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host/generic-access-profile.html#UUID-c0285e71-3f99-d23f-52ba-7f54c527c846)
    fn open_channel(
        &self,
        psm: Psm,
        mode: ChannelMode,
        security: LeSecurity,
    ) -> impl Future<Output = Result<Self::Channel, Self::Error>>;

    /// Acquires a [`gatt::Client`] client for this link.
    fn gatt_client(&self) -> Self::GattClient;

    /// Provides a future that will complete when the link is closed.
    fn closed(&self) -> impl Future<Output = ()>;

    /// Indicates the Bluetooth address of the connected device
    fn address(&self) -> Address;
}

/// Defines the mode of operation of an L2CAP channel.
///
/// [Specification](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host/logical-link-control-and-adaptation-protocol-specification.html#UUID-a7dc7460-0fe6-f926-3208-c1ec298ea98b)
pub enum ChannelMode {
    /// LE [Credit Based Flow Control](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host/logical-link-control-and-adaptation-protocol-specification.html#UUID-5ca5ae23-25b9-13ec-8096-2fd97be027be)
    /// and [Enhanced Credit Based Flow Control](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host/logical-link-control-and-adaptation-protocol-specification.html#UUID-a46fe2ee-e622-c448-1560-46f273fedbeb)
    /// modes.
    CreditBasedFlowControl {
        /// Maximum Transmission Unit (MTU) - the maximum SDU size (in octets) that the L2CAP layer entity can receive.
        mtu: u16,
        /// Maximum PDU Payload Size (MPS) - the maximum PDU payload size (in octets) that the L2CAP layer entity is capable of receiving.
        mps: u16,
        /// Initial credit count - the number of K-frames that the peer device can initially send.
        initial_credits: u16,
    },
}

/// Exclusive access to a channel, multiplexed over a link
pub trait Channel: tokio::io::AsyncRead + tokio::io::AsyncWrite {
    /// Returns which [`Psm`] this channel connected to
    fn psm(&self) -> Psm;

    /// Provides a future that will complete when the channel is closed.
    fn closed(&self) -> impl Future<Output = ()>;
}
