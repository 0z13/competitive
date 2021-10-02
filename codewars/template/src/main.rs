use std::ascii::AsciiExt;

fn main() {
    let v = alphabet_position("The sunset sets at twelve o' clock.");
    println!("{:?}", v)
}

fn alphabetic_two(text: &str) -> String {
    text
        .chars()
        .filter(|x| x.is_alphabetic())
        .map(|x| (x.to_ascii_lowercase() as u8 - 96).to_string())
        .collect::<Vec<String>>()
        .join(" ")
}
