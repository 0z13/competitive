fn main() {
    hammingWeight(1213891);
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