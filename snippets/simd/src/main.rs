use memchr::memchr;
use simdutf8::basic::from_utf8;

fn main() {
    let haystack = b"foo bar baz quuz";
    assert_eq!(Some(10), memchr(b'z', haystack));

    println!(
        "{}",
        from_utf8(b"I \xE2\x9D\xA4\xEF\xB8\x8F UTF-8!").unwrap()
    );
}
