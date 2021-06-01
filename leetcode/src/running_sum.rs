pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
    let mut running = 0;
    let mut v : Vec<i32> = Vec::new()
    for i in nums {
        running = running + i;
        v.push(running);
    }
    v
}
