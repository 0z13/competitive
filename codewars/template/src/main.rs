fn main() {
    let switch: bool = true;
    let val = 3 + if switch {3} else {2};
    let test = [1,2,3,4,5];
    let v =  fold_array(&test, 3);
    println!("{:?}", v)
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

fn uneven_fold(arr: Vec<i32>) -> Vec<i32> {
    let mut v = vec![];
    let mid = (arr.len() as f64 / 2 as f64).floor() as usize;
    let mut bptr = arr.len() - 1;
    for i in (0..mid) {
        v.push(arr[i] + arr[bptr]);
        bptr -= 1;
    }
    v.push(arr[mid]);
    v
}

fn even_fold(arr: Vec<i32>) -> Vec<i32> {
    let mut v = vec![];
    if arr.len() <= 1 {
        return arr.to_vec();
    }
    let mut b = (arr.len() -1);
    let mut f = 0;

    while (b > f) {
        println!("{} b {}", f,b);
        v.push(arr[b] + arr[f]);
        b -= 1;
        f += 1;
    }
    v
}

fn fold_array(arr: &[i32], runs: usize) -> Vec<i32> {
    let mut v = arr.to_vec();
    for i in (0..runs)   {
        if v.len() % 2 == 0 {
            v = even_fold(v);
        } else {
            v = uneven_fold(v)
        }
    }
    v
}



fn spin_words(words: &str) -> String {
    words.split_ascii_whitespace().map(|word| match word.len() >= 5 {
        true => word.chars().rev().collect(),
        false => word.to_string(),
    }).collect::<Vec<_>>().join(" ")
}


fn divisors(integer: u32) -> Result<Vec<u32>, String> {
    let mut v = vec![];
    for i in 1..integer {
        if integer % i == 0 {
            v.push(i);
        }
    }
    match v.len() {
        0 => Err(format!("{} is prime", integer)),
        _ => Ok(v),
    }
}
