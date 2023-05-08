mod front_of_house; // 声明一个模块，但在模块相同的文件名中定义。

// 当前作用域有效条目
// use crate::front_of_house::hosting;

// 将条目引入当前作用域
// 该条目可以被外部引入到它们的作用域
pub use crate::front_of_house::hosting;

fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    // 私有方法无法访问
    // hosting::some_function();
}
