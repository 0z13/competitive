
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

