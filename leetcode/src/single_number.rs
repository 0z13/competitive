impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
    let mut a:i32 = 0;
    for i in nums {
        println!("{:b}   {:b}", i, a);
        a = a ^ i;
    }
    a
    }
}
    
