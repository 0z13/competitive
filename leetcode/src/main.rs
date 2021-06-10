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

pub fn fib(n: i32) -> i32 {
    let mut n = n as usize;
    let mut v: Vec<i32> = vec![0,1];
    
    for i in 2..=n {
        v.push(
            v[i-1] + v[i-2]
        )
        }
    v[n-1]
}

pub fn max_profit(prices: Vec<i32>) -> i32 {
    let len = prices.len();
    let mut max:i32 = 0;
    for i in 0..len-1 {
        for j in i+1..len {
            if (prices[i] - prices[j] > max) {
                max = (prices[i] -  prices[j]);
            }
        }
    }
    max
}