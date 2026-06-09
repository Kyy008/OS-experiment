#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

static mut DATA: usize = 100;

#[unsafe(no_mangle)]
fn main() -> i32 {
    unsafe {
        DATA += 1;
        let addr = &raw const DATA as *const usize as usize;
        let value = DATA;
        println!(
            "addr_a: DATA addr = {:#x}, value = {}",
            addr,
            value
        );
    }
    println!("Test addr_a OK!");
    0
}
