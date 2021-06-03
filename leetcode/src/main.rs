mod sorting_the_sentence;
fn main() {
    println!("Hello, world!");
    sorting_the_sentence::work();
    //replace_digits("a2b4a3".to_string());
    let s = String::from("hello");
    let x : u8 = 1;
    let y : u8 = 104;

    let v = vec![1,1,1,1,2,2,3,3];
    group_by_practice(v);
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
#![feature(slice_group_by)]
pub fn group_by_practice(xs: Vec<i32> ) -> () {
    let a = xs.group_by(|a, b| a == b);

    for i in a {
        let l = i.len();
        let e = i[0];
        println!("e:{}l:{}", e, l);
    }

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


