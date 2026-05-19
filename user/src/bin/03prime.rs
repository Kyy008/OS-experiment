#![no_std]
#![no_main]
#[macro_use]
extern crate user_lib;

#[unsafe(no_mangle)]
fn main() -> i32 {
    println!("Prime numbers from 1 to 100:");
    for n in 2..=100 {
        let mut is_prime = true;
        let mut d = 2;
        while d * d <= n {
            if n % d == 0 {
                is_prime = false;
                break;
            }
            d += 1;
        }
        if is_prime {
            print!("{} ", n);
        }
    }
    println!("");
    0
}
