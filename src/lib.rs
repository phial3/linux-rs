//!
//! This file is part of syscall-rs
//!

#![deny(clippy::all)]

/// System call wrapper.
///
/// Wrapper around `libc` system calls that checks `errno` on failure.
#[macro_export]
macro_rules! syscall {
    ($fn: ident ( $($arg: expr),* $(,)* ) ) => {{
        let res = unsafe { libc::$fn($($arg, )*) };

        if res == -1 {
            Err($crate::Error::Syscall(std::io::Error::last_os_error()))
        } else {
            Ok(res)
        }
    }};
}

mod error;
mod signal;
mod wait;

pub use error::{Error, Result};
pub use signal::{block, restore, Signal, SignalFd, SignalSet};
pub use wait::{wait, WaitStatus};
