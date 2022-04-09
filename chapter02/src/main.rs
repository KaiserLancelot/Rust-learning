pub fn add_two(a: i32) -> i32 {
    a + 2
}

fn main() {
    println!("{}", add_two(2));
}

#[cfg(test)]
mod tests {
    use crate::add_two;

    #[test]
    fn it_works() {
        assert_eq!(add_two(2), 4);
    }
}
