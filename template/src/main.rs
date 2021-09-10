#![allow(dead_code)]
#![allow(unused)]
#![allow(unused_imports)]
use std::cmp::{max, min, Reverse};
use std::cmp::Ordering::{Equal,Less,Greater};
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
    let n = scan.token::<i32>();
    let mut anywhere = true;
    for i in 0..n {
        let items = scan.token::<i32>();
        let name: String = scan.read_str().unwrap();
        let mut p =[false, false];

        for i in 0..items {
            let f = scan.read_str().unwrap();
            let f = f.trim();
            if f.eq("pea soup") {
                p[0] = true;
            }
            if f.eq("pancakes") {
                p[1] = true;
            }
        }
        if p[0] == true && p[1] == true {
            writeln!(w, "{}", name);
            anywhere = false;
            break;
        }
    }
    if anywhere {
        writeln!(w, "{}", "Anywhere is fine I guess");
    }
}


fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = Scanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());
    solve(&mut scan, &mut out);
}
