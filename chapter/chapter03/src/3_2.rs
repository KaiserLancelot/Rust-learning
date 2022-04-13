fn main() {
    // 64 位架构上 isize/usize 是 64 位的, 32 位架构上它们是 32 位
    // 可以使用分隔符
    let _size: usize = 166_169;
    // 类型后缀
    let _size = 166_169usize;

    // debug 模式下整型溢出会 panic (程序因为错误而退出)
    // release 模式下进行 two's complement wrapping (e.g. u8: 256 -> 0)

    // 4 字节, Unicode Scalar Value
    let _c = 'a';
    // 1 字节
    let _c = b'a';

    let tuple = (1, 2.6, 'a');
    // 使用模式匹配 (pattern matching) 来解构 (destructure) 元组值
    let (_x, _y, _z) = tuple;
    println!("{}", tuple.1);

    // 没有任何值的元组 () 是一种特殊的类型, 被称为单元类型 (unit type), 而该值被称为单元值 (unit value)
    // 表示不返回值

    let _a = [1, 2, 3];
    // 5 个元素, 每个的值都是 3
    let _a = [3; 5];

    // 数组越界会 panic (index >= 5)
    // println!("{}", _a[index]);
}
