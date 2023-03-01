// All #! expressions should be at start of file?
#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

#![feature(custom_test_frameworks)] // Include feature custom test framework
#![test_runner(albos::test_runner)] // Register our test runner for custom test framework
#![reexport_test_harness_main = "test_main"] // Redefines test entry point

use core::panic::PanicInfo;
use albos::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    #[cfg(test)]
    test_main(); // Call new test entrypoint, but only if testing
    
    loop {}
}

/// Normal panic handler
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    albos::test_panic_handler(info)
}
