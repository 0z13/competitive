impl Solution {
    pub fn find_numbers(nums: Vec<i32>) -> i32 {
        let mut counter = 0;
        
        for i in nums {
            if len_is_even(i) {
                counter = counter + 1;
            }
        }
        counter
    }
}

pub fn len_is_even(x: i32) -> bool {
    let mut counter = 0;
    let mut y = x;
    while y > 0 {
        counter = counter + 1;
        y = y / 10
    }
    counter % 2 == 0
}