#![allow(dead_code)]
#![allow(unused)]
#![allow(unused_imports)]
use std::cmp::{max, min, Reverse};
use std::cmp::Ordering::{Equal,Less,Greater};
use std::collections::btree_set::Intersection;
use std::collections::{HashMap, BTreeMap, HashSet};
use std::io::{self, prelude::*};
use std::iter::FromIterator;
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


fn unique_digits(x: i64) -> bool {
    let mut counter =0;
    let mut set = HashSet::new();
    let mut num = x;

    while (num > 0) {
        set.insert(num % 10);
        num = num / 10;
        counter += 1;
    }

    counter == set.len()

}

fn div_by_self(x: i64) -> bool {
    let mut num = x;

    while (num > 0) {
        if num % 10 == 0 {
            return false
        }
        if (x % (num % 10) != 0) {
            return false;
        }
        num = num / 10;
    }
    true
}


fn fact(n:f64) -> f64 {
    match n {
        0.0    => 1.0,
        _    => n * fact(n-1.0),
    }
}

fn solve<R, W>(scan: &mut Scanner<R>, w: &mut W)
where
    R: BufRead,
    W: Write,
{
    let n:i32= scan.token();
    let mut f = 0.0;
    let mut sum: f64 = 0.0;
    for i in 0..=n {
        sum += (1.0/fact(f));
        f += 1.0;
    }
    writeln!(w, "{:.15}", sum);
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = Scanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());
    solve(&mut scan, &mut out);
}
