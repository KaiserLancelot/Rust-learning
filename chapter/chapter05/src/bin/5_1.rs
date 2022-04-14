struct User {
    _active: bool,
    // TODO 在第十章前不要使用自身没有所有权的类型
    _username: String,
    _email: String,
    _sign_in_count: u64,
}

fn main() {
    let user1 = build_user(
        String::from("someone@example.com"),
        String::from("someusername123"),
    );

    let _user2 = User {
        _email: String::from("another@example.com"),
        // struct update syntax
        // 其他值来自 user1
        // 注意这样做移动了数据, user1 不再有效
        // 但是如果 username 也制定了值, 那么 user1 仍有效
        ..user1
    };

    // tuple structs
    // 两者类型不同, 其他和元组一致
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);

    // unit-like structs
    // 和 unit type 类似
    // 常常在你想要在某个类型上实现 trait 但不需要在类型中存储数据的时候发挥作用
    // TODO 第十章
    struct AlwaysEqual;
    let _subject = AlwaysEqual;
}

fn build_user(email: String, username: String) -> User {
    User {
        // 参数名与字段名都完全相同时省略字段名, field init shorthand
        _email: email,
        _username: username,
        _active: true,
        _sign_in_count: 1,
    }
}
