fn alphabet_position(text: &str) -> String {
    let mut s = format!("");

    for c in text.chars() {
        if c.is_alphabetic() {
        s.push_str(&(c.to_ascii_lowercase() as u8 - 96).to_string());
        s.push(' ');
        }
    }
    s.trim_end().to_string()
}


