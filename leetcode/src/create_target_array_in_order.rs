impl Solution {
    pub fn create_target_array(nums: Vec<i32>, index: Vec<i32>) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::new();
        let mut iter = nums.iter().zip(index.iter());
        
        for (n, index) in iter {
            println!("{}{}", n, index);
            res.insert(*index as usize, *n);
        }
        
        res
    }
}