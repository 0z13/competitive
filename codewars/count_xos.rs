
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


