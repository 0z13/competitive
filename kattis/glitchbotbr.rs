#![allow(dead_code)]
#![allow(unused)]
#![allow(unused_imports)]
use std::cmp::{max, min, Reverse};
use std::cmp::Ordering::{Equal,Less,Greater};
use std::collections::{HashMap, BTreeMap, HashSet};
use std::io::{self, prelude::*};
use std::str;
use std::f64::consts::PI;
use std::fmt::Display;
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

fn print_elements<T: Display>(v: &Vec<T>) {
    for i in v {
        print!("{} ", i);
    }     
    println!("");
}


#[derive(Debug, Copy, Clone, PartialEq)]
enum Direction {
    Left,
    Right,
    Forward,
    Backward
}

#[derive(Debug)]
struct Robot {
    x: i32,
    y: i32,
    facing:Direction,
}

impl Robot {
    fn rotate(&mut self, instr: String) {
        match (self.facing, instr.as_str()) {
            (Direction::Forward, "Left") => {self.facing = Direction::Left;}
            (Direction::Forward, "Right") => {self.facing = Direction::Right;}
            (Direction::Backward, "Left") => {self.facing = Direction::Right;}
            (Direction::Backward, "Right") => {self.facing = Direction::Left;}
            (Direction::Left, "Left") => {self.facing = Direction::Backward;}
            (Direction::Left, "Right") => {self.facing = Direction::Forward;}
            (Direction::Right, "Left") => {self.facing = Direction::Forward;}
            (Direction::Right, "Right") => {self.facing = Direction::Backward;}
            _                         => {}
        }
    }
    fn step(&mut self) {
        match (self.facing) {
            Direction::Left    => {self.x -= 1;}
            Direction::Right   => {self.x += 1;}
            Direction::Forward => {self.y += 1;}
            Direction::Backward => {self.y -= 1;}
        }
    }

}


fn solve<R: BufRead, W: Write>(scan: &mut Scanner<R>, w: &mut W) {
    let x = scan.token::<i32>();
    let y = scan.token::<i32>();
    let mut robot = Robot{x, y, facing:Direction::Forward};
    let n: i32 = scan.token();
    let instructions:Vec<String> = (0..n).map(|_| scan.token()).collect();
    let mut counter = 0;
    for i in instructions {
        counter += 1;

        println!("ME STUFF NOW:");
        println!("{:?}", robot);
        match i.as_str() {
            "Forward" => {

                if (robot.facing == Direction::Backward && robot.x + 1 == x && y == robot.y ) {
                    println!("{} Left", counter);
                }

                if (robot.facing == Direction::Backward && robot.x - 1 == x && y == robot.y ) {
                    println!("{} Right", counter);
                }

                if (robot.facing == Direction::Forward && robot.x + 1 == x && y == robot.y ) {
                    println!("{} Right", counter);
                }
                
                if (robot.facing == Direction::Forward && robot.x - 1 == x && y == robot.y ) {
                    println!("{} Left", counter);
                }

                if (robot.facing == Direction::Right && robot.x == x && y + 1 == robot.y ) {
                    println!("{} Left", counter);
                }

                if (robot.facing == Direction::Right && robot.x == x && y - 1 == robot.y ) {
                    println!("{} Right", counter);
                }

                if (robot.facing == Direction::Left && robot.x == x && y - 1 == robot.y ) {
                    println!("{} Left", counter);
                }
                if (robot.facing == Direction::Right && robot.x == x && y + 1 == robot.y ) {
                    println!("{} Right", counter);
                }
                robot.step();}
            "Left" => {robot.rotate(i);}
            "Right" => {robot.rotate(i);}
            _ => {panic!("weird parsing");}
        }
    }

} 


fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = Scanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());
    solve(&mut scan, &mut out);
}
