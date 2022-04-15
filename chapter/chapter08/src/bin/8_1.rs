fn main() {
    // let mut v = Vec::new();
    // v.push(1);

    let v = vec![1, 2, 3];
    println!("{:?}", v);
    // 当 vector 被丢弃时, 所有其内容也会被丢弃

    // error: cannot borrow `v` as mutable because it is also borrowed as immutable
    // let mut v = vec![1, 2, 3];
    // let first = &v[0];
    // v.push(1);
    // println!("{}", first);

    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![1, 2, 3];

    for i in &mut v {
        // TODO 第十五章
        *i *= 50;
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let _row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
