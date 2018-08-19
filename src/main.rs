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
    let slice = unsafe { core::slice::from_raw_parts_mut(0xb8000 as *mut u8, 4000) };

    slice[0] = b'h';
    slice[1] = 0x04;
    slice[2] = b'e';
    slice[3] = 0x0E;
    slice[4] = b'l';
    slice[5] = 0x02;
    slice[6] = b'l';
    slice[7] = 0x09;
    slice[8] = b'o';
    slice[9] = 0x0D; 
    slice[10] = b' ';
    slice[11] = 0x05;
    slice[12] = b'w';
    slice[13] = 0x04;
    slice[14] = b'o';
    slice[15] = 0x0E;
    slice[16] = b'r';
    slice[17] = 0x02;
    slice[18] = b'l';
    slice[19] = 0x09;
    slice[20] = b'd';
    slice[21] = 0x0D;

    loop {}
}
