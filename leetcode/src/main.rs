fn main() {
   println!("{}", fib(10));
}



pub fn flip_and_invert_image(image: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut xs:Vec<Vec<i32>> = Vec::new();

    for mut arr in image {
        let mut input_vec: Vec<i32> = Vec::new();
        arr.reverse();
        for i in arr {
            if i == 0 {
                input_vec.push(1);
            } else {
                input_vec.push(0);
            }
        }
        xs.push(input_vec);
    }
    xs
}


fn fib (n : i32) -> i32 {
    let mut dp = vec![1,2];
    let n = n as usize;
    for i in 2..n {
        dp.push(
            dp[i - 2] + dp[i - 1]
        )
    } 
    dp[n - 1]
}