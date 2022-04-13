fn main() {
    // slice 允许引用集合中一段连续的元素序列
    // slice 是一类引用, 没有所有权

    // 它是一个指向二进制程序特定位置的 slice
    let _s = "Hello World";

    let /*mut*/ s = String::from("Hello World");
    let word = first_word(&s);
    // error: cannot borrow `s` as mutable because it is also borrowed as immutable
    // s.clear();
    println!("{}", word);

    // 0 可以省略
    let _hello = &s[0..5];
    // 11 可以省略
    let _world = &s[6..11];
    // 如果尝试从一个多字节字符的中间位置创建字符串 slice, 会 panic

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}

// TODO &str, 第十五章
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    // TODO &item, 第六章
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    s
}
