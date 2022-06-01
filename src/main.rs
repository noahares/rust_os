#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use rust_os::println;

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    // this function is the entry point, since the linker looks for a function
    // named `_start` by default

    // safe write through dedicated interface
    println!("Hello World{}", "!");

    rust_os::init();

    // invoke a breakpoint exception
    x86_64::instructions::interrupts::int3();
    // panic
    // panic!("Some panic message");

    // tests
    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    loop {}
}

// This function is called on panic.
#[cfg(not(test))] // conditional compilation
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_os::test_panic_handler(info)
}
