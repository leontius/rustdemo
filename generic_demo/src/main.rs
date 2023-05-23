use std::fmt::Display;

fn largest<T: PartialOrd + Clone>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for n in list.iter() {
        //-std::cmp::PartialOrd
        if n > &largest {
            largest = n;
        }
    }
    largest
}

struct Point1<T> {
    x: T,
    y: T,
}

impl<T> Point1<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Pair<T> {
        Pair { x, y }
    }
}

/// 在使用泛型类型参数的impl块上使用Trait bound, 我们可以有条件的为实现了特定Trait的类型来实现方法
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

fn main() {
    println!("Hello, world!");

    let str_list = vec![String::from("Hello"), String::from("world")];
    let result = largest(&str_list);
    println!("The largest string is {}", result);

    let integer = Point { x: 1, y: 2 };
    let float = Point { x: 1.0, y: 2.0 };

    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
