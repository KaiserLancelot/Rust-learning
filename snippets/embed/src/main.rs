fn main() {
    let str = include_str!("../data/text.txt");
    assert_eq!(str, "Hello World!\n");
    println!("{}", str);
}
