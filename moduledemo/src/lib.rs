pub mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("add to waitlist");
        }
        pub fn seat_at_table() {
            println!("seat at table");
        }
    }

    pub mod serving {
        pub fn take_order() {
            println!("take order");
        }

        #[warn(dead_code)]
        fn serve_order() {
            println!("serve order");
        }

        #[warn(dead_code)]
        fn take_payment() {
            println!("take payment");
        }
    }
}

fn serve_order() {
    println!("serve order");
}

mod back_of_house {
    fn fix_incorrect_order() {
        println!("fix incorrect order");
        // 相对路径访问函数
        super::serve_order();
        // 绝对路径访问函数
        crate::serve_order();
    }

    pub struct BreakFest {
        // struct 字段需要单独设置pub来变成共有的。
        pub toast: String,
        // struct 字段默认是私有的
        seasonal_fruit: String,
    }

    impl BreakFest {
        pub fn summer(toast: String) -> BreakFest {
            BreakFest {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // 公共枚举里的所有变体都是公共的
    pub enum Appetizer {
        Soup,
        Salad,
    }

    fn cook_order() {
        println!("cook order");
    }
}

pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist();

    front_of_house::serving::take_order();

    let mut meal = back_of_house::BreakFest::summer(String::from("Rye"));
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    // 私有字段无法访问
    // meal.seasonal_fruit = String::from("blueberries");
}
