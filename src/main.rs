
#![no_std]


#![no_main]

//เข้าไปในcore หยิบ panic มาใช้ เอาตรง PanicInfo เพื่อมาเตือนตอนพัง
use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}


#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    loop {}
}