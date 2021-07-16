
use std::collections::HashMap;
fn main() {
    let mut v:Table = HashMap::new();  
    let _a = v.insert("idk".to_string(), vec!["idk".to_string(), "adk".to_string()]);
    f(&mut v);
    for (i,j) in v {
        println!("{:?}", j)
    }
}


fn test_ascii(xs: Vec<u8>) -> () {
    for i in xs {
        println!("{:b}", i)
    }
}


pub fn maximum69_number (num: i32) -> i32 {
    let mut res = String::from("");
    let mut flag = true;
    for i in num.to_string().chars() {
        if i != '9' && flag == true {
            flag = false;
            res.push('9');
        } else {
            res.push('6');
        }
        
    }
    res.parse::<i32>().unwrap() 
}


pub fn hamming_weight (n: u32) -> i32 {
    n.count_ones() as i32
}

pub fn hamming_weight_man(n:u32) -> i32 {
    let mut count = 0;
    let mut inc = n;
    while inc > 0 {
       if (n & 1)  == 1 {
           count += 1;
       }
      inc >>=  1 
    }
    count
}

type Table = HashMap<String, Vec<String>>;

fn f(table:&mut Table)  {
    for (i, j) in table {
        j.sort();
    }
}

pub fn unique_occurrences(arr: Vec<i32>) -> bool {
       let mut m = HashMap::new(); 

       for i in arr {
           if !m.contains_key(&i) {
               let _foo = m.insert(i, 1);
           } else {
               let  x = m.get(&i).unwrap();
               let _fo = m.insert(i, x + 1);
           }
       }

       let k = m[&arr[0]];
       for i in m.values() {
            if k != *i {return false;}
       }
       true
}

pub fn min_deletion_size(strs: Vec<String>) -> i32 {

    let mut v:Vec<String> = Vec::new(); 

    while strs[0].len() > 0 {
        let tmp = "".to_string();
        for i in strs {
            tmp.push(i.remove(0))
        }
    }

    for (i,j) in v.iter().enumerate() {
        if !j.chars().is_sorted() {
            return i as i32
        }   
    }
    return 3
}