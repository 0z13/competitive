use std::{ascii::AsciiExt, cmp::max};

fn main() {
    println!("{:?}", high("man i need a taxi up to ubud"))
}


fn high(input: &str) -> &str {
    let mut mx = 0;
    let mut word = "";
    for i in input.split_ascii_whitespace() {
        let x = score(i);
        if x > mx {
            mx = x;
            word = i;
        }
    }
    word
}

fn score(i: &str) -> i32 {
    let mut score = 0;
    for c in i.chars() {
        let val = (c.to_ascii_lowercase() as u8 - 96) as i32;
        score += val;
    }
    score
}


fn in_array(arr_a: &[&str], arr_b: &[&str]) -> Vec<String> {
    let mut a = arr_a
        .into_iter()
        .filter(|word| !arr_b.contains(word))
        .map(|word| word.to_string())
        .collect::<Vec<String>>();
    a.sort();
    a
}