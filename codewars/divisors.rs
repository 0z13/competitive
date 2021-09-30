fn divisors(integer: u32) -> Result<Vec<u32>, String> {
    let mut v = vec![];
    for i in 1..integer {
        if integer % i == 0 {
            v.push(i);
        }
    }
    match v.len() {
        0 => Err(format!("{} is prime", integer)),
        _ => Ok(v),
    }
}


