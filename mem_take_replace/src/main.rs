fn main() {
    println!("Hello, world!");
    let mut a = MyEnum::A {
        name: "B".to_string(),
        x: 0,
    };
    println!("Before Convert, a is {:?}", a);
    convert_a_to_b(&mut a);
    println!("After Convert, a is {:?}", a);

    convert_a_to_b2(&mut a);
    println!("After Convert, a is {:?}", a);
}

#[derive(Debug)]
enum MyEnum {
    A { name: String, x: u8 },
    B { name: String },
}

fn convert_a_to_b(e: &mut MyEnum) {
    if let MyEnum::A { name, x: 0 } = e {
        *e = MyEnum::B {
            name: std::mem::take(name),
        }
    }
}

fn convert_a_to_b2(e: &mut MyEnum) {
    if let MyEnum::A { name, x: 0 } = e {
        *e = MyEnum::B {
            name: std::mem::replace(name, String::new()),
        }
    }
}

fn convert_a_to_b3(e: &mut MyEnum) {
    if let MyEnum::A { name, x: 0 } = e {
        *e = MyEnum::B { name: name.clone() }
    }
}
