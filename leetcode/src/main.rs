fn main() {
   println!("hi");


    let v = vec![1,2,3,4];
    println!("{}", search(v, 3));
}


pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let pos = nums.iter().position(|&t| t == target);
    match pos {
        Some(n) => return n as i32,
        None          => return -1,
    }
}


pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
    let pos = letters.iter().position(|x| *x == target);
    match pos {
        Some(x) => return letters.to_vec()[x + 1],
        None => return -1  
    }
}


pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
    accounts.iter()
      .map(|x| x.iter().fold(0, |acc, x| acc + x)).max().unwrap()
}


pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
    let mut s1: String = String::new();
    let mut s2: String = String::new();
    
    
    for i in word1 {
        s1.push_str(i.as_str())
    }

    for i in word2 {
        s2.push_str(i.as_str())
    }

    s1 == s2
}

pub fn length_of_last_word(s: String) -> i32 {
    match s.split_ascii_whitespace().last() {
        Some(s) => s.len() as i32,
        None     => 0 
    }
}


pub fn restore_string(s: String, indices: Vec<i32>) -> String {
s.chars()
    .zip(indices)
    .fold(vec![' '; s.len()], |mut v, (c, ind)| {
        v[ind as usize] = c;
        v
    })
    .into_iter()
    .collect()
}