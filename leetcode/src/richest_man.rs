impl Solution {
    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        accounts.iter()
          .map(|x| x.iter().fold(0, |acc, x| acc + x)).max().unwrap()
    }
}
    