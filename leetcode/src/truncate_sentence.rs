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