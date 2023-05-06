use num::complex::Complex;
use std::fmt::Debug;
use std::mem;
use std::mem::size_of_val;
use std::ops::{Range, RangeInclusive};
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

// 要显式处理可能的溢出，可以使用标准库针对原始数字类型提供的这些方法：
// 使用 wrapping_* 方法在所有模式下都按照补码循环溢出规则处理，例如 wrapping_add
// 如果使用 checked_* 方法时发生溢出，则返回 None 值
// 使用 overflowing_* 方法返回该值和一个指示是否存在溢出的布尔值
// 使用 saturating_* 方法使值达到最小值或最大值
fn wrapping() {
    // 假设有一个 u8 ，它可以存放从 0 到 255 的值。那么当你将其修改为范围之外的值，比如 256，则会发生整型溢出。
    let a: u8 = 255;
    // 补位20
    let b = a.wrapping_add(20);
    println!("255 整形溢出, 补位 20 输出 {}", b); // 19
}

fn assert_float_eq() {
    // 这段代码没啥问题吧，实际上它会 panic(程序崩溃，抛出异常)，因为二进制精度问题，导致了 0.1 + 0.2 并不严格等于 0.3，它们可能在小数点 N 位后存在误差。
    // assert!(0.1 + 0.2 == 0.3)
    assert!((0.1_f64 + 0.2 - 0.3).abs() < 0.00001)
}

fn assert_float() {
    let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);

    println!("abc (f32)");
    println!("   0.1 + 0.2: {:x}", (abc.0 + abc.1).to_bits());
    println!("         0.3: {:x}", (abc.2).to_bits());
    println!();

    println!("xyz (f64)");
    println!("   0.1 + 0.2: {:x}", (xyz.0 + xyz.1).to_bits());
    println!("         0.3: {:x}", (xyz.2).to_bits());
    println!();

    // 对 f32 类型做加法时，0.1 + 0.2 的结果是 3e99999a，0.3 也是 3e99999a
    // 因此 f32 下的 0.1 + 0.2 == 0.3 通过测试
    assert!(abc.0 + abc.1 == abc.2);

    // 但是到了 f64 类型时，结果就不一样了，
    // 因为 f64 精度高很多，因此在小数点非常后面发生了一点微小的变化，0.1 + 0.2 以 4 结尾，
    // 但是 0.3 以3结尾，这个细微区别导致 f64 下的测试失败了，并且抛出了异常。
    // assert!(xyz.0 + xyz.1 == xyz.2);
}

fn assert_nan() {
    let x = (-42.0_f32).sqrt();
    // 所有跟 NaN 交互的操作，都会返回一个 NaN，而且 NaN 不能用来比较
    // assert_eq!(x, x);

    // 出于防御性编程的考虑，可以使用 is_nan() 等方法，可以用来判断一个数值是否是 NaN
    if x.is_nan() {
        println!("未定义的数学行为");
    }
}

fn operation_number() {
    // 加法
    let sum = 5 + 10;
    println!("sum :{}", sum);

    // 减法
    let difference = 95.5 - 4.3;
    println!("difference :{}", difference);

    // 乘法
    let product = 4 * 30;
    println!("product :{}", product);

    // 除法
    let quotient = 56.7 / 32.2;
    println!("quotient :{}", quotient);
    // 求余
    let remainder = 43 % 5;
    println!("remainder :{}", remainder);
}

fn print_forty_twos() {
    // 编译器会进行自动推导，给予twenty i32的类型
    let twenty = 20;
    // 类型标注
    let twenty_one: i32 = 21;
    // 通过类型后缀的方式进行类型标注：22是i32类型
    let twenty_two = 22i32;

    // 只有同样类型，才能运算
    let addition = twenty + twenty_one + twenty_two;
    println!(
        "{} + {} + {} = {}",
        twenty, twenty_one, twenty_two, addition
    );

    // 对于较长的数字，可以用_进行分割，提升可读性
    let one_million: i64 = 1_000_000;
    println!("{}", one_million.pow(2));

    // 定义一个f32数组，其中42.0会自动被推导为f32类型
    let forty_twos = [42.0, 42f32, 42.0_f32];

    // 打印数组中第一个值，并控制小数位为2位
    println!("{:.2}", forty_twos[0]);
}

