use moduledemo::{eat_at_restaurant, front_of_house::serving};

fn main() {
    println!("Hello, world!");
    serving::take_order();
    eat_at_restaurant();
}
