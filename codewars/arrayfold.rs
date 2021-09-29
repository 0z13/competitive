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