fn bit_opration() {
    // 二进制为00000010
    let a: i32 = 2;
    // 二进制为00000011
    let b: i32 = 3;

    println!("(a & b) value is {}", a & b);

    println!("(a | b) value is {}", a | b);

    println!("(a ^ b) value is {}", a ^ b);

    println!("(!b) value is {} ", !b);

    println!("(a << b) value is {}", a << b);

    println!("(a >> b) value is {}", a >> b);

    let mut a = a;
    // 注意这些计算符除了!之外都可以加上=进行赋值 (因为!=要用来判断不等于)
    a <<= b;
    println!("(a << b) value is {}", a);
}

// 序列操作
fn operation_range() {
    for i in -1..=5 {
        println!("{} ", i)
    }

    for a in 'a'..='z' {
        print!("{} ", a)
    }
}

// 有理数
fn rational_number() {
    let a = Complex { re: 2.1, im: -1.2 };
    let b = Complex::new(11.1, 22.2);
    let result = a + b;

    println!("\n{} + {}i", result.re, result.im);
}

fn practice() {
    let v: u16 = 38_u8 as u16;
    println!("{}", v);

    assert_eq!(i8::MAX, 127_i8);
    assert_eq!(u8::MAX, 255_u8);

    let v1 = 251_u8 + 3;
    let v2 = i8::checked_add(110, 8).unwrap();
    println!("{},{}", v1, v2);

    let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
    println!("{}", v);
    assert!(v == 1597);

    let x = 1_000.000_1; // f64
    let y: f32 = 0.12; // f32
    let z = 0.01_f64; // f64
    println!("{} {} {}", x, y, z);

    assert!(0.1_f32 + 0.2_f32 == 0.3_f32);

    let f = (0.1_f64 + 0.2 - 0.3).abs();
    println!("f:{}", f);
    assert!(f < 0.001);

    let mut sum = 0;
    for i in -3..2 {
        sum += i
    }

    assert!(sum == -5);

    for c in 97..=122 {
        println!("{}", c);
    }

    assert_eq!((1..5), Range { start: 1, end: 5 });
    assert_eq!((1..=5), RangeInclusive::new(1, 5));

    // 整数加法
    assert!(1u32 + 2 == 3);

    // 整数减法
    assert!(1i32 - 2 == -1);
    assert!(1i8 - 2 == -1);

    assert!(3 * 50 == 150);

    let f = (9.6f64 / 3.2 - 3.0).abs();
    assert!(f < 0.0001); // error ! 修改它让代码工作

    assert!(24 % 5 == 4);

    // 逻辑与或非操作
    assert!(true && false == false);
    assert!(true || false == true);
    assert!(!true == false);

    // 位操作
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    let c1 = 'a';
    assert_eq!(size_of_val(&c1), 4);
    let c2 = '中';
    assert_eq!(size_of_val(&c2), 4);

    print_char(c1);

    let _f: bool = false;

    let t = true;
    if t {
        println!("Success!")
    }

    let f = true;
    let t = true || false;
    assert_eq!(t, f);

    let _v: () = ();
    let v = (2, 3);
    assert_eq!(v, (2, 3));

    let unit: () = ();
    assert!(size_of_val(&unit) == 0);

    // 调用一个函数是表达式，因为会返回一个值，调用宏也是表达式，用花括号包裹最终返回一个值的语句块也是表达式，总之，能返回值
    let y = {
        let x = 3;
        // 该语句块是表达式的原因是：它的最后一行是表达式，返回了 x + 1 的值，注意 x + 1 不能以分号结尾，否则就会从表达式变成语句， 表达式不能包含分号。
        x + 1
    };
    println!("The value of y is:{}", y);

    // 表达式如果不返回任何值，会隐式地返回一个 ()
    assert_eq!(ret_unit_type(), ());
}

fn ret_unit_type() {
    let x = 1;
    // if 语句块也是一个表达式，因此可以用于赋值，也可以直接返回
    // 类似三元运算符，在Rust里我们可以这样写
    let _y = if x % 2 == 1 { "odd" } else { "even" };
    // 或者写成一行
    let _z = if x % 2 == 1 { "odd" } else { "even" };
}

fn print_char(c: char) {
    println!("{}", c);
}

