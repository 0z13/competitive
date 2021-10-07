use std::{ascii::AsciiExt, cmp::max, fmt::format, slice::Split};

fn main() {
    println!("{}", digit_sum(123));
    println!("{}", order_weight(&"2000 10003 1234000 44444444 9999 11 11 22
123"));
    println!("{}", order_weight(&"103 123 4444 99    2000"));
}

//testing("103 123 4444 99 2000", "2000 103 123 4444 99");
//testing("2000 10003 1234000 44444444 9999 11 11 22 123",
 //   "11 11 2000 10003 22 123 1234000 44444444 9999");

fn digit_sum(x: i128) -> i128{
    let mut sum = 0;
    let mut num = x;

    while num > 0 {
        sum += (num % 10);
        num = (num / 10);
    }
    sum
}

struct Weights {
    digit_sum: i128,
    weight: i128,
    alphabetic_weight: String,
}

fn order_weight(s: &str) -> String {
    let v: Vec<i128> = s
        .split(" ")
        .filter(|x| x.chars().all(|x| x.is_numeric()) && !x.is_empty())
        .map(|x| x.parse::<i128>().unwrap())
        .collect();
    let mut weights: Vec<Weights> = vec![];
    let mut res = String::from("");
    for i in v {
        weights.push(Weights {
            digit_sum: digit_sum(i),
            weight: i,
            alphabetic_weight: i.to_string(),
        })
    }
    weights.sort_by_key(|x| x.digit_sum);
    weights.sort_by(|a, b| {
        if a.digit_sum == b.digit_sum {
            a.alphabetic_weight
                .partial_cmp(&b.alphabetic_weight)
                .unwrap()
        } else {
            a.digit_sum.partial_cmp(&b.digit_sum).unwrap()
        }
    });

    for i in weights {
        res.push_str(&i.weight.to_string());
        res.push(' ');
    }
    res.trim_end().to_string()
}

