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