// 以下函数可以获取传入参数的类型，并返回类型的字符串形式，例如  "i8", "u8", "i32", "u32"
fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

fn operation_char() {
    let c = 'Z';
    let z = 'z';
    let g = 'G';
    let x = '中';
    let heart_eyed_cat = '😻';

    println!("{} {} {} {}", c, z, g, heart_eyed_cat);
    println!("字符'中'占用了{}字节内存大小", mem::size_of_val(&x));
}

fn operation_bool() {
    let t = true;
    let f = false;
    if f || t {
        println!("这是段毫无意义的代码.");
    }
}

fn add_with_extra(x: i32, y: i32) -> i32 {
    let x = x + 1; // 语句
    let y = y + 5; // 语句
    x + y // 表达式
}

fn plus_or_minus(x: i32) -> i32 {
    if x > 5 {
        return x - 5;
    }
    // x + 5 没有分号，因为它是一条表达式，这个在上一节中我们也有详细介绍
    x + 5
}

/// 下面的 report 函数会隐式返回一个 ()
fn report<T: Debug>(item: T) {
    println!("{:?}", item);
}

/// 下面的函数显式的返回了 ()
fn clear(text: &mut String) -> () {
    *text = String::from("");
}

fn _add(x: u32, y: u32) -> u32 {
    // 只有没有分号的表达式能反回值,加了分号的都是语句.
    // x + y;
    x + y
}

/// 当用 ! 作函数返回类型的时候，表示该函数永不返回( diverge function )，特别的，这种语法往往用做会导致程序崩溃的函数
fn _dead_end() -> ! {
    panic!("你已经到了穷途末路, 崩溃吧!");
}

fn _forever() {
    let mut a = 1;
    loop {
        a += 1;

        if a > 10 {
            break;
        }
    }
}

// for遍历
fn _foreach() {
    let x = [0, 1, 2, 3, 4, 5];
    for i in x.iter() {
        println!("{}", i);
    }
}

fn takes_ownership(some_string: String) {
    // some_string 进入作用域
    println!("{}", some_string);
} // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放

fn makes_copy(some_integer: i32) {
    // some_integer 进入作用域
    println!("{}", some_integer);
} // 这里，some_integer 移出作用域。不会有特殊操作

fn test_clone() {
    let s = String::from("hello"); // s 进入作用域

    takes_ownership(s); // s 的值移动到函数里 ...
                        // ... 所以到这里不再有效

    let x = 5; // x 进入作用域

    makes_copy(x); // x 应该移动函数里，
                   // 但 i32 是 Copy 的，所以在后面可继续使用 x
    println!("{}", x);
} // 这里, x 先移出了作用域，然后是 s。但因为 s 的值已被移走，
  // 所以不会有特殊操作

fn borrowing() {
    let x = 5;
    // 引用x
    let y = &x;

    assert_eq!(5, x);
    // 解引用y
    assert_eq!(5, *y);
}

