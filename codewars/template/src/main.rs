
fn main() {
    let x = vec![1,2,3,4,5,6];
    calc(x);
}

fn calc(array: Vec<i32>) -> i32 {
    let mut arr = vec![];

    for (i, j) in array.iter().enumerate() {
        let mut num = j.to_owned();
        if num > 0 {
            num = num * num;
        }
        if (i+1) % 3 == 0 {
            num = num *3;
        }
        if (i+1) % 5 == 0 {
            num = num * (-1);
        }
        arr.push(num)
    }
    arr.iter().sum()
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        assert_eq!(calc(vec![0, 2, 1, -6, -3, 3]), 31);
        assert_eq!(calc(vec![0]), 0);
        assert_eq!(calc(vec![1, 1, 1, 1, 1]), 5);
        assert_eq!(calc(vec![1, 1, -9, 9, 16, -15, -45, -73, 26]), 1665);
        assert_eq!(calc(vec![1, -1, 10, -9, 16, 15, 45, -73, -26]), 2584);
        assert_eq!(calc(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]), 0);
        assert_eq!(calc(vec![-5, -5, -5, -5, -5, -5, -5]), -45);
    }
}
