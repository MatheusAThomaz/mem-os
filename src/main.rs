#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// #[no_mangle] doesn't allow the compiler to overwrite the function name
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}
