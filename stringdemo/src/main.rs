fn main() {
    // let mut s = String::new();
    let data = "initial contents of the string";
    let s = data.to_string();
    let s1 = "initial".to_string();
    let s = String::from("initial contents of the string");

    // 更新String 字符串
    let mut s = String::from("foo");
    let s1 = String::from("bar");
    s.push_str(&s1);
    println!("{}", s);

    // 更新String 字符
    let mut s = String::from("lo");
    s.push('l');

    // 拼接String
    let s1 = String::from("hello");
    let s2 = String::from("world");
    // 使用了类似这个签名的方法 fn add(self, s: &str) -> String
    // 标准库中add使用了泛型
    // 只能把&str 添加到String
    // 解引用强制转换 （deref coercion）
    let s3 = s1 + &s2;
    println!("{}", s3);
    // println!("{}", s1); s1已被使用，所有权销毁
    println!("{}", s2); // s2被借用，所以所有权还在

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // let s3 = s1 + "-" + &s2 + "-" + &s3;
    println!("{}", s3);

    // format宏不会获得所有权
    let f = format!("{} - {} - {}", s1, s2, s3);
    println!("{}", f);

    // String类型不支持索引方式获取
    let s = String::from("你好").len();
    // Unicode 标量值
    println!("{}", s);

    let hello = String::from("hello");
    // 字节、标量值、字形簇
    // Byte、ScalarValue、GraphemeCluster
    let w = "संस्कृतम्"; // 梵文书写的印度文
    for w in w.bytes() {
        println!("{}", w);
    }

    // 标量获取
    for c in hello.chars() {
        println!("{}", c);
    }

    // 字形簇获取比较麻烦，只能用三方库

    let hello = "hello";
    // 切割String是允许的，但是必须谨慎使用，切割时跨越了字符边界，程序就会panic
    let s = &hello[0..2];
    println!("{}", s);
}
