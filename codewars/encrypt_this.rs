fn encrypt_this(text: &str) -> String {
    let mut supr = String::from("");
    for word in text.split(' ') {
        let mut result = String::from("");
        let mut s: Vec<char> = word.to_owned().chars().collect();
        let first = s[0].clone() as u8;
        let length = s.len();
        if length > 2 {
           s.swap(1, length - 1)
        }
        result.push_str(&first.to_string());
        s.remove(0);
        for i in s {
            result.push(i);
        }
        supr.push_str(&result);
        supr.push(' ');
    }
    supr.remove(supr.len() -1);
    supr
}

