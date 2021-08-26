#![allow(dead_code)]
#![allow(unused)]
#![allow(unused_imports)]
use std::cmp::{max, min, Reverse};
use std::collections::{HashMap, HashSet};
use std::io::{self, prelude::*};
use std::str;

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
    let s: Vec<char> = scan.token::<String>().chars().collect();
    let mut res:Vec<char> = vec![];
    let mut pointer:usize= 0;
    for i in s {
        if i == 'B' {
            if pointer != 0 {
                pointer -= 1;
            }
            res.remove(pointer);
        } else if i == 'L' {
            if pointer == 0 {
                continue;
            }
            pointer -= 1;
        } else {
            res.insert(pointer, i);
            pointer += 1;
        }
    }
    writeln!(w, "{}", res.into_iter().collect::<String>());
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = Scanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());
    solve(&mut scan, &mut out);
}
