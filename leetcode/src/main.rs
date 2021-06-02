use std::{error::Error, mem::replace, thread::current};

mod sorting_the_sentence;
fn main() {
    println!("Hello, world!");
    sorting_the_sentence::work();
    //replace_digits("a2b4a3".to_string());
    let s = String::from("hello");
    let x : u8 = 1;
    let y : u8 = 104;
}

fn str_if_possible () -> Result<String, Box<dyn Error>> {
    Ok("Works".to_string()) 

}




/* 
// Not sure how i add the byte b'1' AS u8 1.
// whatever.
pub fn replace_digits(s: String) -> String {
    let mut res: String = "".to_string();
    let mut previous : u8 = b'a';
    for word in s.as_bytes() {
        if word.is_ascii_alphabetic() {
            previous = *word;
            res.push(*word as char);
        } else {
            let x = *word + previous;
            println!("{}", x);
            res.push((word + previous) as char);
        }
    }
    res
}
*/
