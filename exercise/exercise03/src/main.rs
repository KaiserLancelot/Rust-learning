use std::collections::HashMap;

fn main() {
    let mut v = vec![1, 24, 6, 2, 34, 9, 78, 0, -3, 4, 1, 0, 44, 66];

    v.sort_unstable();
    println!("{:?}", v);

    println!("{}", median(&v));
    println!("{:?}", mode(&v));
}

fn median(v: &[i32]) -> f64 {
    let len = v.len();

    if len % 2 == 0 {
        (v[len / 2] as f64 + v[len / 2 - 1] as f64) / 2.0
    } else {
        v[len / 2] as f64
    }
}

fn mode(v: &[i32]) -> Vec<i32> {
    let mut map = HashMap::new();

    for i in v {
        let count = map.entry(i).or_insert(0);
        *count += 1;
    }

    let max_value = map.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap();

    let mut result = Vec::new();

    for (&&key, value) in &map {
        if value == max_value.1 {
            result.push(key);
        }
    }

    result
}
