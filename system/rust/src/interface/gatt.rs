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

//! Generic Attribute Profile (GATT)
//!
//! [Specification](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host/generic-attribute-profile--gatt-.html)

use crate::types::Uuid;
use bitflags::bitflags;
use std::fmt;
use std::future::Future;
use std::num::NonZeroU16;
use thiserror::Error;
use tokio_stream::Stream;

/// A client connected to a GATT server
pub trait Client {
    /// Discovers all primary services on the server.
    ///
    /// See Vol 3, Part G - [4.4.1 Discover All Primary Services](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host/generic-attribute-profile--gatt-.html#UUID-240501b0-9cf5-f75a-f694-11af8489bb1e)
    fn discover_all_primary_services(
        &mut self,
    ) -> impl Stream<Item = Result<Service, Error>> + '_ + Unpin;

    /// Discovers primary services on the server by UUID.
    ///
    /// See Vol 3, Part G - [4.4.2 Discover Primary Service by Service UUID](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host/generic-attribute-profile--gatt-.html#UUID-46bda90e-f82f-61d1-601a-0cd37c8dfbe8)
    fn discover_primary_services_by_uuid(
        &mut self,
        uuid: Uuid,
    ) -> impl Stream<Item = Result<Service, Error>> + '_ + Unpin;

    /// Finds included services for a given service.
    ///
    /// See Vol 3, Part G - [4.5.1 Find Included Services](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host/generic-attribute-profile--gatt-.html#UUID-55f2d1b7-19a4-54cb-d149-4e8fee177315)
    fn find_included_services(
        &mut self,
        service: &Service,
    ) -> impl Stream<Item = Result<Include, Error>> + '_ + Unpin;

    /// Discovers all characteristics of a service.
    ///
    /// See Vol 3, Part G - [4.6.1 Discover All Characteristics of a Service](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host/generic-attribute-profile--gatt-.html#UUID-51b94bb0-1dee-29e9-b831-256ff1c16cd0)
    fn discover_all_characteristics_of_a_service(
        &mut self,
        service: &Service,
    ) -> impl Stream<Item = Result<Characteristic, Error>> + '_ + Unpin;

    /// Discovers characteristics of a service by UUID.
    ///
    /// See Vol 3, Part G - [4.6.2 Discover Characteristics by UUID](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host/generic-attribute-profile--gatt-.html#UUID-9fc642cb-9082-2fad-36a6-936d871c4f4f)
    fn discover_characteristics_by_uuid(
        &mut self,
        service: &Service,
        uuid: Uuid,
    ) -> impl Stream<Item = Result<Characteristic, Error>> + '_ + Unpin;

    /// Discovers all characteristic descriptors for a given characteristic.
    ///
    /// See Vol 3, Part G - [4.7.1 Discover All Characteristic Descriptors](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host/generic-attribute-profile--gatt-.html#UUID-5bfc134c-6c84-0ea8-0b95-b86daa8b745b)
    fn discover_all_characteristic_descriptors(
        &mut self,
        characteristic: &Characteristic,
    ) -> impl Stream<Item = Result<Descriptor, Error>> + '_ + Unpin;

    /// Reads the characteristic value.
    ///
    /// The implementation must automatically handle long reads if necessary.
    ///
    /// See Vol 3, Part G - [4.8.1 Read Characteristic Value](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host/generic-attribute-profile--gatt-.html#UUID-910e985b-191f-ffb2-ac33-d776f6bd60b0)
    ///
    /// See Vol 3, Part G - [4.8.3 Read Long Characteristic Value](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host/generic-attribute-profile--gatt-.html#UUID-1842aa5c-4f39-905f-5509-0066344051b9)
    fn read_characteristic_value(
        &mut self,
        characteristic: &Characteristic,
    ) -> impl Future<Output = Result<Vec<u8>, Error>>;

    /// Reads characteristic values using a UUID.
    ///
    /// See Vol 3, Part G - [4.8.2 Read Using Characteristic UUID](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host/generic-attribute-profile--gatt-.html#UUID-459b0600-bf52-aa29-18f4-b874327e4173)
    fn read_using_characteristic_uuid(
        &mut self,
        uuid: Uuid,
        start_handle: AttributeHandle,
        end_handle: AttributeHandle,
    ) -> impl Stream<Item = Result<(AttributeHandle, Vec<u8>), Error>> + '_ + Unpin;

    /// Reads multiple characteristic values with a fixed length.
    ///
    /// Since the response from the server does not contain individual lengths,
    /// the client must know the length of each characteristic beforehand.
    /// This method assumes all characteristics have the same fixed length `N`.
    ///
    /// See Vol 3, Part G - [4.8.4 Read Multiple Characteristic Values](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host/generic-attribute-profile--gatt-.html#UUID-dc81ae59-8558-19ef-c0b0-bf8b39a238a0)
    fn read_multiple_characteristic_values<const N: usize>(
        &mut self,
        characteristics: &[Characteristic],
    ) -> impl Future<Output = Result<Vec<[u8; N]>, Error>>;

