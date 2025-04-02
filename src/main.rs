#![no_std] // 不链接 Rust 标准库
#![no_main] // 禁用所有 Rust 层级的入口点


mod vga_buffer;

use core::panic::PanicInfo;
use vga_buffer::print_something;


///  panic 时被调用
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

//static HELLO: &[u8] = b"Welcome to zz_os!";


// #[unsafe(no_mangle)]
// pub extern "C" fn _start() -> ! {
//     let vga_buffer = 0xb8000 as *mut u8;

//     for (i, &byte) in HELLO.iter().enumerate() {
//         unsafe {
//             *vga_buffer.offset(i as isize * 2) = byte;
//             *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
//         }
//     }

//     loop {}
// }
#[no_mangle]
pub extern "C" fn _start() -> ! {
    print_something();
    loop {}
}