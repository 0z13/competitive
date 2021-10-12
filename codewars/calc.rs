
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




