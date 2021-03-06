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

use super::ContractEnv;
use crate::{
    env::{
        Env as _,
        EnvStorage as _,
        EnvTypes,
    },
    memory::vec::Vec,
    storage::Key,
};

/// The environmental address type.
pub type AccountId = <ContractEnv as EnvTypes>::AccountId;

/// The environmental balance type.
pub type Balance = <ContractEnv as EnvTypes>::Balance;

/// The environmental hash type.
pub type Hash = <ContractEnv as EnvTypes>::Hash;

/// The environmental moment type.
pub type Moment = <ContractEnv as EnvTypes>::Moment;

/// Returns the address of the current smart contract.
pub fn address() -> AccountId {
    ContractEnv::address()
}

/// Returns the balance of the current smart contract.
pub fn balance() -> Balance {
    ContractEnv::balance()
}

/// Returns the address of the caller of the current smart contract execution.
pub fn caller() -> AccountId {
    ContractEnv::caller()
}

/// Returns the uninterpreted input data of the current smart contract execution.
pub fn input() -> Vec<u8> {
    ContractEnv::input()
}

/// Returns the random seed from the latest block.
pub fn random_seed() -> Hash {
    ContractEnv::random_seed()
}

/// Returns the timestamp of the latest block.
pub fn now() -> Moment {
    ContractEnv::now()
}

/// Returns the current gas price.
pub fn gas_price() -> Balance {
    ContractEnv::gas_price()
}

/// Returns the amount of gas left for the contract execution.
pub fn gas_left() -> Balance {
    ContractEnv::gas_left()
}

/// Returns the transferred value.
pub fn value_transferred() -> Balance {
    ContractEnv::value_transferred()
}

/// Returns the current smart contract exection back to the caller
/// and return the given encoded value.
///
/// # Safety
///
/// External callers rely on the correct type of the encoded returned value.
/// This operation is unsafe because it does not provide guarantees on its
/// own to always encode the expected type.
pub unsafe fn r#return<T>(value: T) -> !
where
    T: parity_codec::Encode,
{
    ContractEnv::r#return(&value.encode()[..])
}

/// Prints the given content.
///
/// # Note
///
/// Usable only in development (`--dev`) chains.
pub fn println(content: &str) {
    ContractEnv::println(content)
}

/// Deposits raw event data through the Contracts module.
pub fn deposit_raw_event(topics: &[Hash], data: &[u8]) {
    ContractEnv::deposit_raw_event(topics, data)
}

/// Stores the given value under the specified key in the contract storage.
///
/// # Safety
///
/// This operation is unsafe because it does not check for key integrity.
/// Users can compare this operation with a raw pointer dereferencing in Rust.
pub unsafe fn store(key: Key, value: &[u8]) {
    ContractEnv::store(key, value)
}

/// Clears the data stored at the given key from the contract storage.
///
/// # Safety
///
/// This operation is unsafe because it does not check for key integrity.
/// Users can compare this operation with a raw pointer dereferencing in Rust.
pub unsafe fn clear(key: Key) {
    ContractEnv::clear(key)
}

/// Loads the data stored at the given key from the contract storage.
///
/// # Safety
///
/// This operation is unsafe because it does not check for key integrity.
/// Users can compare this operation with a raw pointer dereferencing in Rust.
pub unsafe fn load(key: Key) -> Option<Vec<u8>> {
    ContractEnv::load(key)
}
