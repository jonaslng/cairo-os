#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(cairo_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use cairo_os::println;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    #[cfg(test)]
    test_main();

    loop {}
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    cairo_os::test_panic_handler(info)
}

/// This function is a trivial test case that asserts that 1 equals 1.
#[test_case]
fn trivial_assertion() {
    assert_eq!(0, 0);
}

// This module provides a way to exit QEMU with a specific exit code.
// It uses the `isa-debug-exit` device, which is a standard way to signal QEMU to exit gracefully. The exit code is written to the port `0xf4
// and can be read by the host system to determine the exit status of the virtual machine.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        println!("{}...\t", core::any::type_name::<T>());
        self();
        println!("[ok]");
    }
}