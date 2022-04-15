// 在另一个与模块同名的文件中加载模块的内容
mod front_of_house;

pub use crate::front_of_house::hosting;

// 默认都是私有的
mod back_of_house {
    pub struct _Breakfast {
        // 默认还是私有的
        pub toast: String,
        seasonal_fruit: String,
    }

    pub enum _Appetizer {
        // 默认公有
        Soup,
        Salad,
    }

    impl _Breakfast {
        pub fn _summer(toast: &str) -> _Breakfast {
            _Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

// use self::front_of_house;
// use std::io::{self, Write};

// 所有公有项引入当前作用域, 常用于测试
// use std::collections::*;
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        // 更倾向于使用绝对路径
        // 同一模块, 尽管 front_of_house 是私有的, 也可以访问
        crate::front_of_house::hosting::_add_to_waitlist();
        super::front_of_house::hosting::_add_to_waitlist();

        // 对于函数, 指定到父模块
        // 对于结构体, 枚举等指定完整路径 (除非冲突)
        use super::front_of_house::hosting;
        hosting::_add_to_waitlist();

        // use std::fmt::Result;
        // use std::io::Result as IoResult;
    }
}
