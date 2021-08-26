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
    let s: String = scan.read_str(); 
    let mut buffer = String::new();
    let mut flag:char = '0';
    let vowels = "aieou";
    for i in s.chars() {
       

        // I was very tired lol
 
        if flag == i {
            flag = '0';
            continue;
        }

       
        if flag != '0' {
            continue;
        }
        if vowels.contains(i) {
            flag = i; 
            buffer.push(i);
            continue;
        } 
        buffer.push(i);
        
    }
    writeln!(w, "{}", buffer);
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = Scanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());
    solve(&mut scan, &mut out);
}
