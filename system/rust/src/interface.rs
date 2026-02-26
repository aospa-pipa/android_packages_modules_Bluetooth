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

//! Interfaces for basic services
//!
//! These are intended to be the API that application-level profiles (e.g. A2DP or Asha) are
//! written against, allowing for easier faking, mocking, and changes to underlying stack
//! implementations.
//!
//! Where possible, these interfaces provide references to the Bluetooth specification to provide
//! additional clarity about expected behavior in cases where the documentation may be incomplete.

pub mod gap;
pub mod gatt;
