impl Solution {
    pub fn maximum69_number (num: i32) -> i32 {
        let mut res = String::from("");
        let mut flag = true;
        for i in num.to_string().chars() {
            if i == '9' {
                res.push('9');
    
            }
            else if i != '9' && flag == true {
                flag = false;
                res.push('9');
            } else {
                res.push('6');
            }
            
        }
        res.parse::<i32>().unwrap() 
    }
    }