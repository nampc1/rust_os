#![no_std] // https://doc.rust-lang.org/1.30.0/book/first-edition/using-rust-without-the-standard-library.html
#![no_main]

use core::panic::PanicInfo;

/*
Summary:
"never" type (!). Read more: https://doc.rust-lang.org/nightly/std/primitive.never.html
*/

static HELLO: &[u8] = b"Hello World!";

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

mod vga_buffer;

#[no_mangle] // no mangling to make sure the program call _main
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, byte) in HELLO.iter().enumerate() {
        // because we use raw pointers, which cannot guarantee the data pointed to is valid
        // unsafe block tells the compiler that we're sure that the operations are valid
        // unsafe block doesn't skip safety checks
        unsafe {
            // multiply by 2 because we write 2 bytes for every entry
            *vga_buffer.offset(i as isize * 2) = *byte; // string byte
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb; // color byte
        }
    }

    loop {}
}
