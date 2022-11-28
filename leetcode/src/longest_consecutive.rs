use std::collections::HashSet;
pub fn longest_consecutive(nums: Vec<i32>) -> Option<usize> {
    let set: HashSet<_> = nums.iter().copied().collect();
    nums.into_iter()
        .filter(|&x| !set.contains(&(x-1)))
        .map(|x| (x..).take_while(|x| set.contains(x)).count()) 
        .max()
}

fn main() {
    let v = vec![1,2,9,4,3];
    println!("{:?}!", longest_consecutive(v));
}
