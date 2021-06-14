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