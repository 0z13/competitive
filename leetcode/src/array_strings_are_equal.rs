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


pub fn count_good_triplets(arr: Vec<i32>, a: i32, b: i32, c: i32) -> i32 {
    let len = arr.len();
    let mut count: i32 = 0;
    for i in (0..len-2) {
        for j in (1..len-1) {
            for k in (1..len) {
                if good_tripple((arr[i] - arr[j]), (arr[j] - arr[k]), (arr[i] - arr[k]), a, b,c) {
                    count += 1
                }
            }
        }
    }
    count
}

pub fn good_tripple(i:usize, j:usize, k:usize, a:i32, b: i32, c:i32) -> bool {
    i <= a && j <= b && k <= c
}