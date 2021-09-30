
fn spin_words(words: &str) -> String {
    let mut s = String::from("");
    for i in words.split_ascii_whitespace(){
        if i.len() >= 5 {
            let tmp = i.chars().rev().collect::<String>();
            s.push_str(&tmp);
            s.push(' ')
        } else {
            s.push_str(i);
            s.push(' ')

        }
    }
    s.trim_end().to_string()
}


