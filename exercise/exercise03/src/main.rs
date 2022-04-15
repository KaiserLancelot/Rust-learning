fn main() {
    let mut v = vec![1, 24, 6, 2, 34, 9, 78, 0, -3, 4, 1, 0, 44, 66];
    v.sort_unstable();
    println!("{}", v[v.len() / 2]);
}
