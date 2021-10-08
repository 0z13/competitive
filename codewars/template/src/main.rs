use std::{ascii::AsciiExt, cmp::max, fmt::format, slice::Split};

fn main() {
    println!("{}", xo(&"ooom"));
}

fn xo(string: &'static str) -> bool {
    string
        .chars()
        .filter(|x| x.to_ascii_lowercase() == 'o')
        .count()
        == string
            .chars()
            .filter(|x| x.to_ascii_lowercase() == 'x')
            .count()
}

#[test]
fn returns_expected() {
    assert_eq!(xo("xo"), true);
    assert_eq!(xo("Xo"), true);
    assert_eq!(xo("xxOo"), true);
    assert_eq!(xo("xxxm"), false);
    assert_eq!(xo("Oo"), false);
    assert_eq!(xo("ooom"), false);
}
