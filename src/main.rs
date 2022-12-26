#![no_std]
#![no_main]

use core::panic::PanicInfo;


fn main() {
    // println!("Hello, world!");
}

/// 这个函数将在 panic 时被调用
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}