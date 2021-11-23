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
    let n:i32 = scan.token();
    let mut v: Vec<i32> = (0..n).map(|_| scan.token()).collect();
    v.sort();
    let mut prev = v.remove(0);
    let mut res : Vec<Vec<i32>> = vec![];
    let mut builder: Vec<i32> = vec![prev];
    for i in v {
        if (i-1) == prev {
            builder.push(i);
        } else {
            res.push(builder.clone());
            builder = vec![];
            builder.push(i);
        }
        prev = i;
    }
    res.push(builder.clone());
    for i in res {
        if i.len() < 3 {
            for j in i {
                print!("{} ", j);
            } 
        }
       else {
            let max = i.iter().max().unwrap();
            let min = i.iter().min().unwrap();
            print!("{}-{} ", min, max);
        }
    }

}
// fn solve<R, W>(scan: &mut Scanner<R>, w: &mut W)
// where
//     R: BufRead,
//     W: Write,
// {
//     let n:i32 = scan.token();
//     let mut v: Vec<char> = (0..n).map(|_| scan.token()).collect();
//     let stack:Vec<char> = vec![];
//     let mut index = 0;
//     loop {
//         if v.len() == 0 {
//             break;
//         }
//         let c = v.remove(0);
//         if c == '(' || c == '{' || c == '[' {
//             stack.push(curr);
//         } else {
//             l_paren = stack.pop()
//             match l_paren {
//                 Some(x) => {
//                     if x == '(' && c == ')' {
                        
//                     }
//                 } 
//                 None => {
//                     println!("{}", "wrongwrong")
//                     return;
//                 }
//             }
//         }
//     }
// }

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = Scanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());
    solve(&mut scan, &mut out);
}
