// 常量必须注明类型
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    println!("{}", THREE_HOURS_IN_SECONDS);

    // 64 位架构上 isize/usize 是 64 位的， 32 位架构上它们是 32 位
    // 可以使用分隔符
    let _size: usize = 166_169;
    // 类型后缀
    let _size = 166_169usize;

    let mut line = String::new();
    std::io::stdin().read_line(&mut line).expect("Error input");
    let fuck: usize = line.trim().parse().expect("Error parse");
    println!("{}", fuck);

    // 4 字节, Unicode Scalar Value
    let _c = 'a';
    // 1 字节
    let _c = b'a';

    let tuple = (1, 2.6, 'a');
    // 使用模式匹配(pattern matching)来解构(destructure)元组值
    let (_x, _y, _z) = tuple;
    println!("{}", tuple.1);

    // 没有任何值的元组 () 是一种特殊的类型, 被称为单元类型(unit type)，而该值被称为单元值(unit value)
    // 如果表达式不返回任何其他值, 则会隐式返回单元值

    let _a = [1, 2, 3];
    // 5 个元素, 每个的值都是 3
    let _a = [3; 5];

    // 数组越界会 panic (程序因为错误而退出)
    println!("{}", _a[fuck]);

    println!("{}", func(2));

    loop_label();

    loop_return();

    while_func();

    for_func();
}

// 必须声明每个参数的类型
// 函数体由一系列的语句和一个可选的结尾表达式构成
// 语句(Statements)是执行一些操作但不返回值的指令; 表达式(Expressions)计算并产生一个值
// 函数定义也是语句
// 用大括号创建的一个新的块作用域也是一个表达式
fn func(a: i32) -> i32 {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);

    let condition = true;
    // if 表达式
    let _number = if condition { 3 } else { 4 };

    a + 1
}

fn loop_label() {
    let mut count = 0;

    'exit: loop {
        println!("{}", count);

        loop {
            count += 1;
            if count == 10 {
                break 'exit;
            }
        }
    }
}

fn loop_return() {
    let mut count = 0;

    let result = loop {
        count += 1;
        if count == 10 {
            break count * 2;
        }
    };

    println!("{}", result);
}

fn while_func() {
    let mut i = 3;

    while i > 0 {
        println!("{}", i);
        i -= 1;
    }
}

fn for_func() {
    let a = [1, 2, 3, 4, 5];

    for ele in a {
        println!("{}", ele);
    }

    for i in (1..4).rev() {
        println!("{}", i);
    }
}
