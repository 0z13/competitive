fn main() {
    let switch: bool = true;
    let val = 3 + if switch {3} else {2};
    println!("{}", val)
}


fn nb_year(p0: i32, percent: f64, aug: i32, p: i32)-> i32 {
    let mut pop = p0 as f64;
    let mut res = 0;
    while pop < (p as f64) {
        res += 1;
        pop = (pop + ((pop * percent)) / 100.0) + (aug as f64);
    }
   res
 }

 fn check_exam(arr_a: &[&str], arr_b: &[&str]) -> i64 {
     let mut score = 0;

     for (x,y) in arr_a.iter().zip(arr_b) {
         if x.eq(y) {
             score += 4;
         } else {
             score -= 1;
         }
     }
     score
}

fn longest_consec(strarr: Vec<&str>, k: usize) -> String {

}
