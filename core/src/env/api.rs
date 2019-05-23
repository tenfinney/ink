// Copyright 2018-2019 Parity Technologies (UK) Ltd.
// This file is part of ink!.
//
// ink! is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// ink! is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with ink!.  If not, see <http://www.gnu.org/licenses/>.

use super::ContractEnvStorage;
use crate::{
    env::{
        EnvStorage as _,
    },
    memory::vec::Vec,
    storage::Key,
};

/// Stores the given value under the specified key in the contract storage.
///
/// # Safety
///
/// This operation is unsafe because it does not check for key integrity.
/// Users can compare this operation with a raw pointer dereferencing in Rust.
pub unsafe fn store(key: Key, value: &[u8]) {
    ContractEnvStorage::store(key, value)
}

/// Clears the data stored at the given key from the contract storage.
///
/// # Safety
///
/// This operation is unsafe because it does not check for key integrity.
/// Users can compare this operation with a raw pointer dereferencing in Rust.
pub unsafe fn clear(key: Key) {
    ContractEnvStorage::clear(key)
}

/// Loads the data stored at the given key from the contract storage.
///
/// # Safety
///
/// This operation is unsafe because it does not check for key integrity.
/// Users can compare this operation with a raw pointer dereferencing in Rust.
pub unsafe fn load(key: Key) -> Option<Vec<u8>> {
    ContractEnvStorage::load(key)
}
