use std::ascii::AsciiExt;

fn main() {
    let v = solution("helloMyMan");
    println!("{:?}", v)
}

fn solution(s: &str) -> String {
    let s= s.chars().collect::<Vec<char>>();
    let mut res = format!("");
    let mut i = 0;
    while i < s.len() - 1 {
        if s[i] == s[i].to_ascii_lowercase() && s[i+1] == s[i+1].to_ascii_uppercase() {
            res.push(s[i]);
            res.push(' ');
            res.push(s[i+1].to_ascii_lowercase());
            i += 1;
        } else {
            res.push(s[i]);
        }
        i += 1;
    }
    res.push(s[s.len() - 1]);
    res
}



