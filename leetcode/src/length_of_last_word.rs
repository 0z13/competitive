pub fn length_of_last_word(s: String) -> i32 {
    match s.split_ascii_whitespace().last() {
        Some(s) => s.len() as i32,
        None     => 0 
    }
}