#![no_std]
#![no_main]

mod vga_buffer;

//เข้าไปในcore หยิบ panic มาใช้ เอาตรง PanicInfo เพื่อมาเตือนตอนพัง
use core::panic::PanicInfo;

static GREETING: &[u8] = b"Sawasdee!";

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;
    for (i, &byte) in GREETING.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
    loop {}
}
