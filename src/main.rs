#![no_std] // https://doc.rust-lang.org/1.30.0/book/first-edition/using-rust-without-the-standard-library.html
#![no_main]

use core::panic::PanicInfo;

/*
Summary:
"never" type (!). Read more: https://doc.rust-lang.org/nightly/std/primitive.never.html
*/

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle] // no mangling to make sure the program call _main
pub extern "C" fn _main() -> ! {
    loop {}
}
    