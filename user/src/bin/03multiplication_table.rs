#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

#[no_mangle]
fn main() -> i32 {
    println!("Multiplication Table:");
    for i in 1..=9 {
        for j in 1..=i {
            print!("{}x{}={} ", j, i, i * j);
        }
        println!("");
    }
    0
}
