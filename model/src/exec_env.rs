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

use core::marker::PhantomData;
use crate::ContractState;
use ink_core::{
    env::Env,
    storage::alloc::{
        Allocate,
        AllocateUsing,
        CellChunkAlloc,
        Initialize,
    },
};

/// Provides a safe interface to an environment given a contract state.
pub struct ExecutionEnv<State, Env> {
    /// The environment handler.
    env_handler: EnvHandler<Env>,
    /// The contract state.
    pub state: State,
}

impl<State, Env> AllocateUsing for ExecutionEnv<State, Env>
where
    State: ContractState,
{
    unsafe fn allocate_using<A>(alloc: &mut A) -> Self
    where
        A: Allocate,
    {
        let env_handler = AllocateUsing::allocate_using(alloc);
        let state = AllocateUsing::allocate_using(alloc);
        Self { env_handler, state }
    }
}

impl<State, Env> Initialize for ExecutionEnv<State, Env>
where
    State: ContractState,
{
    type Args = ();

    fn initialize(&mut self, _: Self::Args) {
        self.env_handler.initialize(());
        self.state.try_default_initialize();
    }
}

impl<State, Env> core::ops::Deref for ExecutionEnv<State, Env> {
    type Target = EnvHandler<Env>;

    fn deref(&self) -> &Self::Target {
        &self.env_handler
    }
}

impl<State, Env> core::ops::DerefMut for ExecutionEnv<State, Env> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.env_handler
    }
}

impl<State, Env> ExecutionEnv<State, Env> {
    /// Splits the execution environment into shared references
    /// to the environment handler and the state.
    ///
    /// # Note
    ///
    /// This can be useful if you want to implement a message as
    /// a method of the state to make it callable from other messages.
    pub fn split(&self) -> (&EnvHandler<Env>, &State) {
        (&self.env_handler, &self.state)
    }

    /// Splits the execution environment into mutable references
    /// to the environment handler and the state.
    ///
    /// # Note
    ///
    /// This can be useful if you want to implement a message as
    /// a method of the state to make it callable from other messages.
    pub fn split_mut(&mut self) -> (&mut EnvHandler<Env>, &mut State) {
        (&mut self.env_handler, &mut self.state)
    }
}

/// The actual handler for the environment and for dynamic
/// allocations and deallocations.
pub struct EnvHandler<T> {
    /// The dynamic allocator.
    pub dyn_alloc: CellChunkAlloc,
    env_marker: PhantomData<T>,
}

impl<T> AllocateUsing for EnvHandler<T> {
    unsafe fn allocate_using<A>(alloc: &mut A) -> Self
    where
        A: Allocate,
    {
        Self {
            dyn_alloc: AllocateUsing::allocate_using(alloc),
            env_marker: PhantomData,
        }
    }
}

impl<T> Initialize for EnvHandler<T> {
    type Args = ();

    fn initialize(&mut self, _: Self::Args) {
        self.dyn_alloc.initialize(())
    }
}

impl<T: Env> EnvHandler<T> {
    /// Returns the address of the current smart contract.
    pub fn address(&self) -> T::AccountId {
        T::address()
    }

    /// Returns the balance of the current smart contract.
    pub fn balance(&self) -> T::Balance {
        T::balance()
    }

    /// Returns the caller address of the current smart contract execution.
    pub fn caller(&self) -> T::AccountId {
        T::caller()
    }

    /// Returns from the current smart contract execution with the given value.
    pub unsafe fn r#return<V>(&self, val: V) -> !
    where
        V: parity_codec::Encode,
    {
        T::r#return(&val.encode()[..])
    }

    /// Prints the given content.
    ///
    /// # Note
    ///
    /// Only usable in development (`--dev`) chains.
    pub fn println(&self, content: &str) {
        T::println(content)
    }

    /// Deposits raw event data through the Contracts module.
    pub fn deposit_raw_event(&self, topics: &[T::Hash], event: &[u8]) {
        T::deposit_raw_event(topics, event)
    }

    /// Returns the random seed from the latest block.
    pub fn random_seed(&self) -> T::Hash {
        T::random_seed()
    }

    /// Returns the timestamp of the latest block.
    pub fn now(&self) -> T::Moment {
        T::now()
    }
}
