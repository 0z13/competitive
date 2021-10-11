fn generate_shape(n: i32) -> String {
    let mut s = format!("");
    for i in 0..n {
        for j in 0..n {
            s.push('+');
        }
        s.push('\n')
    }
    s.trim().to_string()
}