    /// Reads multiple variable length characteristic values.
    ///
    /// See Vol 3, Part G - [4.8.5 Read Multiple Variable Length Characteristic Values](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host/generic-attribute-profile--gatt-.html#UUID-f0a8f819-6d25-efcb-b237-08cd08b72726)
    fn read_multiple_variable_length_characteristic_values(
        &mut self,
        characteristics: &[Characteristic],
    ) -> impl Future<Output = Result<Vec<Vec<u8>>, Error>>;

    /// Writes a characteristic value.
    ///
    /// The implementation must automatically handle long writes if the value exceeds the MTU.
    ///
    /// See Vol 3, Part G - [4.9.3 Write Characteristic Value](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host/generic-attribute-profile--gatt-.html#UUID-53b00647-5dd9-99ae-3e74-8fc688b108d1)
    ///
    /// See Vol 3, Part G - [4.9.4 Write Long Characteristic Value](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host/generic-attribute-profile--gatt-.html#UUID-6dec55a7-3938-eaa1-286d-80dfd34a8ab5)
    fn write_characteristic_value(
        &mut self,
        characteristic: &Characteristic,
        value: &[u8],
    ) -> impl Future<Output = Result<(), Error>>;

    /// Performs a reliable write.
    ///
    /// See Vol 3, Part G - [4.9.5 Reliable Writes](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host/generic-attribute-profile--gatt-.html#UUID-972ea2ea-7ae4-619c-914f-9b9c446b885f)
    fn reliable_write(
        &mut self,
        characteristic: &Characteristic,
        value: &[u8],
    ) -> impl Future<Output = Result<(), Error>>;

    /// Reads a descriptor value.
    ///
    /// The implementation must automatically handle long reads if necessary.
    ///
    /// See Vol 3, Part G - [4.12.1 Read Characteristic Descriptors](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host/generic-attribute-profile--gatt-.html#UUID-1ee24d8c-e4ce-8881-bd1a-e7127e1a5e60)
    ///
    /// See Vol 3, Part G - [4.12.2 Read Long Characteristic Descriptors](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host/generic-attribute-profile--gatt-.html#UUID-ed2e63cb-15b2-140a-71c8-572cb498135a)
    fn read_descriptor_value(
        &mut self,
        descriptor: &Descriptor,
    ) -> impl Future<Output = Result<Vec<u8>, Error>>;

    /// Writes a descriptor value.
    ///
    /// The implementation must automatically handle long writes if necessary.
    ///
    /// See Vol 3, Part G - [4.12.3 Write Characteristic Descriptors](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host/generic-attribute-profile--gatt-.html#UUID-75455264-b4ab-072b-0d02-d040ecd94412)
    ///
    /// See Vol 3, Part G - [4.12.4 Write Long Characteristic Descriptors](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host/generic-attribute-profile--gatt-.html#UUID-db7d3709-37c2-7c9a-34b7-7cbb1e9182d4)
    fn write_descriptor_value(
        &mut self,
        descriptor: &Descriptor,
        value: &[u8],
    ) -> impl Future<Output = Result<(), Error>>;

    /// Subscribes to notifications or indications for a characteristic.
    ///
    /// This procedure handles writing to the Client Characteristic Configuration
    /// descriptor (CCCD).
    ///
    /// See Vol 3, Part G - [4.10 Notifications](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host/generic-attribute-profile--gatt-.html#UUID-45cc7124-64d2-de87-0a3e-3360127930a6)
    ///
    /// See Vol 3, Part G - [4.11 Indications](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host/generic-attribute-profile--gatt-.html#UUID-4ec548b3-7cfc-41a3-438e-fe5dfe8992ad)
    fn subscribe(
        &mut self,
        characteristic: &Characteristic,
        notification_type: NotificationType,
    ) -> impl Future<Output = Result<impl Stream<Item = Vec<u8>> + '_ + Unpin, Error>>;
}

/// Vol 3, Part F - 3.2.2 Attribute handle
/// Attribute handles on any given server shall have unique, non-zero values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
pub struct AttributeHandle(pub NonZeroU16);

/// A GATT Service.
///
/// A service is a collection of data and associated behaviors to accomplish a
/// particular function or feature.
///
/// See Vol 3, Part G - [3.1 Service Definition](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host/generic-attribute-profile--gatt-.html#UUID-8c779eb2-7b0d-75a8-8d44-e4e9073ccb28)
#[derive(Debug, PartialEq, Clone)]
pub struct Service {
    /// The UUID of the service.
    pub uuid: Uuid,
    /// The attribute handle of the primary or secondary service declaration.
    pub attribute_handle: AttributeHandle,
    /// The end group handle of the service.
    pub end_group_handle: AttributeHandle,
}

