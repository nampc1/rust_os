#![no_std] // https://doc.rust-lang.org/1.30.0/book/first-edition/using-rust-without-the-standard-library.html
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use x86_64::instructions::port::Port;

/*
Summary:
"never" type (!). Read more: https://doc.rust-lang.org/nightly/std/primitive.never.html
*/

#[warn(dead_code)]
static HELLO: &[u8] = b"Hello World!";

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    
    loop {}
}

mod vga_buffer;

// slice type https://doc.rust-lang.org/std/primitive.slice.html
// trait object https://doc.rust-lang.org/1.30.0/book/first-edition/trait-objects.html

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    
    for test in tests {
        test();
    }
    
    exit_qemu(QemuExitCode::Success);
}

#[test_case]
fn trivial_assertion() {
    print!("trivial assertion...");
    assert_eq!(1, 1);
    println!("ok");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;
    
    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

#[no_mangle] // no mangling to make sure the program call _main
pub extern "C" fn _start() -> ! {
    println!("Hello World {}", "!");
    
    #[cfg(test)]
    test_main();
    
    // vga_buffer::print_sth();
    // let vga_buffer = 0xb8000 as *mut u8;
    // 
    // for (i, byte) in HELLO.iter().enumerate() {
    //     // because we use raw pointers, which cannot guarantee the data pointed to is valid
    //     // unsafe block tells the compiler that we're sure that the operations are valid
    //     // unsafe block doesn't skip safety checks
    //     unsafe {
    //         // multiply by 2 because we write 2 bytes for every entry
    //         *vga_buffer.offset(i as isize * 2) = *byte; // string byte
    //         *vga_buffer.offset(i as isize * 2 + 1) = 0xb; // color byte
    //     }
    // }

    loop {}
}
