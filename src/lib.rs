/* Copyright (C) 2021 by Jacob Alexander */

#![no_std]
#![feature(lang_items)]

// ----- Crates -----

// ----- Modules -----

mod he;

// ----- Embedded Functionality -----

#[cfg(not(feature = "std"))]
use core::panic::PanicInfo;

#[cfg(not(feature = "std"))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// ----- Host Functionality -----

#[cfg(not(feature = "std"))]
#[lang = "eh_personality"]
extern "C" fn eh_personality() {}
