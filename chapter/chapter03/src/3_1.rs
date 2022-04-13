// 常量必须注明类型
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    // 默认不可变
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    println!("{}", THREE_HOURS_IN_SECONDS);
}
