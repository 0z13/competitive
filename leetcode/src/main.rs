fn main() {
   println!("hi");


    let v = vec![1,2,3,4];
    println!("{}", search(v, 3));
}


pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let pos = nums.iter().position(|&t| t == target);
    match pos {
        Some(n) => return n as i32,
        None          => return -1,
    }
}


pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
    let pos = letters.iter().position(|x| *x == target);
    match pos {
        Some(x) => return letters.to_vec()[x + 1],
        None => return -1  
    }
}


pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
    accounts.iter()
      .map(|x| x.iter().fold(0, |acc, x| acc + x)).max().unwrap()
}