/// An included service.
///
/// An include definition is used to include a service definition.
///
/// See Vol 3, Part G - [3.2 Include Definition](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host/generic-attribute-profile--gatt-.html#UUID-91c92c44-2e6f-8ce2-6735-9d5d0885a736)
#[derive(Debug, PartialEq, Clone)]
pub struct Include {
    /// The attribute handle of the include declaration.
    pub attribute_handle: AttributeHandle,
    /// The service being included.
    pub service: Service,
}

bitflags! {
    /// Vol 3, Part G - [3.3.1.1 Characteristic Properties](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host/generic-attribute-profile--gatt-.html#UUID-957d2ce5-401b-3cf3-1150-152d226887eb)
    #[derive(Debug, PartialEq, Eq, Copy, Clone)]
    pub struct CharacteristicProperties: u8 {
        /// If set, permits broadcasts of the Characteristic Value using Server Characteristic
        /// Configuration Descriptor (0x01).
        const BROADCAST = 0x01;
        /// If set, permits reads of the Characteristic Value using procedures defined in Section
        /// 4.8 (0x02).
        const READ = 0x02;
        /// If set, permit writes of the Characteristic Value without response using procedures
        /// defined in Section 4.9.1 (0x04).
        const WRITE_WITHOUT_RESPONSE = 0x04;
        /// If set, permits writes of the Characteristic Value with response using procedures
        /// defined in Section 4.9.3 or Section 4.9.4 (0x08).
        const WRITE = 0x08;
        /// If set, permits notifications of a Characteristic Value without acknowledgment using
        /// the procedure defined in Section 4.10 (0x10).
        const NOTIFY = 0x10;
        /// If set, permits indications of a Characteristic Value with acknowledgment using the
        /// procedure defined in Section 4.11 (0x20).
        const INDICATE = 0x20;
        /// If set, permits signed writes to the Characteristic Value using the procedure defined
        /// in Section 4.9.2 (0x40).
        const AUTHENTICATED_SIGNED_WRITES = 0x40;
        /// If set, additional characteristic properties are defined in the Characteristic Extended
        /// Properties Descriptor defined in Section 3.3.3.1 (0x80).
        const EXTENDED_PROPERTIES = 0x80;
    }
}

/// Type of server-initiated update to subscribe to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotificationType {
    /// Notifications are used to send updates from the server to the client without
    /// acknowledgment. They are faster and use less power than indications, but are
    /// less reliable as the server does not know if the client received the update.
    ///
    /// See Vol 3, Part G - [4.10 Notifications](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host/generic-attribute-profile--gatt-.html#UUID-45cc7124-64d2-de87-0a3e-3360127930a6)
    Notification,
    /// Indications are used to send updates from the server to the client with
    /// acknowledgment. The client must send a confirmation back to the server.
    /// They are more reliable than notifications but slower, as the server must
    /// wait for a confirmation before sending the next indication.
    ///
    /// See Vol 3, Part G - [4.11 Indications](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host/generic-attribute-profile--gatt-.html#UUID-4ec548b3-7cfc-41a3-438e-fe5dfe8992ad)
    Indication,
}

#[derive(Debug, PartialEq, Clone)]
/// A GATT Characteristic.
///
/// A characteristic is a value used in a service along with properties and
/// configuration information about how the value is accessed and information
/// about how the characteristic is displayed or represented.
///
/// See Vol 3, Part G - [3.3 Characteristic Definition](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host/generic-attribute-profile--gatt-.html#UUID-e48dfe16-7108-9a5d-f864-5f6cb04375e3)
pub struct Characteristic {
    /// The UUID of the characteristic.
    pub uuid: Uuid,
    /// The properties of the characteristic.
    pub properties: CharacteristicProperties,
    /// The attribute handle of the characteristic declaration.
    pub attribute_handle: AttributeHandle,
    /// The attribute handle of the characteristic value.
    pub value_handle: AttributeHandle,
    /// The last attribute handle associated with this characteristic.
    pub end_handle: AttributeHandle,
}

/// A GATT Characteristic Descriptor.
///
/// Characteristic descriptors are used to contain related information about the
/// characteristic value.
///
/// See Vol 3, Part G - [3.3.3 Characteristic Descriptor Declarations](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host/generic-attribute-profile--gatt-.html#UUID-3f449aad-fd07-f4f1-d3fc-871c2eca5086)
#[derive(Debug, PartialEq, Clone)]
pub struct Descriptor {
    /// The UUID of the characteristic descriptor.
    pub uuid: Uuid,
    /// The attribute handle of the characteristic descriptor.
    pub attribute_handle: AttributeHandle,
}

