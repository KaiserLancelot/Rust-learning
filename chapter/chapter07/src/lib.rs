mod front_of_house {
    // 默认都是私有的
    pub mod hosting {
        pub fn _add_to_waitlist() {}

        fn _seat_at_table() {}
    }

    mod serving {
        fn _take_order() {}

        fn _serve_order() {}

        fn _take_payment() {}
    }
}

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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        // 更倾向于使用绝对路径
        // 同一模块, 尽管 front_of_house 是私有的, 也可以访问
        crate::front_of_house::hosting::_add_to_waitlist();
        super::front_of_house::hosting::_add_to_waitlist();
    }
}
