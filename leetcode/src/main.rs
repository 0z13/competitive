mod sorting_the_sentence;
fn main() {
    println!("Hello, world!");
    sorting_the_sentence::work();
    //replace_digits("a2b4a3".to_string());
    let s = String::from("hello");
    let x : u8 = 1;
    let y : u8 = 104;

    let row1 = vec![1,2,3];
    let row2 = vec![1,2,3];
    let row3 = vec![1,2,3];
    let tst = vec![row1,row2,row3];
    println!("{}", diagonal_sum(tst));
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


