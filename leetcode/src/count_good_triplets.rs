pub fn count_good_triplets(arr: Vec<i32>, a: i32, b: i32, c: i32) -> i32 {
    let len = arr.len();
    let mut count: i32 = 0;
    for i in (0..len) {
        for j in (i+1..len) {
            for k in (j+1..len) {
                if good_tripple((arr[i] - arr[j]), (arr[j] - arr[k]), (arr[i] - arr[k]), a, b,c) {
                    count += 1
                }
            }
        }
    }
    count
}
    
    
pub fn good_tripple(i:i32, j:i32, k:i32, a:i32, b: i32, c:i32) -> bool {
    i.abs() <= a && j.abs() <= b && k.abs() <= c
}