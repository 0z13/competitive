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


fn moves(mut s: Vec<char>, mut t: Vec<char>) -> i32 {
    let mut counter = 0;
    let mut diff = Vec::new();
    if s.len() !=  t.len() {
        return -1
    }
    for i in 0..s.len() {
        if s[i] != t[i] && s[i] != '?' {
            diff.push(i);
        }
    } 

    // transform ?
    for i in 0..s.len() {
        if s[i] == '?' {
            let tmp = t[i];
            s[i] = tmp;
            counter += 1;
        }
    }



    // Try to swap
    for mut i in &diff {
        if s.eq(&t) {
            // does this work?
            break;
        }
        for j in &diff {
            
            if s[*i] != t[*i] && t[*i] == s[*j] && t[*j] == s[*i] && i != j {
                let tmp = s[*i];
                s[*i] = s[*j];
                s[*j] = tmp;
                counter += 1;
            }
        }
    }

    
    for i in 0..s.len(){
        if s[i] == '0' && t[i] == '1' {
            s[i] = '1';
            counter += 1
        }
    }

    if !s.eq(&t) {
        return -1
    }
    counter
}



fn moves2(mut s: Vec<char>, mut t: Vec<char>) -> i32 {
    let mut m:HashMap<_,_> = vec![('0', 0), ('1',0), ('2', 0)].into_iter().collect();
    for i in 0..s.len() {
        if s[i] != t[i] {
            let entry = m.entry(s[i]).or_insert(0);
            *entry += 1;
        } 
    }
    if s.iter().filter(|x| **x == '1').count() != t.iter().filter(|x| **x == '0').count() {
        return 0;
    }
    let zeros = m.get(&'0').unwrap();
    let ones  = m.get(&'1').unwrap();

    
    if zeros >= ones {
        return ones + m.get(&'1').unwrap() + m.get(&'?').unwrap();
    } else {
        return zeros + m.get(&'0').unwrap() + m.get(&'?').unwrap();
    }

    0
   
}
fn solve<R: BufRead, W: Write>(scan: &mut Scanner<R>, w: &mut W) {
    let n: i32 = scan.token();

    for i in 1..n+1 {
        let mut s: Vec<char> = scan.token::<String>().chars().collect();
        let mut t: Vec<char> = scan.token::<String>().chars().collect();
        writeln!(w, "Case {}: {}", i, moves2(s,t));
    }
}


fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = Scanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());
    solve(&mut scan, &mut out);
}
