#![no_std]
#![no_main]

use core::panic::PanicInfo;
// use std::println;

mod vga_buffer;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Welcome to{}", "!");
    println!("     ___  __   __  _______  _______  __    _  _______        _______  _______ \n");
    println!("    |   ||  | |  ||       ||   _   ||  |  | ||       |      |       ||       |\n");
    println!("    |   ||  | |  ||  _____||  |_|  ||   |_| ||    ___|      |   _   ||  _____|\n");
    println!("    |   ||  |_|  || |_____ |       ||       ||   | __       |  | |  || |_____ \n");
    println!(" ___|   ||       ||_____  ||       ||  _    ||   ||  |      |  |_|  ||_____  |\n");
    println!("|       ||       | _____| ||   _   || | |   ||   |_| |      |       | _____| |\n");
    println!("|_______||_______||_______||__| |__||_|  |__||_______|      |_______||_______|\n");
    println!("\n                                                   Corporation Version 1.0.0\n");


    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}