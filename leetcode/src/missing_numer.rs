impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let mut counter:i32 = 0;
        
        let mut nums = nums;
        let mxx = nums.len();
        nums.sort();
                

        
        for i in nums.iter() {
            if counter != *i {
                println!("{} {}", counter, i);
                return (counter as i32);
            }
            println!("{}{}", i, counter);
            counter = counter + 1;
        }
        return (mxx as i32)
    }
}