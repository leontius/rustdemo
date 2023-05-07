#[derive(Debug, Clone)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // 方法
    fn area(self: Rectangle) -> u32 {
        self.width * self.height
    }

    fn can_hold(self: Rectangle, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
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
        width: 150,
        height: 30,
    };

    let rect2 = Rectangle {
        width: 50,
        height: 20,
    };

    println!("{:?}", rect.can_hold(&rect2));

    let rect3 = Rectangle {
        width: 50,
        height: 20,
    };

    println!("{}", rect3.area());

    println!("{:#?}", Rectangle::square(100));
}
