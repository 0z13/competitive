mod sorting_the_sentence;
mod foo;
fn main() {
    println!("{}", max_value("-13".to_string(), 2))
}


// I feel like i just figured out lifetimes.
// It clicked.

// So here is the deal.

// Here's your run-of-the-mill struct.
struct Person {
    age: i32,
    name: String,
}

// let's say you wanna implement a get method.
pub fn square_is_white(coordinates: String) -> bool {
    // - 96
    let mut vert = 'a';
    let mut hori = 1;
    let mut iter = coordinates.chars();
    vert = iter.next().unwrap();
    hori = iter.next().unwrap().to_string().parse::<i32>().unwrap();
    let zz = (vert as i32) - 96;
    (hori % 2 == 0  && zz % 2 != 0) || (hori % 2 != 0  && zz % 2 == 0) 
}


pub fn diagonal_sum (xs : Vec<Vec<i32>>) -> i32 {
    let mut sum = 0;
    let mut rpointer = 0;
    let mut lpointer = xs.len() - 1;
    // Get right-to-left diagonal
    for (index, item) in xs.iter().enumerate() {
        for (index, item ) in item.iter().enumerate() {

            if (index == rpointer) {
                sum = sum + item;
            }

            if (index == lpointer) {
                println!("i:{}, n: {}", item, index);
                sum = sum + item;
            }
        }
        rpointer = rpointer + 1;
        if lpointer != 0 {
        lpointer = lpointer - 1;
        }
    }
    sum
}


pub fn max_value(n: String, x: i32) -> String {
    let mut st = n;
    if st.starts_with("-") {
        st.remove(0);
        mk_small(st, x)
    } else {
        mk_large(st, x)
    }
}

fn mk_large(s: String, n: i32) -> String {
    let mut res = String::new();
    let mut m = n;
    for num in s.chars() {
        if ((num as i32) - 48) < m {
            res.push_str(&m.to_string());
            res.push(num);
            m = -1;
        } else {
            res.push(num);
        } 
    }
    if m != -1 {
        res.push_str(&m.to_string())
    }
    res
}
fn mk_small(s: String, n: i32) -> String {
    let mut res = String::from("-");
    let mut m = n;
    for num in s.chars() {
        if ((num as i32) - 48) > m {
            res.push_str(&m.to_string());
            res.push(num);
            m = 1000;
            println!("m: {} num: {}", m, (num as char) )
        } else {
            res.push(num);
            println!("{}", num);
        } 
    }
    if m != 1000 {
        res.push_str(&m.to_string())
    }
    res
}
pub fn find_disappeared_numbers(mut nums: Vec<i32>) -> Vec<i32> {
    let l = nums.len();
    let mut v: Vec<i32> = vec![];
    
    for i in 1..=l {
        if !nums.contains(&(i as i32)) {
            v.push(i as i32)
        }
    }
    v
}

//fk