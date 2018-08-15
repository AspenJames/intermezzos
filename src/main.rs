#![feature(panic_implementation)]
#![feature(core_intrinsics)]
#![no_std]
#![no_main]

extern crate bootloader_precompiled;

use core::intrinsics;
use core::panic::PanicInfo;

#[panic_implementation]
#[no_mangle]
fn panic(_info: &PanicInfo) -> ! {
    unsafe { intrinsics::abort() }
}

#[no_mangle]
pub fn _start() -> ! {
    loop {}
}