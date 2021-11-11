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
    let t: i32 = scan.token();
    let mut v: Vec<i32> = (0..n).map(|_| scan.token()).collect();

    match t {
        1 => {
            writeln!(w, "{}", 7);
        }
        2 => {
            let mut s = String::from("");
            if v[0] > v[1] {
                s = String::from("Bigger");
            } else if v[1] > v[0] {
                s = String::from("Smaller");
            } else {
                s = String::from("Equal");
            }
            writeln!(w, "{}", s);
        }
        3 => {
            let mut v = [v[0], v[1], v[2]];
            v.sort();
            writeln!(w, "{}", v[1]);
        }
        4 => {
            let res: i32 = v.into_iter().sum();
            writeln!(w, "{}", res);
        }
        5 => {
            let sm: i32 = v.into_iter().filter(|x| x % 2 == 0).sum();
            writeln!(w, "{}", sm);
        }
        6 => {
            let tmp: Vec<u8> = v
                .into_iter()
                .map(|x| (x as u8 % 26) + ('a' as u8))
                .collect();

            for i in tmp {
                write!(w, "{}", i as char);
            }
            writeln!(w, "{}", "");
        }

        7 => {
            let mut idx = v[0];
            v[0] = -1;
            loop {
                if idx == -1 {
                    println!("{}", "Cyclic");
                    break;
                }
                if idx >= v.len() as i32 {
                    println!("{}", "Out");
                    break;
                }
                if idx == (v.len() - 1) as i32 {
                    println!("{}", "Done");
                    break;
                }
                v[idx as usize] = -1;
                idx = v[idx as usize];
            }
        }

        _ => {
            panic!("these instructions suck");
        }
    }
}

fn seven(v: Vec<i32>, counter: usize, index: usize) {
    let idx: usize = v[index] as usize;
    if v.len() - 1 < idx {
        println!("{}", "Out");
        return;
    } else if v.len() - 1 == idx {
        println!("{}", "Done");
        return;
    }
    seven(v, (counter + 1), idx)
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = Scanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());
    solve(&mut scan, &mut out);
}
