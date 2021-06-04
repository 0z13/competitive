mod sorting_the_sentence;
fn main() {
    println!("Hello, world!");
    sorting_the_sentence::work();
    //replace_digits("a2b4a3".to_string());
    let s = String::from("hello");
    let x : u8 = 1;
    let y : u8 = 104;

    let v = vec![1,1,1,1,2,2,3,3];
    println!("{}", square_is_white("a1".to_string()))
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


