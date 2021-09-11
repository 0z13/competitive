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

    for i in 0..n {
        let s:Vec<char> = scan.read_str().unwrap().chars().collect();
        let mut sbuf = String::new();
        let mut p = '0';
        for i in s {

            let s = match i.to_ascii_uppercase() {
                'A' => "2",
                'B' => "22",
                'C' => "222",
                'D' => "3",
                'E' => "33",
                'F' => "333",
                'G' => "4",
                'H' => "44",
                'I' => "444",
                'J' => "5",
                'K' => "55",
                'L' => "555",
                'M' => "6",
                'N' => "66",
                'O' => "666",
                'P' => "7",
                'Q' => "77",
                'R' => "777",
                'S' => "7777",
                'T' => "8",
                'U' => "88",
                'V' => "888",
                'W' => "9",
                'X' => "99",
                'Y' => "999",
                'Z' => "9999",
                ' ' => "0",
                '\n' => "\n",
                _ => panic!("heyho")
            };
            let t = s.chars().nth(0).unwrap();
            if p == t {
                sbuf.push(' ');
            }
            sbuf.push_str(s);
            p = t; 
        } 
        write!(w, "Case #{}: {}", (1+i), sbuf);
    }
}


fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = Scanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());
    solve(&mut scan, &mut out);
}
