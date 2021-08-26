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

fn f(mut xs:Vec<String>) -> i32 {
    let mut red = 0;
    let mut black = 0;
    let mut red_nums = vec![];
    let mut ones = 1;
    let mut black_nums = vec![];
    for i in &mut xs {
        
        let comp = i.pop().unwrap();
        if comp == 'R' {
            red += 1;
            let mut x =i.parse::<i32>().unwrap();
            x -= 1;
            red_nums.push(x);
        } else if comp == 'B' {
            black += 1;
            let mut x = i.parse::<i32>().unwrap();
            x -= 1;
            black_nums.push(x);
        }
    }
    let n = min(red, black);
    if (n == 0) {return 0;}
    let mut v = vec![];
    red_nums.sort();
    black_nums.sort();
    for i in 0..n {
        let rnum = red_nums.pop().unwrap();
        if rnum == 1 { ones += 1 }
        v.push(rnum);
        let bnum = black_nums.pop().unwrap();
        if bnum == 1 { ones += 1 }
        v.push(bnum);
    }

    let s = v.iter().sum::<i32>();
    s 
}


fn solve<R: BufRead, W: Write>(scan: &mut Scanner<R>, w: &mut W) {
    let n: i32 = scan.token();

    for i in 0..n {
        let nn: i32 = scan.token();
        let mut xs: Vec<String> = vec![];
        for i in 0..nn {
            xs.push(scan.token::<String>()); 
        }
        writeln!(w, "Case #{}: {}", (i+1), f(xs));
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = Scanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());
    solve(&mut scan, &mut out);
}
