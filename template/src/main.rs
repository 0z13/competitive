#![allow(dead_code)]
#![allow(unused)]
#![allow(unused_imports)]
#![feature(slice_group_by)]
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
    let opt: i32 = scan.token();
    let mut v: Vec<i32> = (0..n).map(|_| scan.token()).collect();

    match opt {
        1 => {
            let mut f = true;
            for i in &v {
                for j in &v {
                    if i != j && i + j == 7777 {
                        writeln!(w, "{}", "Yes");
                        f = false;
                        break;
                    }
                }
            }
            if f {
                writeln!(w, "{}", "No");
            }
        }
        2 => {
            let l1 = v.len();
            let l2: HashSet<i32> = HashSet::from_iter(v);
            if l1 == l2.len() {
                writeln!(w, "{}", "Unique");
            } else {
                writeln!(w, "{}", "Contains duplicate");
            }
        }
        3 => {
            let mut map: HashMap<&i32, usize> = HashMap::new();
            let mut mx = 0;
            let mut entry = 0;
            for i in &v {
                let counter = map.entry(i).or_insert(0);
                *counter += 1;
            }

            for (i, j) in map {
                if j > mx {
                    mx = j;
                    entry = *i;
                }
            }

            if v.len() % 2 == 0 && mx >= (v.len() / 2) {
                writeln!(w, "{}", entry);
            } else if v.len() % 2 != 0 && mx >= (v.len() / 2 + 1) {
                writeln!(w, "{}", entry);
            } else {
                writeln!(w, "{}", -1);
            }
        }
        4 => {
            v.sort();
            let l = v.len();
            let idx = l / 2;
            if l % 2 != 0 {
                writeln!(w, "{}", v[idx]);
            } else {
                writeln!(w, "{} {}", v[idx - 1], v[idx]);
            }
        }
        5 => {
            let mut xs: Vec<i32> = v.into_iter().filter(|x| x >= &100 && x <= &999).collect();
            xs.sort();
            for i in xs {
                write!(w, "{} ", i);
            }
        }
        _ => {}
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = Scanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());
    solve(&mut scan, &mut out);
}
