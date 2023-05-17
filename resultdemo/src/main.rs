use std::error::Error;
use std::fs::File;
use std::io::{self, ErrorKind, Read};

// main 函数的返回类型是()
// main 函数的返回类型也可以是: Result<T, E>
// Box<dyn Error> 是trait对象，简单理解：“任何可能的错误类型"
fn main() -> Result<(), Box<dyn Error>> {
    // Result枚举返回两个变体，Ok(T)表示成功，Err(E)表示失败
    // let f = File::open("input.txt");
    // 使用match表达式来串处理这两种变体
    // let _f = match f {
    //     Ok(file) => file,
    //     Err(e) => {
    //         panic!("Error opning file: {}", e)
    //     }
    // };

    // 匹配不同的错误，用了很多match，但是很原始
    // let _f = match f {
    //     Ok(file) => file,
    //     Err(e) => match e.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Error creating file: {}", e),
    //         },
    //         other_error => panic!("Error opening file: {}", other_error),
    //     },
    // };

    // 闭包（closure）。Result有很多方法它们接受闭包作为参数，使用match实现，使用这些代码可以更简洁
    // unwrap: match 表达式的一个快捷方法
    // let _f = File::open("input.txt").unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("input.txt").unwrap_or_else(|error| {
    //             panic!("Error creating file: {}", error);
    //         })
    //     } else {
    //         panic!("Error opening file: {}", error);
    //     }
    // });

    // let _f = File::open("hello.txt").expect("无法打开文件 input.txt");

    // let _result = read_username_from_file();
    let _f = File::open("input.txt")?;
    Ok(())
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
