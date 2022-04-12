use std::io;

const MAX_N: usize = 94;

fn fibonacci(n: usize) -> usize {
    let mut arr = [0; MAX_N];
    arr[1] = 1;

    let mut index = 2_usize;
    while index < MAX_N && index <= n {
        arr[index] = arr[index - 1] + arr[index - 2];
        index += 1;
    }

    arr[n]
}

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Error: read_line()");
    let n: usize = n.trim().parse().expect("Error: parse()");

    if n < MAX_N {
        println!("fibonacci({}) = {}", n, fibonacci(n));
    } else {
        print!("Too big");
    }
}
