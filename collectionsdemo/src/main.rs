#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let mut v = Vec::new();
    let v1 = vec![1, 2, 3, 4, 5, 6];
    let third = &v1[2];
    println!("The third element is : {}", third);

    match v1.get(2) {
        Some(third) => println!("The third element is : {}", third),
        None => println!("There is no third element."),
    }

    // 无法在同一个作用域内同时使用可变借用和不可变借用。
    // v1.push(100);

    let mut v2 = vec![100, 200, 300];
    for i in &mut v2 {
        *i += 50;
    }
    for i in &v2 {
        println!("{}", i);
    }

    let mut rows = vec![
        SpreadsheetCell::Int(1),
        SpreadsheetCell::Float(2.22),
        SpreadsheetCell::Text(String::from("3.33")),
        SpreadsheetCell::Int(4),
        SpreadsheetCell::Int(5),
    ];

    for row in &mut rows {
        match row {
            SpreadsheetCell::Int(i) => {
                *i += 10;
                println!("{}", i);
            }
            SpreadsheetCell::Float(f) => {
                *f += 0.1;
                println!("{}", f);
            }
            SpreadsheetCell::Text(t) => println!("{}", t),
        }
    }

    // panic, index out of range
    // let v100 = &v1[100];
    let v100 = v1.get(100); // panic
    println!("v100 is : {:?}", v100); // return None;

    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);
}
