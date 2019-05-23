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

/// Abortion in `no_std` Rust wihtout unwinding or touching the panic
/// infrastructure is not yet stable.
///
/// # Note
///
/// This is an attempt at a forced panic due to division by zero
/// that the compiler cannot reason about.
fn abort() -> ! {
    fn abort_impl(val: i32) -> ! {
        loop {
            let _ = 1/(val as u32);
        }
    }
    // Will panic due to division by zero.
    abort_impl(0);
}

#[panic_handler]
pub fn panic(_info: &core::panic::PanicInfo) -> ! {
    abort();
}
