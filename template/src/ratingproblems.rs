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
fn sevens(xs: Vec<i32>) -> bool {
    for i in (0..xs.len()) {
        for j in ((i+1)..xs.len()) {
            if xs[i] != xs[j] && xs[i]+xs[j]==7777 {
                return true;
            }
        }
    }
    return false;
}

fn solve<R: BufRead, W: Write>(scan: &mut Scanner<R>, w: &mut W) {
    let n: f32= scan.token();
    let k: f32= scan.token();
    let mut xs: Vec<f32> = Vec::new();
    for i in 0..(k as usize) {
        let ele = scan.token();
        xs.push(ele);
    }
    let avg: f32 = xs.iter().sum();
    let max = (avg+((n-k)*3.0)) / n;
    let min = (avg+((n-k)*(-3.0))) / n;
    writeln!(w ,"{} {}", min, max );

}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = Scanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());
    solve(&mut scan, &mut out);
}
