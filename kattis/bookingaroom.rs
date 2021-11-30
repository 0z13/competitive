#![allow(dead_code)]
#![allow(unused)]
#![allow(unused_imports)]
use crate::Instr::{DefineExpr, EvalExpr};
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
type Op = char;

#[derive(Debug)]
enum Instr {
    DefineExpr(i32, String),
    EvalExpr(Op, String, String),
}

fn parse(xs: Vec<Vec<String>>) -> Vec<Instr> {
    let mut res: Vec<Instr> = vec![];

    for v in xs {
        if v[0].eq(&String::from("define")) {
            let x: i32 = v[1].parse().unwrap();
            let a = DefineExpr(x, v[2].clone());
            res.push(a)
        }
        if v[0].eq(&String::from("eval")) {
            let op = v[2].parse::<char>().unwrap();
            let a = EvalExpr(op, v[1].clone(), v[3].clone());
            res.push(a);
        }
    }
    res
}

fn solve<R: BufRead, W: Write>(scan: &mut Scanner<R>, w: &mut W) {
    let r: i32 = scan.token();
    let n: i32 = scan.token();
    if r == n {
        writeln!(w, "too late");
        return;
    }

    let mut xs = (0..n).map(|_| scan.token()).collect::<Vec<i32>>();
    xs.sort();

    for i in (1..=r) {
        if !xs.contains(i) {
            writeln(w, "{}", i);
            return;
        }
    }
}
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = Scanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());
    solve(&mut scan, &mut out);
}
