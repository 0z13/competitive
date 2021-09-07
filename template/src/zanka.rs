#![allow(dead_code)]
#![allow(unused)]
#![allow(unused_imports)]
use std::cmp::{max, min, Reverse};
use std::collections::{HashMap, BTreeMap, HashSet};
use std::io::{self, prelude::*};
use std::str;
use std::f64::consts::PI;
struct Scanner<R> {
    reader: R,
    buf_str: Vec<u8>,
    buf_iter: str::SplitWhitespace<'static>,
}
impl<R: BufRead> Scanner<R> {
    fn new(reader: R) -> Self {
        Self {
            reader,
            buf_str: vec![],
            buf_iter: "".split_whitespace(),
        }
    }
    fn token<T: str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buf_iter.next() {
                return token.parse().ok().expect("Failed parse");
            }
            self.buf_str.clear();
            self.reader
                .read_until(b'\n', &mut self.buf_str)
                .expect("Failed read");
            self.buf_iter = unsafe {
                let slice = str::from_utf8_unchecked(&self.buf_str);
                std::mem::transmute(slice.split_whitespace())
            }
        }
    }
    fn read_str(&mut self) -> String {
        let mut s = String::new();
        self.reader.read_line(&mut s);
        return s; 
   }
}

fn digit_sum(x:i32) -> i32{
    let mut num = x;
    let mut sum = 0;
    while (num > 0) {
        sum += (num % 10);
        num /= 10;
    }
    return sum;
}

fn solve<R: BufRead, W: Write>(scan: &mut Scanner<R>, w: &mut W) {
    let l: i32 = scan.token();
    let d: i32 = scan.token();
    let x: i32 = scan.token();
    
    let mut min = l;
    let mut max = d;
    loop {
        if (digit_sum(min) == x) {
            writeln!(w, "{}", min);
            break;
        } else {
            min += 1;
        }
    }

    loop {
        if (digit_sum(max) == x) {
            writeln!(w, "{}", max);
            break;
        } else {
            max -= 1;
        }
    }
}
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = Scanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());
    solve(&mut scan, &mut out);
}
