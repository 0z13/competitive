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



fn solve<R: BufRead, W: Write>(scan: &mut Scanner<R>, w: &mut W) {

    let n: i32 = scan.token();
    let v: Vec<String> = (0..n).map(|_| scan.token()).collect();
    let c =v.iter().map(|elem| elem.to_ascii_lowercase()) 
            .filter(|p| p.contains(&"rose") || p.contains(&"pink")).count();
    if c == 0 {
        writeln!(w, "{}", "I must watch Star Wars with my daughter");
    } else {
    writeln!(w, "{}", c);
    }
} 


fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = Scanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());
    solve(&mut scan, &mut out);
}
