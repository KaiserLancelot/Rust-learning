use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};

fn main() {
    // TODO unwrap_or_else 第十三章
    let _file = match File::open("hello.txt") {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(file) => file,
                Err(error) => panic!("Problem creating the file: {:?}", error),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };

    // 如果 Result 值是成员 Ok, unwrap 会返回 Ok 中的值; 如果 Result 是成员 Err, unwrap 会调用 panic!
    let _file = File::open("hello.txt").unwrap();
    // 和 unwrap() 类似, 可以选择错误信息, unwrap() 使用默认的 panic! 信息
    let _file = File::open("hello.txt").expect("Failed to open hello.txt");
}

// 传播 (propagating) 错误
fn _read_username_from_file() -> Result<String, io::Error> {
    let mut file = match File::open("hello.txt") {
        Ok(file) => file,
        Err(error) => return Err(error),
    };

    let mut username = String::new();

    match file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(error) => Err(error),
    }
}

// 简写: ? 运算符
// 如果 Result 的值是 Ok, 这个表达式将会返回 Ok 中的值而程序将继续执行
// 如果值是 Err, Err 中的值将作为整个函数的返回值
// ? 会调用 from(), 定义于标准库的 From trait 中, 用于转换错误类型
fn _read_username_from_file2() -> Result<String, io::Error> {
    let mut file = File::open("hello.txt")?;
    let mut username = String::new();
    file.read_to_string(&mut username)?;
    Ok(username)
}

fn _read_username_from_file3() -> Result<String, io::Error> {
    let mut username = String::new();
    // 可以链式调用
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

fn _read_username_from_file4() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

// ? 运算符也可用于 Option<T> 值
fn _last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

// TODO 第十七章
// Box<dyn Error> 类型是一个 trait 对象 (trait object)
// 可以理解为任何类型的错误
// 如果 main 返回 Ok(()) 可执行程序会以 0 值退出, 而如果 main 返回 Err 值则会以非零值退出
// fn main() -> Result<(), Box<dyn Error>> {
//     let f = File::open("hello.txt")?;
//     Ok(())
// }
