impl Solution {
    pub fn truncate_sentence(s: String, k: i32) -> String {
        let mut ret: String = "".to_string();
        let mut counter = k;
        for i in s.split(' '){
            if (counter -1) <= 0 {
                ret.push_str(i);
                break;
            }
            counter -= 1;
            ret.push_str(i);
            ret.push_str(" ");
        }
        return ret;
    }
}

pub fn flip_and_invert_image(image: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut xs:Vec<Vec<i32>> = Vec::new();

    for arr in image {
        let mut input_vec: Vec<i32> = Vec::new();
        for i in arr.reverse() {
            if i == 0 {
                input_vec.push(1);
            } else {
                input_vec.push(0);
            }
        }
        xs.push(input_vec);
    }
    xs
}