gh_and_low(numbers: &str) -> String {
    let xs = numbers.split(" ").map(|x|
x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    let min = xs.iter().max().copied().unwrap();
    let max= xs.iter().min().unwrap();
    format!("{} {}", max, min)
}


