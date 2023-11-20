enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
    Colorado,
}

fn val_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("{:?}", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        None => None,
    }
}

fn main() {
    let c = val_in_cents(Coin::Quarter(UsState::Alaska));
    println!("{}", c);

    let five = plus_one(Some(5));
    let six = plus_one(five);
    let none = plus_one(None);

    let v = Some(0u8);
    match v {
        Some(0) => println!("zero"),
        _ => {}
    }

    if let Some(0) = v {
        println!("three");
    } else {
        println!("others.");
    }
}
