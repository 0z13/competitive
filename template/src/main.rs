#![allow(dead_code)]
#![allow(unused)]
#![allow(unused_imports)]
use std::cmp::{max, min, Reverse};
use std::cmp::Ordering::{Equal,Less,Greater};
use std::collections::{HashMap, BTreeMap, HashSet};
use std::io::{self, prelude::*};
use std::str;
use std::f64::consts::PI;
use std::fmt::Display;
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
    // annoying EOF parsing
    fn read_str(&mut self) -> Option<String>{
        // should probably trim stuff
        let mut s = String::new();
        let mut si = self.reader.read_line(&mut s);
        if s.eq("\n") {
            return None
        }
        if s.eq("EOF\n") {
            return None
        }
        if s.eq("EOF") {
            return None
        }
        match si {
            // EOF
            Ok(0) => None,
            _     => Some(s)
        }
  }
}

fn song_decoder(song: &str) -> String {
    let s = song.to_string();
    let xs: String = s.replace("WUB", " ");
    let xs = xs.trim();
    let xs = xs.split(" ");
    println!("{:?}", xs);
    String::from("fuck mig")
}


fn high_and_low(numbers: &str) -> String {
    let xs = numbers.split(" ").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    let min = xs.iter().max().copied().unwrap();
    let max= xs.iter().min().unwrap();
    format!("{} {}", max, min)
}


fn solve<R: BufRead, W: Write>(scan: &mut Scanner<R>, w: &mut W) {

}
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = Scanner::new(stdin.lock());
    let s = high_and_low("3 9 3 2 1");
    println!("{}", s);
    let mut out = io::BufWriter::new(stdout.lock());
    solve(&mut scan, &mut out);
}
