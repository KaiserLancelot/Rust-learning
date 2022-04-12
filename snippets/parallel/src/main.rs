use rayon::prelude::*;

fn sum_of_squares(input: &[i32]) -> i32 {
    input.par_iter().map(|&i| i * i).sum()
}

fn main() {
    let arr = [1, 2, 3, 4, 5];
    println!("{}", sum_of_squares(&arr));
}
