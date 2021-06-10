impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut min:i32 = 123123;
        for price in prices {
            if price < min {
                min = price;
            }
            
            let profit = price - min;
            
            if profit > res {
                res = profit;
            }
    
        }
        res
        }
    }