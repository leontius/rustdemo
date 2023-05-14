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

    // 如果将值的引用插入到HashMap，值本身不会移动
    // 在HashMap有效期间，被引用的值必须保持有效
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    match score {
        Some(s) => println!("{} {}", s, team_name),
        None => println!("No team with name {}", team_name),
    }

    // 遍历HaxhMap
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    for (k, v) in &scores {
        println!("{} {}", k, v);
    }

    // 替换现有的V
    // 如果向HashMap插入一对KV，然后再插入同样的K，但是不同的V，那么原来的V也会被替换
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);

    // 只在K不对应任何值的情况下才会插入V
    // entry方法：检查指定的K是否对应一个V
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    // scores.entry(String::from("Yellow")).or_insert(50);
    let e = scores.entry(String::from("Yellow"));
    println!("{:?}", e);
    // entry的or_insert方法：如果K存在，返回到对应的V的一个可变引用
    // 如果K不存在，将方法参数作为K的新值插进去，返回到这个值的可变引用
    e.or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);

    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        // 没有word会插入一个可变引用0
        let count = map.entry(word).or_insert(0);
        // 解引用后可以更改引用的值
        *count += 1;
    }
    // {
    //     "world": 2,
    //     "wonderful": 1,
    //     "hello": 1,
    // }
    println!("{:#?}", map);

    // Hash函数
    // 默认情况下，HashMap使用加密功能强大的Hash函数，可以抵抗拒绝服务DoS攻击。
    // - 不是可用的最快的Hash算法
    // - 但具有更好的安全性
    // 可以指定不同的hasher trait来切换到另一个函数
}
