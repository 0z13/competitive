impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let xs:String = x.to_string();
        let rev:String = xs.chars().rev().collect();
        xs == rev 
    }

}


