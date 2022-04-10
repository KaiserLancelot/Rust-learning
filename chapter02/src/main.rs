// 默认情况下, Rust 将 prelude 模块中少量的类型引入到每个程序的作用域中
// Rng 是一个 trait, 它定义了随机数生成器应实现的方法, 想使用这些方法的话, 此 trait 必须在作用域中
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    // 1..101: 范围表达式(range expression)
    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please input your guess.");

        // 默认不可变
        // String 是一个标准库提供的字符串类型, 它是 UTF-8 编码的可增长文本块
        // new(): 关联函数(associated function)
        let mut guess = String::new();

        // stdin 函数返回一个 std::io::Stdin 的实例, 这代表终端标准输入句柄的类型
        io::stdin()
            // 读一行, 追加到 guess, 会保存换行符
            // & 表示一个引用, 默认不可变
            .read_line(&mut guess)
            // 如果 io::Result 实例的值是 Err, expect 会导致程序崩溃, 并显示当做参数传递给 expect 的信息
            // 如果 io::Result 实例的值是 Ok, expect 会获取 Ok 中的值并原样返回
            .expect("Failed to read line");

        // 枚举通常和 match 一同使用
        // 隐藏(shadow), 允许我们复用 guess 变量的名字
        let guess: u32 = match guess.trim().parse() {
            // 分支(arms): 模式(pattern) + 代码
            Ok(num) => num,
            // _ 是一个通配符值, 用来匹配所有 Err 值
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
