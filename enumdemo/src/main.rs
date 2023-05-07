#[derive(Debug, Clone)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor { r: u8, g: u8, b: u8 },
}

impl Message {
    fn call(&self) {}
}

fn main() {
    let four = IpAddrKind::V4(127, 0, 0, 1);
    let six = IpAddrKind::V6(String::from("::1"));

    route(four);
    route(six);

    let q = Message::Quit;
    let m = Message::Move { x: 10, y: 20 };
    let w = Message::Write(String::from("hello"));
    let c = Message::ChangeColor {
        r: 10,
        g: 20,
        b: 30,
    };
    q.call();
    m.call();
    w.call();
    c.call();

    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;

    let x = Some(10);
    let y = 5;

    let x = x.unwrap() + y;
    println!("{}", x);
}

fn route(kind: IpAddrKind) {
    println!("{:?}", kind);
}
