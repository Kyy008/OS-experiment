#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

static mut DATA: usize = 200;

#[unsafe(no_mangle)]
fn main() -> i32 {
    unsafe {
        DATA += 2;
        let addr = &raw const DATA as *const usize as usize;
        let value = DATA;
        println!(
            "addr_b: DATA addr = {:#x}, value = {}",
            addr,
            value
        );
    }
    println!("Test addr_b OK!");
    0
}
