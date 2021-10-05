use std::{ascii::AsciiExt, cmp::max};

fn main() {
    println!("{}", rgb(-20, 275, 125));
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

fn rgb(r: i32, g: i32, b: i32) -> String {
    let  mut xs =[r,g,b];
    for i in xs.iter_mut() {
        if *i < 0 {
            *i = 0;
        } else if *i > 255 {
            *i = 255;
        }
    }
    format!("{:0width$X}{:0width$X}{:0width$X}", xs[0], xs[1], xs[2], width=2)
}
