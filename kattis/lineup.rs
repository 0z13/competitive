#![allow(dead_code)]
#![allow(unused)]
#![allow(unused_imports)]
use std::cmp::Ordering::{Equal, Greater, Less};
use std::cmp::{max, min, Reverse};
use std::collections::btree_set::Intersection;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::f64::consts::PI;
use std::fmt::Display;
use std::io::{self, prelude::*};
use std::iter::FromIterator;
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
    // annoying EOF parsing
    fn read_str(&mut self) -> Option<String> {
        let mut s = String::new();
        let mut si = self.reader.read_line(&mut s);
        if s.eq("\n") {
            return None;
        }
        if s.eq("EOF\n") {
            return None;
        }
        if s.eq("EOF") {
            return None;
        }
        match si {
            // EOF
            Ok(0) => None,
            _ => Some(s.trim_end().to_string()),
        }
    }
}

fn eval(instrs: String, xs: &mut Vec<i32>) -> Option<Vec<i32>> {
    for instr in instrs.chars() {
        match instr {
            'R' => xs.reverse(),
            'D' => {
                if xs.len() == 0 {
                    return None;
                }
                xs.remove(0);
            }
            _ => {}
        }
    }
    Some(xs.clone())
}

fn solve<R, W>(scan: &mut Scanner<R>, w: &mut W)
where
    R: BufRead,
    W: Write,
{
    let n: i32 = scan.token();
    let a: Vec<String> = (0..n).map(|_| scan.token()).collect();
    let mut b = a.clone();
    let mut c = a.clone();
    let len = a.len(); 
    // sort

    b.sort_by(|a,b| a.cmp(b));
    c.sort_by(|a,b| b.cmp(a));
    // join
    let a = a.join(" ");
    let b = b.join(" ");
    let c = c.join(" ");

    if a.chars().zip(b.chars()).filter(|(a,b)| a == b).count() == len {
        writeln!(w, "{}", "INCREASING");
    }
    else if a.chars().zip(c.chars()).filter(|(a,b)| a==b).count() == len {
        writeln!(w, "{}", "DECREASING");
    } else {
        writeln!(w, "{}", "NEITHER");
    }
    
    

}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = Scanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());
    solve(&mut scan, &mut out);
}
