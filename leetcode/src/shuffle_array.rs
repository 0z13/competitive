pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
    let (l, r) = nums.split_at(n);
    l.iter().zip(r.iter()).collect::<Vec<i32>>();

}

