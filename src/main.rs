// dont link std lib
#![no_std]

#![no_main]
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> !{
    loop {}
}

// since we're not linking std lib, we need to define a panic handler
// Return type is ! signifying the "never" return type
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
