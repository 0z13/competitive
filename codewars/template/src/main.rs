
#![allow(unused)]
#![allow(dead_code)]

use std::thread;
fn main() {

    #![feature(available_concurrency)]
    let count = thread::available_concurrency();
    println!("{}", count.unwrap());
}
