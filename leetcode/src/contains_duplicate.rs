impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut xs: Vec<i32> = Vec::new();
        
        for i in nums {
            if xs.contains(&i) {
                println!("{}", i);
                return true;
            }
            xs.push(i);
        }
        false
    }
}