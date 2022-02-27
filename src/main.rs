#![no_std]
#![no_main]
#![feature(const_mut_refs)]

use core::fmt::Write;

mod vga_buffer;

use core::panic::PanicInfo;

#[no_mangle]
#[allow(clippy::empty_loop)]
pub extern "C" fn _start() -> ! {
    vga_buffer::WRITER.lock().write_byte(b'H');
    vga_buffer::WRITER.lock().write_string("ello ");
    vga_buffer::WRITER.lock().write_string("Wörld! ");
    write!(vga_buffer::WRITER.lock(), "Numbers: {} {}", 42, 1.337).unwrap();
    println!();
    println!("println test with {} and strings: {}", "numbers", -123);
    panic!("test");

    loop {} // function isn't allowed to exit, so just infinitely loop at the end
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
