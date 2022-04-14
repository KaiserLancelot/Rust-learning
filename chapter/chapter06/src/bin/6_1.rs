enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum _Message {
    Quit,
    // 包含命名字段
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// 可以在枚举上定义方法
impl _Message {
    fn _call(&self) {}
}

fn main() {
    let _home = IpAddr::V4(127, 0, 0, 1);
    let _loopback = IpAddr::V6(String::from("::1"));

    // Option<T> 包含在 prelude 中, 它的成员也是如此
    let _some_number = Some(5);
    let _some_string = Some("a string");
    let _absent_number: Option<i32> = None;
}
