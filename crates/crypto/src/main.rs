use ring::{digest, test};

fn main() {
    let expected_hex = "09ca7e4eaa6e8ae9c7d261167129184883644d07dfba7cbfbc4c8a2e08360d5b";
    let expected: Vec<u8> = test::from_hex(expected_hex).unwrap();
    let actual = digest::digest(&digest::SHA256, b"hello, world");

    assert_eq!(&expected, &actual.as_ref());
}
