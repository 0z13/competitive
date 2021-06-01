use std::io::{self, BufRead};
fn solve () io::Result<()> {
   let mut s = String::new();
   io::stdin().read_to_string(&mut s)?;
   println!("{}", s);
    
   for i in s.lines() {
        v = i.split(' ');
        x = v.next().parse::<i32>() 
        y = v.next().parse::<i32>() 
        println(std::num::abs(x + y))
   }


}
