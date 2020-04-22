/***********************************************************************************************************************
 * Copyright (c) 2019 by the authors
 *
 * Author: André Borrmann
 * License: Appache License 2.0
 **********************************************************************************************************************/
#![doc(html_root_url = "https://docs.rs/ruspiro-gpio-hal/0.1.0")]
#![cfg_attr(not(any(test, doctest)), no_std)]
//! # GPIO Hardware Abstraction Layer
//!
//! This provided traits allowsto abstract the specific implementation of the GPIO peripheral access that
//! will be different on each specific embedded system. The provided function can be implemented individual the target /
//! hardware but keeps the implementation details away from crates that wants to use the GPIO peripheral for the 
//! functionality they provide.
//! 
//! One implementation for the Raspberry Pi could be found [here](https://crates.io/crates/ruspiro-gpio)
//!

extern crate alloc;

mod pin;
pub use pin::*;

mod gpio;
pub use gpio::*;

mod events;
pub use events::*;