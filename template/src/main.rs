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



fn solve<R: BufRead, W: Write>(scan: &mut Scanner<R>, w: &mut W) {
    let mut map: BTreeMap<char ,Vec<char>> = BTreeMap::new();
    // ~_~
    map.insert('G', String::from("G:                                                           ").chars().collect());
    map.insert('F', String::from("F: ----------------------------------------------------------").chars().collect());
    map.insert('E', String::from("E:                                                           ").chars().collect());
    map.insert('D', String::from("D: ----------------------------------------------------------").chars().collect());
    map.insert('C', String::from("C:                                                           ").chars().collect());
    map.insert('B', String::from("B: ----------------------------------------------------------").chars().collect());
    map.insert('A', String::from("A:                                                           ").chars().collect());
    map.insert('g', String::from("g: ----------------------------------------------------------").chars().collect());
    map.insert('f', String::from("f:                                                           ").chars().collect());
    map.insert('e', String::from("e: ----------------------------------------------------------").chars().collect());
    map.insert('d', String::from("d:                                                           ").chars().collect());
    map.insert('c', String::from("c:                                                           ").chars().collect());
    map.insert('b', String::from("b:                                                           ").chars().collect());
    map.insert('a', String::from("a: ----------------------------------------------------------").chars().collect());
    

    let n:i32 = scan.token();
    let mut counter:usize = 3;
    for i in 0..n {
        let mut c = scan.token::<String>();
        let mut c = c.chars();
        match c.next().unwrap() {
            'D' => {
                let num = c.as_str().parse::<i32>();
                let e = map.entry('D').or_default();
                match num {
                    Ok(n) => {
                        for i in 0..n {
                            e[counter] = '*';
                            counter += 1;
                        }
                            e[counter] = ' ';
                    },

                    Err(err) => {
                            e[counter] = '*';
                            counter += 1;
                            e[counter] = ' ';
                    }
                }
            },
            'G' => {
                let num = c.as_str().parse::<i32>();
                let e = map.entry('G').or_default();
                match num {
                    Ok(n) => {
                        for i in 0..n {
                            e[counter] = '*';
                            counter += 1;
                        }
                            e[counter] = ' ';
                    },

                    Err(err) => {
                            e[counter] = '*';
                            counter += 1;
                            e[counter] = ' ';
                    }
                }
            },
             'F' => {
                let num = c.as_str().parse::<i32>();
                let e = map.entry('F').or_default();
                match num {
                    Ok(n) => {
                        for i in 0..n {
                            e[counter] = '*';
                            counter += 1;
                        }
                            e[counter] = ' ';
                    },

                    Err(err) => {
                            e[counter] = '*';
                            counter += 1;
                            e[counter] = ' ';
                    }
                }
            },
              'E' => {
                let num = c.as_str().parse::<i32>();
                let e = map.entry('E').or_default();
                match num {
                    Ok(n) => {
                        for i in 0..n {
                            e[counter] = '*';
                            counter += 1;
                        }
                            e[counter] = ' ';
                    },

                    Err(err) => {
                            e[counter] = '*';
                            counter += 1;
                            e[counter] = ' ';
                    }
                }
            },
               'C' => {
                let num = c.as_str().parse::<i32>();
                let e = map.entry('C').or_default();
                match num {
                    Ok(n) => {
                        for i in 0..n {
                            e[counter] = '*';
                            counter += 1;
                        }
                            e[counter] = ' ';
                    },

                    Err(err) => {
                            e[counter] = '*';
                            counter += 1;
                    }
                }
            },                      
               'B' => {
                let num = c.as_str().parse::<i32>();
                let e = map.entry('B').or_default();
                match num {
                    Ok(n) => {
                        for i in 0..n {
                            e[counter] = '*';
                            counter += 1;
                        }
                            e[counter] = ' ';
                    },

                    Err(err) => {
                            e[counter] = '*';
                            counter += 1;
                            e[counter] = ' ';
                    }
                }
            },            
               'A' => {
                let num = c.as_str().parse::<i32>();
                let e = map.entry('A').or_default();
                match num {
                    Ok(n) => {
                        for i in 0..n {
                            e[counter] = '*';
                            counter += 1;
                        }
                            e[counter] = ' ';
                    },

                    Err(err) => {
                            e[counter] = '*';
                            counter += 1;
                            e[counter] = ' ';
                    }
                }
            },            
               'a' => {
                let num = c.as_str().parse::<i32>();
                let e = map.entry('a').or_default();
                match num {
                    Ok(n) => {
                        for i in 0..n {
                            e[counter] = '*';
                            counter += 1;
                        }
                            e[counter] = ' ';
                    },

                    Err(err) => {
                            e[counter] = '*';
                            counter += 1;
                            e[counter] = ' ';
                    }
                }
            },            
               'b' => {
                let num = c.as_str().parse::<i32>();
                let e = map.entry('b').or_default();
                match num {
                    Ok(n) => {
                        for i in 0..n {
                            e[counter] = '*';
                            counter += 1;
                        }
                            e[counter] = ' ';
                    },

                    Err(err) => {
                            e[counter] = '*';
                            counter += 1;
                            e[counter] = ' ';
                    }
                }
            },            
               'c' => {
                let num = c.as_str().parse::<i32>();
                let e = map.entry('c').or_default();
                match num {
                    Ok(n) => {
                        for i in 0..n {
                            e[counter] = '*';
                            counter += 1;
                        }
                            e[counter] = ' ';
                    },

                    Err(err) => {
                            e[counter] = '*';
                            counter += 1;
                            e[counter] = ' ';
                    }
                }
            },                         
                'd' => {
                let num = c.as_str().parse::<i32>();
                let e = map.entry('d').or_default();
                match num {
                    Ok(n) => {
                        for i in 0..n {
                            e[counter] = '*';
                            counter += 1;
                        }
                            e[counter] = ' ';
                    },

                    Err(err) => {
                            e[counter] = '*';
                            counter += 1;
                            e[counter] = ' ';
                    }
                }
            },                          
                'e' => {
                let num = c.as_str().parse::<i32>();
                let e = map.entry('e').or_default();
                match num {
                    Ok(n) => {
                        for i in 0..n {
                            e[counter] = '*';
                            counter += 1;
                        }
                            e[counter] = ' ';
                    },

                    Err(err) => {
                            e[counter] = '*';
                            counter += 1;
                            e[counter] = ' ';
                    }
                }
            },            
                'g' => {
                let num = c.as_str().parse::<i32>();
                let e = map.entry('g').or_default();
                match num {
                    Ok(n) => {
                        for i in 0..n {
                            e[counter] = '*';
                            counter += 1;
                        }
                            e[counter] = ' ';
                    },

                    Err(err) => {
                            e[counter] = '*';
                            counter += 1;
                            e[counter] = ' ';
                    }
                }
            },            
                'f' => {
                let num = c.as_str().parse::<i32>();
                let e = map.entry('f').or_default();
                match num {
                    Ok(n) => {
                        for i in 0..n {
                            e[counter] = '*';
                            counter += 1;
                        }

                        e[counter] = ' ';
                    },

                    Err(err) => {
                            e[counter] = '*';
                            counter += 1;
                            e[counter] = ' ';
                    }
                }
            },            
            _ => panic!("invalid string")

        }
        counter += 1;
    }

    let mut upper= Vec::new();
    let mut lower = Vec::new();
    for (x, s)  in map {
        if s[0].is_ascii_uppercase() {
            upper.push(s);
        } else {
            lower.push(s);
        }
    }

    upper.sort_by(|a,b| b[0].cmp(&a[0]));
    lower.sort_by(|a,b| b[0].cmp(&a[0]));

    for i in upper {
        for j in i {
            print!("{}", j);
        }
        println!();
    }
    for i in lower{
        for j in i {
            print!("{}", j);
        }
        println!();
    }
} 


fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = Scanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());
    solve(&mut scan, &mut out);
}