/// GATT Error codes.
///
/// Includes both ATT error codes (Vol 3, Part F, 3.4.1.1) and internal errors.
#[derive(Error, Debug, PartialEq, Clone)]
#[repr(u32)]
pub enum Error {
    /// The attribute handle given was not valid on this server (0x01).
    #[error("invalid handle")]
    InvalidHandle = 0x01,
    /// The attribute cannot be read (0x02).
    #[error("read not permitted")]
    ReadNotPermitted = 0x02,
    /// The attribute cannot be written (0x03).
    #[error("write not permitted")]
    WriteNotPermitted = 0x03,
    /// The attribute PDU was invalid (0x04).
    #[error("invalid pdu")]
    InvalidPdu = 0x04,
    /// The attribute requires authentication before it can be read or written (0x05).
    #[error("insufficient authentication")]
    InsufficientAuthentication = 0x05,
    /// The attribute server does not support the request received from the client (0x06).
    #[error("request not supported")]
    RequestNotSupported = 0x06,
    /// Specified offset was past the end of the attribute (0x07).
    #[error("invalid offset")]
    InvalidOffset = 0x07,
    /// The attribute requires authorization before it can be read or written (0x08).
    #[error("insufficient authorization")]
    InsufficientAuthorization = 0x08,
    /// Too many prepare writes have been queued (0x09).
    #[error("prepare queue full")]
    PrepareQueueFull = 0x09,
    /// Attribute not found within the specified range (0x0A).
    #[error("attribute not found")]
    AttributeNotFound = 0x0A,
    /// The attribute cannot be read or written using the Read Blob Request (0x0B).
    #[error("attribute not long")]
    AttributeNotLong = 0x0B,
    /// The attribute requires a higher encryption key size before it can be read or written
    /// (0x0C).
    #[error("insufficient encryption key size")]
    InsufficientEncryptionKeySize = 0x0C,
    /// The attribute value length is invalid for the operation (0x0D).
    #[error("invalid attribute value length")]
    InvalidAttributeValueLength = 0x0D,
    /// The attribute request that was requested has encountered an error that was very unlikely,
    /// and therefore could not be completed as requested (0x0E).
    #[error("unlikely error")]
    UnlikelyError = 0x0E,
    /// The attribute requires encryption before it can be read or written (0x0F).
    #[error("insufficient encryption")]
    InsufficientEncryption = 0x0F,
    /// The attribute type is not a supported grouping attribute as defined by a higher layer
    /// specification (0x10).
    #[error("unsupported group type")]
    UnsupportedGroupType = 0x10,
    /// Insufficient Resources to complete the request (0x11).
    #[error("insufficient resources")]
    InsufficientResources = 0x11,
    /// Cached Database is out of sync and a new discovery should be issued (0x12).
    #[error("database out of sync")]
    DatabaseOutOfSync = 0x12,
    /// The attribute value is not allowed (0x13).
    #[error("value not allowed")]
    ValueNotAllowed = 0x13,
    /// The Client Characteristic Configuration descriptor is not configured correctly for the
    /// procedure (0xFD).
    #[error("cccd improperly configured")]
    CccdImproperlyConfigured = 0xFD,
    /// The procedure cannot be performed because another procedure is already in progress (0xFE).
    #[error("procedure already in progress")]
    ProcedureAlreadyInProgress = 0xFE,
    /// The attribute value is out of range as defined by the profile or service specification
    /// (0xFF).
    #[error("out of range")]
    OutOfRange = 0xFF,
    /// The ACL connection was lost.
    #[error("link disconnected")]
    Disconnected = 0x1000,
    /// Application error code defined by a higher layer specification.
    #[error("application error (code {0})")]
    ApplicationError(u8) = 0x1001,
    /// Common Profile And Service Error Codes defined in Core Specification Supplement Part B.
    #[error("common profile and service error (code {0})")]
    CommonProfileAndServiceError(u8) = 0x1002,
    /// Internal GATT Error.
    /// This means that an invariant has been broken.
    /// This error is usually not recoverable.
    #[error("internal error (reason {0:?})")]
    InternalError(u32) = 0x1003,
}

impl fmt::Display for AttributeHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{:04X}]", self.0)
    }
}

impl From<AttributeHandle> for u16 {
    fn from(handle: AttributeHandle) -> u16 {
        handle.0.into()
    }
}

impl TryFrom<u16> for AttributeHandle {
    type Error = <NonZeroU16 as TryFrom<u16>>::Error;
    fn try_from(handle: u16) -> Result<AttributeHandle, <NonZeroU16 as TryFrom<u16>>::Error> {
        Ok(AttributeHandle(handle.try_into()?))
    }
}

impl From<u8> for CharacteristicProperties {
    fn from(bits: u8) -> CharacteristicProperties {
        // All bits are defined in CharacteristicProperties.
        CharacteristicProperties::from_bits(bits).unwrap()
    }
}
