use std::cmp::Ordering;
#[derive(Eq)]
struct WordNum {
    word: String,
    location: u32
}

impl WordNum {
    fn parse_word_num(s: &str) -> Self {
        let mut n = s.clone().to_string();
        let location = n.pop().unwrap().to_digit(10).unwrap();

        WordNum {word:n, location:location}
    }
}

impl Ord for WordNum {
    fn cmp(&self, other: &Self) -> Ordering {
        self.location.cmp(&other.location)
    }
}

impl PartialOrd for WordNum {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for WordNum {
    fn eq(&self, other: &Self) -> bool {
        self.location == other.location
    }
}

pub fn work() {
    let example = String::from("med2 hej1 dig3");
    let mut v : Vec<WordNum> = Vec::new(); 
    for word in example.split_whitespace() {
        v.push(WordNum::parse_word_num(word))
    }
    v.sort();
    let mut res = String::new();
    
    for i in v {
        res.push_str(&i.word);
        res.push(' ');
    }
    res.pop();
    println!("{}", res)
}
