#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // 方法
    fn area(self: Rectangle) -> u32 {
        self.width * self.height
    }

    // 构造函数
    fn square(width: u32) -> Rectangle {
        Rectangle {
            width,
            height: width,
        }
    }
}

fn main() {
    let rect = Rectangle {
        width: 50,
        height: 20,
    };

    println!("{}", rect.area());
    println!("{:#?}", Rectangle::square(100));
}
