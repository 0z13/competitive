use std::{ascii::AsciiExt, cmp::max, slice::Split};

fn main() {
    println!("{}", infected(&"01000000X000X011X0X"));
}

fn infected(s: &str) -> f64 {
    if !s.contains('1') {
        return 0.0
    }
    let number_of_cases = s.chars().filter(|x| *x != 'X').count();
    let mut counter = 0;
    let ss = s.clone().to_string();
    let ss:Vec<&str> = ss.split('X').collect();
    for i in ss {
        if i.contains('1') {
            counter += i.len();
        }
    }
    (counter as f64 / number_of_cases as f64) * 100
}


