pub fn find_disappeared_numbers(mut nums: Vec<i32>) -> Vec<i32> {
    let l = nums.len();
    let mut v: Vec<i32> = vec![];
    
    for i in 1..=l {
        if !nums.contains(&(i as i32)) {
            v.push(i as i32)
        }
    }
    v
}