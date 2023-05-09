use std::collections::HashMap;

fn main() {
    // HashMap用的比较少，不在prelude中
    // 标准库对其支持较少，没有内置宏来创建HashMap
    let mut scores = HashMap::new();
    // 数据存储在Heap上
    // 同构的，一个HashMap中所有K是同一种类型，所有的V也是同一种类型。
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    // zip方法相当于拉链
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    // HashMap所有权
    // 对于实现了Copy trait，所有权都可以被复制
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    // 对于拥有所有权的值，值会被移动，所有权会转移给HashMap
    let mut map = HashMap::new();
    map.insert(&field_name, &field_value);

    println!("{} {}", field_name, field_value);
}
