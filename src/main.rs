use std::{thread, time};

fn greet_world() {
    let chiness = "你好世界!";
    let regions = [chiness];
    for region in regions {
        println!("{}", region);
    }
}

struct Struct {
    e: i32,
}

fn shadowing() {
    let x = 5;
    // 在shadowing函数的作用域内对之前的x进行遮蔽
    let x = x + 1;
    {
        // 在当前的花括号作用域内，对之前的x进行遮蔽
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }
    println!("The value of x is: {}", x);
}

fn x_equal_y() {
    let x: i32 = 2;
    let y: i32 = 2;
    println!("x v:{} is equal to y v:{}", x, y);

    let (x, y) = (1, 2);
    // x += 2;

    assert_eq!(x, 1);
    assert_eq!(y, 2);
}

fn x_equal_y2() {
    let (x, y);
    (x, ..) = (3, 4);
    [.., y] = [1, 2];
    // 填空，让代码工作
    assert_eq!([x, y], [3, 2]);
}

fn self_increment(y: &i32) -> &i32 {
    let mut x = 1;
    x += 2;

    println!("x = {}, y = {}", x, y);
    return &y;
}

fn define_x() -> &'static str {
    let x = "hello";
    x
}

fn plus() {
    let mut x: i32 = 1;
    println!("{}", x);
    x = 7;
    // 遮蔽且再次绑定
    let x = x;
    // x += 3;

    let y = 4;
    println!("x = {}, y = {}", x, y);
    // 遮蔽
    let y = "I can also be bound to text!";
    println!("x = {}, y = {}", x, y)
}

fn main() {
    greet_world();
    shadowing();
    x_equal_y();
    x_equal_y2();
    plus();
    println!("self increment = {}", self_increment(&10));

    println!("{} world.", define_x());

    // Rust 允许声明相同的变量名，在后面声明的变量会遮蔽掉前面声明的，如下所示：
    // 这个程序首先将数值 5 绑定到 x，然后通过重复使用 let x = 来遮蔽之前的 x，并取原来的值加上 1，
    // 所以 x 的值变成了 6。第三个 let 语句同样遮蔽前面的 x，取之前的值并乘上 2，得到的 x 最终值为 12。当运行此程序，将输出以下内容：
    let (a, b, c, d, e);
    (a, b) = (1, 2);
    [c, .., d, _] = [1, 2, 3, 4, 5];
    Struct { e, .. } = Struct { e: 5 };
    assert_eq!([1, 2, 1, 4, 5], [a, b, c, d, e]);

    // 下面是一个常量声明的例子，其常量名为 MAX_POINTS，值设置为 100,000。
    const MAX_POINTS: u32 = 100_000;
    println!("max_points: {:?}", MAX_POINTS);

    let (a, mut b) = (true, false);
    println!("a = {:?}, b = {:?}", a, b);
    b = true;
    assert_eq!(a, b);

    // 字符串类型
    let spaces = "        ";
    // usize数值类型
    let spaces = spaces.len();
    println!("spaces usize: {}", spaces);

    let pengui_data = "\
    common name,length (cm)
    Little penguin,33
    Yellow-eyed penguin,65
    Fiordland penguin,60
    Invalid,data
    ";

    let records = pengui_data.lines();

    for (i, record) in records.enumerate() {
        if i == 0 || record.trim().len() == 0 {
            continue;
        }

        thread::sleep(time::Duration::from_millis(100));

        // 声明一个 fields 变量，类型是 Vec
        // Vec 是 vector 的缩写，是一个可伸缩的集合类型，可以认为是一个动态数组
        // <_>表示 Vec 中的元素类型由编译器自行推断，在很多场景下，都会帮我们省却不少功夫
        let fields: Vec<_> = record.split(',').map(|field| field.trim()).collect();

        if cfg!(debug_assertions) {
            // 输出到标准错误输出
            eprintln!("debug: {:?} -> {:?}", record, fields);
        }

        let name = fields[0];
        // 1. 尝试把 fields[1] 的值转换为 f32 类型的浮点数，如果成功，则把 f32 值赋给 length 变量
        //
        // 2. if let 是一个匹配表达式，用来从=右边的结果中，匹配出 length 的值：
        //   1）当=右边的表达式执行成功，则会返回一个 Ok(f32) 的类型，若失败，则会返回一个 Err(e) 类型，if let 的作用就是仅匹配 Ok 也就是成功的情况，如果是错误，就直接忽略
        //   2）同时 if let 还会做一次解构匹配，通过 Ok(length) 去匹配右边的 Ok(f32)，最终把相应的 f32 值赋给 length
        //
        // 3. 当然你也可以忽略成功的情况，用 if let Err(e) = fields[1].parse::<f32>() {...}匹配出错误，然后打印出来，但是没啥卵用
        if let Ok(length) = fields[1].parse::<f32>() {
            // 输出到标准输出
            println!("{}, {}cm", name, length);
        }
    }
}
