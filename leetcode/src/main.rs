use std::error::Error;

mod sorting_the_sentence;
fn main() {
    println!("Hello, world!");
    sorting_the_sentence::work();
}



fn str_if_possible () -> Result<String, Box<dyn Error>> {
    Ok("Works".to_string()) 
}
