fn main() {
    let data = "initial contents";
    // 同 String::from
    // to_string 方法, 它能用于任何实现了 Display trait 的类型
    let s1 = data.to_string();
    let mut s2 = data.to_string();

    s2.push_str("str");

    // + -> fn add(self, s: &str) -> String
    // &String 可以被 强转 (coerced) 成 &str
    // &s2 -> &s2[..]
    // TODO deref coercion 第十五章
    // 注意 s1 被移动了
    let s3 = s1 + &s2;
    println!("{}", s3);

    // format! 不获取所有权
    let s3 = format!("{}-{}-{}", s3, s3, s3);
    println!("{}", s3);

    // 不支持索引
    // println!("{}", &s3[0]);

    // panic
    // let hello = "Здравствуйте";
    // let s = &hello[0..1];
    // println!("{}", s);

    let s = String::from("你好世界");
    for c in s.chars() {
        print!("{} ", c);
    }
    println!();

    for b in s.bytes() {
        print!("{} ", b);
    }
}
