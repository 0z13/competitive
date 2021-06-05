
pub fn max_value(n: String, x: i32) -> String {
    let mut st = n;
    if st.starts_with("-") {
        st.remove(0);
        mk_small(st, x)
    } else {
        mk_large(st, x)
    }
}

fn mk_large(s: String, n: i32) -> String {
    let mut res = String::new();
    let mut m = n;
    for num in s.chars() {
        if ((num as i32) - 48) < m {
            res.push_str(&m.to_string());
            res.push(num);
            m = -1;
        } else {
            res.push(num);
        } 
    }
    if m != -1 {
        res.push_str(&m.to_string())
    }
    res
}
fn mk_small(s: String, n: i32) -> String {
    let mut res = String::from("-");
    let mut m = n;
    for num in s.chars() {
        if ((num as i32) - 48) > m {
            res.push_str(&m.to_string());
            res.push(num);
            m = 1000;
            println!("m: {} num: {}", m, (num as char) )
        } else {
            res.push(num);
            println!("{}", num);
        } 
    }
    if m != 1000 {
        res.push_str(&m.to_string())
    }
    res
}