fn push_insert_replace_str() {
    let mut s = String::from("Hello ");

    s.push_str("rust");
    println!("追加字符串 push_str() -> {}", s);

    s.push('你');
    s.push('好');
    s.push('!');
    println!("追加字符 push() -> {}", s);

    // 指定位置插入
    s.insert(5, '🦀');
    s.insert_str(10, "插入*");
    println!("{}", s);

    // 替换
    let string_replace = String::from("I like rust. Learning rust is my favorite!");
    let new_string_replace = string_replace.replace("rust", "RUST");
    println!("{}", new_string_replace);
    dbg!(new_string_replace);

    // 替换指定数量
    let string_replace = "I like rust. Learning rust is my favorite!";
    let new_string_replacen = string_replace.replacen("rust", "RUST", 1);
    dbg!(new_string_replacen);

    // 替换范围
    let mut string_replace_range = String::from("I like rust!");
    string_replace_range.replace_range(7..8, "R");
    dbg!(string_replace_range);

    // 删除并反回最后一个字符
    let mut string_pop = String::from("rust pop 中文!");
    let p1 = string_pop.pop();
    let p2 = string_pop.pop();
    dbg!(p1);
    dbg!(p2);
    dbg!(string_pop);

    // 删除并返回指定位置字符
    let mut string_remove = String::from("测试remove方法");
    println!(
        "string_remove 占 {} 个字符",
        mem::size_of_val(string_remove.as_str())
    );
    // 删除第一个字符
    string_remove.remove(0);
    // 下面代码会发生错误
    // string_remove.remove(1);
    // 直接删除第二个汉字
    string_remove.remove(3);
    dbg!(string_remove);

    // 清空字符串
    let mut string_clear = String::from("string clear");
    string_clear.clear();
    dbg!(string_clear);

    // 连接
    let string_append = String::from("hello ");
    let string_rust = String::from("rust");
    // &string_rust会自动解引用为&str
    let result = string_append + &string_rust;
    let mut result = result + "!";
    result += "!!!";

    println!("连接字符串 + -> {}", result);

    let s1 = String::from("hello,");
    let s2 = String::from("world!");
    // 在下句中，s1的所有权被转移走了，因此后面不能再使用s1
    let s3 = s1 + &s2;
    assert_eq!(s3, "hello,world!");
    // 下面的语句如果去掉注释，就会报错
    // println!("{}",s1);

    // 同理 + 号连接字符串, s1变量所有权转移到了add()方法里面
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // String = String + &str + &str + &str + &str
    let s = s1 + "-" + &s2 + "-" + &s3;
    dbg!(s);

    let s1 = "hello";
    let s2 = String::from("rust");
    let s = format!("{} {}!", s1, s2);
    dbg!(s);

    // 通过 \ + 字符的十六进制表示，转义输出一个字符
    let byte_escape = "I'm writing \x52\x75\x73\x74!";
    println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

    // \u 可以输出一个 unicode 字符
    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

    println!(
        "Unicode character {} (U+211D) is called {}",
        unicode_codepoint, character_name
    );

    // 换行了也会保持之前的字符串格式
    let long_string = "String literals
                            can span multiple lines.
                            The linebreak and indentation here ->\
                            <- can be escaped too!";
    println!("{}", long_string);

    // 当然，在某些情况下，可能你会希望保持字符串的原样，不要转义
    println!("{}", "hello \\x52\\x75\\x73\\x74");
    let raw_str = r"Escapes don't work here: \x3F \u{211D}";
    println!("{}", raw_str);

    // 如果字符串包含双引号，可以在开头和结尾加 #
    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);

    // 如果还是有歧义，可以继续增加，没有限制
    let longer_delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", longer_delimiter);

    for c in "中国人".chars() {
        println!("{}", c);
    }

    for b in "中国人".as_bytes() {
        dbg!(b);
    }

    // 想要准确的从 UTF-8 字符串中获取子串是较为复杂的事情, 可以考虑utf8_slice库
    let c = utf8_slice::from("中国人", 1);
    dbg!(c);

    let c = utf8_slice::slice("中国人", 0, 1);
    dbg!(c);
}

fn main() {
    greet_world();
    shadowing();
    x_equal_y();
    x_equal_y2();
    plus();
    wrapping();
    assert_float_eq();
    assert_float();
    assert_nan();
    operation_number();
    print_forty_twos();
    bit_opration();
    operation_range();
    rational_number();
    practice();
    operation_char();
    operation_bool();
    report("item");
    test_clone();
    borrowing();
    push_insert_replace_str();

    // 使用尽可能多的方法来通过编译
    let x = &String::from("hello");
    let y = x;
    println!("{},{}", x, y);

    let x = "hello";
    let y = x;
    println!("{},{}", x, y);

    let x = String::from("hello");
    let y = x.clone();
    println!("{},{}", x, y);

    let x = String::from("hello");
    let y = x.as_str();
    println!("{},{}", x, y);

    let mut x = String::from("test");
    clear(&mut x);

    println!("{}", add_with_extra(1, 2));

    let x = plus_or_minus(6);
    println!("The value of x is: {}", x);

    let x: u32 = 5;
    assert_eq!("u32".to_string(), type_of(&x));

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

    // 下一句会有编译报错(consider giving `guess` a type), 变量没有指定类型信息, 编译器无法自动推导类型
    // let guess = "42".parse().expect("Not a number!");
    let msg = "Not a number";
    let guess = "42".parse::<i32>().expect(msg);
    println!("print guest: {}", guess);
    let guess: i32 = "42".parse().expect(msg);
    println!("print guest: {}", guess);

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
