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
fn solve<R: BufRead, W: Write>(scan: &mut Scanner<R>, w: &mut W) {
    let s:Vec<char> = scan.read_str().to_ascii_uppercase().trim().chars().collect();
    let mut counter = 0;
    let mut zz = 0; 
    let mut id1 = 0;
    let mut id2 = 1;
    let mut id3 = 2;
    while (zz != (s.len() / 3)) {
        if s[id1] != 'P' {
            counter += 1;
        }
        if s[id2] != 'E'{
            counter += 1;
        }
        if s[id3] != 'R'{
            counter += 1;
        } 
        zz += 1;
        id1 += 3;
        id2 += 3;
        id3 += 3;
    }

    writeln!(w, "{:?}", counter);
}
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = Scanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());
    solve(&mut scan, &mut out);
}
