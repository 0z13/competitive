impl Solution {
    pub fn diagonal_sum (xs : Vec<Vec<i32>>) -> i32 {
    let mut sum = 0;
    let mut rpointer = 0;
    let mut lpointer = xs.len() - 1;
    // Get right-to-left diagonal
    for (index, item) in xs.iter().enumerate() {
        for (index, item ) in item.iter().enumerate() {

            if (rpointer == lpointer && index == lpointer) {
                sum = sum - item;
            }
            
            if (index == rpointer) {
                sum = sum + item;
            }

            if (index == lpointer) {
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
}


